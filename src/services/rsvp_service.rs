use serde_json::Value;
use sqlx::PgPool;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use uuid::Uuid;

use crate::{
    common::error::AppError,
    dto::rsvp::{
        PublicSubmitRsvpRequest, PublicSubmitRsvpResponse, RsvpFormResponse, RsvpQuestionInput,
        RsvpQuestionResponse, RsvpResponseItem, RsvpResponsesAggregates, RsvpResponsesListQuery,
        RsvpResponsesListResponse, UpsertRsvpFormRequest,
    },
    repos::{events_repo, guests_repo, pages_repo, rsvp_repo},
};

const ALLOWED_STATUSES: [&str; 3] = ["confirmed", "declined", "maybe"];
const ALLOWED_QUESTION_TYPES: [&str; 6] = [
    "text",
    "textarea",
    "select",
    "multiselect",
    "boolean",
    "number",
];

pub async fn get_form(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
) -> Result<RsvpFormResponse, AppError> {
    ensure_event_owned(pool, owner_id, event_id).await?;
    let form = rsvp_repo::ensure_form_for_event(pool, event_id).await?;
    let questions = rsvp_repo::list_questions_by_form(pool, form.id)
        .await?
        .into_iter()
        .map(RsvpQuestionResponse::from)
        .collect::<Vec<RsvpQuestionResponse>>();

    Ok(RsvpFormResponse::from_parts(form, questions))
}

pub async fn upsert_form(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    payload: UpsertRsvpFormRequest,
) -> Result<RsvpFormResponse, AppError> {
    ensure_event_owned(pool, owner_id, event_id).await?;
    let current_form = rsvp_repo::ensure_form_for_event(pool, event_id).await?;
    let normalized = normalize_form_request(current_form.title, current_form.settings, payload)?;
    let form = rsvp_repo::upsert_form_and_replace_questions(pool, event_id, &normalized)
        .await
        .map_err(map_write_error)?;
    let questions = rsvp_repo::list_questions_by_form(pool, form.id)
        .await?
        .into_iter()
        .map(RsvpQuestionResponse::from)
        .collect::<Vec<RsvpQuestionResponse>>();

    Ok(RsvpFormResponse::from_parts(form, questions))
}

pub async fn list_responses(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    query: RsvpResponsesListQuery,
) -> Result<RsvpResponsesListResponse, AppError> {
    ensure_event_owned(pool, owner_id, event_id).await?;
    let normalized_query = normalize_list_query(query)?;
    let items = rsvp_repo::list_responses_for_owner(pool, owner_id, event_id, &normalized_query)
        .await?
        .into_iter()
        .map(RsvpResponseItem::from)
        .collect::<Vec<RsvpResponseItem>>();
    let total =
        rsvp_repo::count_responses_for_owner(pool, owner_id, event_id, &normalized_query).await?;
    let status_counts = rsvp_repo::count_statuses_for_event(pool, event_id).await?;
    let guests_total = rsvp_repo::count_total_guests_for_event(pool, event_id).await?;
    let responded_guests =
        rsvp_repo::count_distinct_guest_responses_for_event(pool, event_id).await?;
    let pending = (guests_total - responded_guests).max(0);
    let aggregate_total = if guests_total > 0 {
        guests_total
    } else {
        status_counts.confirmed + status_counts.declined + status_counts.maybe
    };

    Ok(RsvpResponsesListResponse {
        items,
        page: normalized_query.page,
        per_page: normalized_query.per_page,
        total,
        aggregates: RsvpResponsesAggregates {
            total: aggregate_total,
            confirmed: status_counts.confirmed,
            declined: status_counts.declined,
            maybe: status_counts.maybe,
            pending,
        },
    })
}

pub async fn get_response(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    response_id: Uuid,
) -> Result<Option<RsvpResponseItem>, AppError> {
    let response =
        rsvp_repo::find_response_by_id_for_owner(pool, owner_id, event_id, response_id).await?;
    Ok(response.map(RsvpResponseItem::from))
}

