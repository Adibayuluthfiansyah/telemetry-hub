use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use server::repositories::DeviceRepository;
use telemetry_core::Device;

pub struct MockDeviceRepository {
    devices: Arc<Mutex<HashMap<String, Device>>>,
}

impl MockDeviceRepository {
    pub fn new() -> Self {
        Self {
            devices: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl DeviceRepository for MockDeviceRepository {
    async fn save(&self, device: &Device) -> anyhow::Result<()> {
        self.devices
            .lock()
            .unwrap()
            .insert(device.code.clone(), device.clone());
        Ok(())
    }
    async fn find_by_code(&self, code: &str) -> anyhow::Result<Option<Device>> {
        Ok(self.devices.lock().unwrap().get(code).cloned())
    }
    async fn find_by_id(&self, id: uuid::Uuid) -> anyhow::Result<Option<Device>> {
        Ok(self
            .devices
            .lock()
            .unwrap()
            .values()
            .find(|d| d.id == id)
            .cloned())
    }
}
