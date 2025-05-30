use axum::{
    extract::{State, Query},
    http::{StatusCode, HeaderMap},
    response::Json,
    routing::{get, post},
    Router,
};
use axum_csrf::CsrfToken;
use hyper::client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;
use std::sync::Arc;

use crate::AppState;
use crate::app_error::app_error::AppError;
use crate::models::{
    security_events::{EventType, record_event},
    users::{User, UserInput},
    auth_challenges::{AuthChallenge, validate_address}
};
use crate::utils::{
    server_utils::extract_client_info,
    rate_limiter::check_rate_limit,
    jwt::generate_token_pair
};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ChallengeRequest {
    #[validate(length(min = 42, max = 42, message = "Invalid Ethereum address format"))]
    pub ethereum_address: String,
}

#[derive(Debug, Serialize)]
pub struct ChallengeResponse {
    pub challenge_id: Uuid,
    pub message: String,
    pub expires_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    pub challenge_id: Uuid,
    #[validate(length(min = 42, max = 42, message = "Invalid Ethereum address format"))]
    pub ethereum_address: String,
    #[validate(length(min = 132, max = 132, message = "Invalid signature format"))]
    pub signature: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub ethereum_address: String,
    pub is_verified: bool,
    pub is_admin: bool,
}

// #[derive(Debug, Serialize, Deserialize, Validate)]
// pub struct ChallengeRequestPayload {
//     #[validate(length(min = 42, max = 42, message = "Invalid Ethereum address format"))]
//     pub ethereum_address: String,
//     #[validate(url(message = "Invalid domain format"))]
//     pub domain: String,
//     pub challenge_id: Uuid,
// }

// #[derive(Debug, Serialize, Deserialize, Validate)]
// pub struct VerifySignaturePayload {
//     pub challenge_id: Uuid,
//     #[validate(length(min = 42, max = 42, message = "Invalid Ethereum address format"))]
//     pub ethereum_address: String,
//     #[validate(length(min = 132, max = 132, message = "Invalid signature format"))]
//     pub signature: String,
// }

// #[derive(Debug, Serialize)]
// pub struct AuthSuccessResponse {
//     pub access_token: String,
//     pub refresh_token: String,
//     pub expires_in: i64,
//     pub user: UserInfo
// }

// #[derive(Debug, Serialize)]
// pub struct UserInfo {
//     pub id: Uuid,
//     pub ethereum_address: String,
//     pub email: Option<String>,
//     pub username: Option<String>,
//     pub is_verified: bool,
// }

// #[derive(Debug, Serialize)]
// pub struct ErrorResponse {
//     pub error: String,
//     pub message: String,
//     pub code: u16,
// }

//========================================================================================
// AUTH ROUTES CONFIGURATION
//========================================================================================

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/challenge", post(create_challenge))
        .route("/login", post(login))
        // .route("refresh_token", post(refresh_token))
        // .route("/logout", post(logout))
        // .route("/me", get(get_current_user))
        // .route("/admin", get(get_admin_info))
}

