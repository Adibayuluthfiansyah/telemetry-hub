use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use telemetry_core::{Event, EventType};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct EventEnvelope {
    pub event_id: Uuid,
    pub event_type: EventType,
    pub device_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub payload: Option<serde_json::Value>,
}

impl From<Event> for EventEnvelope {
    fn from(event: Event) -> Self {
        Self {
            event_id: event.id,
            event_type: event.event_type,
            device_id: event.device_id,
            created_at: event.created_at,
            payload: None,
        }
    }
}

impl EventEnvelope {
    pub fn with_payload(mut self, payload: serde_json::Value) -> Self {
        self.payload = Some(payload);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn sample_event(event: EventType) -> Event {
        Event::new(event, Uuid::new_v4(), Uuid::new_v4(), Utc::now())
    }
    #[test]
    fn all_event_types_round_trip_through_json() -> Result<(), Box<dyn std::error::Error>> {
        let event_types = [
            EventType::TelemetryReceived,
            EventType::DeviceConnected,
            EventType::DeviceDisconnected,
            EventType::AlertRaised,
        ];
        for event_type in event_types {
            let envelope = EventEnvelope::from(sample_event(event_type));
            let json = serde_json::to_string(&envelope)?;
            let back: EventEnvelope = serde_json::from_str(&json)?;
            assert_eq!(envelope, back);
        }
        Ok(())
    }

    #[test]
    fn from_event_sets_payload_none() {
        let envelope = EventEnvelope::from(sample_event(EventType::TelemetryReceived));
        assert_eq!(envelope.payload, None);
    }

    #[test]
    fn with_payload_preserves_event_field() {
        let event = sample_event(EventType::TelemetryReceived);
        let envelope =
            EventEnvelope::from(event.clone()).with_payload(serde_json::json!({"key" : "battery"}));
        assert_eq!(envelope.event_id, event.id);
        assert_eq!(envelope.device_id, event.device_id);
        assert!(envelope.payload.is_some());
    }

    #[test]
    fn with_payload_json_value_preserved_through_serde() {
        let event = sample_event(EventType::TelemetryReceived);
        let payload =
            serde_json::json!({"metrics": [{"key": "temp", "value": 25.5, "unit": "celsius"}]});
        let envelope = EventEnvelope::from(event.clone()).with_payload(payload.clone());
        let json = serde_json::to_string(&envelope).unwrap();
        let back: EventEnvelope = serde_json::from_str(&json).unwrap();
        assert_eq!(back.payload.unwrap(), payload);
        assert_eq!(back.event_id, event.id);
        assert_eq!(back.device_id, event.device_id);
        assert_eq!(back.created_at, event.created_at);
    }

    #[test]
    fn envelope_from_invalid_json_returns_error() {
        let bad_uuid = r#"{"event_id":"not-a-uuid","event_type":"TELEMETRY_RECEIVED","device_id":"00000000-0000-0000-0000-000000000001","created_at":"2026-01-01T00:00:00Z","payload":null}"#;
        assert!(serde_json::from_str::<EventEnvelope>(bad_uuid).is_err());

        let missing_field = r#"{"event_id":"00000000-0000-0000-0000-000000000001","event_type":"TELEMETRY_RECEIVED","device_id":"00000000-0000-0000-0000-000000000002"}"#;
        assert!(serde_json::from_str::<EventEnvelope>(missing_field).is_err());

        let unknown_variant = r#"{"event_id":"00000000-0000-0000-0000-000000000001","event_type":"UNKNOWN","device_id":"00000000-0000-0000-0000-000000000002","created_at":"2026-01-01T00:00:00Z","payload":null}"#;
        assert!(serde_json::from_str::<EventEnvelope>(unknown_variant).is_err());
    }
}
