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
   
    encode(&header, claims, &encoding_key)
        .map_err(|e| AppError::ServerError(format!("Failed to validate access token: {}", e)))?;
}

pub fn validate_access_token(token: &str, secret: &str) -> Result<JwtClaims, AppError> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<JwtClaims>(token, &decoding_key, &validation)
        .map_err(|e| AppError::ServerError(format!("Failed to validate access token: {}", e)))?;

    if token_data.claims.tokentype != "access" {
        return Err(AppError::ServerError("Invalid token type".to_string()));
    }
    if token_data.claims.exp < Utc::now().naive_utc() {
        return Err(AppError::ServerError("Access token has expired".to_string()));
    }

    Ok(token_data.claims)
}

pub fn validate_refresh_token(token: &str, secret: &str) -> Result<JwtClaims, AppError> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<JwtClaims>(token, &decoding_key, &validation)
        .map_err(|e| AppError::ServerError(format!("Failed to validate refresh token: {}", e)))?;

    if token_data.claims.tokentype != "refresh" {
        return Err(AppError::ServerError("Invalid token type".to_string()));
    }
    if token_data.claims.exp < Utc::now().naive_utc() {
        return Err(AppError::ServerError("Refresh token has expired".to_string()));
    }
    
    Ok(token_data.claims)
}