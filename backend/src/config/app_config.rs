use config:: {Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
    pub max_connections: u32,
    pub timeout: u64,
    
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub host: String,
    pub port: u16,
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
pub struct AppConfig {
    pub database: Database,
    pub server: Server,
    pub ethereum: Ethereum,
    pub auth: Auth,
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
    let config = AppConfig::new().expect("Failed to load configuration");
    let db_url = &config.database.url;
    let max_connections = config.database.max_connections;

    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(db_url)
}