use std::env;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("APP_PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(8080);
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set. Copy .env.example to .env if needed.");

        Self {
            host,
            port,
            database_url,
        }
    }
}
