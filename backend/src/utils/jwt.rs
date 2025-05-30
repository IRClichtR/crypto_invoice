use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{Utc, NaiveDateTime};
use crate::app_error::app_error::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub sub: Uuid, // Subject (user ID)
    pub iat: NaiveDateTime, // Issued at time
    pub exp: NaiveDateTime, // Expiration time
    pub jti: String, // JWT ID
    pub token_type: String, // Type of token (e.g., "access", "refresh")
    pub eth_address: String, // Ethereum address
    pub is_admin: bool, // Admin status
}

pub fn generate_jwt_token(claims: &JwtClaims, secret: &str) -> Result<String, AppError> {
    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());
   
    let token = encode(&header, claims, &encoding_key)
        .map_err(|e| AppError::ServerError(format!("Failed to validate access token: {}", e)))?;

    Ok(token)
}

pub fn generate_token_pair(
    user_id: Uuid,
    eth_address: &str,
    is_admin: bool,
    secret: &str,
) -> Result<(String, String), AppError> {
    let now = Utc::now().naive_utc();

    let access_exp = now + chrono::Duration::minutes(15);
    let refresh_exp = now + chrono::Duration::days(30);

    let access_claims = JwtClaims {
        sub: user_id,
        iat: now,
        exp: access_exp,
        jti: Uuid::new_v4().to_string(),
        token_type: "access".to_string(),
        eth_address: eth_address.to_string(),
        is_admin,
    };

    let refresh_claims = JwtClaims {
        sub: user_id,
        iat: now,
        exp: refresh_exp,
        jti: Uuid::new_v4().to_string(),
        token_type: "refresh".to_string(),
        eth_address: eth_address.to_string(),
        is_admin,
    };

    let access_token = generate_jwt_token(&access_claims, secret)?;
    let refresh_token = generate_jwt_token(&refresh_claims, secret)?;

    Ok((access_token, refresh_token))
}

pub fn validate_access_token(token: &str, secret: &str) -> Result<JwtClaims, AppError> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<JwtClaims>(token, &decoding_key, &validation)
        .map_err(|e| {
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    AppError::ServerError("Access token has expired".to_string())
                }
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    AppError::ServerError("Invalid access token".to_string())
                }
                _ => AppError::ServerError(format!("Failed to validate access token: {}", e))
            }
        })?;

    if token_data.claims.token_type != "access" {
        return Err(AppError::ServerError("Invalid token type".to_string()));
    }

    Ok(token_data.claims)
}

pub fn validate_refresh_token(token: &str, secret: &str) -> Result<JwtClaims, AppError> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<JwtClaims>(token, &decoding_key, &validation)
        .map_err(|e| {
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    AppError::ServerError("Refresh token has expired".to_string())
                }
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    AppError::ServerError("Invalid refresh token".to_string())
                }
                _ => AppError::ServerError(format!("Failed to validate refresh token: {}", e))
            }
        })?;

    if token_data.claims.token_type != "refresh" {
        return Err(AppError::ServerError("Invalid token type".to_string()));
    }

    Ok(token_data.claims)
}