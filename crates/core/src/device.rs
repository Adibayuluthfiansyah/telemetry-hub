use crate::enums::{DeviceStatus, DeviceType};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Device {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub status: DeviceStatus,
    pub device_type: DeviceType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Device {
    pub fn new(
        id: Uuid,
        code: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        name: String,
        device_type: DeviceType,
    ) -> Self {
        Self {
            id,
            name,
            device_type,
            status: DeviceStatus::Online,
            code,
            created_at,
            updated_at,
        }
    }
    pub fn is_online(&self) -> bool {
        self.status == DeviceStatus::Online
    }
}
