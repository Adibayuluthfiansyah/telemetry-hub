use crate::repositories::DeviceRepository;
use async_trait::async_trait;
use sqlx::PgPool;
use telemetry_core::Device;

pub struct PostresDeviceRepository {
    pool: PgPool,
}

impl PostresDeviceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DeviceRepository for PostresDeviceRepository {
    async fn save(&self, device: &Device) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO devices (
                id,
                code,
                name,
                device_type,
                status,
                created_at,
                updated_at,
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(device.id)
        .bind(&device.code)
        .bind(&device.name)
        .bind(device.device_type.to_string())
        .bind(device.status.to_string())
        .bind(device.created_at)
        .bind(device.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_by_code(&self, _code: &str) -> anyhow::Result<Option<Device>> {
        todo!()
    }
}
