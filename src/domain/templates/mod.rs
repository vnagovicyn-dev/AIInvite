use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow)]
pub struct Template {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub category: String,
    pub preview_url: Option<String>,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
}
