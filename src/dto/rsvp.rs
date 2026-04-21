use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::format_description::well_known::Rfc3339;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::domain::rsvp::{RsvpForm, RsvpQuestion, RsvpResponse};

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpsertRsvpFormRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub deadline_at: Option<String>,
    #[schema(value_type = Object)]
    pub settings: Option<Value>,
    pub questions: Vec<RsvpQuestionInput>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RsvpQuestionInput {
    pub code: String,
    pub label: String,
    pub question_type: String,
    pub required: Option<bool>,
    #[schema(value_type = Value, example = json!(["Да", "Нет"]))]
    pub options: Option<Value>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct RsvpFormResponse {
    pub id: Uuid,
    pub event_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub deadline_at: Option<String>,
    #[schema(value_type = Object)]
    pub settings: Value,
    pub questions: Vec<RsvpQuestionResponse>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct RsvpQuestionResponse {
    pub id: Uuid,
    pub position: i32,
    pub code: String,
    pub label: String,
    pub question_type: String,
    pub required: bool,
    #[schema(value_type = Value)]
    pub options: Value,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(default)]
pub struct RsvpResponsesListQuery {
    #[param(example = 1, minimum = 1)]
    pub page: u32,
    #[param(example = 20, minimum = 1, maximum = 100)]
    pub per_page: u32,
    #[param(example = "confirmed")]
    pub status: Option<String>,
}

impl Default for RsvpResponsesListQuery {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
            status: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct RsvpResponseItem {
    pub id: Uuid,
    pub event_id: Uuid,
    pub guest_id: Option<Uuid>,
    pub status: String,
    pub plus_one_count: i32,
    #[schema(value_type = Object)]
    pub answers: Value,
    pub submitted_at: Option<String>,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct RsvpResponsesAggregates {
    pub total: i64,
    pub confirmed: i64,
    pub declined: i64,
    pub maybe: i64,
    pub pending: i64,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct RsvpResponsesListResponse {
    pub items: Vec<RsvpResponseItem>,
    pub page: u32,
    pub per_page: u32,
    pub total: i64,
    pub aggregates: RsvpResponsesAggregates,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PublicSubmitRsvpRequest {
    pub status: String,
    pub plus_one_count: Option<i32>,
    pub guest_id: Option<Uuid>,
    #[schema(value_type = Object)]
    pub answers: Value,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct PublicSubmitRsvpResponse {
    pub id: Uuid,
    pub status: String,
    pub submitted_at: String,
}

impl RsvpFormResponse {
    pub fn from_parts(form: RsvpForm, questions: Vec<RsvpQuestionResponse>) -> Self {
        Self {
            id: form.id,
            event_id: form.event_id,
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

impl From<RsvpQuestion> for RsvpQuestionResponse {
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

impl From<RsvpResponse> for RsvpResponseItem {
    fn from(response: RsvpResponse) -> Self {
        Self {
            id: response.id,
            event_id: response.event_id,
            guest_id: response.guest_id,
            status: response.status,
            plus_one_count: response.plus_one_count,
            answers: response.answers,
            submitted_at: response
                .submitted_at
                .and_then(|value| value.format(&Rfc3339).ok()),
            created_at: response
                .created_at
                .format(&Rfc3339)
                .unwrap_or_else(|_| response.created_at.to_string()),
        }
    }
}
