use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow)]
pub struct Guest {
    pub id: Uuid,
    pub event_id: Uuid,
    pub full_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub group_name: Option<String>,
    pub tags: Vec<String>,
    pub plus_one_allowed: bool,
    pub is_child: bool,
    pub vip: bool,
    pub notes: Option<String>,
    pub created_at: OffsetDateTime,
}
