use tokio;
use tokio::signal;
use axum::{
    http::HeaderMap, 
    middleware::Next, 
    response::Response, 
    extract::Request
};
use sqlx::types::ipnetwork::IpNetwork;

use crate::config::app_config::AppConfig;
use crate::app_error::app_error::AppError;


pub async fn shutdown_signal(config: AppConfig) {
    // Wait for the signal to be received
    let _ = signal::ctrl_c()
        .await
        .map_err(|e| (
            AppError::SignalError(format!("Failed to receive CTRL+C signal: {}", e))
        ));
    println!("Received CTRL+C, shutting down...");
    config.drop_config();
}

pub fn extract_client_info(headers: &HeaderMap) -> Result<(IpNetwork, String), AppError> {
    let client_ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<IpNetwork>().ok())
        .or_else(|| {
            headers
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<IpNetwork>().ok())
        })
        .ok_or_else(|| AppError::ServerError("Client IP not found".to_string()))?;

    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    Ok((client_ip, user_agent))
}

// pub async fn restrict_origin(
//     headers: HeaderMap, 
//     request: Request, 
//     next: Next
// ) -> Result<Response, AppError> {
//     if let Some(origin) = headers.get("origin") {
//         if origin != "http://localhost:3000" {
//             return Err(AppError::ServerError(
//                 "Invalid origin".to_string()
//             ));
//         }
//     } else {
//         return Err(AppError::ServerError(
//             "Missing origin header".to_string()
//         ));
//     }
//     Ok(next.run(request).await)
// }