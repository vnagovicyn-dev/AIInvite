use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow)]
pub struct Event {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub template_id: Option<Uuid>,
    pub title: String,
    pub slug: String,
    pub event_type: String,
    pub status: String,
    pub event_date: Option<OffsetDateTime>,
    pub timezone: String,
    pub venue_name: Option<String>,
    pub venue_address: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
