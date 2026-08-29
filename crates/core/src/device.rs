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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn device_new_sets_online_and_fields() {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let device = Device::new(
            id,
            "TEST-001".to_string(),
            now,
            now,
            "Test Device".to_string(),
            DeviceType::Simulator,
        );
        assert_eq!(device.id, id);
        assert_eq!(device.code, "TEST-001");
        assert_eq!(device.name, "Test Device");
        assert_eq!(device.device_type, DeviceType::Simulator);
        assert_eq!(device.status, DeviceStatus::Online);
        assert_eq!(device.created_at, now);
        assert_eq!(device.updated_at, now);
        assert!(device.is_online());
    }

    #[test]
    fn is_online_true_then_false() {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let mut device = Device::new(
            id,
            "TEST-002".to_string(),
            now,
            now,
            "Test Device".to_string(),
            DeviceType::Esp32,
        );
        assert!(device.is_online());
        device.status = DeviceStatus::Offline;
        assert!(!device.is_online());
        device.status = DeviceStatus::Error;
        assert!(!device.is_online());
        device.status = DeviceStatus::Online;
        assert!(device.is_online());
    }
}
