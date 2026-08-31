mod common;
use chrono::Utc;
use common::test_pool;
use server::repositories::{DeviceRepository, PostgresDeviceRepository};
use telemetry_core::{Device, DeviceType};
use uuid::Uuid;

#[tokio::test]
async fn save_should_persist_device() {
    let pool = test_pool().await;
    sqlx::query(
        r#"
        DELETE FROM telemetry
        WHERE device_id IN (SELECT id FROM devices WHERE code = $1)
        "#,
    )
    .bind("TEST-001")
    .execute(&pool)
    .await
    .expect("Failed to clean test telemetry");
    sqlx::query("DELETE FROM devices WHERE code = 'TEST-001'")
        .execute(&pool)
        .await
        .expect("Failed to clean test device");
    let repository = PostgresDeviceRepository::new(pool);
    let now = Utc::now();
    let device = Device::new(
        Uuid::new_v4(),
        "TEST-001".to_string(),
        now,
        now,
        "Test Device".to_string(),
        DeviceType::Simulator,
    );
    repository
        .save(&device)
        .await
        .expect("Failed to save device");
    let result = repository
        .find_by_code("TEST-001")
        .await
        .expect("Failed to find device");

    let found = result.expect("Device should exist");
    assert_eq!(found.id, device.id);
    assert_eq!(found.code, device.code);
    assert_eq!(found.name, device.name);
    assert_eq!(found.device_type, device.device_type);
    assert_eq!(found.status, device.status);
}

#[tokio::test]
async fn find_by_code_should_return_none_when_missing() {
    let pool = test_pool().await;
    let repository = PostgresDeviceRepository::new(pool);
    let result = repository
        .find_by_code("DOES-NOT-EXIST")
        .await
        .expect("Failed to query device");
    assert!(result.is_none());
}

#[tokio::test]
async fn find_by_id_should_return_device() {
    let pool = test_pool().await;
    let device_code = "TEST-FIND-ID-001";
    sqlx::query("DELETE FROM devices WHERE code = $1")
        .bind(device_code)
        .execute(&pool)
        .await
        .expect("Failed to clean test device");
    let repository = PostgresDeviceRepository::new(pool);
    let now = Utc::now();
    let device = Device::new(
        Uuid::new_v4(),
        device_code.to_string(),
        now,
        now,
        "Find By ID Device".to_string(),
        DeviceType::Simulator,
    );
    repository
        .save(&device)
        .await
        .expect("Failed to save device");
    let found = repository
        .find_by_id(device.id)
        .await
        .expect("Failed to find by id")
        .expect("Device should exist");
    assert_eq!(found.id, device.id);
    assert_eq!(found.code, device.code);
    assert_eq!(found.name, device.name);
    assert_eq!(found.device_type, device.device_type);
    assert_eq!(found.status, device.status);
}

#[tokio::test]
async fn find_by_id_should_return_none_when_missing() {
    let pool = test_pool().await;
    let repository = PostgresDeviceRepository::new(pool);
    let result = repository
        .find_by_id(Uuid::new_v4())
        .await
        .expect("Failed to query device");
    assert!(result.is_none());
}

#[tokio::test]
async fn save_should_fail_on_duplicate_code() {
    let pool = test_pool().await;
    let device_code = "TEST-DUPLICATE-001";
    sqlx::query("DELETE FROM devices WHERE code = $1")
        .bind(device_code)
        .execute(&pool)
        .await
        .expect("Failed to clean test device");
    let repository = PostgresDeviceRepository::new(pool);
    let now = Utc::now();
    let device = Device::new(
        Uuid::new_v4(),
        device_code.to_string(),
        now,
        now,
        "Original Device".to_string(),
        DeviceType::Simulator,
    );
    repository
        .save(&device)
        .await
        .expect("Failed to save first device");
    let duplicate = Device::new(
        Uuid::new_v4(),
        device_code.to_string(),
        now,
        now,
        "Duplicate Device".to_string(),
        DeviceType::Esp32,
    );
    let result = repository.save(&duplicate).await;
    assert!(result.is_err());
}
