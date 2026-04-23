use serde::Serialize;
use serde_json::Value;
use time::format_description::well_known::Rfc3339;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::{
    events::Event,
    page_sections::PageSection,
    rsvp::{RsvpForm, RsvpQuestion},
};

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct PublicEventPageResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub event_type: String,
    pub status: String,
    pub event_date: Option<String>,
    pub timezone: String,
    pub venue_name: Option<String>,
    pub venue_address: Option<String>,
    pub sections: Vec<PublicPageSectionResponse>,
    pub rsvp: PublicRsvpFormResponse,
    pub guest: Option<PublicGuestContextResponse>,
}

impl PublicEventPageResponse {
    pub fn from_parts(
        event: Event,
        sections: Vec<PublicPageSectionResponse>,
        rsvp: PublicRsvpFormResponse,
    ) -> Self {
        Self {
            id: event.id,
            title: event.title,
            slug: event.slug,
            event_type: event.event_type,
            status: event.status,
            event_date: event
                .event_date
                .and_then(|value| value.format(&Rfc3339).ok()),
            timezone: event.timezone,
            venue_name: event.venue_name,
            venue_address: event.venue_address,
            sections,
            rsvp,
            guest: None,
        }
    }

    pub fn with_guest(mut self, guest: Option<PublicGuestContextResponse>) -> Self {
        self.guest = guest;
        self
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct PublicPageSectionResponse {
    pub id: Uuid,
    pub section_type: String,
    pub position: i32,
    pub is_enabled: bool,
    pub title: Option<String>,
    pub content: Value,
}

impl From<PageSection> for PublicPageSectionResponse {
    fn from(section: PageSection) -> Self {
        Self {
            id: section.id,
            section_type: section.section_type,
            position: section.position,
            is_enabled: section.is_enabled,
            title: section.title,
            content: section.content,
        }
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct PublicRsvpFormResponse {
    pub title: String,
    pub description: Option<String>,
    pub deadline_at: Option<String>,
    #[schema(value_type = Object)]
    pub settings: Value,
    pub questions: Vec<PublicRsvpQuestionResponse>,
}

impl PublicRsvpFormResponse {
    pub fn empty() -> Self {
        Self {
            title: "Подтверждение участия".to_string(),
            description: None,
            deadline_at: None,
            settings: Value::Object(Default::default()),
            questions: Vec::new(),
        }
    }

    pub fn from_parts(form: RsvpForm, questions: Vec<PublicRsvpQuestionResponse>) -> Self {
        Self {
            title: form.title,
            description: form.description,
            deadline_at: form
                .deadline_at
                .and_then(|value| value.format(&Rfc3339).ok()),
            settings: form.settings,
            questions,
        }
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct PublicRsvpQuestionResponse {
    pub id: Uuid,
    pub position: i32,
    pub code: String,
    pub label: String,
    pub question_type: String,
    pub required: bool,
    #[schema(value_type = Value)]
    pub options: Value,
}

impl From<RsvpQuestion> for PublicRsvpQuestionResponse {
    fn from(question: RsvpQuestion) -> Self {
        Self {
            id: question.id,
            position: question.position,
            code: question.code,
            label: question.label,
            question_type: question.question_type,
            required: question.required,
            options: question.options,
        }
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct PublicGuestContextResponse {
    pub id: Uuid,
    pub invite_token: String,
    pub full_name: String,
    pub group_name: Option<String>,
    pub plus_one_allowed: bool,
    pub is_child: bool,
    pub vip: bool,
}
