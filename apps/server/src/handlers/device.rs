use crate::{
    dto::{CreateDeviceRequest, DeviceResponse},
    error::AppError,
    state::AppState,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub async fn create_device(
    State(state): State<AppState>,
    Json(request): Json<CreateDeviceRequest>,
) -> Result<(StatusCode, Json<DeviceResponse>), AppError> {
    let device = state
        .device_service
        .create_device(request.code, request.name, request.device_type)
        .await
        .map_err(|error| {
            let message = error.to_string();
            if message.contains("already exists") {
                AppError::Conflict(message)
            } else {
                AppError::Internal(message)
            }
        })?;
    let response = DeviceResponse {
        id: device.id,
        code: device.code,
        name: device.name,
        status: device.status.to_string(),
        device_type: device.device_type.to_string(),
    };
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_device(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<(StatusCode, Json<DeviceResponse>), AppError> {
    let device = state
        .device_service
        .get_by_code(&code)
        .await
        .map_err(|error| {
            let message = error.to_string();
            if message.contains("not found") {
                AppError::NotFound(message)
            } else {
                AppError::Internal(message)
            }
        })?;
    Ok((
        StatusCode::OK,
        Json(DeviceResponse {
            id: device.id,
            code: device.code,
            name: device.name,
            status: device.status.to_string(),
            device_type: device.device_type.to_string(),
        }),
    ))
}
