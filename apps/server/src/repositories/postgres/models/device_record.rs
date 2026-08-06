use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct DeviceRecord {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub device_type: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

use telemetry_core::{Device, DeviceStatus, DeviceType};

impl From<DeviceRecord> for Device {
    fn from(record: DeviceRecord) -> Self {
        Self {
        todo!()
        }
    }
}
