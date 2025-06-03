use sqlx::{PgPool, query};
use chrono::{NaiveDateTime, Utc, Duration};
use sqlx::{types::ipnetwork::IpNetwork, FromRow};
use uuid::Uuid;
use crate::app_error::app_error::AppError;
use crate::utils::server_utils::extract_client_info;

#[derive(FromRow)]
struct RateLimitEntry {
    id: Uuid,
    identifier: String,
    action_type: String,
    attempt_count: i32,
    window_start: NaiveDateTime,
    last_attempt: NaiveDateTime,
}


// check rate limit for a specific action type and client IP
pub async fn check_rate_limit(
    pool: &PgPool,
    client_ip: &IpNetwork,
    action_type: &str,
    max_attempts: i32,
    window_seconds: i64,
) -> Result<(), AppError> {
    let now = Utc::now().naive_utc();
    let window_start = now - Duration::seconds(window_seconds);
    let identifier = format!("{}:{}", client_ip.network(), action_type);


    /// clean up old rate limit entries
    let _ = query!(
        "DELETE FROM rate_limits WHERE window_start < $1",
        now - Duration::hours(24)
    )
    .execute(pool)
    .await;

    /// get existing rate limit entry
    let mut entry = match get_rate_limit_entry(pool, &identifier, action_type).await?
    {
        Some(existing) =>  {
            if existing.window_start < window_start {
                reset_rate_limit_entry(pool, &identifier, action_type, now).await?
            } else {
                existing
            }
        },
        None => {
            create_rate_limit_entry(pool, &identifier, action_type, now).await?
        }
    };

    // Check if the entry is within the rate limit window
    // if not return an error
    if entry.attempt_count >= max_attempts {
        return Err(AppError::ServerError(format!(
            "Rate limit exceeded for {}: {} attempts in the last {} seconds",
            identifier, entry.attempt_count, window_seconds
        )));
    }

    // If the entry is within the rate limit window, increment the attempt count
    increment_rate_limit_entry(pool, &identifier, action_type, now).await?;

    Ok(())
}

//==================== Rate Limit Functions ====================

// Create a new rate limit entry
async fn create_rate_limit_entry(
    pool: &PgPool,
    identifier: &str,
    action_type: &str,
    now: chrono::NaiveDateTime,
) -> Result<RateLimitEntry, AppError> {
    let id = uuid::Uuid::new_v4();
    
    query!(
        r#"
        INSERT INTO rate_limits (id, identifier, action_type, attempts_count, window_start, last_attempt)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        id,
        identifier,
        action_type,
        1,
        now,
        now
    )
    .execute(pool)
    .await?;
    
    Ok(RateLimitEntry {
        id,
        identifier: identifier.to_string(),
        action_type: action_type.to_string(),
        attempt_count: 1,
        window_start: now,
        last_attempt: now,
    })
}

// Retrieve an existing rate limit entry
async fn get_rate_limit_entry(
    pool: &PgPool,
    identifier: &str,
    action_type: &str,
) -> Result<Option<RateLimitEntry>, AppError> {
    let entry = query!(
        r#"
        SELECT id, identifier, action_type, attempts_count, window_start, last_attempt
        FROM rate_limits
        WHERE identifier = $1 AND action_type = $2
        "#,
        identifier,
        action_type
    )
    .fetch_optional(pool)
    .await?;
    
    match entry {
        Some(row) => Ok(Some(RateLimitEntry {
            id: row.id,
            identifier: row.identifier,
            action_type: row.action_type,
            attempt_count: row.attempts_count,
            window_start: row.window_start,
            last_attempt: row.last_attempt,
        })),
        None => Ok(None),
    }
}

// Reset an existing rate limit entry
async fn reset_rate_limit_entry(
    pool: &PgPool,
    identifier: &str,
    action_type: &str,
    now: chrono::NaiveDateTime,
) -> Result<RateLimitEntry, AppError> {
    query!(
        r#"
        UPDATE rate_limits
        SET attempts_count = 1, window_start = $3, last_attempt = $4
        WHERE identifier = $1 AND action_type = $2
        "#,
        identifier,
        action_type,
        now,
        now
    )
    .execute(pool)
    .await?;
    Ok(RateLimitEntry {
        id: Uuid::new_v4(),
        identifier: identifier.to_string(),
        action_type: action_type.to_string(),
        attempt_count: 1,
        window_start: now,
        last_attempt: now,
    })
}

// Increment the attempt count for an existing rate limit entry
async fn increment_rate_limit_entry(
    pool: &PgPool,
    identifier: &str,
    action_type: &str,
    now: chrono::NaiveDateTime,
) -> Result<(), AppError> {
    query!(
        r#"
        UPDATE rate_limits
        SET attempts_count = attempts_count + 1, last_attempt = $3
        WHERE identifier = $1 AND action_type = $2
        "#,
        identifier,
        action_type,
        now
    )
    .execute(pool)
    .await?;
    
    Ok(())
}