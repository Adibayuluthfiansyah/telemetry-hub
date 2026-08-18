use crate::{
    dto::{TelemetryQuery, TelemetryQueryResponse, TelemetryRequest, TelemetryResponse},
    error::AppError,
    state::AppState,
};
use axum::{
    Json,
    extract::{Query, State, rejection::JsonRejection, rejection::QueryRejection},
    http::StatusCode,
};
use telemetry_core::{Event, EventType};
use telemetry_transport::EventEnvelope;

pub async fn create_telemetry(
    State(state): State<AppState>,
    request: Result<Json<TelemetryRequest>, JsonRejection>,
) -> Result<(StatusCode, Json<TelemetryResponse>), AppError> {
    let Json(request) = request?;
    let telemetry = state
        .telemetry_service
        .create_telemetry(request)
        .await
        .map_err(|error| {
            let message = error.to_string();
            if message.contains("not found") {
                AppError::NotFound(message)
            } else if message.contains("empty") {
                AppError::BadRequest(message)
            } else {
                AppError::Internal(message)
            }
        })?;
    let envelope = EventEnvelope::from(Event::new(
        EventType::TelemetryReceived,
        telemetry.device_id,
        telemetry.id,
        telemetry.recorded_at,
    ));
    let envelope = match serde_json::to_value(&telemetry.metrics) {
        Ok(metrics) => envelope.with_payload(serde_json::json!({ "metrics": metrics })),
        Err(error) => {
            tracing::error!(?error, "failed to serialize metrics payload");
            envelope
        }
    };
    state.event_publisher.publish(envelope);
    Ok((
        StatusCode::CREATED,
        Json(TelemetryResponse {
            success: true,
            message: "Telemetry created successfully".to_string(),
        }),
    ))
}

pub async fn get_telemetry(
    State(state): State<AppState>,
    query: Result<Query<TelemetryQuery>, QueryRejection>,
) -> Result<Json<TelemetryQueryResponse>, AppError> {
    let Query(query) = query?;
    let limit = query.limit.unwrap_or(100);
    let samples = state
        .telemetry_service
        .get_telemetry(query.device_id, limit)
        .await
        .map_err(|error| {
            let message = error.to_string();
            if message.contains("not found") {
                AppError::NotFound(message)
            } else {
                AppError::Internal(message)
            }
        })?;
    let count = samples.len();
    Ok(Json(TelemetryQueryResponse {
        device_id: query.device_id,
        count,
        samples: samples.into_iter().map(Into::into).collect(),
    }))
}
