use sqlx::{postgres::PgRow, PgPool, Postgres, QueryBuilder, Row};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{domain::events::Event, dto::events::EventListQuery};

pub struct NewEvent {
    pub owner_id: Uuid,
    pub title: String,
    pub slug: String,
    pub event_type: String,
    pub status: String,
    pub template_id: Option<Uuid>,
    pub event_date: Option<OffsetDateTime>,
    pub timezone: String,
    pub venue_name: Option<String>,
    pub venue_address: Option<String>,
}

pub struct UpdateEventChanges {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub event_type: Option<String>,
    pub template_id: Option<Option<Uuid>>,
    pub event_date: Option<Option<OffsetDateTime>>,
    pub timezone: Option<String>,
    pub venue_name: Option<Option<String>>,
    pub venue_address: Option<Option<String>>,
}

pub async fn create(pool: &PgPool, new_event: &NewEvent) -> Result<Event, sqlx::Error> {
    sqlx::query_as::<_, Event>(
        r#"
        INSERT INTO events (
            id,
            owner_id,
            template_id,
            title,
            slug,
            event_type,
            status,
            event_date,
            timezone,
            venue_name,
            venue_address
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING
            id,
            owner_id,
            template_id,
            title,
            slug,
            event_type,
            status,
            event_date,
            timezone,
            venue_name,
            venue_address,
            created_at,
            updated_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(new_event.owner_id)
    .bind(new_event.template_id)
    .bind(&new_event.title)
    .bind(&new_event.slug)
    .bind(&new_event.event_type)
    .bind(&new_event.status)
    .bind(new_event.event_date)
    .bind(&new_event.timezone)
    .bind(&new_event.venue_name)
    .bind(&new_event.venue_address)
    .fetch_one(pool)
    .await
}

pub async fn list_by_owner(
    pool: &PgPool,
    owner_id: Uuid,
    query: &EventListQuery,
) -> Result<Vec<Event>, sqlx::Error> {
    let mut query_builder = QueryBuilder::<Postgres>::new(
        r#"
        SELECT
            id,
            owner_id,
            template_id,
            title,
            slug,
            event_type,
            status,
            event_date,
            timezone,
            venue_name,
            venue_address,
            created_at,
            updated_at
        FROM events
        WHERE owner_id = 
        "#,
    );
    query_builder.push_bind(owner_id);

    if let Some(status) = &query.status {
        query_builder.push(" AND status = ");
        query_builder.push_bind(status);
    }

    query_builder.push(" ORDER BY created_at DESC");
    query_builder.push(" LIMIT ");
    query_builder.push_bind(query.per_page as i64);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(((query.page - 1) * query.per_page) as i64);

    query_builder
        .build_query_as::<Event>()
        .fetch_all(pool)
        .await
}

pub async fn count_by_owner(
    pool: &PgPool,
    owner_id: Uuid,
    query: &EventListQuery,
) -> Result<i64, sqlx::Error> {
    let mut query_builder = QueryBuilder::<Postgres>::new(
        r#"
        SELECT COUNT(*) AS total
        FROM events
        WHERE owner_id = 
        "#,
    );
    query_builder.push_bind(owner_id);

    if let Some(status) = &query.status {
        query_builder.push(" AND status = ");
        query_builder.push_bind(status);
    }

    let row: PgRow = query_builder.build().fetch_one(pool).await?;
    Ok(row.get::<i64, _>("total"))
}

pub async fn find_by_id_and_owner(
    pool: &PgPool,
    owner_id: Uuid,
    id: Uuid,
) -> Result<Option<Event>, sqlx::Error> {
    sqlx::query_as::<_, Event>(
        r#"
        SELECT
            id,
            owner_id,
            template_id,
            title,
            slug,
            event_type,
            status,
            event_date,
            timezone,
            venue_name,
            venue_address,
            created_at,
            updated_at
        FROM events
        WHERE id = $1 AND owner_id = $2
        "#,
    )
    .bind(id)
    .bind(owner_id)
    .fetch_optional(pool)
    .await
}

pub async fn update_by_owner(
    pool: &PgPool,
    owner_id: Uuid,
    id: Uuid,
    changes: &UpdateEventChanges,
) -> Result<Option<Event>, sqlx::Error> {
    sqlx::query_as::<_, Event>(
        r#"
        UPDATE events
        SET
            title = CASE WHEN $1 THEN $2 ELSE title END,
            slug = CASE WHEN $3 THEN $4 ELSE slug END,
            event_type = CASE WHEN $5 THEN $6 ELSE event_type END,
            template_id = CASE WHEN $7 THEN $8 ELSE template_id END,
            event_date = CASE WHEN $9 THEN $10 ELSE event_date END,
            timezone = CASE WHEN $11 THEN $12 ELSE timezone END,
            venue_name = CASE WHEN $13 THEN $14 ELSE venue_name END,
            venue_address = CASE WHEN $15 THEN $16 ELSE venue_address END,
            updated_at = now()
        WHERE id = $17 AND owner_id = $18
        RETURNING
            id,
            owner_id,
            template_id,
            title,
            slug,
            event_type,
            status,
            event_date,
            timezone,
            venue_name,
            venue_address,
            created_at,
            updated_at
        "#,
    )
    .bind(changes.title.is_some())
    .bind(&changes.title)
    .bind(changes.slug.is_some())
    .bind(&changes.slug)
    .bind(changes.event_type.is_some())
    .bind(&changes.event_type)
    .bind(changes.template_id.is_some())
    .bind(changes.template_id.flatten())
    .bind(changes.event_date.is_some())
    .bind(changes.event_date.flatten())
    .bind(changes.timezone.is_some())
    .bind(&changes.timezone)
    .bind(changes.venue_name.is_some())
    .bind(changes.venue_name.clone().flatten())
    .bind(changes.venue_address.is_some())
    .bind(changes.venue_address.clone().flatten())
    .bind(id)
    .bind(owner_id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_by_owner(pool: &PgPool, owner_id: Uuid, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM events
        WHERE id = $1 AND owner_id = $2
        "#,
    )
    .bind(id)
    .bind(owner_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
