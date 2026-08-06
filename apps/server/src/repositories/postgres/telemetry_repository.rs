use sqlx::PgPool;

pub struct PostgresTelemetryRepository {
    pool: PgPool,
}

impl PostgresTelemetryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
