mod common;

use chrono::{SubsecRound, Utc};
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

#[tokio::test]
async fn find_by_device_should_return_rows() {
    let pool = test_pool().await;
    let device_id = Uuid::new_v4();
    let device_code = "TEST-QUERY-001";

    // Cleanup first so the test can be rerun safely.
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
    .bind(device_code)
    .execute(&pool)
    .await
    .expect("Failed to clean test telemetry");

    sqlx::query(
        r#"
        DELETE FROM devices
        WHERE code = $1
        "#,
    )
    .bind(device_code)
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
    .bind(device_code)
    .bind("Query Test Device")
    .bind("ONLINE")
    .execute(&pool)
    .await
    .expect("Failed to create test device");

    let recorded_at = Utc::now().trunc_subsecs(3);

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

    let samples = repository
        .find_by_device(device_id, 100)
        .await
        .expect("Failed to find telemetry");
    assert_eq!(samples.len(), 2);

    let temperature = samples
        .iter()
        .find(|sample| sample.key == "temperature")
        .expect("temperature sample not found");

    assert_eq!(temperature.value, 25.5);
    assert_eq!(temperature.unit, "celsius");
    assert_eq!(temperature.recorded_at, recorded_at);

    let humidity = samples
        .iter()
        .find(|sample| sample.key == "humidity")
        .expect("humidity sample not found");

    assert_eq!(humidity.value, 60.0);
    assert_eq!(humidity.unit, "percent");
    assert_eq!(humidity.recorded_at, recorded_at);
}

#[tokio::test]
async fn find_by_device_should_return_empty_vec() {
    let pool = test_pool().await;
    let device_id = Uuid::new_v4();
    let device_code = "TEST-QUERY-EMPTY";

    // Cleanup first so the test can be rerun safely.
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
    .bind(device_code)
    .execute(&pool)
    .await
    .expect("Failed to clean test telemetry");

    sqlx::query(
        r#"
        DELETE FROM devices
        WHERE code = $1
        "#,
    )
    .bind(device_code)
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
    .bind(device_code)
    .bind("Empty Query Device")
    .bind("ONLINE")
    .execute(&pool)
    .await
    .expect("Failed to create test device");

    let repository = PostgresTelemetryRepository::new(pool);

    let samples = repository
        .find_by_device(device_id, 100)
        .await
        .expect("Failed to find telemetry");

    assert!(samples.is_empty());
}

#[tokio::test]
async fn find_by_device_should_respect_limit() {
    let pool = test_pool().await;
    let device_id = Uuid::new_v4();
    let device_code = "TEST-QUERY-LIMIT";

    // Cleanup first so the test can be rerun safely.
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
    .bind(device_code)
    .execute(&pool)
    .await
    .expect("Failed to clean test telemetry");

    sqlx::query(
        r#"
        DELETE FROM devices
        WHERE code = $1
        "#,
    )
    .bind(device_code)
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
    .bind(device_code)
    .bind("Limit Query Device")
    .bind("ONLINE")
    .execute(&pool)
    .await
    .expect("Failed to create test device");

    let recorded_at = Utc::now().trunc_subsecs(3);

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
            Metric {
                key: "battery".to_string(),
                value: 95.0,
                unit: "percent".to_string(),
            },
        ],
        recorded_at,
    };

    let repository = PostgresTelemetryRepository::new(pool);

    repository
        .save(&telemetry)
        .await
        .expect("Failed to save telemetry");

    let samples = repository
        .find_by_device(device_id, 2)
        .await
        .expect("Failed to find telemetry");

    assert_eq!(samples.len(), 2)
}

