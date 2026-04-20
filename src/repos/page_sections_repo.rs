use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::page_sections::PageSection;

pub struct NewPageSection {
    pub event_id: Uuid,
    pub section_type: String,
    pub position: i32,
    pub is_enabled: bool,
    pub title: Option<String>,
    pub content: Value,
}

pub struct UpdatePageSectionChanges {
    pub section_type: Option<String>,
    pub position: Option<i32>,
    pub is_enabled: Option<bool>,
    pub title: Option<Option<String>>,
    pub content: Option<Value>,
}

pub async fn create(
    pool: &PgPool,
    new_section: &NewPageSection,
) -> Result<PageSection, sqlx::Error> {
    sqlx::query_as::<_, PageSection>(
        r#"
        INSERT INTO page_sections (
            id,
            event_id,
            section_type,
            position,
            is_enabled,
            title,
            content
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING
            id,
            event_id,
            section_type,
            position,
            is_enabled,
            title,
            content,
            created_at,
            updated_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(new_section.event_id)
    .bind(&new_section.section_type)
    .bind(new_section.position)
    .bind(new_section.is_enabled)
    .bind(&new_section.title)
    .bind(&new_section.content)
    .fetch_one(pool)
    .await
}

pub async fn list_by_event(pool: &PgPool, event_id: Uuid) -> Result<Vec<PageSection>, sqlx::Error> {
    sqlx::query_as::<_, PageSection>(
        r#"
        SELECT
            id,
            event_id,
            section_type,
            position,
            is_enabled,
            title,
            content,
            created_at,
            updated_at
        FROM page_sections
        WHERE event_id = $1
        ORDER BY position ASC, created_at ASC
        "#,
    )
    .bind(event_id)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id_for_owner(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    id: Uuid,
) -> Result<Option<PageSection>, sqlx::Error> {
    sqlx::query_as::<_, PageSection>(
        r#"
        SELECT
            ps.id,
            ps.event_id,
            ps.section_type,
            ps.position,
            ps.is_enabled,
            ps.title,
            ps.content,
            ps.created_at,
            ps.updated_at
        FROM page_sections ps
        INNER JOIN events e ON e.id = ps.event_id
        WHERE ps.id = $1 AND ps.event_id = $2 AND e.owner_id = $3
        "#,
    )
    .bind(id)
    .bind(event_id)
    .bind(owner_id)
    .fetch_optional(pool)
    .await
}

pub async fn update_for_owner(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    id: Uuid,
    changes: &UpdatePageSectionChanges,
) -> Result<Option<PageSection>, sqlx::Error> {
    sqlx::query_as::<_, PageSection>(
        r#"
        UPDATE page_sections ps
        SET
            section_type = CASE WHEN $1 THEN $2 ELSE ps.section_type END,
            position = CASE WHEN $3 THEN $4 ELSE ps.position END,
            is_enabled = CASE WHEN $5 THEN $6 ELSE ps.is_enabled END,
            title = CASE WHEN $7 THEN $8 ELSE ps.title END,
            content = CASE WHEN $9 THEN $10 ELSE ps.content END,
            updated_at = now()
        FROM events e
        WHERE ps.id = $11
          AND ps.event_id = $12
          AND e.id = ps.event_id
          AND e.owner_id = $13
        RETURNING
            ps.id,
            ps.event_id,
            ps.section_type,
            ps.position,
            ps.is_enabled,
            ps.title,
            ps.content,
            ps.created_at,
            ps.updated_at
        "#,
    )
    .bind(changes.section_type.is_some())
    .bind(&changes.section_type)
    .bind(changes.position.is_some())
    .bind(changes.position)
    .bind(changes.is_enabled.is_some())
    .bind(changes.is_enabled)
    .bind(changes.title.is_some())
    .bind(changes.title.clone().flatten())
    .bind(changes.content.is_some())
    .bind(&changes.content)
    .bind(id)
    .bind(event_id)
    .bind(owner_id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_for_owner(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    id: Uuid,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM page_sections ps
        USING events e
        WHERE ps.id = $1
          AND ps.event_id = $2
          AND e.id = ps.event_id
          AND e.owner_id = $3
        "#,
    )
    .bind(id)
    .bind(event_id)
    .bind(owner_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
