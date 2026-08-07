use axum::{Router, routing::get};

use crate::{handlers::health, state::AppState};

pub fn create_router() -> Router<AppState> {
    let api_v1 = Router::new().route("/health", get(health::health));
    Router::new().nest("/api/v1", api_v1)
}
