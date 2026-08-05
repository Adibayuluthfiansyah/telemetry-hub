use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::config::Config;


pub async fn connect (config:&Config) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
}
