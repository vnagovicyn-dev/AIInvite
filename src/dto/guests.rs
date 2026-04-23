use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{domain::guests::Guest, dto::events::PatchValue};

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateGuestRequest {
    pub full_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub group_name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub plus_one_allowed: Option<bool>,
    pub is_child: Option<bool>,
    pub vip: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateGuestRequest {
    pub full_name: Option<String>,
    #[serde(default)]
    #[schema(value_type = Option<String>)]
    pub phone: PatchValue<String>,
    #[serde(default)]
    #[schema(value_type = Option<String>)]
    pub email: PatchValue<String>,
    #[serde(default)]
    #[schema(value_type = Option<String>)]
    pub group_name: PatchValue<String>,
    pub tags: Option<Vec<String>>,
    pub plus_one_allowed: Option<bool>,
    pub is_child: Option<bool>,
    pub vip: Option<bool>,
    #[serde(default)]
    #[schema(value_type = Option<String>)]
    pub notes: PatchValue<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(default)]
pub struct GuestListQuery {
    #[param(example = 1, minimum = 1)]
    pub page: u32,
    #[param(example = 20, minimum = 1, maximum = 100)]
    pub per_page: u32,
    #[param(example = "Anna")]
    pub search: Option<String>,
    #[param(example = true)]
    pub vip: Option<bool>,
    #[param(example = "Family")]
    pub group_name: Option<String>,
}

impl Default for GuestListQuery {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
            search: None,
            vip: None,
            group_name: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct GuestResponse {
    pub id: Uuid,
    pub event_id: Uuid,
    pub invite_token: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub group_name: Option<String>,
    pub tags: Vec<String>,
    pub plus_one_allowed: bool,
    pub is_child: bool,
    pub vip: bool,
    pub notes: Option<String>,
    pub created_at: String,
}

impl From<Guest> for GuestResponse {
    fn from(guest: Guest) -> Self {
        Self {
            id: guest.id,
            event_id: guest.event_id,
            invite_token: guest.invite_token,
            full_name: guest.full_name,
            phone: guest.phone,
            email: guest.email,
            group_name: guest.group_name,
            tags: guest.tags,
            plus_one_allowed: guest.plus_one_allowed,
            is_child: guest.is_child,
            vip: guest.vip,
            notes: guest.notes,
            created_at: guest
                .created_at
                .format(&Rfc3339)
                .unwrap_or_else(|_| guest.created_at.to_string()),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GuestListResponse {
    pub items: Vec<GuestResponse>,
    pub page: u32,
    pub per_page: u32,
    pub total: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GuestImportResponse {
    pub imported_count: usize,
    pub skipped_count: usize,
    pub errors: Vec<String>,
}
