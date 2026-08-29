use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::enums::EventType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub event_type: EventType,
    pub device_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl Event {
    pub fn new(
        event_type: EventType,
        device_id: Uuid,
        id: Uuid,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            event_type,
            device_id,
            id,
            created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    #[test]
    fn event_round_trips_through_json() -> Result<(), Box<dyn std::error::Error>> {
        let event = Event::new(
            EventType::TelemetryReceived,
            Uuid::new_v4(),
            Uuid::new_v4(),
            Utc::now(),
        );
        let json = serde_json::to_string(&event)?;
        let back: Event = serde_json::from_str(&json)?;
        assert_eq!(event, back);
        Ok(())
    }

    #[test]
    fn event_round_trips_all_variants() -> Result<(), Box<dyn std::error::Error>> {
        for event_type in [
            EventType::TelemetryReceived,
            EventType::DeviceConnected,
            EventType::DeviceDisconnected,
            EventType::AlertRaised,
        ] {
            let event = Event::new(event_type, Uuid::new_v4(), Uuid::new_v4(), Utc::now());
            let json = serde_json::to_string(&event)?;
            let back: Event = serde_json::from_str(&json)?;
            assert_eq!(event, back);
            assert_eq!(back.event_type, event_type);
        }
        Ok(())
    }

    #[test]
    fn event_from_invalid_json_returns_error() {
        let bad_uuid = r#"{"id":"not-a-uuid","event_type":"TELEMETRY_RECEIVED","device_id":"00000000-0000-0000-0000-000000000000","created_at":"2026-01-01T00:00:00Z"}"#;
        assert!(serde_json::from_str::<Event>(bad_uuid).is_err());

        let missing_field = r#"{"id":"00000000-0000-0000-0000-000000000001","event_type":"TELEMETRY_RECEIVED","device_id":"00000000-0000-0000-0000-000000000002"}"#;
        assert!(serde_json::from_str::<Event>(missing_field).is_err());

        let unknown_variant = r#"{"id":"00000000-0000-0000-0000-000000000001","event_type":"UNKNOWN","device_id":"00000000-0000-0000-0000-000000000002","created_at":"2026-01-01T00:00:00Z"}"#;
        assert!(serde_json::from_str::<Event>(unknown_variant).is_err());
    }

    #[test]
    fn event_new_preserves_fields() {
        let id = Uuid::new_v4();
        let device_id = Uuid::new_v4();
        let now = Utc::now();
        let event = Event::new(EventType::DeviceConnected, device_id, id, now);
        assert_eq!(event.id, id);
        assert_eq!(event.device_id, device_id);
        assert_eq!(event.event_type, EventType::DeviceConnected);
        assert_eq!(event.created_at, now);
    }
}
