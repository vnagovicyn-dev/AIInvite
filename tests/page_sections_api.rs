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
    format!("sections-{}@example.com", Uuid::new_v4())
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
                        "full_name": "Sections User"
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

    let login_body: Value = serde_json::from_slice(
        &to_bytes(login_response.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();

    login_body["access_token"].as_str().unwrap().to_string()
}

async fn create_event(app: &axum::Router, token: &str, title: &str) -> Value {
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
                        "title": title,
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

#[tokio::test]
async fn create_and_get_page_section_work() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let event = create_event(&app, &token, "Section Event").await;
    let event_id = event["id"].as_str().unwrap();

    let created = create_section(
        &app,
        &token,
        event_id,
        json!({
            "section_type": "hero",
            "title": "Welcome",
            "content": {
                "headline": "Join us"
            }
        }),
    )
    .await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/api/events/{event_id}/sections/{}",
                    created["id"].as_str().unwrap()
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

    assert_eq!(body["section_type"], "hero");
    assert_eq!(body["position"], 1);
    assert_eq!(body["title"], "Welcome");
}

#[tokio::test]
async fn list_page_sections_returns_sorted_items() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let event = create_event(&app, &token, "Sections List Event").await;
    let event_id = event["id"].as_str().unwrap();

    let _ = create_section(
        &app,
        &token,
        event_id,
        json!({
            "section_type": "program",
            "content": {"items": []}
        }),
    )
    .await;

    tokio::time::sleep(Duration::from_millis(5)).await;

    let _first = create_section(
        &app,
        &token,
        event_id,
        json!({
            "section_type": "hero",
            "content": {"headline": "Hello"}
        }),
    )
    .await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/events/{event_id}/sections"))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();

    assert_eq!(body["items"].as_array().unwrap().len(), 2);
    assert_eq!(body["items"][0]["position"], 1);
    assert_eq!(body["items"][1]["position"], 2);
}

#[tokio::test]
async fn update_page_section_allows_null_title_and_updates_content() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let event = create_event(&app, &token, "Sections Update Event").await;
    let event_id = event["id"].as_str().unwrap();
    let created = create_section(
        &app,
        &token,
        event_id,
        json!({
            "section_type": "hero",
            "title": "Initial title",
            "content": {"headline": "Initial"}
        }),
    )
    .await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!(
                    "/api/events/{event_id}/sections/{}",
                    created["id"].as_str().unwrap()
                ))
                .header("authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "is_enabled": false,
                        "title": null,
                        "content": {"headline": "Updated"}
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

    assert_eq!(body["position"], 1);
    assert_eq!(body["is_enabled"], false);
    assert!(body["title"].is_null());
    assert_eq!(body["content"]["headline"], "Updated");
}

#[tokio::test]
async fn delete_page_section_removes_item() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let event = create_event(&app, &token, "Sections Delete Event").await;
    let event_id = event["id"].as_str().unwrap();
    let created = create_section(
        &app,
        &token,
        event_id,
        json!({
            "section_type": "gallery",
            "content": {"images": []}
        }),
    )
    .await;

    let second = create_section(
        &app,
        &token,
        event_id,
        json!({
            "section_type": "faq",
            "content": {"items": []}
        }),
    )
    .await;

    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!(
                    "/api/events/{event_id}/sections/{}",
                    created["id"].as_str().unwrap()
                ))
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
                .uri(format!(
                    "/api/events/{event_id}/sections/{}",
                    created["id"].as_str().unwrap()
                ))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(detail_response.status(), StatusCode::NOT_FOUND);

    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/events/{event_id}/sections"))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body: Value = serde_json::from_slice(
        &to_bytes(list_response.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(list_body["items"].as_array().unwrap().len(), 1);
    assert_eq!(list_body["items"][0]["id"], second["id"]);
    assert_eq!(list_body["items"][0]["position"], 1);
}

#[tokio::test]
async fn reorder_sections_normalizes_positions() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let event = create_event(&app, &token, "Sections Reorder Event").await;
    let event_id = event["id"].as_str().unwrap();

    let first = create_section(
        &app,
        &token,
        event_id,
        json!({
            "section_type": "hero",
            "content": {"headline": "First"}
        }),
    )
    .await;
    let second = create_section(
        &app,
        &token,
        event_id,
        json!({
            "section_type": "program",
            "content": {"items": []}
        }),
    )
    .await;
    let third = create_section(
        &app,
        &token,
        event_id,
        json!({
            "section_type": "faq",
            "content": {"items": []}
        }),
    )
    .await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/events/{event_id}/sections/reorder"))
                .header("authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "section_ids": [
                            third["id"],
                            first["id"],
                            second["id"]
                        ]
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

    assert_eq!(body["items"][0]["id"], third["id"]);
    assert_eq!(body["items"][0]["position"], 1);
    assert_eq!(body["items"][1]["id"], first["id"]);
    assert_eq!(body["items"][1]["position"], 2);
    assert_eq!(body["items"][2]["id"], second["id"]);
    assert_eq!(body["items"][2]["position"], 3);
}

#[tokio::test]
async fn other_user_cannot_access_foreign_page_section() {
    let app = test_app().await;
    let owner_token = register_and_login(&app).await;
    let other_token = register_and_login(&app).await;
    let event = create_event(&app, &owner_token, "Private Sections Event").await;
    let event_id = event["id"].as_str().unwrap();
    let created = create_section(
        &app,
        &owner_token,
        event_id,
        json!({
            "section_type": "hero",
            "content": {"headline": "Private"}
        }),
    )
    .await;

    let detail_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/api/events/{event_id}/sections/{}",
                    created["id"].as_str().unwrap()
                ))
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
                .uri(format!(
                    "/api/events/{event_id}/sections/{}",
                    created["id"].as_str().unwrap()
                ))
                .header("authorization", format!("Bearer {other_token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "is_enabled": false
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
                .uri(format!(
                    "/api/events/{event_id}/sections/{}",
                    created["id"].as_str().unwrap()
                ))
                .header("authorization", format!("Bearer {other_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(delete_response.status(), StatusCode::NOT_FOUND);

    let reorder_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/events/{event_id}/sections/reorder"))
                .header("authorization", format!("Bearer {other_token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "section_ids": [created["id"]]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(reorder_response.status(), StatusCode::NOT_FOUND);
}
