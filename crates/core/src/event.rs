use crate::enums::EventType;

#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub device_id: String,
    pub timestamp: u64,
}

impl Event {
    pub fn new(event_type: EventType, device_id: String, timestamp: u64) -> Self {
        Self {
            event_type,
            device_id,
            timestamp,
        }
    }
}
