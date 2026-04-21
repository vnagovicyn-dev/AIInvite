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
    format!("guests-{}@example.com", Uuid::new_v4())
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
                        "full_name": "Guest Manager"
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

async fn create_guest(app: &axum::Router, token: &str, event_id: &str, payload: Value) -> Value {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/events/{event_id}/guests"))
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

fn multipart_body(csv: &str, boundary: &str) -> String {
    format!(
        "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"guests.csv\"\r\nContent-Type: text/csv\r\n\r\n{csv}\r\n--{boundary}--\r\n"
    )
}

#[tokio::test]
async fn create_guest_works() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let event = create_event(&app, &token, "Guests Event").await;
    let event_id = event["id"].as_str().unwrap();

    let guest = create_guest(
        &app,
        &token,
        event_id,
        json!({
            "full_name": "Anna Smith",
            "email": "anna@example.com",
            "group_name": "Family",
            "vip": true,
            "tags": ["close-family", "ceremony"]
        }),
    )
    .await;

    assert_eq!(guest["full_name"], "Anna Smith");
    assert_eq!(guest["email"], "anna@example.com");
    assert_eq!(guest["group_name"], "Family");
    assert_eq!(guest["vip"], true);
}

#[tokio::test]
async fn list_search_and_filter_guests_work() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let event = create_event(&app, &token, "Guest Filters Event").await;
    let event_id = event["id"].as_str().unwrap();

    let _ = create_guest(
        &app,
        &token,
        event_id,
        json!({
            "full_name": "Anna Smith",
            "group_name": "Family",
            "vip": true
        }),
    )
    .await;
    let _ = create_guest(
        &app,
        &token,
        event_id,
        json!({
            "full_name": "Igor Brown",
            "group_name": "Friends",
            "vip": false
        }),
    )
    .await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/api/events/{event_id}/guests?page=1&per_page=10&search=Anna&vip=true&group_name=Family"
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
    assert_eq!(body["total"], 1);
    assert_eq!(body["items"][0]["full_name"], "Anna Smith");
}

#[tokio::test]
async fn update_guest_works() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let event = create_event(&app, &token, "Guest Update Event").await;
    let event_id = event["id"].as_str().unwrap();
    let guest = create_guest(
        &app,
        &token,
        event_id,
        json!({
            "full_name": "Guest Original",
            "email": "guest@example.com"
        }),
    )
    .await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!(
                    "/api/events/{event_id}/guests/{}",
                    guest["id"].as_str().unwrap()
                ))
                .header("authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "full_name": "Guest Updated",
                        "email": null,
                        "vip": true
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
    assert_eq!(body["full_name"], "Guest Updated");
    assert!(body["email"].is_null());
    assert_eq!(body["vip"], true);
}

#[tokio::test]
async fn delete_guest_works() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let event = create_event(&app, &token, "Guest Delete Event").await;
    let event_id = event["id"].as_str().unwrap();
    let guest = create_guest(
        &app,
        &token,
        event_id,
        json!({
            "full_name": "Delete Me"
        }),
    )
    .await;

    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!(
                    "/api/events/{event_id}/guests/{}",
                    guest["id"].as_str().unwrap()
                ))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);

    let get_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/api/events/{event_id}/guests/{}",
                    guest["id"].as_str().unwrap()
                ))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn import_guests_csv_returns_summary() {
    let app = test_app().await;
    let token = register_and_login(&app).await;
    let event = create_event(&app, &token, "Guest Import Event").await;
    let event_id = event["id"].as_str().unwrap();
    let boundary = "X-BOUNDARY";
    let csv = "full_name,phone,email,group_name,vip\nAnna Smith,+111,anna@example.com,Family,true\nBroken Row,,not-an-email,Friends,false\nIgor Brown,+222,igor@example.com,Friends,no";

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/events/{event_id}/guests/import"))
                .header("authorization", format!("Bearer {token}"))
                .header(
                    "content-type",
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .body(Body::from(multipart_body(csv, boundary)))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body["imported_count"], 2);
    assert_eq!(body["skipped_count"], 1);
    assert_eq!(body["errors"].as_array().unwrap().len(), 1);
}
