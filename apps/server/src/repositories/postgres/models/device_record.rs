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

impl TryFrom<DeviceRecord> for Device {
    type Error = anyhow::Error;
    fn try_from(record: DeviceRecord) -> Result<Self, Self::Error> {
        let device_type =
            DeviceType::try_from(record.device_type.as_str()).map_err(anyhow::Error::msg)?;
        let status = DeviceStatus::try_from(record.status.as_str()).map_err(anyhow::Error::msg)?;
        Ok(Self {
            id: record.id,
            code: record.code,
            name: record.name,
            device_type,
            status,
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_record(device_type: &str, status: &str) -> DeviceRecord {
        let now = Utc::now();

        DeviceRecord {
            id: Uuid::new_v4(),
            code: "TEST-001".to_string(),
            name: "Test Device".to_string(),
            device_type: device_type.to_string(),
            status: status.to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    #[test]
    fn try_from_should_return_error_when_device_type_is_invalid() {
        let record = create_record("INVALID", "ONLINE");
        let result = Device::try_from(record);
        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert_eq!(error, "Unknown device type: INVALID");
    }

    #[test]
    fn try_from_should_return_error_when_status_is_invalid() {
        let record = create_record("SIMULATOR", "INVALID");
        let result = Device::try_from(record);
        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert_eq!(error, "Unknown device status: INVALID");
    }
}
