use serde_json::Value;
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow)]
pub struct PageSection {
    pub id: Uuid,
    pub event_id: Uuid,
    pub section_type: String,
    pub position: i32,
    pub is_enabled: bool,
    pub title: Option<String>,
    pub content: Value,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
