mod common;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use chrono::Utc;
use common::test_pool;
use serde_json::json;
use server::{
    app::create_app,
    events::publisher::NoopEventPublisher,
    repositories::postgres::{PostgresDeviceRepository, PostgresTelemetryRepository},
    services::{DeviceService, TelemetryService},
    state::AppState,
};
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn create_telemetry_should_return_201_and_persist_all_metrics() {
    let pool = test_pool().await;

    sqlx::query(
        r#"
        DELETE FROM telemetry
        WHERE device_id IN (
            SELECT id
            FROM devices
            WHERE code = 'API-TELEMETRY-001'
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("failed to clean test telemetry");

    sqlx::query(
        r#"
        DELETE FROM devices
        WHERE code = 'API-TELEMETRY-001'
        "#,
    )
    .execute(&pool)
    .await
    .expect("failed to clean test device");

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
    };

    let app = create_app(state);

    let device_payload = json!({
        "code": "API-TELEMETRY-001",
        "name": "Telemetry API Test Device",
        "device_type": "SIMULATOR"
    });

    let create_device_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices")
                .header("content-type", "application/json")
                .body(Body::from(device_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_device_response.status(), StatusCode::CREATED);

    let device_id: uuid::Uuid = sqlx::query_scalar(
        r#"
        SELECT id
        FROM devices
        WHERE code = 'API-TELEMETRY-001'
        "#,
    )
    .fetch_one(&pool)
    .await
    .expect("failed to find test device");

    let telemetry_payload = json!({
        "device_code": "API-TELEMETRY-001",
        "metrics": [
            {
                "key": "temperature",
                "value": 25.5,
                "unit": "celsius"
            },
            {
                "key": "humidity",
                "value": 60.0,
                "unit": "percent"
            }
        ]
    });

    let telemetry_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/telemetry")
                .header("content-type", "application/json")
                .body(Body::from(telemetry_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(telemetry_response.status(), StatusCode::CREATED);

    let body = axum::body::to_bytes(telemetry_response.into_body(), usize::MAX)
        .await
        .expect("failed to read response body");

    let response_json: serde_json::Value =
        serde_json::from_slice(&body).expect("failed to parse response JSON");
    assert_eq!(response_json["success"], true);
    assert_eq!(response_json["message"], "Telemetry created successfully");

    let telemetry_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM telemetry
        WHERE device_id = $1
        "#,
    )
    .bind(device_id)
    .fetch_one(&pool)
    .await
    .expect("failed to count telemetry records");

    assert_eq!(telemetry_count, 2);

    let temperature: (f64, String) = sqlx::query_as(
        r#"
        SELECT value, unit
        FROM telemetry
        WHERE device_id = $1
          AND key = 'temperature'
        "#,
    )
    .bind(device_id)
    .fetch_one(&pool)
    .await
    .expect("failed to find temperature telemetry");

    assert_eq!(temperature.0, 25.5);
    assert_eq!(temperature.1, "celsius");

    let humidity: (f64, String) = sqlx::query_as(
        r#"
        SELECT value, unit
        FROM telemetry
        WHERE device_id = $1
          AND key = 'humidity'
        "#,
    )
    .bind(device_id)
    .fetch_one(&pool)
    .await
    .expect("failed to find humidity telemetry");

    assert_eq!(humidity.0, 60.0);
    assert_eq!(humidity.1, "percent");
}

#[tokio::test]
async fn create_telemetry_should_return_404_when_device_does_not_exist() {
    let pool = test_pool().await;

    let device_code = "API-TELEMETRY-NOT-FOUND";

    sqlx::query(
        r#"
        DELETE FROM telemetry
        WHERE device_id IN (
            SELECT id
            FROM devices
            WHERE code = $1
        )
        "#,
    )
    .bind(device_code)
    .execute(&pool)
    .await
    .expect("failed to clean test telemetry");

    sqlx::query(
        r#"
        DELETE FROM devices
        WHERE code = $1
        "#,
    )
    .bind(device_code)
    .execute(&pool)
    .await
    .expect("failed to clean test device");

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
    };

    let app = create_app(state);

    let payload = json!({
        "device_code": device_code,
        "metrics": [
            {
                "key": "temperature",
                "value": 25.5,
                "unit": "celsius"
            }
        ]
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/telemetry")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn create_telemetry_should_return_400_when_metrics_empty() {
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
    };

    let app = create_app(state);

    let payload = json!({
        "device_code": "ANY-DEVICE",
        "metrics": []
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/telemetry")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_telemetry_should_return_400_when_payload_is_malformed() {
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
    };

    let app = create_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/telemetry")
                .header("content-type", "application/json")
                .body(Body::from("{ not valid json".to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("failed to read response body");

    let response_json: serde_json::Value =
        serde_json::from_slice(&body).expect("failed to parse response JSON");
    assert_eq!(response_json["success"], false);
}

#[tokio::test]
async fn get_telemetry_should_return_200_with_samples() {
    let pool = test_pool().await;
    let device_code = "API-QUERY-001";

    sqlx::query(
        r#"
        DELETE FROM telemetry
        WHERE device_id IN (
            SELECT id FROM devices WHERE code = $1
        )
        "#,
    )
    .bind(device_code)
    .execute(&pool)
    .await
    .expect("failed to clean test telemetry");

    sqlx::query("DELETE FROM devices WHERE code = $1")
        .bind(device_code)
        .execute(&pool)
        .await
        .expect("failed to clean test device");

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
    };

    let app = create_app(state);

    let device_payload = json!({
        "code": device_code,
        "name": "Telemetry Query Test Device",
        "device_type": "SIMULATOR"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices")
                .header("content-type", "application/json")
                .body(Body::from(device_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let telemetry_payload = json!({
        "device_code": device_code,
        "metrics": [
            {
                "key": "temperature",
                "value": 25.5,
                "unit": "celsius"
            },
            {
                "key": "humidity",
                "value": 60.0,
                "unit": "percent"
            }
        ]
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/telemetry")
                .header("content-type", "application/json")
                .body(Body::from(telemetry_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let device_id: uuid::Uuid = sqlx::query_scalar("SELECT id FROM devices WHERE code = $1")
        .bind(device_code)
        .fetch_one(&pool)
        .await
        .expect("failed to find device");

    let uri = format!("/api/v1/telemetry?device_id={}&limit=100", device_id);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(uri)
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

    assert_eq!(response_json["device_id"], device_id.to_string());
    assert_eq!(response_json["count"], 2);

    let samples = response_json["samples"]
        .as_array()
        .expect("samples should be an array");

    assert_eq!(samples.len(), 2);

    let temperature = samples
        .iter()
        .find(|sample| sample["key"] == "temperature")
        .expect("temperature sample not found");

    assert_eq!(temperature["value"], 25.5);
    assert_eq!(temperature["unit"], "celsius");

    let humidity = samples
        .iter()
        .find(|sample| sample["key"] == "humidity")
        .expect("humidity sample not found");

    assert_eq!(humidity["value"], 60.0);
    assert_eq!(humidity["unit"], "percent");
}

#[tokio::test]
async fn get_telemetry_should_return_200_when_empty() {
    let pool = test_pool().await;
    let device_code = "API-QUERY-EMPTY";

    sqlx::query(
        r#"
        DELETE FROM telemetry
        WHERE device_id IN (
            SELECT id FROM devices WHERE code = $1
        )
        "#,
    )
    .bind(device_code)
    .execute(&pool)
    .await
    .expect("failed to clean test telemetry");

    sqlx::query("DELETE FROM devices WHERE code = $1")
        .bind(device_code)
        .execute(&pool)
        .await
        .expect("failed to clean test device");

    let device_id = uuid::Uuid::new_v4();

    sqlx::query(
        r#"
        INSERT INTO devices (
            id,
            code,
            name,
            status
        )
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(device_id)
    .bind(device_code)
    .bind("Empty Telemetry Device")
    .bind("ONLINE")
    .execute(&pool)
    .await
    .expect("failed to create test device");

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
    };

    let app = create_app(state);

    let uri = format!("/api/v1/telemetry?device_id={}", device_id);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(uri)
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

    assert_eq!(response_json["device_id"], device_id.to_string());
    assert_eq!(response_json["count"], 0);
    assert!(response_json["samples"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn get_telemetry_should_return_404_when_device_does_not_exist() {
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
    };

    let app = create_app(state);

    let device_id = uuid::Uuid::new_v4();

    let uri = format!("/api/v1/telemetry?device_id={}", device_id);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_telemetry_should_return_400_when_device_id_is_missing() {
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
    };

    let app = create_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/telemetry")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("failed to read response body");

    let response_json: serde_json::Value =
        serde_json::from_slice(&body).expect("failed to parse response JSON");
    assert_eq!(response_json["success"], false);
}

#[tokio::test]
async fn get_telemetry_should_return_400_when_device_id_is_invalid() {
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
    };

    let app = create_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/telemetry?device_id=not-a-uuid")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("failed to read response body");

    let response_json: serde_json::Value =
        serde_json::from_slice(&body).expect("failed to parse response JSON");
    assert_eq!(response_json["success"], false);
}

#[tokio::test]
async fn get_telemetry_should_clamp_limit_to_1000() {
    let pool = test_pool().await;
    let device_code = "API-QUERY-LIMIT";

    sqlx::query(
        r#"
        DELETE FROM telemetry
        WHERE device_id IN (
            SELECT id FROM devices WHERE code = $1
        )
        "#,
    )
    .bind(device_code)
    .execute(&pool)
    .await
    .expect("failed to clean test telemetry");

    sqlx::query("DELETE FROM devices WHERE code = $1")
        .bind(device_code)
        .execute(&pool)
        .await
        .expect("failed to clean test device");

    let device_id = uuid::Uuid::new_v4();

    sqlx::query(
        r#"
        INSERT INTO devices (
            id,
            code,
            name,
            status
        )
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(device_id)
    .bind(device_code)
    .bind("Limit Query Device")
    .bind("ONLINE")
    .execute(&pool)
    .await
    .expect("failed to create test device");

    for i in 0..1001 {
        sqlx::query(
            r#"
            INSERT INTO telemetry (
                id,
                device_id,
                key,
                value,
                unit,
                recorded_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(uuid::Uuid::new_v4())
        .bind(device_id)
        .bind("temperature")
        .bind(i as f64)
        .bind("celsius")
        .bind(Utc::now())
        .execute(&pool)
        .await
        .expect("failed to insert test telemetry");
    }

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
    };

    let app = create_app(state);

    let uri = format!("/api/v1/telemetry?device_id={}&limit=5000", device_id);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(uri)
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

    assert_eq!(response_json["device_id"], device_id.to_string());
    assert_eq!(response_json["count"], 1000);

    let samples = response_json["samples"]
        .as_array()
        .expect("samples should be an array");

    assert_eq!(samples.len(), 1000);
}
