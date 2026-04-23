use sqlx::PgPool;

use crate::{
    common::error::AppError,
    dto::public::{
        PublicEventPageResponse, PublicGuestContextResponse, PublicPageSectionResponse,
        PublicRsvpFormResponse, PublicRsvpQuestionResponse,
    },
    repos::{guests_repo, pages_repo, rsvp_repo},
};

pub async fn get_public_page_by_slug(
    pool: &PgPool,
    slug: &str,
    invite_token: Option<&str>,
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

    let rsvp = match rsvp_repo::find_form_by_event(pool, event.id).await? {
        Some(form) => {
            let questions = rsvp_repo::list_questions_by_form(pool, form.id)
                .await?
                .into_iter()
                .map(PublicRsvpQuestionResponse::from)
                .collect::<Vec<PublicRsvpQuestionResponse>>();
            PublicRsvpFormResponse::from_parts(form, questions)
        }
        None => PublicRsvpFormResponse::empty(),
    };

    let guest = if let Some(token) = invite_token {
        guests_repo::find_by_event_and_invite_token(pool, event.id, token)
            .await?
            .map(|guest| PublicGuestContextResponse {
                id: guest.id,
                invite_token: guest.invite_token,
                full_name: guest.full_name,
                group_name: guest.group_name,
                plus_one_allowed: guest.plus_one_allowed,
                is_child: guest.is_child,
                vip: guest.vip,
            })
    } else {
        None
    };

    Ok(Some(
        PublicEventPageResponse::from_parts(event, sections, rsvp).with_guest(guest),
    ))
}
