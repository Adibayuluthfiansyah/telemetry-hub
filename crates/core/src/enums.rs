#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Simulator,
    Esp32,
    Arduino,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
