use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{events::Event, page_sections::PageSection};

pub async fn find_published_event_by_slug(
    pool: &PgPool,
    slug: &str,
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
        WHERE slug = $1 AND status = 'published'
        "#,
    )
    .bind(slug.trim())
    .fetch_optional(pool)
    .await
}

pub async fn list_enabled_sections_for_event(
    pool: &PgPool,
    event_id: Uuid,
) -> Result<Vec<PageSection>, sqlx::Error> {
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
          AND is_enabled = TRUE
        ORDER BY position ASC, created_at ASC
        "#,
    )
    .bind(event_id)
    .fetch_all(pool)
    .await
}
