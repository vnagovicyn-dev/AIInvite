use axum::extract::Multipart;
use csv::ReaderBuilder;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::{error::AppError, slug},
    dto::{
        events::PatchValue,
        guests::{
            CreateGuestRequest, GuestImportResponse, GuestListQuery, GuestListResponse,
            GuestResponse, UpdateGuestRequest,
        },
    },
    repos::{events_repo, guests_repo},
};

pub async fn create(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    payload: CreateGuestRequest,
) -> Result<GuestResponse, AppError> {
    ensure_event_owned(pool, owner_id, event_id).await?;
    let guest = normalize_create_request(event_id, payload)?;

    guests_repo::create(pool, &guest)
        .await
        .map(GuestResponse::from)
        .map_err(AppError::from)
}

pub async fn list(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    query: GuestListQuery,
) -> Result<GuestListResponse, AppError> {
    ensure_event_owned(pool, owner_id, event_id).await?;
    let normalized_query = normalize_list_query(query);
    let items = guests_repo::list_by_event(pool, event_id, &normalized_query)
        .await?
        .into_iter()
        .map(GuestResponse::from)
        .collect::<Vec<GuestResponse>>();
    let total = guests_repo::count_by_event(pool, event_id, &normalized_query).await?;

    Ok(GuestListResponse {
        items,
        page: normalized_query.page,
        per_page: normalized_query.per_page,
        total,
    })
}

pub async fn get_by_id(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    guest_id: Uuid,
) -> Result<Option<GuestResponse>, AppError> {
    let guest = guests_repo::find_by_id_for_owner(pool, owner_id, event_id, guest_id).await?;
    Ok(guest.map(GuestResponse::from))
}

pub async fn update(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    guest_id: Uuid,
    payload: UpdateGuestRequest,
) -> Result<Option<GuestResponse>, AppError> {
    let changes = normalize_update_request(payload)?;

    guests_repo::update_for_owner(pool, owner_id, event_id, guest_id, &changes)
        .await
        .map(|guest| guest.map(GuestResponse::from))
        .map_err(AppError::from)
}

pub async fn delete(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    guest_id: Uuid,
) -> Result<bool, AppError> {
    guests_repo::delete_for_owner(pool, owner_id, event_id, guest_id)
        .await
        .map_err(AppError::from)
}

