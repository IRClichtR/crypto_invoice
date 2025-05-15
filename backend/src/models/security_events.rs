use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, types::Json, FromRow, PgPool, Type};
use std::collections::HashMap;

use crate::app_error::app_error::AppError;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Type, PartialEq)]
#[sqlx(type_name = "event_type", rename_all = "lowercase")]
pub enum EventType {
    Login,
    FailedLogin,
    WalletConnected,
    WalletDisconnected,
    AccountLocked,
    AccountUnlocked
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_type: EventType,
    pub timestamp: NaiveDateTime,
    pub client_ip: String,
    pub user_agent: String,
    pub metadata: Option<Json<HashMap<String, serde_json::Value>>>,
}

pub async fn record_event(
    pool: &PgPool,
    event_type: EventType,
    user_id: Uuid,
    client_ip: &str,
    user_agent: &str,
    metadata: Option<Json<HashMap<String, serde_json::Value>>>,
) -> Result<(), AppError> {
    let now = Utc::now().naive_utc();
    let metadata = metadata.unwrap_or_default();
    let event_type_str = serde_json::to_string(&event_type)
        .map_err(|e| AppError::OtherError(format!("Failed to serialize event type: {}", e)))?;

    let query = sqlx::query!(
        r#"
        INSERT INTO security_events (
            id, event_type, user_id, timestamp, client_ip, user_agent, metadata
            )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        Uuid::new_v4(),
        event_type as EventType,
        user_id,
        now,
        client_ip,
        user_agent,
        metadata,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_events_for_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<SecurityEvent>, AppError> {
    let events = sqlx::query_as!(
        SecurityEvent,
        r#"
        SELECT * FROM security_events WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(events)
}

pub async fn get_events_by_type(
    pool: &PgPool,
    event_type: EventType,
) -> Result<Vec<SecurityEvent>, AppError> {
    let events = sqlx::query_as!(
        SecurityEvent,
        r#"
        SELECT * FROM security_events WHERE event_type = $1
        "#,
        event_type as EventType
    )
    .fetch_all(pool)
    .await?;

    Ok(events)
}

pub async fn get_all_events(
    pool: &PgPool,
) -> Result<Vec<SecurityEvent>, AppError> {
    let events = sqlx::query_as!(
        SecurityEvent,
        r#"
        SELECT * FROM security_events
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(events)
}

pub async fn add_token_to_blacklist(
    pool: &PgPool,
    user_id: Uuid,
    jti: &str,
    issued_at: NaiveDateTime,
    expires_at: NaiveDateTime,
    reason: &str,
) -> Result<(), AppError> {
    let now = Utc::now().naive_utc();

    query!(
        r#"
        INSERT INTO token_blacklist (
            id, user_id, jti, issued_at, expires_at, reason
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        user_id,
        jti,
        now,
        expires_at,
        now,
        reason
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn is_blacklisted(
    pool: &PgPool,
    jti: &str,
) -> Result<bool, AppError> {
    let now = Utc::now().naive_utc();

    let blacklisted = query!(
        r#"
        SELECT EXISTS (
            SELECT 1 FROM token_blacklist WHERE jti = $1
        ) as "exists!"
        "#,
        jti
    )
    .fetch_one(pool)
    .await?;

    Ok(blacklisted)
}
