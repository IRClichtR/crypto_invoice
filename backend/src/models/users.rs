use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, PgPool};
use validator::Validate;
use serde_json::Value as JsonValue;
// use rand::Rng;

use crate::app_error::app_error::AppError;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub ethereum_address: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    is_active: bool,
    is_admin: bool,
    is_verified: bool,
    pub metadata: Option<JsonValue>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInput {
    pub ethereum_address: String,
    pub metadata: JsonValue
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserInputUpdate {
    #[validate(email)]
    pub email: String,
    pub username: String,
    pub is_active: bool,
    pub is_admin: bool,
    pub metadata: Option<JsonValue>

}

impl User {
    pub async fn create(
        pool: &PgPool,
        user_input: &UserInput,
    ) -> Result<User, AppError> {
        let now = Utc::now().naive_utc();

        let metadata = if user_input.metadata.is_null() {
            serde_json::json!({})
        } else {
            user_input.metadata.clone()
        };

        let user= query_as!(
            User,
            r#"
            INSERT INTO users (
                ethereum_address, 
                created_at, 
                updated_at, 
                is_active, 
                is_admin, 
                is_verified, 
                metadata
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, ethereum_address, created_at, updated_at,
                      is_active, is_admin, is_verified, metadata as "metadata: JsonValue"

            "#,
            user_input.ethereum_address,
            now,
            now,
            true, // is_active
            false, // is_admin
            false, // is_verified
            metadata, // metadata

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
            SELECT id, ethereum_address, created_at, updated_at,
                   is_active, is_admin, is_verified, metadata as "metadata: JsonValue"

            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        user.is_active = user_input.is_active;
        user.is_admin = user_input.is_admin;

        // Update metadata if provided
        if let Some(metadata) = &user_input.metadata {
            user.metadata = Some(metadata.clone());
        } else {
            user.metadata = Some(serde_json::json!({}))
        }

        user.updated_at = now;

        query!(
            r#"
            UPDATE users
            SET 
                is_active = $1,
                is_admin = $2,
                updated_at = $3,
                metadata = $4
            WHERE id = $5
            "#,
            user.is_active,
            user.is_admin,
            user.updated_at,
            user.metadata,
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
            SELECT id, ethereum_address, created_at, updated_at,
                   is_active, is_admin, is_verified, metadata as "metadata: JsonValue"
            FROM users
            WHERE ethereum_address = $1
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
            SELECT id, ethereum_address, created_at, updated_at,
                   is_active, is_admin, is_verified, metadata as "metadata: JsonValue"
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