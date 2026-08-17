use telemetry_transport::EventEnvelope;
use tokio::sync::broadcast;

pub trait EventPublisher: Send + Sync {
    fn publish(&self, envelope: EventEnvelope);
}

#[derive(Debug, Clone)]
pub struct BroadcastEventPublisher {
    tx: broadcast::Sender<EventEnvelope>,
}

impl BroadcastEventPublisher {
    pub fn new(tx: broadcast::Sender<EventEnvelope>) -> Self {
        Self { tx }
    }
}

impl EventPublisher for BroadcastEventPublisher {
    fn publish(&self, envelope: EventEnvelope) {
        let _ = self.tx.send(envelope);
    }
}

#[derive(Clone, Default)]
pub struct NoopEventPublisher;

impl EventPublisher for NoopEventPublisher {
    fn publish(&self, _envelope: EventEnvelope) {}
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;
    use telemetry_core::{Event, EventType};
    use uuid::Uuid;
    fn envelope() -> EventEnvelope {
        EventEnvelope::from(Event::new(
            EventType::TelemetryReceived,
            Uuid::new_v4(),
            Uuid::new_v4(),
            Utc::now(),
        ))
    }
    #[test]
    fn broadcast_publisher_delivers_to_receivers() {
        let (tx, mut rx) = broadcast::channel(16);
        let publisher = BroadcastEventPublisher::new(tx);
        let envelope = envelope();
        publisher.publish(envelope.clone());
        assert_eq!(rx.try_recv(), Ok(envelope));
        assert_eq!(rx.try_recv(), Err(broadcast::error::TryRecvError::Empty));
    }

    #[test]
    fn publish_without_receivres_does_not_panic() {
        let (tx, rx) = broadcast::channel(16);
        drop(rx);
        let publisher = BroadcastEventPublisher::new(tx);
        publisher.publish(envelope());
    }

    #[test]
    fn noop_publisher_does_nothing() {
        let publisher = NoopEventPublisher;
        publisher.publish(envelope());
    }
}
