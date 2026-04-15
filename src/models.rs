use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub database: &'static str,
    pub version: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Healthcheck {
    pub id: i64,
    pub service_name: String,
    pub checked_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateHealthcheck {
    pub service_name: String,
}

#[derive(Deserialize)]
pub struct UpdateHealthcheck {
    pub service_name: String,
}

#[derive(Serialize)]
pub struct RootResponse {
    pub service: &'static str,
    pub routes: Vec<&'static str>,
}

#[derive(Serialize)]
pub struct DeleteResponse {
    pub deleted: bool,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Deserialize)]
pub struct LeadRequestInput {
    pub name: String,
    pub phone: String,
    pub event_date: String,
    pub event_type: String,
    pub comment: String,
}

#[derive(Serialize, FromRow)]
pub struct LeadRequest {
    pub id: i64,
    pub name: String,
    pub phone: String,
    pub event_date: String,
    pub event_type: String,
    pub comment: String,
    pub created_at: DateTime<Utc>,
}