#[tokio::test]
async fn find_by_device_should_exclude_other_devices() {
    let pool = test_pool().await;
    let device_a_id = Uuid::new_v4();
    let device_a_code = "TEST-ISO-A";
    let device_b_id = Uuid::new_v4();
    let device_b_code = "TEST-ISO-B";

    for code in [&device_a_code, &device_b_code] {
        sqlx::query(
            "DELETE FROM telemetry WHERE device_id IN (SELECT id FROM devices WHERE code = $1)",
        )
        .bind(*code)
        .execute(&pool)
        .await
        .expect("Failed to clean telemetry");
        sqlx::query("DELETE FROM devices WHERE code = $1")
            .bind(*code)
            .execute(&pool)
            .await
            .expect("Failed to clean device");
    }

    for (id, code) in [(device_a_id, device_a_code), (device_b_id, device_b_code)] {
        sqlx::query("INSERT INTO devices (id, code, name, status) VALUES ($1, $2, $3, $4)")
            .bind(id)
            .bind(code)
            .bind("Isolation Device")
            .bind("ONLINE")
            .execute(&pool)
            .await
            .expect("Failed to create device");
    }

    let recorded_at = Utc::now();
    let repo = PostgresTelemetryRepository::new(pool.clone());

    let telemetry_a = Telemetry {
        id: Uuid::new_v4(),
        device_id: device_a_id,
        metrics: vec![Metric {
            key: "temp".to_string(),
            value: 1.0,
            unit: "c".to_string(),
        }],
        recorded_at,
    };
    let telemetry_b = Telemetry {
        id: Uuid::new_v4(),
        device_id: device_b_id,
        metrics: vec![Metric {
            key: "temp".to_string(),
            value: 2.0,
            unit: "c".to_string(),
        }],
        recorded_at,
    };

    repo.save(&telemetry_a).await.expect("Failed to save A");
    repo.save(&telemetry_b).await.expect("Failed to save B");

    let samples_a = repo
        .find_by_device(device_a_id, 100)
        .await
        .expect("Failed to find A");
    assert_eq!(samples_a.len(), 1);
    assert_eq!(samples_a[0].value, 1.0);

    let samples_b = repo
        .find_by_device(device_b_id, 100)
        .await
        .expect("Failed to find B");
    assert_eq!(samples_b.len(), 1);
    assert_eq!(samples_b[0].value, 2.0);
}

#[tokio::test]
async fn find_by_device_should_return_in_desc_order() {
    let pool = test_pool().await;
    let device_id = Uuid::new_v4();
    let device_code = "TEST-ORDER-001";

    sqlx::query(
        "DELETE FROM telemetry WHERE device_id IN (SELECT id FROM devices WHERE code = $1)",
    )
    .bind(device_code)
    .execute(&pool)
    .await
    .expect("Failed to clean telemetry");
    sqlx::query("DELETE FROM devices WHERE code = $1")
        .bind(device_code)
        .execute(&pool)
        .await
        .expect("Failed to clean device");

    sqlx::query("INSERT INTO devices (id, code, name, status) VALUES ($1, $2, $3, $4)")
        .bind(device_id)
        .bind(device_code)
        .bind("Order Device")
        .bind("ONLINE")
        .execute(&pool)
        .await
        .expect("Failed to create device");

    let now = Utc::now();
    let earlier = now - chrono::Duration::seconds(10);
    let repo = PostgresTelemetryRepository::new(pool.clone());

    let telemetry_old = Telemetry {
        id: Uuid::new_v4(),
        device_id,
        metrics: vec![Metric {
            key: "temp".to_string(),
            value: 10.0,
            unit: "c".to_string(),
        }],
        recorded_at: earlier,
    };
    let telemetry_new = Telemetry {
        id: Uuid::new_v4(),
        device_id,
        metrics: vec![Metric {
            key: "temp".to_string(),
            value: 20.0,
            unit: "c".to_string(),
        }],
        recorded_at: now,
    };

    repo.save(&telemetry_old).await.expect("Failed to save old");
    repo.save(&telemetry_new).await.expect("Failed to save new");

    let samples = repo
        .find_by_device(device_id, 100)
        .await
        .expect("Failed to find");
    assert_eq!(samples.len(), 2);
    assert_eq!(samples[0].value, 20.0);
    assert_eq!(samples[1].value, 10.0);
}

