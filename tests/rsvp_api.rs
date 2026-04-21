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
    format!("rsvp-{}@example.com", Uuid::new_v4())
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
                        "full_name": "RSVP User"
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
                        "title": "RSVP Event",
                        "slug": slug,
                        "event_type": "wedding"
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

async fn publish_event(app: &axum::Router, token: &str, event_id: &str) {
    let response = app
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
    assert_eq!(response.status(), StatusCode::OK);
}

async fn put_form(app: &axum::Router, token: &str, event_id: &str, payload: Value) -> Value {
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

async fn submit_public_rsvp(
    app: &axum::Router,
    slug: &str,
    payload: Value,
) -> axum::http::Response<Body> {
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/public/{slug}/rsvp"))
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn create_and_get_rsvp_form_work() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let slug = format!("rsvp-form-{}", &Uuid::new_v4().simple().to_string()[..8]);
    let event = create_event(&app, &token, &slug).await;
    let event_id = event["id"].as_str().unwrap();

    let updated = put_form(
        &app,
        &token,
        event_id,
        json!({
            "title": "Подтверждение участия на свадьбу",
            "description": "Ответьте до конца месяца",
            "deadline_at": "2030-05-01T12:00:00Z",
            "settings": {
                "show_guest_name_hint": true
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

    assert_eq!(updated["title"], "Подтверждение участия на свадьбу");
    assert_eq!(updated["questions"].as_array().unwrap().len(), 2);
    assert_eq!(updated["questions"][0]["position"], 1);

    let get_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/events/{event_id}/rsvp-form"))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(get_response.status(), StatusCode::OK);

    let body: Value = serde_json::from_slice(
        &to_bytes(get_response.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(body["title"], "Подтверждение участия на свадьбу");
    assert_eq!(body["questions"][1]["code"], "comment");
}

#[tokio::test]
async fn public_rsvp_submit_works() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let slug = format!("public-rsvp-{}", &Uuid::new_v4().simple().to_string()[..8]);
    let event = create_event(&app, &token, &slug).await;
    let event_id = event["id"].as_str().unwrap();
    let guest = create_guest(&app, &token, event_id, "Anna Smith").await;

    let _ = put_form(
        &app,
        &token,
        event_id,
        json!({
            "questions": [
                {
                    "code": "attendance",
                    "label": "Сможете прийти?",
                    "question_type": "select",
                    "required": true,
                    "options": ["yes", "no", "maybe"]
                }
            ]
        }),
    )
    .await;
    publish_event(&app, &token, event_id).await;

    let response = submit_public_rsvp(
        &app,
        &slug,
        json!({
            "status": "confirmed",
            "guest_id": guest["id"],
            "plus_one_count": 1,
            "answers": {
                "attendance": "yes"
            }
        }),
    )
    .await;
    assert_eq!(response.status(), StatusCode::CREATED);

    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body["status"], "confirmed");
    assert!(body["id"].is_string());
}

#[tokio::test]
async fn public_rsvp_submit_validates_required_questions() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let slug = format!(
        "required-rsvp-{}",
        &Uuid::new_v4().simple().to_string()[..8]
    );
    let event = create_event(&app, &token, &slug).await;
    let event_id = event["id"].as_str().unwrap();

    let _ = put_form(
        &app,
        &token,
        event_id,
        json!({
            "questions": [
                {
                    "code": "attendance",
                    "label": "Сможете прийти?",
                    "question_type": "select",
                    "required": true,
                    "options": ["yes", "no", "maybe"]
                }
            ]
        }),
    )
    .await;
    publish_event(&app, &token, event_id).await;

    let response = submit_public_rsvp(
        &app,
        &slug,
        json!({
            "status": "confirmed",
            "answers": {}
        }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn public_rsvp_submit_fails_for_expired_deadline() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let slug = format!("expired-rsvp-{}", &Uuid::new_v4().simple().to_string()[..8]);
    let event = create_event(&app, &token, &slug).await;
    let event_id = event["id"].as_str().unwrap();

    let _ = put_form(
        &app,
        &token,
        event_id,
        json!({
            "deadline_at": "2000-01-01T00:00:00Z",
            "questions": []
        }),
    )
    .await;
    publish_event(&app, &token, event_id).await;

    let response = submit_public_rsvp(
        &app,
        &slug,
        json!({
            "status": "confirmed",
            "answers": {}
        }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn list_responses_returns_aggregates() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let slug = format!("list-rsvp-{}", &Uuid::new_v4().simple().to_string()[..8]);
    let event = create_event(&app, &token, &slug).await;
    let event_id = event["id"].as_str().unwrap();
    let guest_one = create_guest(&app, &token, event_id, "Anna Smith").await;
    let _guest_two = create_guest(&app, &token, event_id, "Igor Brown").await;

    let _ = put_form(
        &app,
        &token,
        event_id,
        json!({
            "questions": [
                {
                    "code": "attendance",
                    "label": "Сможете прийти?",
                    "question_type": "select",
                    "required": true,
                    "options": ["yes", "no", "maybe"]
                }
            ]
        }),
    )
    .await;
    publish_event(&app, &token, event_id).await;

    let confirmed = submit_public_rsvp(
        &app,
        &slug,
        json!({
            "status": "confirmed",
            "guest_id": guest_one["id"],
            "answers": {
                "attendance": "yes"
            }
        }),
    )
    .await;
    assert_eq!(confirmed.status(), StatusCode::CREATED);

    let maybe = submit_public_rsvp(
        &app,
        &slug,
        json!({
            "status": "maybe",
            "answers": {
                "attendance": "maybe"
            }
        }),
    )
    .await;
    assert_eq!(maybe.status(), StatusCode::CREATED);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/api/events/{event_id}/rsvp-responses?page=1&per_page=20"
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
    assert_eq!(body["total"], 2);
    assert_eq!(body["aggregates"]["confirmed"], 1);
    assert_eq!(body["aggregates"]["maybe"], 1);
    assert_eq!(body["aggregates"]["declined"], 0);
    assert_eq!(body["aggregates"]["pending"], 1);
    assert_eq!(body["aggregates"]["total"], 2);
    assert_eq!(body["items"].as_array().unwrap().len(), 2);
}
