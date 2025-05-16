use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, types::{ipnetwork::IpNetwork, Json}, FromRow, PgPool, Type};
use std::collections::HashMap;

use crate::app_error::app_error::AppError;

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[sqlx(type_name = "event_type", rename_all = "lowercase")]
pub enum EventType {
    Login,
    FailedLogin,
    WalletConnected,
    WalletDisconnected,
    AccountLocked,
    AccountUnlocked
}

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct SecurityEvent {
    pub id: Uuid,
    pub user_id: Uuid,
    #[sqlx(rename = "event_type")]
    pub event_type: EventType,
    pub timestamp: NaiveDateTime,
    pub client_ip: IpNetwork,
    pub user_agent: String,
    pub metadata: Option<Json<HashMap<String, serde_json::Value>>>,
}

pub async fn record_event(
    pool: &PgPool,
    event_type: EventType,
    user_id: Uuid,
    client_ip: IpNetwork,
    user_agent: &str,
    metadata: Option<Json<HashMap<String, serde_json::Value>>>,
) -> Result<(), AppError> {
    let now = Utc::now().naive_utc();
    let metadata = metadata.unwrap_or_default();

    let _query = sqlx::query!(
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
        serde_json::to_value(metadata.0)
            .map_err(|e| AppError::OtherError(format!("Failed to serialize metadata: {}", e)))?,
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
        SELECT id, user_id, event_type as "event_type: String", client_ip, user_agent, metadata, timestamp
        FROM security_events 
        WHERE user_id = $1
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
    let query = "SELECT id, user_id, event_type, client_ip, user_agent, metadata, timestamp 
                 FROM security_events 
                 WHERE event_type = $1::event_type";
    
    let events = sqlx::query_as::<_, SecurityEvent>(query)
        .bind(event_type)  // Bind directement l'enum
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
        SELECT 
            id,
            user_id,
            event_type as "event_type!: EventType",
            timestamp,
            client_ip as "client_ip?: PgInet",
            user_agent,
            metadata as "metadata: JsonValue"
        FROM security_events
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(events)
}

// pub async fn get_all_events(
//     pool: &PgPool,
// ) -> Result<Vec<SecurityEvent>, AppError> {
//     let events = sqlx::query_as!(
//         SecurityEvent,
//         r#"
//         SELECT 
//             id, 
//             timestamp, 
//             event_type as "event_type: EventType", 
//             user_id
//         FROM security_events
//         "#,
//     )
//     .fetch_all(pool)
//     .await?;

//     Ok(events)
// }
// pub async fn get_all_events(
//     pool: &PgPool,
// ) -> Result<Vec<SecurityEvent>, AppError> {
//     let events = sqlx::query_as!(
//         SecurityEvent,
//         r#"
//         SELECT * FROM security_events
//         "#,
//     )
//     .fetch_all(pool)
//     .await?;

//     Ok(events)
// }

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
            id, user_id, jti, expires_at, issued_at, blacklisted_at, reason
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        user_id,
        jti,
        expires_at,
        now,
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

    Ok(blacklisted.exists)
}
