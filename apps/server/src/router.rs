use axum::{Router, routing::get};

use crate::{handlers::health, state::AppState};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/health", get(health::health))
}
