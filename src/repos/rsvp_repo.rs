use serde_json::Value;
use sqlx::{postgres::PgRow, Acquire, PgPool, Postgres, QueryBuilder, Row};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    domain::rsvp::{RsvpForm, RsvpQuestion, RsvpResponse},
    dto::rsvp::RsvpResponsesListQuery,
};

const DEFAULT_FORM_TITLE: &str = "Подтверждение участия";

pub struct UpsertRsvpFormData {
    pub title: String,
    pub description: Option<String>,
    pub deadline_at: Option<OffsetDateTime>,
    pub settings: Value,
    pub questions: Vec<NewRsvpQuestion>,
}

pub struct NewRsvpQuestion {
    pub position: i32,
    pub code: String,
    pub label: String,
    pub question_type: String,
    pub required: bool,
    pub options: Value,
}

pub struct NewRsvpResponse {
    pub event_id: Uuid,
    pub guest_id: Option<Uuid>,
    pub public_token: Option<String>,
    pub status: String,
    pub plus_one_count: i32,
    pub answers: Value,
    pub submitted_at: OffsetDateTime,
}

pub struct RsvpStatusCounts {
    pub confirmed: i64,
    pub declined: i64,
    pub maybe: i64,
}

pub async fn ensure_form_for_event(pool: &PgPool, event_id: Uuid) -> Result<RsvpForm, sqlx::Error> {
    sqlx::query_as::<_, RsvpForm>(
        r#"
        INSERT INTO rsvp_forms (id, event_id, title, settings)
        VALUES ($1, $2, $3, '{}'::jsonb)
        ON CONFLICT (event_id) DO UPDATE
        SET event_id = EXCLUDED.event_id
        RETURNING
            id,
            event_id,
            title,
            description,
            deadline_at,
            settings
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(event_id)
    .bind(DEFAULT_FORM_TITLE)
    .fetch_one(pool)
    .await
}

pub async fn find_form_by_event(
    pool: &PgPool,
    event_id: Uuid,
) -> Result<Option<RsvpForm>, sqlx::Error> {
    sqlx::query_as::<_, RsvpForm>(
        r#"
        SELECT
            id,
            event_id,
            title,
            description,
            deadline_at,
            settings
        FROM rsvp_forms
        WHERE event_id = $1
        "#,
    )
    .bind(event_id)
    .fetch_optional(pool)
    .await
}

pub async fn list_questions_by_form(
    pool: &PgPool,
    form_id: Uuid,
) -> Result<Vec<RsvpQuestion>, sqlx::Error> {
    sqlx::query_as::<_, RsvpQuestion>(
        r#"
        SELECT
            id,
            form_id,
            position,
            code,
            label,
            question_type,
            required,
            options
        FROM rsvp_questions
        WHERE form_id = $1
        ORDER BY position ASC, code ASC
        "#,
    )
    .bind(form_id)
    .fetch_all(pool)
    .await
}

pub async fn upsert_form_and_replace_questions(
    pool: &PgPool,
    event_id: Uuid,
    payload: &UpsertRsvpFormData,
) -> Result<RsvpForm, sqlx::Error> {
    let mut transaction = pool.begin().await?;
    let conn = transaction.acquire().await?;

    let form = sqlx::query_as::<_, RsvpForm>(
        r#"
        INSERT INTO rsvp_forms (
            id,
            event_id,
            title,
            description,
            deadline_at,
            settings
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (event_id) DO UPDATE
        SET
            title = EXCLUDED.title,
            description = EXCLUDED.description,
            deadline_at = EXCLUDED.deadline_at,
            settings = EXCLUDED.settings
        RETURNING
            id,
            event_id,
            title,
            description,
            deadline_at,
            settings
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(event_id)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(payload.deadline_at)
    .bind(&payload.settings)
    .fetch_one(&mut *conn)
    .await?;

    sqlx::query(
        r#"
        DELETE FROM rsvp_questions
        WHERE form_id = $1
        "#,
    )
    .bind(form.id)
    .execute(&mut *conn)
    .await?;

    for question in &payload.questions {
        sqlx::query(
            r#"
            INSERT INTO rsvp_questions (
                id,
                form_id,
                position,
                code,
                label,
                question_type,
                required,
                options
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(form.id)
        .bind(question.position)
        .bind(&question.code)
        .bind(&question.label)
        .bind(&question.question_type)
        .bind(question.required)
        .bind(&question.options)
        .execute(&mut *conn)
        .await?;
    }

    transaction.commit().await?;
    Ok(form)
}

pub async fn list_responses_for_owner(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    query: &RsvpResponsesListQuery,
) -> Result<Vec<RsvpResponse>, sqlx::Error> {
    let mut builder = QueryBuilder::<Postgres>::new(
        r#"
        SELECT
            rr.id,
            rr.event_id,
            rr.guest_id,
            rr.public_token,
            rr.status,
            rr.plus_one_count,
            rr.answers,
            rr.submitted_at,
            rr.created_at
        FROM rsvp_responses rr
        INNER JOIN events e ON e.id = rr.event_id
        WHERE rr.event_id =
        "#,
    );
    builder.push_bind(event_id);
    builder.push(" AND e.owner_id = ");
    builder.push_bind(owner_id);
    apply_status_filter(&mut builder, query);
    builder.push(" ORDER BY rr.created_at DESC");
    builder.push(" LIMIT ");
    builder.push_bind(query.per_page as i64);
    builder.push(" OFFSET ");
    builder.push_bind(((query.page - 1) * query.per_page) as i64);

    builder
        .build_query_as::<RsvpResponse>()
        .fetch_all(pool)
        .await
}

pub async fn count_responses_for_owner(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    query: &RsvpResponsesListQuery,
) -> Result<i64, sqlx::Error> {
    let mut builder = QueryBuilder::<Postgres>::new(
        r#"
        SELECT COUNT(*) AS total
        FROM rsvp_responses rr
        INNER JOIN events e ON e.id = rr.event_id
        WHERE rr.event_id =
        "#,
    );
    builder.push_bind(event_id);
    builder.push(" AND e.owner_id = ");
    builder.push_bind(owner_id);
    apply_status_filter(&mut builder, query);

    let row: PgRow = builder.build().fetch_one(pool).await?;
    Ok(row.get::<i64, _>("total"))
}

pub async fn find_response_by_id_for_owner(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    response_id: Uuid,
) -> Result<Option<RsvpResponse>, sqlx::Error> {
    sqlx::query_as::<_, RsvpResponse>(
        r#"
        SELECT
            rr.id,
            rr.event_id,
            rr.guest_id,
            rr.public_token,
            rr.status,
            rr.plus_one_count,
            rr.answers,
            rr.submitted_at,
            rr.created_at
        FROM rsvp_responses rr
        INNER JOIN events e ON e.id = rr.event_id
        WHERE rr.id = $1
          AND rr.event_id = $2
          AND e.owner_id = $3
        "#,
    )
    .bind(response_id)
    .bind(event_id)
    .bind(owner_id)
    .fetch_optional(pool)
    .await
}

pub async fn count_statuses_for_event(
    pool: &PgPool,
    event_id: Uuid,
) -> Result<RsvpStatusCounts, sqlx::Error> {
    let row: PgRow = sqlx::query(
        r#"
        SELECT
            COUNT(*) FILTER (WHERE status = 'confirmed') AS confirmed,
            COUNT(*) FILTER (WHERE status = 'declined') AS declined,
            COUNT(*) FILTER (WHERE status = 'maybe') AS maybe
        FROM rsvp_responses
        WHERE event_id = $1
        "#,
    )
    .bind(event_id)
    .fetch_one(pool)
    .await?;

    Ok(RsvpStatusCounts {
        confirmed: row.get::<i64, _>("confirmed"),
        declined: row.get::<i64, _>("declined"),
        maybe: row.get::<i64, _>("maybe"),
    })
}

pub async fn count_total_guests_for_event(
    pool: &PgPool,
    event_id: Uuid,
) -> Result<i64, sqlx::Error> {
    let row: PgRow = sqlx::query(
        r#"
        SELECT COUNT(*) AS total
        FROM guests
        WHERE event_id = $1
        "#,
    )
    .bind(event_id)
    .fetch_one(pool)
    .await?;

    Ok(row.get::<i64, _>("total"))
}

pub async fn count_distinct_guest_responses_for_event(
    pool: &PgPool,
    event_id: Uuid,
) -> Result<i64, sqlx::Error> {
    let row: PgRow = sqlx::query(
        r#"
        SELECT COUNT(DISTINCT guest_id) AS total
        FROM rsvp_responses
        WHERE event_id = $1
          AND guest_id IS NOT NULL
        "#,
    )
    .bind(event_id)
    .fetch_one(pool)
    .await?;

    Ok(row.get::<i64, _>("total"))
}

pub async fn create_response(
    pool: &PgPool,
    payload: &NewRsvpResponse,
) -> Result<RsvpResponse, sqlx::Error> {
    sqlx::query_as::<_, RsvpResponse>(
        r#"
        INSERT INTO rsvp_responses (
            id,
            event_id,
            guest_id,
            public_token,
            status,
            plus_one_count,
            answers,
            submitted_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING
            id,
            event_id,
            guest_id,
            public_token,
            status,
            plus_one_count,
            answers,
            submitted_at,
            created_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(payload.event_id)
    .bind(payload.guest_id)
    .bind(&payload.public_token)
    .bind(&payload.status)
    .bind(payload.plus_one_count)
    .bind(&payload.answers)
    .bind(payload.submitted_at)
    .fetch_one(pool)
    .await
}

fn apply_status_filter<'a>(
    builder: &mut QueryBuilder<'a, Postgres>,
    query: &'a RsvpResponsesListQuery,
) {
    if let Some(status) = &query.status {
        builder.push(" AND rr.status = ");
        builder.push_bind(status);
    }
}
