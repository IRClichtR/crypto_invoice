use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, types::Json, FromRow, PgPool};
use std::collections::HashMap;
use validator::Validate;

use crate::app_error::app_error::AppError;



#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub ethereum_address: String,
    pub email: String,
    pub username: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    is_active: bool,
    is_admin: bool,
    is_verified: bool,
    metadata: Json<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserInput {
    pub ethereum_address: String,
    #[validate(email)]
    pub email: String,
    pub username: String,
    pub metadata: Option<Json<HashMap<String, serde_json::Value>>>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserInputUpdate {
    #[validate(email)]
    pub email: String,
    pub username: String,
    pub is_active: bool,
    pub is_admin: bool,
    pub metadata: Option<Json<HashMap<String, serde_json::Value>>>,
}

#[derive(Debug, FromRow)]
pub struct AuthChallenge {
    pub id: Uuid,
    pub ethereum_address: String,
    pub challenge: String,
    pub expires_at: NaiveDateTime,
    pub used: bool,
    pub created_at: NaiveDateTime,
}

impl User {
    pub async fn create(
        pool: &PgPool,
        user_input: &UserInput,
    ) -> Result<User, AppError> {
        let now = Utc::now().naive_utc();
        let metadata = user_input
            .metadata.clone()
            .unwrap_or(Json(HashMap::new()));

        let user= query_as!(
            User,
            r#"
            INSERT INTO users (
                ethereum_address, 
                email, 
                username, 
                created_at, 
                updated_at, 
                is_active, 
                is_admin, 
                is_verified, 
                metadata
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, ethereum_address, email, username, created_at, updated_at,
                      is_active, is_admin, is_verified, metadata
            "#,
            user_input.ethereum_address,
            user_input.email,
            user_input.username,
            now,
            now,
            true, // is_active
            false, // is_admin
            false, // is_verified
            Json(metadata) as _,
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn update_user(
        pool: &PgPool,
        user_id: Uuid,
        user_input: &UserInputUpdate,
    ) -> Result<User, AppError> {
        let now = Utc::now().naive_utc();

        // Fetch the existing user
        let mut user = query_as!(
            User,
            r#"
            SELECT id, ethereum_address, email, username, created_at, updated_at,
                   is_active, is_admin, is_verified, metadata
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        // Update only the fields that are provided
        if !user_input.email.is_empty() {
            user.email = user_input.email.clone();
        }

        if !user_input.username.is_empty() {
            user.username = user_input.username.clone();
        }

        user.is_active = user_input.is_active;
        user.is_admin = user_input.is_admin;

        if let Some(metadata) = &user_input.metadata {
            user.metadata = metadata.clone();
        }

        user.updated_at = now;

        query!(
            r#"
            UPDATE users
            SET 
                email = $1,
                username = $2,
                is_active = $3,
                is_admin = $4,
                updated_at = $5,
                metadata = $6
            WHERE id = $7
            "#,
            user.email,
            user.username,
            user.is_active,
            user.is_admin,
            user.updated_at,
            user.metadata as _,
            user.id
        )
        .execute(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_eth_address(
        pool: &PgPool,
        address: &str,
    ) -> Result<Option<User>, AppError> {

        let normalized_address = address.to_lowercase();
        let user = query_as!(
            User,
            r#"
            SELECT id, ethereum_address, email, username, created_at, updated_at,
                   is_active, is_admin, is_verified, metadata
            FROM users
            WHERE normalized_address = $1
            "#,
            normalized_address
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_id(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Option<User>, AppError> {
        let user = query_as!(
            User,
            r#"
            SELECT id, ethereum_address, email, username, created_at, updated_at,
                   is_active, is_admin, is_verified, metadata
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }
}

impl AuthChallenge {
    pub async fn create_for_address(
        pool: &PgPool,
        address: &str,
    ) -> Result<AuthChallenge, AppError> {
        let now = Utc::now().naive_utc();
        let expires_at = now + chrono::Duration::minutes(5);

        let challenge = format!(
            "Sign this message to verify ownership of this address {}: {}", 
            address, 
            Uuid::new_v4()
        );

        let auth_challenge = query_as! (
            AuthChallenge,
            r#"
            INSERT INTO auth_challenges {
            id, ethereum_address, challenge, expires_at, used, created_at
            } VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, ethereum_address, challenge, expires_at, used, created_at
            "#,
            Uuid::new_v4(),
            address,
            challenge,
            expires_at,
            false,
            now
        )
        .fetch_one(pool)
        .await?;

        Ok(auth_challenge)
    }

    pub async fn finc_active_for_address(
        pool: &PgPool,
        address: &str,
    ) -> Result<Option<AuthChallenge>, AppError> {
        let now = Utc::now().naive_utc();

        let auth_challenge = query_as!(
            AuthChallenge,
            r#"
            SELECT
                id, ethereum_address, challenge, expires_at, used, created_at
            FROM auth_challenges
            WHERE
                ethereum_address = $1 AND
                expires_at > $2 AND
                used = false
                ORDER BY created_at DESC
            LIMIT 1
            "#,
            address,
            now
        )
        .fetch_optional(pool)
        .await?;

        Ok(auth_challenge)
    }

    pub async fn mark_as_used(
        pool: &PgPool,
        challenge_id: Uuid,
    ) -> Result<(), AppError> {
        let now = Utc::now().naive_utc();

        query!(
            r#"
            UPDATE auth_challenges
            SET used = true, updated_at = $1
            WHERE id = $2
            "#,
            now,
            challenge_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}