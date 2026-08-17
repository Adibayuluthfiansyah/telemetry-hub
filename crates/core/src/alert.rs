use crate::enums::AlertSeverity;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Alert {
    pub device_id: String,
    pub severity: AlertSeverity,
    pub message: String,
}

impl Alert {
    pub fn new(device_id: String, severity: AlertSeverity, message: String) -> Self {
        Self {
            device_id,
            severity,
            message,
        }
    }
}
