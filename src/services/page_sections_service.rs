use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::error::AppError,
    dto::{
        events::PatchValue,
        page_sections::{
            CreatePageSectionRequest, PageSectionListResponse, PageSectionResponse,
            UpdatePageSectionRequest,
        },
    },
    repos::{events_repo, page_sections_repo},
};

pub async fn create(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    payload: CreatePageSectionRequest,
) -> Result<PageSectionResponse, AppError> {
    ensure_event_owned(pool, owner_id, event_id).await?;
    let new_section = normalize_create_request(event_id, payload)?;

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
    payload: CreatePageSectionRequest,
) -> Result<page_sections_repo::NewPageSection, AppError> {
    let section_type =
        normalize_required_string(&payload.section_type, "section_type is required")?;
    let position = normalize_position(payload.position)?;

    Ok(page_sections_repo::NewPageSection {
        event_id,
        section_type,
        position,
        is_enabled: payload.is_enabled.unwrap_or(true),
        title: normalize_optional_string(payload.title),
        content: payload.content.unwrap_or_else(empty_content),
    })
}

fn normalize_update_request(
    payload: UpdatePageSectionRequest,
) -> Result<page_sections_repo::UpdatePageSectionChanges, AppError> {
    let section_type = payload
        .section_type
        .map(|value| normalize_required_string(&value, "section_type is invalid"))
        .transpose()?;
    let position = payload.position.map(normalize_position).transpose()?;
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

    if section_type.is_none()
        && position.is_none()
        && payload.is_enabled.is_none()
        && title.is_none()
        && content.is_none()
    {
        return Err(AppError::bad_request("no fields to update"));
    }

    Ok(page_sections_repo::UpdatePageSectionChanges {
        section_type,
        position,
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

fn normalize_position(value: i32) -> Result<i32, AppError> {
    if value <= 0 {
        Err(AppError::bad_request("position must be greater than 0"))
    } else {
        Ok(value)
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
