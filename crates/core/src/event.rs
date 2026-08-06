use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::enums::EventType;

#[derive(Debug, Clone)]
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
