use tokio;
use tokio::signal;
use axum::{
    http::HeaderMap, 
    middleware::Next, 
    response::Response, 
    extract::Request
};

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