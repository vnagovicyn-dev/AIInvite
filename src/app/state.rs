use sqlx::PgPool;

use crate::common::auth::JwtSettings;

#[derive(Clone)]
pub struct AppState {
    pub frontend_origins: Vec<String>,
    pub service_name: String,
    pub pool: PgPool,
    pub jwt: JwtSettings,
}

impl AppState {
    pub fn new(service_name: impl Into<String>, pool: PgPool, jwt: JwtSettings) -> Self {
        Self {
            frontend_origins: vec![
                "http://127.0.0.1:3000".to_string(),
                "http://127.0.0.1:3001".to_string(),
            ],
            service_name: service_name.into(),
            pool,
            jwt,
        }
    }

    pub fn with_frontend_origins(
        service_name: impl Into<String>,
        frontend_origins: Vec<String>,
        pool: PgPool,
        jwt: JwtSettings,
    ) -> Self {
        Self {
            frontend_origins,
            service_name: service_name.into(),
            pool,
            jwt,
        }
    }
}
