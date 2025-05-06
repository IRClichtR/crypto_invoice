mod config;
mod utils;
mod routes;
mod models;
mod app_error;

use tokio;
use axum;
use axum::{Router, routing::get};
use crate::app_error::app_error::AppError;


#[tokio::main]
async fn main() {
    let config = config::app_config::AppConfig::new()
        .expect("Failed to load configuration");

    let pool = config::app_config::init_config(config.clone())
        .await
        .map_err(|e| {
            AppError::DatabaseError(format!("Failed to initialize database: {}", e))
        })
        .expect("Failed to initialize database");

    // let router = routes::create_router(config.clone());
    let route = Router::<()>::new()
        .route("/", axum::routing::get(|| async { "Hello, World!" }));

    let addr = format!("{}:{}", config.server.host, config.server.port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind TCP listener");
    println!("Listening on port {}", config.server.port);

    axum::serve(listener, route)
        .await
        .expect("Failed to start server");
    // Drop the database pool
    pool.close().await;

}