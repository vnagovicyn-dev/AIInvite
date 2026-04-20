use sqlx::PgPool;

use crate::common::auth::JwtSettings;

#[derive(Clone)]
pub struct AppState {
    pub service_name: String,
    pub pool: PgPool,
    pub jwt: JwtSettings,
}

impl AppState {
    pub fn new(service_name: impl Into<String>, pool: PgPool, jwt: JwtSettings) -> Self {
        Self {
            service_name: service_name.into(),
            pool,
            jwt,
        }
    }
}
