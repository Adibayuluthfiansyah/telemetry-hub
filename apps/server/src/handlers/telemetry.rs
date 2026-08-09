use crate::{
    dto::{TelemetryRequest, TelemetryResponse},
    error::AppError,
    state::AppState,
};
use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
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
