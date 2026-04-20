use std::env;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("missing environment variable: {0}")]
    MissingEnv(#[from] env::VarError),
    #[error("invalid JWT_EXPIRES_IN_MINUTES value")]
    InvalidJwtExpiry(#[source] std::num::ParseIntError),
}

#[derive(Clone, Debug)]
pub struct Config {
    pub app_name: String,
    pub database_url: String,
    pub host: String,
    pub jwt_expires_in_minutes: u64,
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let app_name = env::var("APP_NAME").unwrap_or_else(|_| "invite-platform".to_string());
        let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("APP_PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(8080);
        let database_url = env::var("DATABASE_URL")?;
        let jwt_expires_in_minutes = env::var("JWT_EXPIRES_IN_MINUTES")?
            .parse::<u64>()
            .map_err(ConfigError::InvalidJwtExpiry)?;
        let jwt_secret = env::var("JWT_SECRET")?;

        Ok(Self {
            app_name,
            database_url,
            host,
            jwt_expires_in_minutes,
            jwt_secret,
            port,
        })
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
