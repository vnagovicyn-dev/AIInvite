use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::error::AppError,
    dto::{
        events::PatchValue,
        page_sections::{
            CreatePageSectionRequest, PageSectionListResponse, PageSectionResponse,
            ReorderPageSectionsRequest, UpdatePageSectionRequest,
        },
    },
    repos::{events_repo, page_sections_repo},
};

const ALLOWED_SECTION_TYPES: [&str; 10] = [
    "hero",
    "story",
    "program",
    "location",
    "dress_code",
    "faq",
    "gift",
    "gallery",
    "video",
    "rsvp",
];

pub async fn create(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    payload: CreatePageSectionRequest,
) -> Result<PageSectionResponse, AppError> {
    ensure_event_owned(pool, owner_id, event_id).await?;
    let next_position = page_sections_repo::next_position_for_event(pool, event_id).await?;
    let new_section = normalize_create_request(event_id, next_position, payload)?;

    page_sections_repo::create(pool, &new_section)
        .await
        .map(PageSectionResponse::from)
        .map_err(map_write_error)
}

pub async fn list(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
) -> Result<PageSectionListResponse, AppError> {
    ensure_event_owned(pool, owner_id, event_id).await?;
    let items = page_sections_repo::list_by_event(pool, event_id)
        .await?
        .into_iter()
        .map(PageSectionResponse::from)
        .collect::<Vec<PageSectionResponse>>();

    Ok(PageSectionListResponse { items })
}

pub async fn get_by_id(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    id: Uuid,
) -> Result<Option<PageSectionResponse>, AppError> {
    let section = page_sections_repo::find_by_id_for_owner(pool, owner_id, event_id, id).await?;
    Ok(section.map(PageSectionResponse::from))
}

pub async fn update(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    id: Uuid,
    payload: UpdatePageSectionRequest,
) -> Result<Option<PageSectionResponse>, AppError> {
    let changes = normalize_update_request(payload)?;

    page_sections_repo::update_for_owner(pool, owner_id, event_id, id, &changes)
        .await
        .map(|section| section.map(PageSectionResponse::from))
        .map_err(map_write_error)
}

pub async fn reorder(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    payload: ReorderPageSectionsRequest,
) -> Result<PageSectionListResponse, AppError> {
    ensure_event_owned(pool, owner_id, event_id).await?;

    if payload.section_ids.is_empty() {
        return Err(AppError::bad_request("section_ids must not be empty"));
    }

    let existing_ids = page_sections_repo::list_ids_for_event(pool, event_id).await?;
    if existing_ids.len() != payload.section_ids.len() {
        return Err(AppError::bad_request(
            "section_ids must include all event sections exactly once",
        ));
    }

    let mut expected = existing_ids;
    let mut provided = payload.section_ids.clone();
    expected.sort_unstable();
    provided.sort_unstable();
    if expected != provided {
        return Err(AppError::bad_request(
            "section_ids must include all event sections exactly once",
        ));
    }

    page_sections_repo::reorder_for_owner(pool, owner_id, event_id, &payload.section_ids).await?;
    list(pool, owner_id, event_id).await
}

pub async fn delete(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    id: Uuid,
) -> Result<bool, AppError> {
    page_sections_repo::delete_for_owner(pool, owner_id, event_id, id)
        .await
        .map_err(AppError::from)
}

fn normalize_create_request(
    event_id: Uuid,
    position: i32,
    payload: CreatePageSectionRequest,
) -> Result<page_sections_repo::NewPageSection, AppError> {
    let section_type =
        normalize_required_string(&payload.section_type, "section_type is required")?;
    validate_section_type(&section_type)?;

    Ok(page_sections_repo::NewPageSection {
        event_id,
        section_type,
        position,
        is_enabled: payload.is_enabled.unwrap_or(true),
        title: normalize_optional_string(payload.title),
        content: payload.content,
    })
}

fn normalize_update_request(
    payload: UpdatePageSectionRequest,
) -> Result<page_sections_repo::UpdatePageSectionChanges, AppError> {
    let title = match payload.title {
        PatchValue::Missing => None,
        PatchValue::Null => Some(None),
        PatchValue::Value(value) => Some(normalize_optional_string(Some(value))),
    };
    let content = match payload.content {
        PatchValue::Missing => None,
        PatchValue::Null => Some(empty_content()),
        PatchValue::Value(value) => Some(value),
    };

    if payload.is_enabled.is_none() && title.is_none() && content.is_none() {
        return Err(AppError::bad_request("no fields to update"));
    }

    Ok(page_sections_repo::UpdatePageSectionChanges {
        is_enabled: payload.is_enabled,
        title,
        content,
    })
}

async fn ensure_event_owned(pool: &PgPool, owner_id: Uuid, event_id: Uuid) -> Result<(), AppError> {
    let event = events_repo::find_by_id_and_owner(pool, owner_id, event_id).await?;
    if event.is_some() {
        Ok(())
    } else {
        Err(AppError::not_found("event not found"))
    }
}

fn normalize_required_string(value: &str, message: &str) -> Result<String, AppError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(AppError::bad_request(message))
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalize_optional_string(value: Option<String>) -> Option<String> {
    value.and_then(|item| {
        let trimmed = item.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    })
}

fn validate_section_type(value: &str) -> Result<(), AppError> {
    if ALLOWED_SECTION_TYPES.contains(&value) {
        Ok(())
    } else {
        Err(AppError::bad_request("section_type is invalid"))
    }
}

fn map_write_error(error: sqlx::Error) -> AppError {
    if let sqlx::Error::Database(database_error) = &error {
        match database_error.code().as_deref() {
            Some("23505") => return AppError::conflict("position already exists"),
            Some("23503") => return AppError::bad_request("event_id is invalid"),
            _ => {}
        }
    }

    error.into()
}

fn empty_content() -> Value {
    Value::Object(Default::default())
}
