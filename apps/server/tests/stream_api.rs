mod common;

use axum::{body::Body, http::Request};
use common::test_pool;
use futures_util::StreamExt;
use serde_json::json;
use server::{
    app::create_app,
    events::publisher::BroadcastEventPublisher,
    repositories::postgres::{PostgresDeviceRepository, PostgresTelemetryRepository},
    services::{DeviceService, TelemetryService},
    state::AppState,
};
use std::sync::Arc;
use telemetry_core::EventType;
use telemetry_transport::EventEnvelope;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::time::{Duration, timeout};
use tokio_tungstenite::tungstenite::Message;
use tower::ServiceExt;
use uuid::Uuid;

async fn register_device(app: axum::Router, code: &str) -> Uuid {
    let payload = json!({
        "code": code,
        "name": "WS Test",
        "device_type": "SIMULATOR"
    });
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/devices")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 201);
    let body = axum::body::to_bytes(response.into_body(), 8192)
        .await
        .unwrap();
    serde_json::from_slice::<serde_json::Value>(&body).unwrap()["id"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap()
}

async fn receive_envelope(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
    expected_type: EventType,
) -> EventEnvelope {
    let message = timeout(Duration::from_secs(5), socket.next())
        .await
        .expect("timed out waiting for event")
        .expect("stream closed")
        .expect("websocket error");
    let Message::Text(text) = message else {
        panic!("expected text message");
    };
    let envelope: EventEnvelope = serde_json::from_str(&text).expect("invalid envelope json");
    assert_eq!(envelope.event_type, expected_type);
    envelope
}

async fn build_state(pool: sqlx::PgPool) -> (AppState, broadcast::Sender<EventEnvelope>) {
    let (event_tx, _) = broadcast::channel(256);
    let device_service = DeviceService::new(PostgresDeviceRepository::new(pool.clone()));
    let telemetry_service = TelemetryService::new(
        PostgresDeviceRepository::new(pool.clone()),
        PostgresTelemetryRepository::new(pool.clone()),
    );
    let state = AppState {
        db: pool.clone(),
        device_service: Arc::new(device_service),
        telemetry_service: Arc::new(telemetry_service),
        event_publisher: Arc::new(BroadcastEventPublisher::new(event_tx.clone())),
        event_tx: event_tx.clone(),
    };
    (state, event_tx)
}

#[tokio::test]
async fn stream_delivers_live_events() {
    let pool = test_pool().await;

    sqlx::query(
        r#"
        DELETE FROM telemetry
        WHERE device_id IN (
            SELECT id FROM devices WHERE code = 'WS-TEST-001'
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("failed to clean test telemetry");

    sqlx::query("DELETE FROM devices WHERE code = 'WS-TEST-001'")
        .execute(&pool)
        .await
        .expect("failed to clean test device");

    let (state, _event_tx) = build_state(pool).await;

    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind failed");
    let addr = listener.local_addr().expect("addr failed");
    tokio::spawn(axum::serve(listener, create_app(state.clone())).into_future());

    let (mut socket, _) = tokio_tungstenite::connect_async(format!("ws://{addr}/api/v1/stream"))
        .await
        .expect("ws connect failed");

    let device_id = register_device(create_app(state.clone()), "WS-TEST-001").await;

    let connected = receive_envelope(&mut socket, EventType::DeviceConnected).await;
    assert_eq!(connected.device_id, device_id);

    let telemetry = json!({
        "device_code": "WS-TEST-001",
        "metrics": [{"key": "cpu", "value": 42.5, "unit": "percent"}]
    });
    let response = create_app(state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/telemetry")
                .header("content-type", "application/json")
                .body(Body::from(telemetry.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 201);

    let received = receive_envelope(&mut socket, EventType::TelemetryReceived).await;
    assert_eq!(received.device_id, device_id);
    let payload = received
        .payload
        .as_ref()
        .expect("telemetry event carries a payload");
    assert_eq!(payload["metrics"][0]["key"], "cpu");
    assert_eq!(payload["metrics"][0]["value"], 42.5);
    assert_eq!(payload["metrics"][0]["unit"], "percent");
}

#[tokio::test]
async fn stream_filters_events_for_other_devices() {
    let pool = test_pool().await;

    sqlx::query(
        r#"
        DELETE FROM telemetry
        WHERE device_id IN (
            SELECT id FROM devices WHERE code = 'WS-TEST-002'
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("failed to clean test telemetry");

    sqlx::query("DELETE FROM devices WHERE code = 'WS-TEST-002'")
        .execute(&pool)
        .await
        .expect("failed to clean test device");

    let (state, _event_tx) = build_state(pool).await;

    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind failed");
    let addr = listener.local_addr().expect("addr failed");
    tokio::spawn(axum::serve(listener, create_app(state.clone())).into_future());

    let other_device = Uuid::new_v4();
    let (mut socket, _) = tokio_tungstenite::connect_async(format!(
        "ws://{addr}/api/v1/stream?device_id={other_device}"
    ))
    .await
    .expect("ws connect failed");

    register_device(create_app(state.clone()), "WS-TEST-002").await;

    let silence = timeout(Duration::from_millis(500), socket.next()).await;
    assert!(
        silence.is_err(),
        "expected no events for an unrelated device filter"
    );
}

#[tokio::test]
async fn stream_should_reject_invalid_device_id() {
    let pool = test_pool().await;
    let (state, _event_tx) = build_state(pool).await;
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind failed");
    let addr = listener.local_addr().expect("addr failed");
    tokio::spawn(axum::serve(listener, create_app(state.clone())).into_future());
    let result =
        tokio_tungstenite::connect_async(format!("ws://{addr}/api/v1/stream?device_id=not-a-uuid"))
            .await;
    assert!(
        result.is_err(),
        "expected handshake failure for invalid UUID"
    );
}
