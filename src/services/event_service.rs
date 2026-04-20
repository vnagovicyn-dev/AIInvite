use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    common::{error::AppError, slug},
    dto::events::{
        CreateEventRequest, EventListQuery, EventListResponse, EventResponse, PatchValue,
        UpdateEventRequest,
    },
    repos::events_repo,
};

const DEFAULT_EVENT_STATUS: &str = "draft";
const DEFAULT_TIMEZONE: &str = "Europe/Amsterdam";
const GENERATED_SLUG_ATTEMPTS: usize = 5;

pub async fn create(
    pool: &PgPool,
    owner_id: Uuid,
    payload: CreateEventRequest,
) -> Result<EventResponse, AppError> {
    let normalized = normalize_create_request(payload)?;

    if let Some(slug) = normalized.slug.clone() {
        let new_event = events_repo::NewEvent {
            owner_id,
            title: normalized.title,
            slug,
            event_type: normalized.event_type,
            status: DEFAULT_EVENT_STATUS.to_string(),
            template_id: normalized.template_id,
            event_date: normalized.event_date,
            timezone: normalized.timezone,
            venue_name: normalized.venue_name,
            venue_address: normalized.venue_address,
        };

        return events_repo::create(pool, &new_event)
            .await
            .map(EventResponse::from)
            .map_err(map_write_error);
    }

    for _ in 0..GENERATED_SLUG_ATTEMPTS {
        let new_event = events_repo::NewEvent {
            owner_id,
            title: normalized.title.clone(),
            slug: slug::generate_slug(&normalized.title),
            event_type: normalized.event_type.clone(),
            status: DEFAULT_EVENT_STATUS.to_string(),
            template_id: normalized.template_id,
            event_date: normalized.event_date,
            timezone: normalized.timezone.clone(),
            venue_name: normalized.venue_name.clone(),
            venue_address: normalized.venue_address.clone(),
        };

        match events_repo::create(pool, &new_event).await {
            Ok(event) => return Ok(EventResponse::from(event)),
            Err(error) if is_unique_violation(&error) => continue,
            Err(error) => return Err(map_write_error(error)),
        }
    }

    Err(AppError::conflict("failed to generate unique slug"))
}

pub async fn list(
    pool: &PgPool,
    owner_id: Uuid,
    query: EventListQuery,
) -> Result<EventListResponse, AppError> {
    let normalized_query = normalize_list_query(query);
    let items = events_repo::list_by_owner(pool, owner_id, &normalized_query)
        .await?
        .into_iter()
        .map(EventResponse::from)
        .collect::<Vec<EventResponse>>();
    let total = events_repo::count_by_owner(pool, owner_id, &normalized_query).await?;

    Ok(EventListResponse {
        items,
        page: normalized_query.page,
        per_page: normalized_query.per_page,
        total,
    })
}

pub async fn get_by_id(
    pool: &PgPool,
    owner_id: Uuid,
    id: Uuid,
) -> Result<Option<EventResponse>, AppError> {
    let event = events_repo::find_by_id_and_owner(pool, owner_id, id).await?;
    Ok(event.map(EventResponse::from))
}

pub async fn update(
    pool: &PgPool,
    owner_id: Uuid,
    id: Uuid,
    payload: UpdateEventRequest,
) -> Result<Option<EventResponse>, AppError> {
    let changes = normalize_update_request(payload)?;

    let updated_event = events_repo::update_by_owner(pool, owner_id, id, &changes)
        .await
        .map_err(map_write_error)?;

    Ok(updated_event.map(EventResponse::from))
}

pub async fn delete(pool: &PgPool, owner_id: Uuid, id: Uuid) -> Result<bool, AppError> {
    events_repo::delete_by_owner(pool, owner_id, id)
        .await
        .map_err(AppError::from)
}

fn normalize_create_request(
    payload: CreateEventRequest,
) -> Result<NormalizedCreateEvent, AppError> {
    let title = normalize_required_string(&payload.title, "title is required")?;
    let slug = match payload.slug {
        Some(value) => Some(
            slug::normalize_slug(&value).ok_or_else(|| AppError::bad_request("slug is invalid"))?,
        ),
        None => None,
    };
    let event_type = normalize_required_string(&payload.event_type, "event_type is required")?;
    let timezone =
        normalize_optional_string(payload.timezone).unwrap_or_else(|| DEFAULT_TIMEZONE.to_string());

    if timezone.is_empty() {
        return Err(AppError::bad_request("timezone is invalid"));
    }

    Ok(NormalizedCreateEvent {
        title,
        slug,
        event_type,
        template_id: payload.template_id,
        event_date: parse_optional_datetime(payload.event_date)?,
        timezone,
        venue_name: normalize_nullable_string(payload.venue_name),
        venue_address: normalize_nullable_string(payload.venue_address),
    })
}

