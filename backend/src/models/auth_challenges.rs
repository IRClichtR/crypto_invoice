use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, PgPool};
use validator::Validate;
use rand::Rng;
use sha3::{Keccak256, Digest};
use hex;
use secp256k1::{Message, Secp256k1};
use secp256k1::ecdsa::{RecoverableSignature, RecoveryId};

use crate::app_error::app_error::AppError;

// https://eips.ethereum.org/EIPS/eip-4361

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

    pub async fn find_active_challenge(
        pool: PgPool,
        address: &str,
        challenge_id: Uuid,
    ) -> Result<Option<AuthChallenge>, AppError> {
        let normalized_address = normalize_ethereum_address(address)?;
        let now = Utc::now().naive_utc();

        let challenge = query_as!(
            AuthChallenge,
            r#"
            SELECT id, ethereum_address, nonce, challenge_message, expires_at, used, created_at, domain, chal_timestamp
            FROM auth_challenges
            WHERE ethereum_address = $1
              AND id = $2
              AND used = false
              AND expires_at > $3
            "#,
            normalized_address,
            challenge_id,
            now
        )
        .fetch_optional(&pool)
        .await?;

        Ok(challenge)
    }

    pub async fn mark_as_used(
        pool: &PgPool,
        challenge_id: Uuid,
    ) -> Result<(), AppError> {

        query!(
            r#"
            UPDATE auth_challenges
            SET used = true
            WHERE id = $1
            "#,
            challenge_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn cleanup_expired(
        pool: &PgPool,
    ) -> Result<u64, AppError> {
        let now = Utc::now().naive_utc();

        let result = query!(
            r#"
            DELETE FROM auth_challenges
            WHERE expires_at < $1
            "#,
            now
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

    pub fn is_valid(&self) -> bool {
        let now = Utc::now().naive_utc();
        !self.used && self.expires_at > now
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

pub fn verify_signature(
    signature: &str,
    message: &str,
    expected_address: &str,
) -> Result<bool, AppError> {
    let prefixed_message = format!("\x19Ethereum Signed Message:\n{}", message.len()) + message;
    let message_hash = Keccak256::digest(prefixed_message.as_bytes());

    let signature_bytes = hex::decode(&signature[2..])
        .map_err(|_| AppError::OtherError("Invalid signature format".to_string()))?;

    if signature_bytes.len() != 65 {
        return Err(AppError::OtherError("Invalid Signature".to_string()));
    } 

    let recovery_id = signature_bytes[64];
    let signature_part = &signature_bytes[0..64];

    let recovered_address = recover_address_from_signature(
        &message_hash,
        signature_part,
        recovery_id,
    )?;

    // normalize the recovered address
    let normalized_recovered_address = normalize_ethereum_address(&recovered_address)?;
    let normalized_expected_address = normalize_ethereum_address(expected_address)?;

    // Return true if the addresses match
    Ok(normalized_recovered_address == normalized_expected_address)
}

fn recover_address_from_signature(
    message_hash: &[u8],
    signature: &[u8],
    recovery_id: u8,
) -> Result<String, AppError> {

    let secp = Secp256k1::new();

    // Normalize v: in Ethereum, it might be 27 or 28 → convert to 0 or 1
    let normalized_v = match recovery_id {
        27 | 28 => recovery_id - 27,
        0 | 1 => recovery_id,
        _ => return Err(AppError::OtherError("Invalid recovery ID".to_string())),
    };

    let rec_id = RecoveryId::from_u8_masked(normalized_v);

    let rsig = RecoverableSignature::from_compact(signature, rec_id)
        .map_err(|_| AppError::OtherError("Invalid signature".to_string()))?;

    let msg = Message::from_digest(
        message_hash.try_into()
        .map_err(|_| AppError::OtherError("Invalid message hash length".to_string()))?);

    let pub_key = secp.recover_ecdsa(msg, &rsig)
        .map_err(|_| AppError::OtherError("Failed to recover public key".to_string()))?
        .serialize_uncompressed();

    let hash = Keccak256::digest(&pub_key[1..]);
    let address_bytes = &hash[12..];

    Ok(format!("0x{}", hex::encode(address_bytes)))
}
