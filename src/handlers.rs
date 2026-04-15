use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::Html,
    Json,
};

use crate::{
    db,
    error::AppError,
    models::{
        AppState, CreateHealthcheck, DeleteResponse, HealthResponse, LeadRequestInput,
        RootResponse, UpdateHealthcheck,
    },
    site,
};

pub async fn api_root() -> Json<RootResponse> {
    Json(RootResponse {
        service: "aiinvite",
        routes: vec!["/health", "/api/healthchecks", "/api/healthchecks/:id"],
    })
}

pub async fn home() -> Html<String> {
    Html(site::home_page())
}

pub async fn about() -> Html<String> {
    Html(site::about_page())
}

pub async fn services() -> Html<String> {
    Html(site::services_page())
}

pub async fn service_details(Path(slug): Path<String>) -> Result<Html<String>, AppError> {
    let service = site::find_service(&slug)
        .ok_or_else(|| AppError::not_found(format!("service page `{slug}` not found")))?;
    Ok(Html(site::service_page(service)))
}

pub async fn portfolio() -> Html<String> {
    Html(site::portfolio_page())
}

pub async fn portfolio_case(Path(slug): Path<String>) -> Result<Html<String>, AppError> {
    let case_item = site::find_case(&slug)
        .ok_or_else(|| AppError::not_found(format!("portfolio case `{slug}` not found")))?;
    Ok(Html(site::case_page(case_item)))
}

pub async fn prices() -> Html<String> {
    Html(site::prices_page())
}

pub async fn reviews() -> Html<String> {
    Html(site::reviews_page())
}

pub async fn contacts() -> Html<String> {
    Html(site::contacts_page())
}

pub async fn blog() -> Html<String> {
    Html(site::blog_page())
}

pub async fn blog_post(Path(slug): Path<String>) -> Result<Html<String>, AppError> {
    let post = site::find_post(&slug)
        .ok_or_else(|| AppError::not_found(format!("blog article `{slug}` not found")))?;
    Ok(Html(site::blog_post_page(post)))
}

pub async fn submit_request(
    State(state): State<AppState>,
    Form(payload): Form<LeadRequestInput>,
) -> Result<Html<String>, AppError> {
    validate_lead_request(&payload)?;
    db::create_lead_request(&state.pool, &payload).await?;
    Ok(Html(site::thank_you_page()))
}

pub async fn health(State(state): State<AppState>) -> Result<Json<HealthResponse>, AppError> {
    let version = db::fetch_database_version(&state.pool).await?;

    Ok(Json(HealthResponse {
        status: "ok",
        database: "connected",
        version,
    }))
}

pub async fn list_healthchecks(
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::models::Healthcheck>>, AppError> {
    let items = db::list_healthchecks(&state.pool).await?;
    Ok(Json(items))
}

pub async fn get_healthcheck(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<crate::models::Healthcheck>, AppError> {
    let item = db::get_healthcheck(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::not_found(format!("healthcheck {id} not found")))?;

    Ok(Json(item))
}

pub async fn create_healthcheck(
    State(state): State<AppState>,
    Json(payload): Json<CreateHealthcheck>,
) -> Result<(StatusCode, Json<crate::models::Healthcheck>), AppError> {
    validate_service_name(&payload.service_name)?;
    let record = db::create_healthcheck(&state.pool, &payload).await?;
    Ok((StatusCode::CREATED, Json(record)))
}

pub async fn update_healthcheck(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateHealthcheck>,
) -> Result<Json<crate::models::Healthcheck>, AppError> {
    validate_service_name(&payload.service_name)?;
    let record = db::update_healthcheck(&state.pool, id, &payload)
        .await?
        .ok_or_else(|| AppError::not_found(format!("healthcheck {id} not found")))?;

    Ok(Json(record))
}

pub async fn delete_healthcheck(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<DeleteResponse>, AppError> {
    let deleted = db::delete_healthcheck(&state.pool, id).await?;
    if !deleted {
        return Err(AppError::not_found(format!("healthcheck {id} not found")));
    }

    Ok(Json(DeleteResponse { deleted: true }))
}

fn validate_service_name(service_name: &str) -> Result<(), AppError> {
    if service_name.trim().is_empty() {
        return Err(AppError::bad_request("service_name must not be empty"));
    }

    Ok(())
}

fn validate_lead_request(payload: &LeadRequestInput) -> Result<(), AppError> {
    if payload.name.trim().is_empty() {
        return Err(AppError::bad_request("name must not be empty"));
    }

    if payload.phone.trim().is_empty() {
        return Err(AppError::bad_request("phone must not be empty"));
    }

    if payload.event_date.trim().is_empty() {
        return Err(AppError::bad_request("event_date must not be empty"));
    }

    if payload.event_type.trim().is_empty() {
        return Err(AppError::bad_request("event_type must not be empty"));
    }

    Ok(())
}
