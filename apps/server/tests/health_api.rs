mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use common::test_pool;
use server::{
    app::create_app,
    events::publisher::NoopEventPublisher,
    repositories::postgres::{PostgresDeviceRepository, PostgresTelemetryRepository},
    services::{DeviceService, TelemetryService},
    state::AppState,
};
use std::sync::Arc;
use tokio::sync::broadcast;
use tower::ServiceExt;

#[tokio::test]
async fn health_should_return_200_when_database_is_available() {
    let pool = test_pool().await;

    let device_service = DeviceService::new(PostgresDeviceRepository::new(pool.clone()));

    let telemetry_service = TelemetryService::new(
        PostgresDeviceRepository::new(pool.clone()),
        PostgresTelemetryRepository::new(pool.clone()),
    );

    let state = AppState {
        db: pool.clone(),
        device_service: Arc::new(device_service),
        telemetry_service: Arc::new(telemetry_service),
        event_publisher: Arc::new(NoopEventPublisher),
        event_tx: broadcast::channel(256).0,
    };

    let app = create_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("failed to read response body");

    let response_json: serde_json::Value =
        serde_json::from_slice(&body).expect("failed to parse response JSON");

    assert_eq!(response_json["status"], "ok");
    assert_eq!(response_json["service"], "telemetry-hub");
    assert_eq!(response_json["database"], "up");
}

#[tokio::test]
async fn health_should_return_503_when_database_unavailable() {
    let pool = test_pool().await;
    pool.close().await;

    let device_service = DeviceService::new(PostgresDeviceRepository::new(pool.clone()));
    let telemetry_service = TelemetryService::new(
        PostgresDeviceRepository::new(pool.clone()),
        PostgresTelemetryRepository::new(pool.clone()),
    );
    let state = AppState {
        db: pool.clone(),
        device_service: Arc::new(device_service),
        telemetry_service: Arc::new(telemetry_service),
        event_publisher: Arc::new(NoopEventPublisher),
        event_tx: broadcast::channel(256).0,
    };
    let app = create_app(state);
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("failed to read response body");
    let response_json: serde_json::Value =
        serde_json::from_slice(&body).expect("failed to parse response JSON");
    assert_eq!(response_json["success"], false);
    assert!(
        response_json["message"]
            .as_str()
            .unwrap()
            .contains("Database")
    );
}
