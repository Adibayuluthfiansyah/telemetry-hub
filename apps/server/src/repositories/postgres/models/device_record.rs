use chrono::{DateTime, Utc};
use sqlx::FromRow;
use std::convert::TryFrom;
use telemetry_core::{Device, DeviceStatus, DeviceType};
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

impl From<DeviceRecord> for Device {
    fn from(record: DeviceRecord) -> Self {
        Self {
            id: record.id,
            code: record.code,
            name: record.name,
            device_type: DeviceType::try_from(record.device_type.as_str())
                .expect("Invalid device_type in database"),
            status: DeviceStatus::try_from(record.status.as_str())
                .expect("Invalid device_status in database"),
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}

impl From<Device> for DeviceRecord {
    fn from(device: Device) -> Self {
        Self {
            id: device.id,
            code: device.code,
            name: device.name,
            device_type: device.device_type.to_string(),
            status: device.status.to_string(),
            created_at: device.created_at,
            updated_at: device.updated_at,
        }
    }
}
