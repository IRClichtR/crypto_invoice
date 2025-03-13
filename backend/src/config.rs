use std::env;
use dotenv::dotenv;

// Config struct to store the configuration values
pub struct Config {
    pub jwt_secret: String,
    pub jwt_expires_in: u64,
    pub port: u16,
}

// Implement the Config struct
impl Config {
    // store the configuration values in the Config struct
    pub fn init() -> <Self, std::env::VarError> {
        dotenv().ok();

        Self {
            jwt_secret: env::var("JWT_SECRET") // get the JWT_SECRET from the .env file
                .unwrap_or_else(|_| "your_jwt_secret_key".into()),
            jwt_expires_in: env::var("JWT_EXPIRES_IN") // get the JWT_EXPIRES_IN from the .env file
                .unwrap_or_else(|_| "60".into())
                .parse::<u64>()
                .unwrap_or_else(|_| 60),
            port: env::var("PORT") // get the PORT from the .env file
                .unwrap_or_else(|_| "3000", into())
                .parse<u16>()
                .unwrap_or_else(|_| 3000),
        }
    }
}