use async_trait::async_trait;
use telemetry_core::{Sample, Telemetry};
use uuid::Uuid;

#[async_trait]
pub trait TelemetryRepository: Send + Sync {
    async fn save(&self, telemetry: &Telemetry) -> anyhow::Result<()>;
    async fn find_by_device(&self, device_id: Uuid, limit: i64) -> anyhow::Result<Vec<Sample>>;
}
