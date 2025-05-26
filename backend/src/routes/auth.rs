use axum::{
    extract::{State, Query},
    http::{StatusCode, HeaderMap},
    response::Json,
    routing::{get, post},
    Router,
};
use axum_csrf::CsrfToken;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;
use std::sync::Arc;

use crate::app_error::app_error::AppError;
use crate::models::{
    security_events::{EventType, SecurityEvent, record_event},
    users::{User, UserInputUpdate},
    auth_challenges::{AuthChallenge, verify_signature}
};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ChallengeRequestPayload {
    #[validate(length(min = 42, max = 42, message = "Invalid Ethereum address format"))]
    pub ethereum_address: String,
    #[validate(url(message = "Invalid domain format"))]
    pub domain: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VerifySignaturePayload {
    pub challenge_id: Uuid,
    #[validate(length(min = 42, max = 42, message = "Invalid Ethereum address format"))]
    pub ethereum_address: String,
    #[validate(length(min = 132, max = 132, message = "Invalid signature format"))]
    pub signature: String,
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthSuccessResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user: UserInfo
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub ethereum_address: String,
    pub email: Option<String>,
    pub username: Option<String>,
    pub is_verified: bool,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub code: u16,
}

//========================================================================================
// AUTH ROUTES CONFIGURATION
//========================================================================================

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/challenge", post(create_challenge))
        .route("/verify_signature", post(verify_signature))
        .route("refresh_token", post(refresh_token))
        .route("/logout", post(logout))
        .route("/me", get(get_current_user))
}

//========================================================================================
// HANDLER CHALLENGE GENERATION
//========================================================================================

async fn create_challenge(
    State(app_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<ChallengeRequestPayload>,  
) -> Result<Json<ChallengeResponse>, AppError> {
    payload.validate()
        .map_err(|e| AppError::OtherError(format!("Invalid Input: {}", e)))?;

    let challenge = format!("Challenge for {}", payload.ethereum_address);
    Ok(Json(ChallengeResponse { challenge }))
}

#[derive(Debug, Serialize)]
pub struct ChallengeResponse {
    pub challenge: String,
}