use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::format_description::well_known::Rfc3339;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{domain::page_sections::PageSection, dto::events::PatchValue};

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePageSectionRequest {
    pub section_type: String,
    pub position: i32,
    pub is_enabled: Option<bool>,
    pub title: Option<String>,
    pub content: Option<Value>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePageSectionRequest {
    pub section_type: Option<String>,
    pub position: Option<i32>,
    pub is_enabled: Option<bool>,
    #[serde(default)]
    #[schema(value_type = Option<String>)]
    pub title: PatchValue<String>,
    #[serde(default)]
    #[schema(value_type = Object)]
    pub content: PatchValue<Value>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct PageSectionResponse {
    pub id: Uuid,
    pub event_id: Uuid,
    pub section_type: String,
    pub position: i32,
    pub is_enabled: bool,
    pub title: Option<String>,
    pub content: Value,
    pub created_at: String,
    pub updated_at: String,
}

impl From<PageSection> for PageSectionResponse {
    fn from(section: PageSection) -> Self {
        Self {
            id: section.id,
            event_id: section.event_id,
            section_type: section.section_type,
            position: section.position,
            is_enabled: section.is_enabled,
            title: section.title,
            content: section.content,
            created_at: section
                .created_at
                .format(&Rfc3339)
                .unwrap_or_else(|_| section.created_at.to_string()),
            updated_at: section
                .updated_at
                .format(&Rfc3339)
                .unwrap_or_else(|_| section.updated_at.to_string()),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PageSectionListResponse {
    pub items: Vec<PageSectionResponse>,
}
