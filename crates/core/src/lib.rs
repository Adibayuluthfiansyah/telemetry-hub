pub mod alert;
pub mod device;
pub mod enums;
pub mod event;
pub mod metric;
pub mod sample;
pub mod telemetry;
pub use alert::Alert;
pub use device::Device;
pub use enums::{AlertSeverity, DeviceStatus, DeviceType, EventType};
pub use event::Event;
pub use sample::Sample;
pub use telemetry::Telemetry;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
