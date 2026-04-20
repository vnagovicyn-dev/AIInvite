use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::domain::templates::Template;

#[derive(Debug, Deserialize, IntoParams)]
#[serde(default)]
pub struct TemplateListQuery {
    #[param(example = 1, minimum = 1)]
    pub page: u32,
    #[param(example = 20, minimum = 1, maximum = 100)]
    pub per_page: u32,
    #[param(example = "wedding")]
    pub category: Option<String>,
    #[param(example = true)]
    pub is_active: Option<bool>,
}

impl Default for TemplateListQuery {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
            category: None,
            is_active: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct TemplateItemResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub category: String,
    pub preview_url: Option<String>,
    pub is_active: bool,
    pub created_at: String,
}

impl From<Template> for TemplateItemResponse {
    fn from(template: Template) -> Self {
        Self {
            id: template.id,
            code: template.code,
            name: template.name,
            category: template.category,
            preview_url: template.preview_url,
            is_active: template.is_active,
            created_at: template
                .created_at
                .format(&Rfc3339)
                .unwrap_or_else(|_| template.created_at.to_string()),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TemplateListResponse {
    pub items: Vec<TemplateItemResponse>,
    pub page: u32,
    pub per_page: u32,
    pub total: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TemplateCategoryResponse {
    pub category: String,
}
