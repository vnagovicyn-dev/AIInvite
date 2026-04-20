use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub app_name: String,
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        let app_name = env::var("APP_NAME").unwrap_or_else(|_| "invite-platform".to_string());
        let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("APP_PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(8080);
        let database_url = env::var("DATABASE_URL")?;

        Ok(Self {
            app_name,
            database_url,
            host,
            port,
        })
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