pub async fn import_csv(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    mut multipart: Multipart,
) -> Result<GuestImportResponse, AppError> {
    ensure_event_owned(pool, owner_id, event_id).await?;

    let mut file_bytes: Option<Vec<u8>> = None;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| AppError::bad_request("invalid multipart payload"))?
    {
        if field.name() == Some("file") {
            let bytes = field
                .bytes()
                .await
                .map_err(|_| AppError::bad_request("failed to read uploaded file"))?;
            file_bytes = Some(bytes.to_vec());
            break;
        }
    }

    let bytes =
        file_bytes.ok_or_else(|| AppError::bad_request("multipart field `file` is required"))?;

    let mut reader = ReaderBuilder::new()
        .trim(csv::Trim::All)
        .flexible(true)
        .from_reader(bytes.as_slice());

    let headers = reader
        .headers()
        .map_err(|_| AppError::bad_request("invalid csv header"))?
        .clone();
    validate_import_headers(&headers)?;

    let mut imported_count = 0usize;
    let mut skipped_count = 0usize;
    let mut errors: Vec<String> = Vec::new();

    for (index, record_result) in reader.records().enumerate() {
        let row_number = index + 2;
        let record = match record_result {
            Ok(record) => record,
            Err(error) => {
                skipped_count += 1;
                errors.push(format!("row {row_number}: {error}"));
                continue;
            }
        };

        let full_name = record.get(0).unwrap_or_default().trim().to_string();
        if full_name.is_empty() {
            skipped_count += 1;
            errors.push(format!("row {row_number}: full_name is required"));
            continue;
        }

        let phone = normalize_optional_string(record.get(1).map(ToString::to_string));
        let email = match normalize_optional_string(record.get(2).map(ToString::to_string)) {
            Some(value) => match validate_email(Some(value.clone())) {
                Ok(validated) => validated,
                Err(_) => {
                    skipped_count += 1;
                    errors.push(format!("row {row_number}: email is invalid"));
                    continue;
                }
            },
            None => None,
        };
        let group_name = normalize_optional_string(record.get(3).map(ToString::to_string));
        let vip = match parse_csv_bool(record.get(4).unwrap_or_default()) {
            Ok(value) => value,
            Err(message) => {
                skipped_count += 1;
                errors.push(format!("row {row_number}: {message}"));
                continue;
            }
        };

        let guest = guests_repo::NewGuest {
            event_id,
            invite_token: generate_invite_token(),
            full_name,
            phone,
            email,
            group_name,
            tags: Vec::new(),
            plus_one_allowed: false,
            is_child: false,
            vip,
            notes: None,
        };

        match guests_repo::create(pool, &guest).await {
            Ok(_) => imported_count += 1,
            Err(error) => {
                skipped_count += 1;
                errors.push(format!("row {row_number}: {error}"));
            }
        }
    }

    Ok(GuestImportResponse {
        imported_count,
        skipped_count,
        errors,
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

fn normalize_create_request(
    event_id: Uuid,
    payload: CreateGuestRequest,
) -> Result<guests_repo::NewGuest, AppError> {
    let full_name = normalize_required_string(&payload.full_name, "full_name is required")?;
    let email = validate_email(normalize_optional_string(payload.email))?;

    Ok(guests_repo::NewGuest {
        event_id,
        invite_token: generate_invite_token(),
        full_name,
        phone: normalize_optional_string(payload.phone),
        email,
        group_name: normalize_optional_string(payload.group_name),
        tags: normalize_tags(payload.tags),
        plus_one_allowed: payload.plus_one_allowed.unwrap_or(false),
        is_child: payload.is_child.unwrap_or(false),
        vip: payload.vip.unwrap_or(false),
        notes: normalize_optional_string(payload.notes),
    })
}

fn normalize_update_request(
    payload: UpdateGuestRequest,
) -> Result<guests_repo::UpdateGuestChanges, AppError> {
    let full_name = payload
        .full_name
        .map(|value| normalize_required_string(&value, "full_name is invalid"))
        .transpose()?;
    let phone = normalize_patch_string(payload.phone);
    let email = match payload.email {
        PatchValue::Missing => None,
        PatchValue::Null => Some(None),
        PatchValue::Value(value) => Some(validate_email(Some(value))?),
    };
    let group_name = normalize_patch_string(payload.group_name);
    let notes = normalize_patch_string(payload.notes);
    let tags = payload.tags.map(|tags| normalize_tags(Some(tags)));

    if full_name.is_none()
        && phone.is_none()
        && email.is_none()
        && group_name.is_none()
        && tags.is_none()
        && payload.plus_one_allowed.is_none()
        && payload.is_child.is_none()
        && payload.vip.is_none()
        && notes.is_none()
    {
        return Err(AppError::bad_request("no fields to update"));
    }

    Ok(guests_repo::UpdateGuestChanges {
        full_name,
        phone,
        email,
        group_name,
        tags,
        plus_one_allowed: payload.plus_one_allowed,
        is_child: payload.is_child,
        vip: payload.vip,
        notes,
    })
}

fn normalize_list_query(mut query: GuestListQuery) -> GuestListQuery {
    if query.page == 0 {
        query.page = 1;
    }

    if query.per_page == 0 {
        query.per_page = 20;
    } else if query.per_page > 100 {
        query.per_page = 100;
    }

    query.search = normalize_optional_string(query.search);
    query.group_name = normalize_optional_string(query.group_name);
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

fn normalize_patch_string(value: PatchValue<String>) -> Option<Option<String>> {
    match value {
        PatchValue::Missing => None,
        PatchValue::Null => Some(None),
        PatchValue::Value(value) => Some(normalize_optional_string(Some(value))),
    }
}

fn normalize_tags(tags: Option<Vec<String>>) -> Vec<String> {
    let mut normalized = tags
        .unwrap_or_default()
        .into_iter()
        .filter_map(|tag| normalize_optional_string(Some(tag)))
        .collect::<Vec<String>>();
    normalized.sort();
    normalized.dedup();
    normalized
}

fn validate_email(email: Option<String>) -> Result<Option<String>, AppError> {
    if let Some(value) = email {
        if !value.contains('@') {
            return Err(AppError::bad_request("email is invalid"));
        }
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

fn validate_import_headers(headers: &csv::StringRecord) -> Result<(), AppError> {
    let expected = ["full_name", "phone", "email", "group_name", "vip"];
    for (index, column) in expected.iter().enumerate() {
        if headers.get(index).map(str::trim) != Some(*column) {
            return Err(AppError::bad_request("invalid csv header"));
        }
    }

    Ok(())
}

fn parse_csv_bool(value: &str) -> Result<bool, &'static str> {
    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "" | "false" | "0" | "no" => Ok(false),
        "true" | "1" | "yes" => Ok(true),
        _ => Err("vip must be one of: true,false,1,0,yes,no"),
    }
}

fn generate_invite_token() -> String {
    slug::generate_slug("guest").replace('-', "")
}
