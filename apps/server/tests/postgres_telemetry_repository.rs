mod common;
use chrono::Utc;
use common::test_pool;
use server::repositories::{PostgresTelemetryRepository, TelemetryRepository};
use sqlx::FromRow;
use telemetry_core::{Telemetry, metric::Metric};
use uuid::Uuid;

#[derive(Debug, FromRow)]
struct TelemetryTestRow {
    key: String,
    value: f64,
    unit: String,
}

#[tokio::test]
async fn save_should_persist_all_metrics() {
    let pool = test_pool().await;
    let device_id = Uuid::new_v4();
    sqlx::query(
        r#"
    DELETE FROM telemetry
    WHERE device_id IN (
        SELECT id
        FROM devices
        WHERE code = $1
    )
    "#,
    )
    .bind("TEST-001")
    .execute(&pool)
    .await
    .expect("Failed to clean test telemetry");

    sqlx::query(
        r#"
    DELETE FROM devices
    WHERE code = $1
    "#,
    )
    .bind("TEST-001")
    .execute(&pool)
    .await
    .expect("Failed to clean test device");

    sqlx::query(
        r#"
    INSERT INTO devices (
        id,
        code,
        name,
        status
    )
    VALUES ($1, $2, $3, $4)
    "#,
    )
    .bind(device_id)
    .bind("TEST-001")
    .bind("Test Device")
    .bind("ONLINE")
    .execute(&pool)
    .await
    .expect("Failed to create test device");

    let recorded_at = Utc::now();
    let telemetry = Telemetry {
        id: Uuid::new_v4(),
        device_id,
        metrics: vec![
            Metric {
                key: "temperature".to_string(),
                value: 25.5,
                unit: "celsius".to_string(),
            },
            Metric {
                key: "humidity".to_string(),
                value: 60.0,
                unit: "percent".to_string(),
            },
        ],
        recorded_at,
    };
    let repository = PostgresTelemetryRepository::new(pool.clone());

    repository
        .save(&telemetry)
        .await
        .expect("Failed to save telemetry");

    let rows = sqlx::query_as::<_, TelemetryTestRow>(
        r#"
        SELECT key, value, unit
        FROM telemetry
        WHERE device_id = $1
        ORDER BY key
        "#,
    )
    .bind(device_id)
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch telemetry");

    assert_eq!(rows.len(), 2);

    assert_eq!(rows[0].key, "humidity");
    assert_eq!(rows[0].value, 60.0);
    assert_eq!(rows[0].unit, "percent");

    assert_eq!(rows[1].key, "temperature");
    assert_eq!(rows[1].value, 25.5);
    assert_eq!(rows[1].unit, "celsius");
}

#[tokio::test]
async fn save_should_rollback_when_insert_fails() {
    let pool = test_pool().await;
    let device_id = Uuid::new_v4();
    sqlx::query(
        r#"
        DELETE FROM telemetry
        WHERE device_id IN (
            SELECT id
            FROM devices
            WHERE code = $1
        )
        "#,
    )
    .bind("TEST-ROLLBACK")
    .execute(&pool)
    .await
    .expect("Failed to clean test telemetry");

    sqlx::query(
        r#"
        DELETE FROM devices
        WHERE code = $1
        "#,
    )
    .bind("TEST-ROLLBACK")
    .execute(&pool)
    .await
    .expect("Failed to clean test device");

    sqlx::query(
        r#"
        INSERT INTO devices (
            id,
            code,
            name,
            status
        )
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(device_id)
    .bind("TEST-ROLLBACK")
    .bind("Rollback Test Device")
    .bind("ONLINE")
    .execute(&pool)
    .await
    .expect("Failed to create test device");
    let recorded_at = Utc::now();
    let telemetry = Telemetry {
        id: Uuid::new_v4(),
        device_id,
        metrics: vec![
            Metric {
                key: "temperature".to_string(),
                value: 25.5,
                unit: "celsius".to_string(),
            },
            Metric {
                key: "a".repeat(101),
                value: 10.0,
                unit: "invalid".to_string(),
            },
        ],
        recorded_at,
    };
    let repository = PostgresTelemetryRepository::new(pool.clone());
    let result = repository.save(&telemetry).await;
    assert!(result.is_err());

    let count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM telemetry
        WHERE device_id = $1
        "#,
    )
    .bind(device_id)
    .fetch_one(&pool)
    .await
    .expect("Failed to count telemetry rows");

    assert_eq!(count, 0);
}
