use aiinvite::{
    app::{config::Config, router::build_router, state::AppState},
    common::auth::JwtSettings,
    db::pool,
};
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde_json::Value;
use tower::util::ServiceExt;

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

#[tokio::test]
async fn templates_list_returns_paginated_items() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/templates?page=1&per_page=3")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();

    assert_eq!(body["page"], 1);
    assert_eq!(body["per_page"], 3);
    assert_eq!(body["items"].as_array().unwrap().len(), 3);
    assert!(body["total"].as_i64().unwrap() >= 10);
    assert_eq!(body["items"][0]["is_active"], true);
}

#[tokio::test]
async fn template_detail_returns_expected_item() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/templates/11111111-1111-1111-1111-111111111111")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();

    assert_eq!(body["code"], "wedding-classic");
    assert_eq!(body["category"], "wedding");
}

#[tokio::test]
async fn template_categories_returns_unique_sorted_categories() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/templates/categories")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();

    let categories = body
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item["category"].as_str().unwrap().to_string())
        .collect::<Vec<String>>();

    assert_eq!(
        categories,
        vec!["anniversary", "baby", "birthday", "corporate", "wedding"]
    );
}
