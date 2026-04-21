use serde_json::Value;
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow)]
pub struct RsvpForm {
    pub id: Uuid,
    pub event_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub deadline_at: Option<OffsetDateTime>,
    pub settings: Value,
}

#[derive(Clone, Debug, FromRow)]
pub struct RsvpQuestion {
    pub id: Uuid,
    pub form_id: Uuid,
    pub position: i32,
    pub code: String,
    pub label: String,
    pub question_type: String,
    pub required: bool,
    pub options: Value,
}

#[derive(Clone, Debug, FromRow)]
pub struct RsvpResponse {
    pub id: Uuid,
    pub event_id: Uuid,
    pub guest_id: Option<Uuid>,
    pub public_token: Option<String>,
    pub status: String,
    pub plus_one_count: i32,
    pub answers: Value,
    pub submitted_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}