pub async fn submit_public_response(
    pool: &PgPool,
    slug: &str,
    payload: PublicSubmitRsvpRequest,
) -> Result<PublicSubmitRsvpResponse, AppError> {
    let event = pages_repo::find_published_event_by_slug(pool, slug)
        .await?
        .ok_or_else(|| AppError::not_found("event not found"))?;
    let form = rsvp_repo::find_form_by_event(pool, event.id).await?;
    let questions = if let Some(existing_form) = &form {
        if let Some(deadline_at) = existing_form.deadline_at {
            if deadline_at < OffsetDateTime::now_utc() {
                return Err(AppError::bad_request("rsvp deadline has passed"));
            }
        }

        rsvp_repo::list_questions_by_form(pool, existing_form.id).await?
    } else {
        Vec::new()
    };

    let normalized = normalize_public_submit_request(payload, &questions)?;
    if let Some(guest_id) = normalized.guest_id {
        let guest = guests_repo::find_by_id_and_event(pool, event.id, guest_id).await?;
        if guest.is_none() {
            return Err(AppError::bad_request("guest_id is invalid"));
        }
    }

    let inserted = rsvp_repo::create_response(
        pool,
        &rsvp_repo::NewRsvpResponse {
            event_id: event.id,
            guest_id: normalized.guest_id,
            public_token: None,
            status: normalized.status,
            plus_one_count: normalized.plus_one_count,
            answers: normalized.answers,
            submitted_at: OffsetDateTime::now_utc(),
        },
    )
    .await?;

    Ok(PublicSubmitRsvpResponse {
        id: inserted.id,
        status: inserted.status,
        submitted_at: inserted
            .submitted_at
            .and_then(|value| value.format(&Rfc3339).ok())
            .unwrap_or_else(|| OffsetDateTime::now_utc().format(&Rfc3339).unwrap()),
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

fn normalize_form_request(
    current_title: String,
    current_settings: Value,
    payload: UpsertRsvpFormRequest,
) -> Result<rsvp_repo::UpsertRsvpFormData, AppError> {
    let title = payload
        .title
        .and_then(|value| normalize_optional_string(Some(value)))
        .unwrap_or(current_title);
    let description = normalize_optional_string(payload.description);
    let deadline_at = payload
        .deadline_at
        .map(|value| parse_datetime(&value, "deadline_at must be a valid RFC3339 datetime"))
        .transpose()?;
    let settings = payload.settings.unwrap_or(current_settings);
    if !settings.is_object() {
        return Err(AppError::bad_request("settings must be a JSON object"));
    }

    let mut questions = Vec::with_capacity(payload.questions.len());
    let mut seen_codes = std::collections::BTreeSet::new();
    for (index, question) in payload.questions.into_iter().enumerate() {
        let normalized = normalize_question_input(question, index as i32 + 1)?;
        if !seen_codes.insert(normalized.code.clone()) {
            return Err(AppError::bad_request("question codes must be unique"));
        }
        questions.push(normalized);
    }

    Ok(rsvp_repo::UpsertRsvpFormData {
        title,
        description,
        deadline_at,
        settings,
        questions,
    })
}

fn normalize_question_input(
    payload: RsvpQuestionInput,
    position: i32,
) -> Result<rsvp_repo::NewRsvpQuestion, AppError> {
    let code = normalize_required_string(&payload.code, "question code is required")?;
    let label = normalize_required_string(&payload.label, "question label is required")?;
    let question_type =
        normalize_required_string(&payload.question_type, "question_type is required")?;
    validate_question_type(&question_type)?;
    let options = payload.options.unwrap_or_else(|| Value::Array(Vec::new()));
    if !options.is_array() {
        return Err(AppError::bad_request("question options must be an array"));
    }
    if matches!(question_type.as_str(), "select" | "multiselect")
        && options.as_array().is_none_or(Vec::is_empty)
    {
        return Err(AppError::bad_request(
            "select and multiselect questions require non-empty options",
        ));
    }

    Ok(rsvp_repo::NewRsvpQuestion {
        position,
        code,
        label,
        question_type,
        required: payload.required.unwrap_or(false),
        options,
    })
}

fn normalize_public_submit_request(
    payload: PublicSubmitRsvpRequest,
    questions: &[crate::domain::rsvp::RsvpQuestion],
) -> Result<NormalizedPublicSubmit, AppError> {
    let status = normalize_required_string(&payload.status, "status is required")?;
    validate_status(&status)?;
    let plus_one_count = payload.plus_one_count.unwrap_or(0);
    if plus_one_count < 0 {
        return Err(AppError::bad_request(
            "plus_one_count must be greater than or equal to 0",
        ));
    }
    if !payload.answers.is_object() {
        return Err(AppError::bad_request("answers must be a JSON object"));
    }

    validate_answers(&payload.answers, questions)?;

    Ok(NormalizedPublicSubmit {
        status,
        plus_one_count,
        guest_id: payload.guest_id,
        answers: payload.answers,
    })
}

fn validate_answers(
    answers: &Value,
    questions: &[crate::domain::rsvp::RsvpQuestion],
) -> Result<(), AppError> {
    let answer_map = answers
        .as_object()
        .ok_or_else(|| AppError::bad_request("answers must be a JSON object"))?;

    for question in questions {
        let answer = answer_map.get(&question.code);
        if question.required && !has_meaningful_answer(answer) {
            return Err(AppError::bad_request(format!(
                "answer for `{}` is required",
                question.code
            )));
        }

        if let Some(value) = answer {
            validate_answer_type(&question.question_type, value, &question.code)?;
        }
    }

    Ok(())
}

fn has_meaningful_answer(answer: Option<&Value>) -> bool {
    match answer {
        Some(Value::String(value)) => !value.trim().is_empty(),
        Some(Value::Array(values)) => !values.is_empty(),
        Some(Value::Null) | None => false,
        Some(_) => true,
    }
}

fn validate_answer_type(question_type: &str, value: &Value, code: &str) -> Result<(), AppError> {
    let is_valid = match question_type {
        "text" | "textarea" | "select" => {
            value.as_str().is_some_and(|item| !item.trim().is_empty())
        }
        "multiselect" => value.as_array().is_some_and(|items| !items.is_empty()),
        "boolean" => value.is_boolean(),
        "number" => value.is_number(),
        _ => false,
    };

    if is_valid {
        Ok(())
    } else {
        Err(AppError::bad_request(format!(
            "answer for `{code}` has invalid type"
        )))
    }
}

fn normalize_list_query(
    mut query: RsvpResponsesListQuery,
) -> Result<RsvpResponsesListQuery, AppError> {
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
    if let Some(status) = &query.status {
        if !ALLOWED_STATUSES.contains(&status.as_str()) {
            return Err(AppError::bad_request("status filter is invalid"));
        }
    }
    Ok(query)
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

fn parse_datetime(value: &str, message: &str) -> Result<OffsetDateTime, AppError> {
    OffsetDateTime::parse(value.trim(), &Rfc3339).map_err(|_| AppError::bad_request(message))
}

fn validate_question_type(value: &str) -> Result<(), AppError> {
    if ALLOWED_QUESTION_TYPES.contains(&value) {
        Ok(())
    } else {
        Err(AppError::bad_request("question_type is invalid"))
    }
}

fn validate_status(value: &str) -> Result<(), AppError> {
    if ALLOWED_STATUSES.contains(&value) {
        Ok(())
    } else {
        Err(AppError::bad_request("status is invalid"))
    }
}

fn map_write_error(error: sqlx::Error) -> AppError {
    if let sqlx::Error::Database(database_error) = &error {
        match database_error.code().as_deref() {
            Some("23505") => return AppError::conflict("question code already exists"),
            Some("23503") => return AppError::bad_request("event_id is invalid"),
            _ => {}
        }
    }

    error.into()
}

struct NormalizedPublicSubmit {
    status: String,
    plus_one_count: i32,
    guest_id: Option<Uuid>,
    answers: Value,
}
