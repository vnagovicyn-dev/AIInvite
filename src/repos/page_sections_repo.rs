use serde_json::Value;
use sqlx::{Acquire, PgPool};
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

pub async fn list_ids_for_event(pool: &PgPool, event_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error> {
    let rows = sqlx::query_scalar::<_, Uuid>(
        r#"
        SELECT id
        FROM page_sections
        WHERE event_id = $1
        ORDER BY position ASC, created_at ASC
        "#,
    )
    .bind(event_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn next_position_for_event(pool: &PgPool, event_id: Uuid) -> Result<i32, sqlx::Error> {
    let next_position = sqlx::query_scalar::<_, i32>(
        r#"
        SELECT COALESCE(MAX(position), 0) + 1
        FROM page_sections
        WHERE event_id = $1
        "#,
    )
    .bind(event_id)
    .fetch_one(pool)
    .await?;

    Ok(next_position)
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
            is_enabled = CASE WHEN $1 THEN $2 ELSE ps.is_enabled END,
            title = CASE WHEN $3 THEN $4 ELSE ps.title END,
            content = CASE WHEN $5 THEN $6 ELSE ps.content END,
            updated_at = now()
        FROM events e
        WHERE ps.id = $7
          AND ps.event_id = $8
          AND e.id = ps.event_id
          AND e.owner_id = $9
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
    let mut transaction = pool.begin().await?;
    let conn = transaction.acquire().await?;

    let deleted_position = sqlx::query_scalar::<_, i32>(
        r#"
        DELETE FROM page_sections ps
        USING events e
        WHERE ps.id = $1
          AND ps.event_id = $2
          AND e.id = ps.event_id
          AND e.owner_id = $3
        RETURNING ps.position
        "#,
    )
    .bind(id)
    .bind(event_id)
    .bind(owner_id)
    .fetch_optional(&mut *conn)
    .await?;

    let Some(position) = deleted_position else {
        transaction.commit().await?;
        return Ok(false);
    };

    sqlx::query(
        r#"
        UPDATE page_sections
        SET
            position = position - 1,
            updated_at = now()
        WHERE event_id = $1
          AND position > $2
        "#,
    )
    .bind(event_id)
    .bind(position)
    .execute(&mut *conn)
    .await?;

    transaction.commit().await?;
    Ok(true)
}

pub async fn reorder_for_owner(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    section_ids: &[Uuid],
) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    let conn = transaction.acquire().await?;

    for (index, section_id) in section_ids.iter().enumerate() {
        let rows_affected = sqlx::query(
            r#"
            UPDATE page_sections ps
            SET
                position = $1,
                updated_at = now()
            FROM events e
            WHERE ps.id = $2
              AND ps.event_id = $3
              AND e.id = ps.event_id
              AND e.owner_id = $4
            "#,
        )
        .bind(-((index as i32) + 1))
        .bind(section_id)
        .bind(event_id)
        .bind(owner_id)
        .execute(&mut *conn)
        .await?
        .rows_affected();

        if rows_affected != 1 {
            transaction.rollback().await?;
            return Err(sqlx::Error::RowNotFound);
        }
    }

    for (index, section_id) in section_ids.iter().enumerate() {
        sqlx::query(
            r#"
            UPDATE page_sections
            SET
                position = $1,
                updated_at = now()
            WHERE id = $2
              AND event_id = $3
            "#,
        )
        .bind((index as i32) + 1)
        .bind(section_id)
        .bind(event_id)
        .execute(&mut *conn)
        .await?;
    }

    transaction.commit().await?;
    Ok(())
}
