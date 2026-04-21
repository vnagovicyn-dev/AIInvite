use sqlx::PgPool;

use crate::{
    common::error::AppError,
    dto::public::{PublicEventPageResponse, PublicPageSectionResponse},
    repos::pages_repo,
};

pub async fn get_public_page_by_slug(
    pool: &PgPool,
    slug: &str,
) -> Result<Option<PublicEventPageResponse>, AppError> {
    let event = match pages_repo::find_published_event_by_slug(pool, slug).await? {
        Some(event) => event,
        None => return Ok(None),
    };

    let sections = pages_repo::list_enabled_sections_for_event(pool, event.id)
        .await?
        .into_iter()
        .map(PublicPageSectionResponse::from)
        .collect::<Vec<PublicPageSectionResponse>>();

    Ok(Some(PublicEventPageResponse::from_parts(event, sections)))
}
