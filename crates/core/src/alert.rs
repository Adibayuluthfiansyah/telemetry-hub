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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_new_sets_fields() {
        let alert = Alert::new(
            "device-123".to_string(),
            AlertSeverity::Warning,
            "test message".to_string(),
        );
        assert_eq!(alert.device_id, "device-123");
        assert_eq!(alert.severity, AlertSeverity::Warning);
        assert_eq!(alert.message, "test message");
    }

    #[test]
    fn alert_serde_three_severities_round_trip() {
        for severity in [
            AlertSeverity::Info,
            AlertSeverity::Warning,
            AlertSeverity::Critical,
        ] {
            let alert = Alert::new("dev-1".to_string(), severity, "msg".to_string());
            let json = serde_json::to_string(&alert).unwrap();
            let back: Alert = serde_json::from_str(&json).unwrap();
            assert_eq!(back.severity, severity);
            assert_eq!(back.device_id, "dev-1");
            assert_eq!(back.message, "msg");
        }
    }

    #[test]
    fn alert_serde_unknown_severity_error() {
        let json = r#"{"device_id":"dev-1","severity":"UNKNOWN","message":"msg"}"#;
        let err = serde_json::from_str::<Alert>(json).unwrap_err();
        assert!(err.to_string().contains("unknown variant") || err.to_string().contains("UNKNOWN"));
    }
}