//========================================================================================
// HANDLER CHALLENGE GENERATION
//========================================================================================
async fn create_challenge(
    State(app_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<ChallengeRequest>,
) -> Result<Json<ChallengeResponse>, AppError> {
    // validate payload
    payload.validate()
        .map_err(|e| AppError::OtherError(format!("Invalid Input: {}", e)))?;

    // extract client IP and user agent from headers
    let (client_ip, user_agent) = extract_client_info(&headers)?;

    // check if request exceeds rate limit
    check_rate_limit(
        &app_state.pool, 
        &client_ip, 
        "challenge generation", 
        3, 
        60
    )
    .await?;

    // spawn a task to clean up expired challenges
    tokio::spawn({
        let pool = app_state.pool.clone();
        async move {
            let _ = AuthChallenge::cleanup_expired(&pool).await;
        }
    });

    let domain = &app_state.config.server.domain.clone();
    
    let challenge = AuthChallenge::create_challenge_for_addr(
        &app_state.pool,
        &payload.ethereum_address,
        domain.as_str()
    ).await?;

    // record event for audit
    let _ = record_event(
        &app_state.pool,
        EventType::ChallengeCreated,
        None,
        client_ip,
        user_agent.as_str(),
        json!({
            "ethereum_address": payload.ethereum_address,
            "challenge_id": challenge.id,
        }),
    ).await?;

    Ok(Json(ChallengeResponse {
        challenge_id: challenge.id,
        message: challenge.challenge_message,
        expires_at: challenge.expires_at,
    }))
}

//========================================================================================
// HANDLER LOGIN
//========================================================================================

async fn login(
    State(app_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<LoginRequest>,  
) -> Result<Json<LoginResponse>, AppError> {
    // check if paylkoad has correct format
    payload.validate()
        .map_err(|e| AppError::OtherError(format!("Invalid Input: {}", e)))?;

    // extract client IP and user agent from headers 
    let (client_ip, user_agent) = extract_client_info(&headers)?;

    // check if request exceeds rate limit 
    check_rate_limit(
        &app_state.pool, 
        &client_ip, 
        "signature verification", 
        3,
        60
    ).await?;

    // spawn a task to clean up expired challenges
    tokio::spawn({
        let pool = app_state.pool.clone();
        async move {
            let _ = AuthChallenge::cleanup_expired(&pool).await;
        }
    });

    // find active challenge for user 
    let challenge = AuthChallenge::find_active_challenge(
        app_state.pool,
        &payload.ethereum_address.as_str(),
        payload.challenge_id,   
    )
    .await?
    .ok_or_else(|| AppError::OtherError("No active challenge found".to_string()))?;

    // validate the Ethereum address signature
    let is_valid = validate_address(
        &payload.signature,
        &challenge.challenge_message,
        &payload.ethereum_address,
    )?;

    if !is_valid {
        let _ = record_event(
            &app_state.pool,
            EventType::FailedLogin,
            None,
            client_ip,
            user_agent.as_str(),
            json!({
                "ethereum_address": payload.ethereum_address,
                "challenge_id": payload.challenge_id,
            }),
        ).await?;

        return Err(AppError::OtherError("Invalid signature".to_string()));
    }

    // mark the challenge as used
    AuthChallenge::mark_as_used(&app_state.pool, challenge.id).await?;
    // record used challenge event
    let _ = record_event(
        &app_state.pool,
        EventType::ChallengeUsed,
        None,
        client_ip,
        user_agent.as_str(),
        json!({
            "ethereum_address": payload.ethereum_address,
            "challenge_id": challenge.id,
        }),
    ).await?;

    // find user or create a new one
    let user = match User::get_user_by_eth_address(
        &app_state.pool, 
        &payload.ethereum_address,  
    ).await? {
        Some(user) => user,
        None => {
            let user_input = UserInput {
                ethereum_address: payload.ethereum_address.clone(),
                metadata: json!({}),
            };
            User::create(&app_state.pool, &user_input)
                .await?
        }
    };

    let (access_token, refresh_token) = generate_token_pair(
        user.id, 
        &payload.ethereum_address, 
        user.is_admin, 
        &app_state.config.auth.jwt_secret,)
        .map_err(|e| AppError::OtherError(format!("Failed to generate tokens: {}", e)))?;

    // record successful login event
    let _ = record_event(
        &app_state.pool,
        EventType::Login,
        Some(user.id),
        client_ip,
        user_agent.as_str(),
        json!({
            "ethereum_address": payload.ethereum_address,
            "user_id": user.id,
        }),
    ).await?;

    Ok(Json(LoginResponse {
        access_token,
        refresh_token,
        expires_in: app_state.config.auth.token_expires_in as i64,
        user: UserInfo {
            id: user.id,
            ethereum_address: user.ethereum_address,
            is_verified: user.is_verified,
            is_admin: user.is_admin,
        },
    })) 
}