mod config;
mod utils;
mod routes;
mod models;

use tokio;
use axum;

#[tokio::main]
async fn main() {
    let config = config::app_config::AppConfig::new()
        .expect("Failed to load configuration");

    let pool = config::app_config::init_config(config.clone())
        .await
        .expect("Failed to initialize database connection pool");

    let router = routes::create_router(config.clone());

    let addr = format!("{}:{}", config.server.host, config.server.port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind TCP listener");
    println!("Server started. Listening on port {}", config.server.port);

    axum::serve(listener, router)
        .with_graceful_shutdown(async {
            if let Err(e) = utils::server_utils::shutdown_signal(config).await {
                eprintln!("Error during shutdown: {}", e);
            }
        })
        .await
        .expect("Failed to start server");
}