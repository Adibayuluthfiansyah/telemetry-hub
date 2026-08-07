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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
