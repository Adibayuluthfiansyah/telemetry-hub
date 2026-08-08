mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use common::test_pool;
use serde_json::json;
use server::{
    app::create_app, repositories::PostgresDeviceRepository, services::DeviceService,
    state::AppState,
};
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn create_device_should_return_409_when_code_exist() {
    let pool = test_pool().await;
    sqlx::query("DELETE FROM devices WHERE code = 'API-TEST-002'")
        .execute(&pool)
        .await
        .expect("failed to clean test device");
    let repository = PostgresDeviceRepository::new(pool);
    let service = DeviceService::new(repository);
    let state = AppState {
        device_service: Arc::new(service),
    };

    let app = create_app(state);
    let payload = json!({
         "code": "API-TEST-002",
        "name": "API Test Device",
        "device_type": "SIMULATOR"
    });
    let request = || {
        Request::builder()
            .method("POST")
            .uri("/api/v1/devices")
            .header("content-type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap()
    };
    let first_response = app.clone().oneshot(request()).await.unwrap();
    assert_eq!(first_response.status(), StatusCode::CREATED);

    let second_response = app.clone().oneshot(request()).await.unwrap();
    assert_eq!(second_response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn get_device_should_return_200_when_device_exists() {
    let pool = test_pool().await;

    sqlx::query("DELETE FROM devices WHERE code = 'API-TEST-003'")
        .execute(&pool)
        .await
        .expect("failed to clean test device");

    let repository = PostgresDeviceRepository::new(pool);

    let service = DeviceService::new(repository);

    let state = AppState {
        device_service: Arc::new(service),
    };

    let app = create_app(state);

    let create_payload = json!({
        "code": "API-TEST-003",
        "name": "API GET Test Device",
        "device_type": "SIMULATOR"
    });

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices")
                .header("content-type", "application/json")
                .body(Body::from(create_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_response.status(), StatusCode::CREATED);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/devices/API-TEST-003")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn get_device_should_return_404_when_device_does_not_exist() {
    let pool = test_pool().await;

    sqlx::query("DELETE FROM devices WHERE code = 'API-TEST-NOT-FOUND'")
        .execute(&pool)
        .await
        .expect("failed to clean test device");

    let repository = PostgresDeviceRepository::new(pool);
    let service = DeviceService::new(repository);

    let state = AppState {
        device_service: Arc::new(service),
    };

    let app = create_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/devices/API-TEST-NOT-FOUND")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
