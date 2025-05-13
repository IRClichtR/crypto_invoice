use tokio;
use tokio::signal;

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
