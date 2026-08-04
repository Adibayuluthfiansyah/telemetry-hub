use crate::enums::{DeviceStatus, DeviceType};

#[derive(Debug, Clone)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub status: DeviceStatus,
}

impl Device {
    pub fn new(id: String, name: String, device_type: DeviceType) -> Self {
        Self {
            id,
            name,
            device_type,
            status: DeviceStatus::Online,
        }
    }
    pub fn is_online(&self) -> bool {
        self.status == DeviceStatus::Online
    }
}
