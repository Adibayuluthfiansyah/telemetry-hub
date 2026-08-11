# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Device registration API: `POST /api/v1/devices` (HTTP 201, 409 on duplicate code).
- Device lookup API: `GET /api/v1/devices/{code}` (HTTP 200, 404).
- Telemetry ingestion API: `POST /api/v1/telemetry` (HTTP 201; 404 for unknown
  device; 400 for empty metrics or malformed JSON).
- Interval-driven telemetry simulator: `cargo run -p simulator` registers a
  `SIMULATOR` device and emits temperature/humidity/battery samples on an
  interval (configurable via `SIMULATOR_INTERVAL_MS`, `SIMULATOR_SERVER_URL`,
  `SIMULATOR_DEVICE_CODE`, `SIMULATOR_DEVICE_NAME`).
- Telemetry query API: `GET /api/v1/telemetry?device_id=<uuid>&limit=<n>`
  (HTTP 200 with `{device_id, count, samples}`; 404 unknown device; 400
  invalid/missing `device_id`; limit clamped to 1–1000, default 100).


No releases yet. This section will collect user-visible changes before the
first tag (`v0.1.0`).
