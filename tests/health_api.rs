use aiinvite::app::{router::build_router, state::AppState};
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde_json::{json, Value};
use tower::util::ServiceExt;

#[tokio::test]
async fn health_route_returns_expected_payload() {
    let app = build_router(AppState::new("invite-platform"));

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
