use sqlx::PgPool;

pub async fn run(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("../../migrations").run(pool).await?;
    Ok(())
}
