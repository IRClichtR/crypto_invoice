mod config;
mod utils;
mod routes;
mod models;
mod app_error;

use tokio;
use axum;
use axum::Router;
use crate::app_error::app_error::AppError;


#[tokio::main]
async fn main() {
    let config = config::app_config::AppConfig::new()
        .expect("Failed to load configuration");

    let pool = config::app_config::init_config(config.clone())
        .await
        .expect("Failed to initialize database connection pool");

    // let router = routes::create_router(config.clone());

    let addr = format!("{}:{}", config.server.host, config.server.port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind TCP listener");
    println!("Server started. Listening on port {}", config.server.port);
}