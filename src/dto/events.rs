use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::domain::events::Event;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEventRequest {
    pub title: String,
    pub slug: Option<String>,
    pub event_type: String,
    pub template_id: Option<Uuid>,
    pub event_date: Option<String>,
    pub timezone: Option<String>,
    pub venue_name: Option<String>,
    pub venue_address: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateEventRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub event_type: Option<String>,
    #[serde(default)]
    #[schema(value_type = Option<Uuid>)]
    pub template_id: PatchValue<Uuid>,
    #[serde(default)]
    #[schema(value_type = Option<String>)]
    pub event_date: PatchValue<String>,
    pub timezone: Option<String>,
    #[serde(default)]
    #[schema(value_type = Option<String>)]
    pub venue_name: PatchValue<String>,
    #[serde(default)]
    #[schema(value_type = Option<String>)]
    pub venue_address: PatchValue<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(default)]
pub struct EventListQuery {
    #[param(example = 1, minimum = 1)]
    pub page: u32,
    #[param(example = 20, minimum = 1, maximum = 100)]
    pub per_page: u32,
    #[param(example = "draft")]
    pub status: Option<String>,
}

impl Default for EventListQuery {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
            status: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct EventResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub event_type: String,
    pub status: String,
    pub template_id: Option<Uuid>,
    pub event_date: Option<String>,
    pub timezone: String,
    pub venue_name: Option<String>,
    pub venue_address: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Event> for EventResponse {
    fn from(event: Event) -> Self {
        Self {
            id: event.id,
            title: event.title,
            slug: event.slug,
            event_type: event.event_type,
            status: event.status,
            template_id: event.template_id,
            event_date: event
                .event_date
                .and_then(|value| value.format(&Rfc3339).ok()),
            timezone: event.timezone,
            venue_name: event.venue_name,
            venue_address: event.venue_address,
            created_at: event
                .created_at
                .format(&Rfc3339)
                .unwrap_or_else(|_| event.created_at.to_string()),
            updated_at: event
                .updated_at
                .format(&Rfc3339)
                .unwrap_or_else(|_| event.updated_at.to_string()),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EventListResponse {
    pub items: Vec<EventResponse>,
    pub page: u32,
    pub per_page: u32,
    pub total: i64,
}

#[derive(Debug, Clone, Default)]
pub enum PatchValue<T> {
    #[default]
    Missing,
    Null,
    Value(T),
}

impl<T> PatchValue<T> {
    pub fn is_missing(&self) -> bool {
        matches!(self, Self::Missing)
    }
}

impl<'de, T> Deserialize<'de> for PatchValue<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Option::<T>::deserialize(deserializer)?;
        Ok(match value {
            Some(inner) => Self::Value(inner),
            None => Self::Null,
        })
    }
}
