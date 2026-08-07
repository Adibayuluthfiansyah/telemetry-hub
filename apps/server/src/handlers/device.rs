use crate::{
    dto::{CreateDeviceRequest, DeviceResponse},
    state::AppState,
};
use axum::{Json, extract::State, http::StatusCode};

pub async fn create_device(
    State(state): State<AppState>,
    Json(request): Json<CreateDeviceRequest>,
) -> Result<(StatusCode, Json<DeviceResponse>), (StatusCode, String)> {
    todo!()
}
