mod config;
mod utils;
mod routes;
mod models;
mod app_error;

use axum::{
    Router,
    routing::get
};
use axum_csrf::{
    CsrfConfig, CsrfLayer, Key
};
use hyper::header;
use tower_cookies::CookieManagerLayer;
use tokio;
use tower_http::{services::ServeDir, cors::{CorsLayer, Any}};
use hyper::http::{Method, HeaderName, HeaderValue};
use std::{sync::Arc, path::Path};
use crate::app_error::app_error::AppError;
// Removed incomplete use statement

#[derive(Clone)]
pub struct AppState {
    pub vue_dist_path: String,
    pub config: config::app_config::AppConfig,
    pub pool: sqlx::PgPool,
}

pub struct AppCsrfConfig {
    pub csrf_key: Key,
    pub csrf_config: CsrfConfig,
}

impl AppCsrfConfig {
    pub fn new() -> Self {
        let csrf_key = Key::generate();
        let csrf_config = CsrfConfig::new()
        .with_key(Some(csrf_key.clone()))
        .with_cookie_path("/".to_string())
        .with_http_only(true)    
        .with_secure(true)       
        .with_cookie_same_site(axum_csrf::SameSite::Strict) 
        .with_secure(Some("csrf_token".to_string()).is_some())
        .with_cookie_name(&"_csrf".to_string());

        AppCsrfConfig { csrf_key, csrf_config }
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Load env
    dotenv::dotenv()
        .map_err(|e| AppError::ConfigError(format!("Failed to load .env file: {}", e)))?;

    //Set up csrf
    let csrf_config = AppCsrfConfig::new();

    // define the path to the Vue.js dist directory
    let vue_dist_path = std::env::var("VUE_DIST_PATH")
        .unwrap_or_else(|_| {
            Path::new("dist").to_string_lossy().to_string()
        });


    // Set up configuration
    let config = config::app_config::AppConfig::new()
        .expect("Failed to load configuration");

    // Create pool for postgres
    let pool = config::app_config::init_config(config.clone())
        .await
        .map_err(|e| {
            AppError::DatabaseError(format!("Failed to initialize database: {}", e))
        })
        .expect("Failed to initialize database");

    // Create application state
    let app_state = Arc::new(AppState {
        vue_dist_path: vue_dist_path.clone(),
        config: config.clone(),
        pool: pool.clone(),
    });

    // configure CORS
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>()
            .map_err(|e| {
                AppError::ServerError(format!("Failed to parse CORS origin: {}", e))
            })?)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
            HeaderName::from_static("x-csrf-token"),
        ])
        .allow_credentials(true);

    // Create the router
    let app = Router::new()
        .route("/", get(routes::home::serve_home))
        // other routes to be added here
        .nest_service(
            "/assets", ServeDir::new(format!("{}/assets", vue_dist_path))
        )
        .layer(CookieManagerLayer::new())
        .layer(CsrfLayer::new(csrf_config.csrf_config.clone()))
        .layer(
            tower_http::set_header::SetResponseHeaderLayer::if_not_present(
                header::X_CONTENT_TYPE_OPTIONS,
                header::HeaderValue::from_static("nosniff"),
            )
        )
        .layer(cors)
        .with_state(app_state);

    let addr = format!("{}:{}", config.server.host, config.server.port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind TCP listener");
    println!("Listening on port {}", config.server.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(
            utils::server_utils::shutdown_signal(config.clone())
        )
        .await
        .expect("Failed to start server");

    pool.close().await;

    Ok(())
}