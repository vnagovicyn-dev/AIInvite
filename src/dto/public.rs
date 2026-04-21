use serde::Serialize;
use serde_json::Value;
use time::format_description::well_known::Rfc3339;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::{events::Event, page_sections::PageSection};

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct PublicEventPageResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub event_type: String,
    pub status: String,
    pub event_date: Option<String>,
    pub timezone: String,
    pub venue_name: Option<String>,
    pub venue_address: Option<String>,
    pub sections: Vec<PublicPageSectionResponse>,
}

impl PublicEventPageResponse {
    pub fn from_parts(event: Event, sections: Vec<PublicPageSectionResponse>) -> Self {
        Self {
            id: event.id,
            title: event.title,
            slug: event.slug,
            event_type: event.event_type,
            status: event.status,
            event_date: event
                .event_date
                .and_then(|value| value.format(&Rfc3339).ok()),
            timezone: event.timezone,
            venue_name: event.venue_name,
            venue_address: event.venue_address,
            sections,
        }
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct PublicPageSectionResponse {
    pub id: Uuid,
    pub section_type: String,
    pub position: i32,
    pub is_enabled: bool,
    pub title: Option<String>,
    pub content: Value,
}

impl From<PageSection> for PublicPageSectionResponse {
    fn from(section: PageSection) -> Self {
        Self {
            id: section.id,
            section_type: section.section_type,
            position: section.position,
            is_enabled: section.is_enabled,
            title: section.title,
            content: section.content,
        }
    }
}
