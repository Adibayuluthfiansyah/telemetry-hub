use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::device;
use crate::{handlers::health, state::AppState};

pub fn create_router() -> Router<AppState> {
    let api_v1 = Router::new()
        .route("/health", get(health::health))
        .route("/devices", post(device::create_device))
        .route("/devices/{code}", get(device::get_device));
    Router::new().nest("/api/v1", api_v1)
}
