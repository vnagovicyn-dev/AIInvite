use std::time::Duration;

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

const TEMPLATE_ID: &str = "11111111-1111-1111-1111-111111111111";

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
    format!("events-{}@example.com", Uuid::new_v4())
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
                        "full_name": "Events User"
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

async fn create_event(app: &axum::Router, token: &str, payload: Value) -> Value {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/events")
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

#[tokio::test]
async fn create_event_returns_defaults_and_generated_slug() {
    let app = test_app().await;
    let token = register_and_login(&app).await;

    let body = create_event(
        &app,
        &token,
        json!({
            "title": "Spring Wedding",
            "event_type": "wedding",
            "template_id": TEMPLATE_ID
        }),
    )
    .await;

    assert_eq!(body["title"], "Spring Wedding");
    assert_eq!(body["event_type"], "wedding");
    assert_eq!(body["status"], "draft");
    assert_eq!(body["timezone"], "Europe/Amsterdam");
    assert_eq!(body["template_id"], TEMPLATE_ID);
    assert!(body["slug"]
        .as_str()
        .unwrap()
        .starts_with("spring-wedding-"));
}

#[tokio::test]
async fn get_event_detail_returns_owned_event() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let custom_slug = format!(
        "birthday-bash-{}",
        &Uuid::new_v4().simple().to_string()[..8]
    );
    let created = create_event(
        &app,
        &token,
        json!({
            "title": "Birthday Bash",
            "slug": custom_slug,
            "event_type": "birthday",
            "venue_name": "Main Hall"
        }),
    )
    .await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/events/{}", created["id"].as_str().unwrap()))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();

    assert_eq!(body["id"], created["id"]);
    assert_eq!(body["slug"], created["slug"]);
    assert_eq!(body["venue_name"], "Main Hall");
}

#[tokio::test]
async fn list_events_returns_only_current_user_items() {
    let app = test_app().await;
    let owner_token = register_and_login(&app).await;
    let other_token = register_and_login(&app).await;

    let first = create_event(
        &app,
        &owner_token,
        json!({
            "title": "Owner Event One",
            "event_type": "corporate"
        }),
    )
    .await;

    tokio::time::sleep(Duration::from_millis(5)).await;

    let second = create_event(
        &app,
        &owner_token,
        json!({
            "title": "Owner Event Two",
            "event_type": "corporate"
        }),
    )
    .await;

    let _ = create_event(
        &app,
        &other_token,
        json!({
            "title": "Other User Event",
            "event_type": "birthday"
        }),
    )
    .await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/events?page=1&per_page=10&status=draft")
                .header("authorization", format!("Bearer {owner_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();

    assert_eq!(body["total"], 2);
    assert_eq!(body["items"].as_array().unwrap().len(), 2);
    assert_eq!(body["items"][0]["id"], second["id"]);
    assert_eq!(body["items"][1]["id"], first["id"]);
}

#[tokio::test]
async fn update_event_updates_owned_event() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let created = create_event(
        &app,
        &token,
        json!({
            "title": "Original Event",
            "event_type": "wedding",
            "venue_name": "Old Venue"
        }),
    )
    .await;

    tokio::time::sleep(Duration::from_millis(5)).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/api/events/{}", created["id"].as_str().unwrap()))
                .header("authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "title": "Updated Event",
                        "timezone": "Europe/Berlin",
                        "venue_name": null,
                        "venue_address": "New Address 123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();

    assert_eq!(body["title"], "Updated Event");
    assert_eq!(body["timezone"], "Europe/Berlin");
    assert!(body["venue_name"].is_null());
    assert_eq!(body["venue_address"], "New Address 123");
    assert_ne!(body["updated_at"], created["updated_at"]);
}

#[tokio::test]
async fn delete_event_removes_owned_event() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let created = create_event(
        &app,
        &token,
        json!({
            "title": "Disposable Event",
            "event_type": "baby"
        }),
    )
    .await;

    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/events/{}", created["id"].as_str().unwrap()))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);

    let detail_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/events/{}", created["id"].as_str().unwrap()))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(detail_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn other_user_cannot_access_update_or_delete_event() {
    let app = test_app().await;
    let owner_token = register_and_login(&app).await;
    let other_token = register_and_login(&app).await;
    let created = create_event(
        &app,
        &owner_token,
        json!({
            "title": "Private Event",
            "event_type": "anniversary"
        }),
    )
    .await;

    let detail_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/events/{}", created["id"].as_str().unwrap()))
                .header("authorization", format!("Bearer {other_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(detail_response.status(), StatusCode::NOT_FOUND);

    let update_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/api/events/{}", created["id"].as_str().unwrap()))
                .header("authorization", format!("Bearer {other_token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "title": "Hacked Title"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(update_response.status(), StatusCode::NOT_FOUND);

    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/events/{}", created["id"].as_str().unwrap()))
                .header("authorization", format!("Bearer {other_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(delete_response.status(), StatusCode::NOT_FOUND);
}
