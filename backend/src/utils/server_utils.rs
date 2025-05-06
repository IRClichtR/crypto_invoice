use tokio;
use tokio::signal;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::config::app_config::AppConfig;

// use crate::error::app_error::ServerError;

pub async fn shutdown_signal(config: AppConfig) {
    // Wait for the signal to be received
    signal::ctrl_c()
        .await
        .map_err(|e| {
            // ServerError::SignalError(format!("Failed to set up signal handler: {}", e))
            println!("Failed to set up signal handler: {}", e);
        });
    println!("Received CTRL+C, shutting down...");
    config.drop_config();
}
