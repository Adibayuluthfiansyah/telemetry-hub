use crate::{error::AppError, state::AppState};
use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use std::time::Duration;
use tokio::time::timeout;
#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    service: &'static str,
    database: &'static str,
}

pub async fn health(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<HealthResponse>), AppError> {
    timeout(
        Duration::from_secs(2),
        sqlx::query("SELECT 1").execute(&state.db),
    )
    .await
    .map_err(|_| AppError::ServiceUnavailable("Database health check timed out".to_string()))?
    .map_err(|error| AppError::ServiceUnavailable(format!("Database unavailable: {}", error)))?;
    Ok((
        StatusCode::OK,
        Json(HealthResponse {
            status: "ok",
            service: "telemetry-hub",
            database: "up",
        }),
    ))
}
