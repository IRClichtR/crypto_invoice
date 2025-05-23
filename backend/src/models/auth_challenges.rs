use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow, PgPool};
use validator::Validate;
use rand::Rng;

use crate::app_error::app_error::AppError;


#[derive(Debug, FromRow)]
pub struct AuthChallenge {
    pub id: Uuid,
    pub ethereum_address: String,
    pub nonce: String,
    pub challenge_message: String,
    pub expires_at: NaiveDateTime,
    pub used: bool,
    pub created_at: NaiveDateTime,
    pub domain: String,
    pub chal_timestamp: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ChallengeRequest {
    #[validate(length(min = 42, max = 42))]
    pub ethereum_address: String,
}

pub struct ChallengeResponse {
    pub challenge_id: Uuid,
    pub message: String,
    pub expires_at: NaiveDateTime,
}

impl AuthChallenge {
    pub async fn create_challenge_for_addr(
        pool: &PgPool,
        address: &str,
        domain: &str,
    ) -> Result<AuthChallenge, AppError> {
        let now = Utc::now().naive_utc();
        let expires_at = now + chrono::Duration::minutes(5);

        let nonce = nonce_gen();

        let normalized_address = normalize_ethereum_address(address)?;
        let challenge_message = create_siwe_message(
            &normalized_address,
            domain,
            &nonce,
            &now
        );

        let auth_challenge = query_as!(
            AuthChallenge,
            r#"
            INSERT INTO auth_challenges (
                id,
                ethereum_address,
                nonce,
                challenge_message,
                expires_at,
                used,
                domain,
                chal_timestamp
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, ethereum_address, nonce, challenge_message, expires_at, used, created_at, domain, chal_timestamp
            "#,
            Uuid::new_v4(),
            normalized_address,
            nonce,
            challenge_message,
            expires_at,
            false,
            domain,
            now,
        )
        .fetch_one(pool)
        .await?;

        Ok(auth_challenge)
    }
}

fn nonce_gen() -> String {
    let mut rng = rand::rng();
    let bytes: [u8; 16] = rng.random();
    hex::encode(bytes)
}

fn normalize_ethereum_address(address: &str) -> Result<String, AppError> {
    let address = address.trim();

    if !address.starts_with("0x") 
        || address.len() != 42 
        || !address.chars().skip(2).all(|c| c.is_ascii_hexdigit()) {
        return Err(AppError::OtherError(
            format!("Invalid address: {}", address)
        ));
    }
    Ok(address.to_lowercase())
}

fn create_siwe_message(
    address: &str,
    domain: &str,
    nonce: &str,
    timestamp: &NaiveDateTime,
) -> String {
    format!(
        "Sign this message to verify ownership of this address {}: {}. This is a one-time nonce: {}. Timestamp: {}",
        address,
        domain,
        nonce,
        timestamp.format("%Y-%m-%d %H:%M:%S")
    )
}
