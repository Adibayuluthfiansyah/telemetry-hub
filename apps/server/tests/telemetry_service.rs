mod mocks;

use chrono::Utc;
use mocks::{MockDeviceRepository, MockTelemetryRepository};
use server::dto::{MetricRequest, TelemetryRequest};
use server::repositories::DeviceRepository;
use server::services::TelemetryService;
use telemetry_core::{Device, DeviceType};
use uuid::Uuid;

#[tokio::test]
async fn create_telemetry_should_create_and_save_telemetry() {
    let device_repository = MockDeviceRepository::new();
    let telemetry_repository = MockTelemetryRepository::new();

    let device = Device::new(
        Uuid::new_v4(),
        "SIM-001".to_string(),
        Utc::now(),
        Utc::now(),
        "Simulator 001".to_string(),
        DeviceType::Simulator,
    );

    device_repository.save(&device).await.unwrap();
    let service = TelemetryService::new(device_repository, telemetry_repository);
    let request = TelemetryRequest {
        device_code: "SIM-001".to_string(),
        metrics: vec![
            MetricRequest {
                key: "temperature".to_string(),
                value: 25.5,
                unit: "celsius".to_string(),
            },
            MetricRequest {
                key: "humidity".to_string(),
                value: 60.0,
                unit: "percent".to_string(),
            },
        ],
    };
    let result = service.create_telemetry(request).await;
    assert!(result.is_ok());
    let telemetry = result.unwrap();
    assert_eq!(telemetry.device_id, device.id);
    assert_eq!(telemetry.metrics.len(), 2);
    assert_eq!(telemetry.metrics[0].key, "temperature");
    assert_eq!(telemetry.metrics[0].value, 25.5);
    assert_eq!(telemetry.metrics[0].unit, "celsius");
    assert_eq!(telemetry.metrics[1].key, "humidity");
    assert_eq!(telemetry.metrics[1].value, 60.0);
    assert_eq!(telemetry.metrics[1].unit, "percent");
}

#[tokio::test]
async fn create_telemetry_should_reject_unknown_device() {
    let device_repository = MockDeviceRepository::new();
    let telemetry_repository = MockTelemetryRepository::new();

    let service = TelemetryService::new(device_repository, telemetry_repository);

    let request = TelemetryRequest {
        device_code: "UNKNOWN-001".to_string(),
        metrics: vec![MetricRequest {
            key: "temperature".to_string(),
            value: 25.5,
            unit: "celsius".to_string(),
        }],
    };

    let result = service.create_telemetry(request).await;

    assert!(result.is_err());

    let error = result.unwrap_err().to_string();

    assert_eq!(error, "Device with code UNKNOWN-001 not found");
}

#[tokio::test]
async fn create_telemetry_should_return_error_when_repository_fails() {
    let device_repository = MockDeviceRepository::new();
    let telemetry_repository = MockTelemetryRepository::failing();

    let device = Device::new(
        Uuid::new_v4(),
        "SIM-001".to_string(),
        Utc::now(),
        Utc::now(),
        "Simulator 001".to_string(),
        DeviceType::Simulator,
    );

    device_repository.save(&device).await.unwrap();

    let service = TelemetryService::new(device_repository, telemetry_repository);

    let request = TelemetryRequest {
        device_code: "SIM-001".to_string(),
        metrics: vec![MetricRequest {
            key: "temperature".to_string(),
            value: 25.5,
            unit: "celsius".to_string(),
        }],
    };

    let result = service.create_telemetry(request).await;

    assert!(result.is_err());

    let error = result.unwrap_err().to_string();

    assert_eq!(error, "Failed to save telemetry");
}

#[tokio::test]
async fn create_telemetry_should_reject_empty_metrics() {
    let device_repository = MockDeviceRepository::new();
    let telemetry_repository = MockTelemetryRepository::new();

    let service = TelemetryService::new(device_repository, telemetry_repository);

    let request = TelemetryRequest {
        device_code: "SIM-001".to_string(),
        metrics: vec![],
    };

    let result = service.create_telemetry(request).await;

    assert!(result.is_err());

    let error = result.unwrap_err().to_string();

    assert_eq!(error, "Metrics cannot be empty");
}
#[tokio::test]
async fn get_telemetry_should_return_samples() {
    let device_repository = MockDeviceRepository::new();
    let telemetry_repository = MockTelemetryRepository::new();

    let device = Device::new(
        Uuid::new_v4(),
        "SIM-001".to_string(),
        Utc::now(),
        Utc::now(),
        "Simulator 001".to_string(),
        DeviceType::Simulator,
    );

    device_repository.save(&device).await.unwrap();

    let service = TelemetryService::new(device_repository, telemetry_repository);

    let request = TelemetryRequest {
        device_code: "SIM-001".to_string(),
        metrics: vec![
            MetricRequest {
                key: "temperature".to_string(),
                value: 25.5,
                unit: "celsius".to_string(),
            },
            MetricRequest {
                key: "humidity".to_string(),
                value: 60.0,
                unit: "percent".to_string(),
            },
        ],
    };

    service.create_telemetry(request).await.unwrap();

    let samples = service.get_telemetry(device.id, 100).await.unwrap();

    assert_eq!(samples.len(), 2);

    assert_eq!(samples[0].key, "temperature");
    assert_eq!(samples[0].value, 25.5);
    assert_eq!(samples[0].unit, "celsius");

    assert_eq!(samples[1].key, "humidity");
    assert_eq!(samples[1].value, 60.0);
    assert_eq!(samples[1].unit, "percent");
}
#[tokio::test]
async fn get_telemetry_should_return_not_found() {
    let device_repository = MockDeviceRepository::new();
    let telemetry_repository = MockTelemetryRepository::new();

    let service = TelemetryService::new(device_repository, telemetry_repository);

    let device_id = Uuid::new_v4();

    let result = service.get_telemetry(device_id, 100).await;

    assert!(result.is_err());

    let error = result.unwrap_err().to_string();

    assert_eq!(error, format!("Device with id {} not found", device_id));
}
