#[allow(dead_code, unused_imports)]
mod mocks;
use mocks::MockDeviceRepository;
use server::services::device_service::DeviceService;
use telemetry_core::DeviceType;

#[tokio::test]
async fn create_device_should_create_device() {
    let repository = MockDeviceRepository::new();
    let service = DeviceService::new(repository);

    let device = service
        .create_device(
            "SIM-TEST-001".to_string(),
            "Simulator Test".to_string(),
            DeviceType::Simulator,
        )
        .await
        .unwrap();

    assert_eq!(device.code, "SIM-TEST-001");
    assert_eq!(device.name, "Simulator Test");
    assert_eq!(device.device_type, DeviceType::Simulator);
    assert!(device.is_online());
}

#[tokio::test]
async fn create_device_should_reject_duplicate_code() {
    let repository = MockDeviceRepository::new();
    let service = DeviceService::new(repository);
    service
        .create_device(
            "SIM-TEST-001".to_string(),
            "Simulator Test".to_string(),
            DeviceType::Simulator,
        )
        .await
        .unwrap();
    let result = service
        .create_device(
            "SIM-TEST-001".to_string(),
            "Another Simulator".to_string(),
            DeviceType::Simulator,
        )
        .await;
    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    assert!(error.contains("already exists"));
}

#[tokio::test]
async fn get_by_code_should_return_device() {
    let repository = MockDeviceRepository::new();
    let service = DeviceService::new(repository);

    service
        .create_device(
            "SIM-TEST-001".to_string(),
            "Simulator Test".to_string(),
            DeviceType::Simulator,
        )
        .await
        .unwrap();
    let device = service.get_by_code("SIM-TEST-001").await.unwrap();

    assert_eq!(device.code, "SIM-TEST-001");
    assert_eq!(device.name, "Simulator Test");
}

#[tokio::test]
async fn get_by_code_should_return_not_found() {
    let repository = MockDeviceRepository::new();
    let service = DeviceService::new(repository);
    let result = service.get_by_code("DOES-NOT-EXIST").await;
    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    assert!(error.contains("not found"));
}

#[tokio::test]
async fn create_device_should_bubble_db_error() {
    let repository = MockDeviceRepository::failing();
    let service = DeviceService::new(repository);
    let result = service
        .create_device(
            "SIM-TEST-001".to_string(),
            "Simulator Test".to_string(),
            DeviceType::Simulator,
        )
        .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("DB error"));
}

#[tokio::test]
async fn get_by_code_should_bubble_db_error() {
    let repository = MockDeviceRepository::failing();
    let service = DeviceService::new(repository);
    let result = service.get_by_code("ANY-CODE").await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("DB error"));
}
