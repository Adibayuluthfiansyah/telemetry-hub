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
}
