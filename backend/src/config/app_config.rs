use config:: {Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::env;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;
use crate::app_error::app_error::AppError; // Ensure app_error.rs exists and is correctly defined

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
    pub max_connections: u32,
    pub timeout: u64,
}

impl Database {
    pub fn validate_db(&self) -> Result<(), AppError> {
        if self.url.is_empty() {
            return Err(AppError::DatabaseError("Database URL is empty".to_string()));
        }
        if self.max_connections == 0 {
            return Err(AppError::DatabaseError("Max connections must be greater than 0".to_string()));
        }
        if self.timeout == 0 {
            return Err(AppError::DatabaseError("Timeout must be greater than 0".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub domain: String,
}

impl Server {
    pub fn validate_server(&self) -> Result<(), AppError> {
        if self.host.is_empty() {
            return Err(AppError::ServerError("Server host is empty".to_string()));
        }
        if self.port == 0 {
            return Err(AppError::ServerError("Server port must be greater than 0".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ethereum {
    pub rpc_url: String,
    pub private_key: Option<String>,
    pub contract_address: String,
    pub chain_id: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Auth {
    pub jwt_secret: String,
    pub token_expires_in: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FrontendConfig {
    pub api_url: String,
    pub dev_server_port : u16,
    pub assets_path: String,
    pub debug: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database: Database,
    pub server: Server,
    pub ethereum: Ethereum,
    pub auth: Auth,
    pub frontend: FrontendConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let env = env::var("RUN_ENV")
            .unwrap_or_else(|_| "development".to_string());

        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", env)))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        config.try_deserialize::<AppConfig>()
            .map_err(|e| ConfigError::Message(format!("Failed to deserialize config: {}", e)))
    }

    pub fn drop_config(&self) {
        println!("Dropping config...");
        // Placeholder for database pool cleanup logic
    }
}

pub async fn init_config(config: AppConfig) -> Result<PgPool, sqlx::Error> {
    let db_url = &config.database.url;
    let max_connections = config.database.max_connections;

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(config.database.timeout))
        .idle_timeout(Duration::from_secs(config.database.timeout))
        .connect(db_url)
        .await?;

    // Test
    _ = sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            AppError::DatabaseError(format!("Failed to connect to database: {}", e))
        });

    Ok(pool)
}

#[derive(Serialize)]
pub struct SerializableFrontendConfig {
    pub csrf_token: String,
    pub api_url: String,
    pub dev_server_port: u16,
    pub assets_path: String,
    pub debug: bool
}

pub fn get_serializable_frontend_config(
    config: &FrontendConfig,
    csrf_token: String,
) -> SerializableFrontendConfig {
    SerializableFrontendConfig {
        csrf_token,
        api_url: config.api_url.clone(),
        dev_server_port: config.dev_server_port,
        assets_path: config.assets_path.clone(),
        debug: config.debug,
    }
}