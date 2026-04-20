use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub service_name: String,
    pub pool: PgPool,
}

impl AppState {
    pub fn new(service_name: impl Into<String>, pool: PgPool) -> Self {
        Self {
            service_name: service_name.into(),
            pool,
        }
    }
}
