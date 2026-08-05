use sqlx::PgPool;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub db: PgPool,
}
