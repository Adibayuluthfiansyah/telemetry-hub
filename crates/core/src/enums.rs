use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{self};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceType {
    Simulator,
    Esp32,
    Arduino,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum DeviceStatus {
    Online,
    Offline,
    Error,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    TelemetryReceived,
    DeviceConnected,
    DeviceDisconnected,
    AlertRaised,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            DeviceType::Simulator => "SIMULATOR",
            DeviceType::Esp32 => "ESP32",
            DeviceType::Arduino => "ARDUINO",
        };
        write!(f, "{value}")
    }
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            DeviceStatus::Online => "ONLINE",
            DeviceStatus::Offline => "OFFLINE",
            DeviceStatus::Error => "ERROR",
        };
        write!(f, "{value}")
    }
}

impl TryFrom<&str> for DeviceType {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "SIMULATOR" => Ok(DeviceType::Simulator),
            "ESP32" => Ok(DeviceType::Esp32),
            "ARDUINO" => Ok(DeviceType::Arduino),
            _ => Err(format!("Unknown device type: {}", value)),
        }
    }
}

impl TryFrom<&str> for DeviceStatus {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, String> {
        match value {
            "ONLINE" => Ok(DeviceStatus::Online),
            "OFFLINE" => Ok(DeviceStatus::Offline),
            "ERROR" => Ok(DeviceStatus::Error),

            _ => Err(format!("Unknown device status: {}", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn device_type_display() {
        assert_eq!(DeviceType::Simulator.to_string(), "SIMULATOR");
        assert_eq!(DeviceType::Esp32.to_string(), "ESP32");
        assert_eq!(DeviceType::Arduino.to_string(), "ARDUINO");
    }

    #[test]
    fn device_type_try_from_round_trip() {
        assert_eq!(
            DeviceType::try_from("SIMULATOR").unwrap(),
            DeviceType::Simulator
        );
        assert_eq!(DeviceType::try_from("ESP32").unwrap(), DeviceType::Esp32);
        assert_eq!(
            DeviceType::try_from("ARDUINO").unwrap(),
            DeviceType::Arduino
        );
        for variant in [
            DeviceType::Simulator,
            DeviceType::Esp32,
            DeviceType::Arduino,
        ] {
            let s = variant.to_string();
            assert_eq!(DeviceType::try_from(s.as_str()).unwrap(), variant);
        }
    }

    #[test]
    fn device_type_try_from_unknown_error() {
        let err = DeviceType::try_from("UNKNOWN").unwrap_err();
        assert!(err.contains("Unknown device type"));
        assert!(err.contains("UNKNOWN"));
    }

    #[test]
    fn device_status_display() {
        assert_eq!(DeviceStatus::Online.to_string(), "ONLINE");
        assert_eq!(DeviceStatus::Offline.to_string(), "OFFLINE");
        assert_eq!(DeviceStatus::Error.to_string(), "ERROR");
    }

    #[test]
    fn device_status_try_from_round_trip() {
        assert_eq!(
            DeviceStatus::try_from("ONLINE").unwrap(),
            DeviceStatus::Online
        );
        assert_eq!(
            DeviceStatus::try_from("OFFLINE").unwrap(),
            DeviceStatus::Offline
        );
        assert_eq!(
            DeviceStatus::try_from("ERROR").unwrap(),
            DeviceStatus::Error
        );
        for variant in [
            DeviceStatus::Online,
            DeviceStatus::Offline,
            DeviceStatus::Error,
        ] {
            let s = variant.to_string();
            assert_eq!(DeviceStatus::try_from(s.as_str()).unwrap(), variant);
        }
    }

    #[test]
    fn device_status_try_from_unknown_error() {
        let err = DeviceStatus::try_from("UNKNOWN").unwrap_err();
        assert!(err.contains("Unknown device status"));
    }

    #[test]
    fn device_type_serde_round_trip() {
        for variant in [
            DeviceType::Simulator,
            DeviceType::Esp32,
            DeviceType::Arduino,
        ] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: DeviceType = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back);
        }
        let json = serde_json::to_string(&DeviceType::Simulator).unwrap();
        assert_eq!(json, "\"SIMULATOR\"");
    }

    #[test]
    fn device_status_serde_round_trip() {
        for variant in [
            DeviceStatus::Online,
            DeviceStatus::Offline,
            DeviceStatus::Error,
        ] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: DeviceStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back);
        }
    }

    #[test]
    fn alert_severity_serde_round_trip() {
        for variant in [
            AlertSeverity::Info,
            AlertSeverity::Warning,
            AlertSeverity::Critical,
        ] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: AlertSeverity = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back);
        }
        assert_eq!(
            serde_json::to_string(&AlertSeverity::Info).unwrap(),
            "\"INFO\""
        );
        assert_eq!(
            serde_json::to_string(&AlertSeverity::Warning).unwrap(),
            "\"WARNING\""
        );
        assert_eq!(
            serde_json::to_string(&AlertSeverity::Critical).unwrap(),
            "\"CRITICAL\""
        );
    }

    #[test]
    fn event_type_serde_round_trip() {
        for variant in [
            EventType::TelemetryReceived,
            EventType::DeviceConnected,
            EventType::DeviceDisconnected,
            EventType::AlertRaised,
        ] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: EventType = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back);
        }
    }

    #[test]
    fn enums_serde_unknown_variant_error() {
        let err = serde_json::from_str::<DeviceType>("\"UNKNOWN\"").unwrap_err();
        assert!(err.to_string().contains("unknown variant") || err.to_string().contains("UNKNOWN"));
        let err = serde_json::from_str::<DeviceStatus>("\"UNKNOWN\"").unwrap_err();
        assert!(err.to_string().contains("unknown variant") || err.to_string().contains("UNKNOWN"));
    }
}
