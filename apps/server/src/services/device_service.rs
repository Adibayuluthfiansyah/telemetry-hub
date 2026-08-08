use crate::repositories::DeviceRepository;
use chrono::Utc;
use telemetry_core::{Device, DeviceType};
use uuid::Uuid;

pub struct DeviceService<R>
where
    R: DeviceRepository,
{
    repository: R,
}

impl<R> DeviceService<R>
where
    R: DeviceRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_device(
        &self,
        code: String,
        name: String,
        device_type: DeviceType,
    ) -> anyhow::Result<Device> {
        let existing = self.repository.find_by_code(&code).await?;
        if existing.is_some() {
            anyhow::bail!("Device with code {} already exists", code);
        }
        let now = Utc::now();
        let id = Uuid::new_v4();
        let device = Device::new(id, code, now, now, name, device_type);
        self.repository.save(&device).await?;
        Ok(device)
    }
    pub async fn get_by_code(&self, code: &str) -> anyhow::Result<Device> {
        self.repository
            .find_by_code(code)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Device with code {} not found", code))
    }
}
