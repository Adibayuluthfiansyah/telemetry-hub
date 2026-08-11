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

pub async fn create_telemetry(
    State(state): State<AppState>,
    request: Result<Json<TelemetryRequest>, JsonRejection>,
) -> Result<(StatusCode, Json<TelemetryResponse>), AppError> {
    let Json(request) = request?;
    state
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
