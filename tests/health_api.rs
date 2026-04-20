use aiinvite::app::{router::build_router, state::AppState};
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde_json::{json, Value};
use sqlx::postgres::PgPoolOptions;
use tower::util::ServiceExt;

fn test_pool() -> sqlx::PgPool {
    PgPoolOptions::new()
        .connect_lazy("postgres://postgres:postgres@127.0.0.1:5432/invite_platform")
        .expect("lazy test pool should be created")
}

#[tokio::test]
async fn health_route_returns_expected_payload() {
    let app = build_router(AppState::new("invite-platform", test_pool()));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();

    assert_eq!(
        body,
        json!({
            "status": "ok",
            "service": "invite-platform"
        })
    );
}
