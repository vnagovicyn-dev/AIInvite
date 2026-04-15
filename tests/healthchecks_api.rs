use aiinvite::{app::build_router, db};
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::util::ServiceExt;

async fn test_pool() -> PgPool {
    dotenvy::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for integration tests");
    let pool = db::connect_pool(&database_url).await.unwrap();
    db::run_migrations(&pool).await.unwrap();
    sqlx::query("truncate table healthchecks restart identity")
        .execute(&pool)
        .await
        .unwrap();
    pool
}

#[tokio::test]
async fn healthcheck_crud_flow_works() {
    let pool = test_pool().await;
    let app = build_router(pool.clone());

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/healthchecks")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({ "service_name": "integration-test" }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_response.status(), StatusCode::CREATED);
    let created: Value = serde_json::from_slice(
        &to_bytes(create_response.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    let id = created["id"].as_i64().unwrap();

    let get_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/healthchecks/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    let update_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/healthchecks/{id}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({ "service_name": "integration-test-updated" }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(update_response.status(), StatusCode::OK);
    let updated: Value = serde_json::from_slice(
        &to_bytes(update_response.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(updated["service_name"], "integration-test-updated");

    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/healthchecks")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(list_response.status(), StatusCode::OK);
    let listed: Value = serde_json::from_slice(
        &to_bytes(list_response.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(listed.as_array().unwrap().len(), 1);

    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/healthchecks/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(delete_response.status(), StatusCode::OK);

    let missing_response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/healthchecks/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(missing_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn create_rejects_empty_service_name() {
    let pool = test_pool().await;
    let app = build_router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/healthchecks")
                .header("content-type", "application/json")
                .body(Body::from(json!({ "service_name": "   " }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