#[tokio::test]
async fn find_by_device_should_return_empty_when_limit_zero() {
    let pool = test_pool().await;
    let device_id = Uuid::new_v4();
    let device_code = "TEST-LIMIT-ZERO";

    sqlx::query(
        "DELETE FROM telemetry WHERE device_id IN (SELECT id FROM devices WHERE code = $1)",
    )
    .bind(device_code)
    .execute(&pool)
    .await
    .expect("Failed to clean telemetry");
    sqlx::query("DELETE FROM devices WHERE code = $1")
        .bind(device_code)
        .execute(&pool)
        .await
        .expect("Failed to clean device");

    sqlx::query("INSERT INTO devices (id, code, name, status) VALUES ($1, $2, $3, $4)")
        .bind(device_id)
        .bind(device_code)
        .bind("Limit Zero Device")
        .bind("ONLINE")
        .execute(&pool)
        .await
        .expect("Failed to create device");

    let repo = PostgresTelemetryRepository::new(pool.clone());
    let telemetry = Telemetry {
        id: Uuid::new_v4(),
        device_id,
        metrics: vec![Metric {
            key: "temp".to_string(),
            value: 25.0,
            unit: "c".to_string(),
        }],
        recorded_at: Utc::now(),
    };
    repo.save(&telemetry).await.expect("Failed to save");

    let samples = repo
        .find_by_device(device_id, 0)
        .await
        .expect("Failed to find");
    assert!(samples.is_empty());
}

#[tokio::test]
async fn find_by_device_should_error_when_limit_negative() {
    let pool = test_pool().await;
    let device_id = Uuid::new_v4();
    let device_code = "TEST-LIMIT-NEG";

    sqlx::query(
        "DELETE FROM telemetry WHERE device_id IN (SELECT id FROM devices WHERE code = $1)",
    )
    .bind(device_code)
    .execute(&pool)
    .await
    .expect("Failed to clean telemetry");
    sqlx::query("DELETE FROM devices WHERE code = $1")
        .bind(device_code)
        .execute(&pool)
        .await
        .expect("Failed to clean device");

    sqlx::query("INSERT INTO devices (id, code, name, status) VALUES ($1, $2, $3, $4)")
        .bind(device_id)
        .bind(device_code)
        .bind("Limit Neg Device")
        .bind("ONLINE")
        .execute(&pool)
        .await
        .expect("Failed to create device");

    let repo = PostgresTelemetryRepository::new(pool.clone());
    let telemetry = Telemetry {
        id: Uuid::new_v4(),
        device_id,
        metrics: vec![Metric {
            key: "temp".to_string(),
            value: 1.0,
            unit: "c".to_string(),
        }],
        recorded_at: Utc::now(),
    };
    repo.save(&telemetry).await.expect("Failed to save");

    let result = repo.find_by_device(device_id, -1).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn find_by_device_should_cap_at_high_limit() {
    let pool = test_pool().await;
    let device_id = Uuid::new_v4();
    let device_code = "TEST-LIMIT-CAP";

    sqlx::query(
        "DELETE FROM telemetry WHERE device_id IN (SELECT id FROM devices WHERE code = $1)",
    )
    .bind(device_code)
    .execute(&pool)
    .await
    .expect("Failed to clean telemetry");
    sqlx::query("DELETE FROM devices WHERE code = $1")
        .bind(device_code)
        .execute(&pool)
        .await
        .expect("Failed to clean device");

    sqlx::query("INSERT INTO devices (id, code, name, status) VALUES ($1, $2, $3, $4)")
        .bind(device_id)
        .bind(device_code)
        .bind("Limit Cap Device")
        .bind("ONLINE")
        .execute(&pool)
        .await
        .expect("Failed to create device");

    let repo = PostgresTelemetryRepository::new(pool.clone());
    let telemetry = Telemetry {
        id: Uuid::new_v4(),
        device_id,
        metrics: vec![
            Metric {
                key: "temp".to_string(),
                value: 1.0,
                unit: "c".to_string(),
            },
            Metric {
                key: "humid".to_string(),
                value: 2.0,
                unit: "%".to_string(),
            },
            Metric {
                key: "batt".to_string(),
                value: 3.0,
                unit: "%".to_string(),
            },
        ],
        recorded_at: Utc::now(),
    };
    repo.save(&telemetry).await.expect("Failed to save");

    let samples = repo
        .find_by_device(device_id, 1000)
        .await
        .expect("Failed to find");
    assert_eq!(samples.len(), 3);
}