fn normalize_update_request(
    payload: UpdateEventRequest,
) -> Result<events_repo::UpdateEventChanges, AppError> {
    let title = payload
        .title
        .map(|value| normalize_required_string(&value, "title is invalid"))
        .transpose()?;
    let slug = payload
        .slug
        .map(|value| {
            slug::normalize_slug(&value).ok_or_else(|| AppError::bad_request("slug is invalid"))
        })
        .transpose()?;
    let event_type = payload
        .event_type
        .map(|value| normalize_required_string(&value, "event_type is invalid"))
        .transpose()?;
    let timezone = payload
        .timezone
        .map(|value| normalize_required_string(&value, "timezone is invalid"))
        .transpose()?;
    let template_id = match payload.template_id {
        PatchValue::Missing => None,
        PatchValue::Null => Some(None),
        PatchValue::Value(value) => Some(Some(value)),
    };
    let event_date = parse_patch_datetime(payload.event_date)?;
    let venue_name = match payload.venue_name {
        PatchValue::Missing => None,
        PatchValue::Null => Some(None),
        PatchValue::Value(value) => Some(normalize_nullable_string(Some(value))),
    };
    let venue_address = match payload.venue_address {
        PatchValue::Missing => None,
        PatchValue::Null => Some(None),
        PatchValue::Value(value) => Some(normalize_nullable_string(Some(value))),
    };

    if title.is_none()
        && slug.is_none()
        && event_type.is_none()
        && template_id.is_none()
        && event_date.is_none()
        && timezone.is_none()
        && venue_name.is_none()
        && venue_address.is_none()
    {
        return Err(AppError::bad_request("no fields to update"));
    }

    Ok(events_repo::UpdateEventChanges {
        title,
        slug,
        event_type,
        template_id,
        event_date,
        timezone,
        venue_name,
        venue_address,
    })
}

fn normalize_list_query(mut query: EventListQuery) -> EventListQuery {
    if query.page == 0 {
        query.page = 1;
    }

    if query.per_page == 0 {
        query.per_page = 20;
    } else if query.per_page > 100 {
        query.per_page = 100;
    }

    query.status = query
        .status
        .and_then(|value| normalize_optional_string(Some(value)));
    query
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

fn normalize_nullable_string(value: Option<String>) -> Option<String> {
    normalize_optional_string(value)
}

fn parse_optional_datetime(value: Option<String>) -> Result<Option<OffsetDateTime>, AppError> {
    value.map(|item| parse_datetime(&item)).transpose()
}

fn parse_patch_datetime(
    value: PatchValue<String>,
) -> Result<Option<Option<OffsetDateTime>>, AppError> {
    match value {
        PatchValue::Missing => Ok(None),
        PatchValue::Null => Ok(Some(None)),
        PatchValue::Value(value) => Ok(Some(Some(parse_datetime(&value)?))),
    }
}

fn parse_datetime(value: &str) -> Result<OffsetDateTime, AppError> {
    OffsetDateTime::parse(value.trim(), &time::format_description::well_known::Rfc3339)
        .map_err(|_| AppError::bad_request("event_date must be a valid RFC3339 datetime"))
}

fn map_write_error(error: sqlx::Error) -> AppError {
    if let sqlx::Error::Database(database_error) = &error {
        match database_error.code().as_deref() {
            Some("23505") => return AppError::conflict("slug already exists"),
            Some("23503") => return AppError::bad_request("template_id is invalid"),
            _ => {}
        }
    }

    error.into()
}

fn is_unique_violation(error: &sqlx::Error) -> bool {
    matches!(
        error,
        sqlx::Error::Database(database_error) if database_error.code().as_deref() == Some("23505")
    )
}

struct NormalizedCreateEvent {
    title: String,
    slug: Option<String>,
    event_type: String,
    template_id: Option<Uuid>,
    event_date: Option<OffsetDateTime>,
    timezone: String,
    venue_name: Option<String>,
    venue_address: Option<String>,
}
