use axum::Router;

use crate::{router::create_router, state::AppState};
pub fn create_app(state: AppState) -> Router {
    create_router().with_state(state)
}
