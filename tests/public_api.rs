use aiinvite::{
    app::{config::Config, router::build_router, state::AppState},
    common::auth::JwtSettings,
    db::pool,
};
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde_json::{json, Value};
use tower::util::ServiceExt;
use uuid::Uuid;

async fn test_app() -> axum::Router {
    dotenvy::dotenv().ok();
    if std::env::var("JWT_SECRET").is_err() {
        std::env::set_var("JWT_SECRET", "test-secret");
    }
    if std::env::var("JWT_EXPIRES_IN_MINUTES").is_err() {
        std::env::set_var("JWT_EXPIRES_IN_MINUTES", "60");
    }
    let config = Config::from_env().expect("config should load");
    let pool = pool::create_pool(&config.database_url)
        .await
        .expect("pool should connect");
    pool::run_migrations(&pool)
        .await
        .expect("migrations should run");

    let state = AppState::new(
        config.app_name,
        pool,
        JwtSettings::new(config.jwt_secret, config.jwt_expires_in_minutes),
    );

    build_router(state)
}

fn unique_email() -> String {
    format!("public-{}@example.com", Uuid::new_v4())
}

async fn register_and_login(app: &axum::Router) -> String {
    let email = unique_email();

    let register_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "email": email,
                        "password": "password123",
                        "full_name": "Public User"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(register_response.status(), StatusCode::CREATED);

    let login_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "email": email,
                        "password": "password123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(login_response.status(), StatusCode::OK);

    let login_body: Value = serde_json::from_slice(
        &to_bytes(login_response.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();

    login_body["access_token"].as_str().unwrap().to_string()
}

async fn create_event(app: &axum::Router, token: &str, slug: &str) -> Value {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/events")
                .header("authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "title": "Public Event",
                        "slug": slug,
                        "event_type": "wedding",
                        "venue_name": "Grand Hall",
                        "venue_address": "Amsterdam"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap()
}

async fn create_section(app: &axum::Router, token: &str, event_id: &str, payload: Value) -> Value {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/events/{event_id}/sections"))
                .header("authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap()
}

async fn create_guest(app: &axum::Router, token: &str, event_id: &str, full_name: &str) -> Value {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/events/{event_id}/guests"))
                .header("authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "full_name": full_name
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap()
}

async fn upsert_rsvp_form(
    app: &axum::Router,
    token: &str,
    event_id: &str,
    payload: Value,
) -> Value {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/events/{event_id}/rsvp-form"))
                .header("authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap()
}

#[tokio::test]
async fn publish_changes_event_status_to_published() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let slug = format!(
        "publish-event-{}",
        &Uuid::new_v4().simple().to_string()[..8]
    );
    let event = create_event(&app, &token, &slug).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/events/{}/publish",
                    event["id"].as_str().unwrap()
                ))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body["status"], "published");
}

#[tokio::test]
async fn unpublish_changes_event_status_back_to_draft() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let slug = format!(
        "unpublish-event-{}",
        &Uuid::new_v4().simple().to_string()[..8]
    );
    let event = create_event(&app, &token, &slug).await;

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/events/{}/publish",
                    event["id"].as_str().unwrap()
                ))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/events/{}/unpublish",
                    event["id"].as_str().unwrap()
                ))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body["status"], "draft");
}

#[tokio::test]
async fn public_endpoint_returns_published_event_with_enabled_sections() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let slug = format!("public-page-{}", &Uuid::new_v4().simple().to_string()[..8]);
    let event = create_event(&app, &token, &slug).await;
    let event_id = event["id"].as_str().unwrap();
    let guest = create_guest(&app, &token, event_id, "Anna Smith").await;

    let _ = create_section(
        &app,
        &token,
        event_id,
        json!({
            "section_type": "program",
            "is_enabled": false,
            "title": "Private draft block",
            "content": {"items": []}
        }),
    )
    .await;

    let _ = create_section(
        &app,
        &token,
        event_id,
        json!({
            "section_type": "hero",
            "title": "Welcome",
            "content": {"headline": "Join us"}
        }),
    )
    .await;

    let _ = upsert_rsvp_form(
        &app,
        &token,
        event_id,
        json!({
            "title": "Подтвердите участие",
            "description": "Ответьте до конца недели",
            "settings": {
                "submit_label": "Отправить ответ"
            },
            "questions": [
                {
                    "code": "attendance",
                    "label": "Сможете прийти?",
                    "question_type": "select",
                    "required": true,
                    "options": ["yes", "no", "maybe"]
                },
                {
                    "code": "comment",
                    "label": "Комментарий",
                    "question_type": "textarea"
                }
            ]
        }),
    )
    .await;

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/events/{event_id}/publish"))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/api/public/{slug}?invite_token={}",
                    guest["invite_token"].as_str().unwrap()
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();

    assert_eq!(body["slug"], slug);
    assert_eq!(body["status"], "published");
    assert_eq!(body["sections"].as_array().unwrap().len(), 1);
    assert_eq!(body["sections"][0]["section_type"], "hero");
    assert_eq!(body["sections"][0]["position"], 2);
    assert_eq!(body["sections"][0]["is_enabled"], true);
    assert_eq!(body["rsvp"]["title"], "Подтвердите участие");
    assert_eq!(body["rsvp"]["questions"].as_array().unwrap().len(), 2);
    assert_eq!(body["rsvp"]["questions"][0]["code"], "attendance");
    assert_eq!(body["rsvp"]["questions"][0]["required"], true);
    assert_eq!(body["rsvp"]["settings"]["submit_label"], "Отправить ответ");
    assert_eq!(body["guest"]["full_name"], "Anna Smith");
    assert_eq!(body["guest"]["invite_token"], guest["invite_token"]);
}

#[tokio::test]
async fn public_endpoint_returns_404_for_unpublished_event() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let slug = format!("draft-page-{}", &Uuid::new_v4().simple().to_string()[..8]);
    let _ = create_event(&app, &token, &slug).await;

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/public/{slug}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn public_endpoint_returns_empty_rsvp_shape_when_form_missing() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let slug = format!(
        "public-no-form-{}",
        &Uuid::new_v4().simple().to_string()[..8]
    );
    let event = create_event(&app, &token, &slug).await;

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/events/{}/publish",
                    event["id"].as_str().unwrap()
                ))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/public/{slug}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();

    assert_eq!(body["rsvp"]["title"], "Подтверждение участия");
    assert_eq!(body["rsvp"]["questions"].as_array().unwrap().len(), 0);
    assert!(body["guest"].is_null());
}

#[tokio::test]
async fn public_endpoint_ignores_unknown_invite_token() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let slug = format!(
        "public-bad-token-{}",
        &Uuid::new_v4().simple().to_string()[..8]
    );
    let event = create_event(&app, &token, &slug).await;

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/events/{}/publish",
                    event["id"].as_str().unwrap()
                ))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/public/{slug}?invite_token=unknown-token"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert!(body["guest"].is_null());
}
