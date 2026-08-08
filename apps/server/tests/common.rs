use server::{config::Config, database};
pub async fn test_pool() -> sqlx::PgPool {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");

    let config = Config {
        app_name: "test".to_string(),
        app_host: "127.0.0.1".to_string(),
        app_port: 3000,
        database_url,
    };
    let pool = database::connect(&config)
        .await
        .expect("Failed to connect to database");
    database::run(&pool)
        .await
        .expect("Failed to run migrations");
    pool
}
