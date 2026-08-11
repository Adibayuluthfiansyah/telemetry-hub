use async_trait::async_trait;
use telemetry_core::Device;

#[async_trait]
pub trait DeviceRepository: Send + Sync {
    async fn save(&self, device: &Device) -> anyhow::Result<()>;
    async fn find_by_code(&self, code: &str) -> anyhow::Result<Option<Device>>;
    async fn find_by_id(&self, id: uuid::Uuid) -> anyhow::Result<Option<Device>>;
}
