# Telemetry Hub

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust: Edition 2024](https://img.shields.io/badge/rust-edition_2024-orange.svg)](Cargo.toml)
[![Status: Pre-release](https://img.shields.io/badge/status-pre--release-yellow.svg)](ROADMAP.md)
[![Database: PostgreSQL 17](https://img.shields.io/badge/database-postgresql_17-336791.svg)](docker/docker-compose.yml)

A device-agnostic telemetry platform in Rust. The backend is developed against a
virtual device simulator first, so hardware — an ESP32, an Arduino, anything —
is never a prerequisite for building, testing, or extending the platform.

## Why Telemetry Hub exists

Building an IoT backend is usually blocked by a chicken-and-egg problem: the
backend waits for hardware, and the hardware waits for a backend. Telemetry Hub
breaks that loop by making a telemetry source a matter of *data shape*, not
hardware: anything that can emit a metric — `{key, value, unit}` — is a
first-class citizen. The simulator is not a demo; it is the platform's first
device, and it exercises the exact same contract a future ESP32 will use.

The project is also a learning platform: a production-shaped codebase where
Rust developers can study clean architecture, domain modeling, and
trait-based extensibility on a real problem instead of a tutorial CRUD app.
The full identity and decision filter live in
[`docs/PRODUCT_VISION.md`](docs/PRODUCT_VISION.md).

## Project status

**Pre-release (v0.1.0).** The end-to-end ingestion path works: a device
registers itself, sends telemetry, and the samples can be read back through
the query API — run the [Quick start](#quick-start) to see it live. Breaking
changes may still land; the README describes what exists, the
[roadmap](ROADMAP.md) describes what will exist. The two never intentionally
diverge.

## Architecture

Telemetry Hub is a Clean Architecture workspace: domain logic lives in one
place with zero framework dependencies, and everything else is an adapter
plugged into a defined seam.

```text
                    ┌────────────────────────────┐
                    │          DEVICES           │
                    │  Simulator   ESP32   Arduino│
                    │           (future)         │
                    └─────────────┬──────────────┘
                                  │  {key, value, unit} metrics
                    ┌─────────────▼──────────────┐
                    │         TRANSPORT          │
                    │   REST        WebSocket    │
                    │               (planned)    │
                    │   MQTT (planned)           │
                    └─────────────┬──────────────┘
                                  ▼
                    ┌────────────────────────────┐
                    │         HANDLERS           │   HTTP layer, DTOs
                    └─────────────┬──────────────┘
                                  ▼
                    ┌────────────────────────────┐
                    │         SERVICES           │   generic over repo traits
                    └─────────────┬──────────────┘
                                  ▼
                    ┌────────────────────────────┐
                    │    REPOSITORY TRAITS       │   DeviceRepository,
                    │                            │   TelemetryRepository
                    └─────────────┬──────────────┘
                                  ▼
                    ┌────────────────────────────┐
                    │    POSTGRES REPOSITORY     │   SQLx implementation
                    └─────────────┬──────────────┘
                                  ▼
                    ┌────────────────────────────┐
                    │        POSTGRESQL 17       │
                    └─────────────┬──────────────┘
                                  ▼
                    ┌────────────────────────────┐
                    │   FUTURE STORAGE           │   behind the same traits
                    └────────────────────────────┘
```

Dependencies point inward: handlers → services → repository traits ← Postgres
implementation → domain. Replacing Postgres with another store, or adding
WebSocket/MQTT, is a new adapter at an existing seam — not a rewrite.

## Workspace

| Path | Crate | Responsibility | Status |
|---|---|---|---|
| `crates/core` | `telemetry_core` | Domain types, enums, conversions. Zero framework dependencies | ✅ Ready |
| `apps/server` | `server` | HTTP API: config, state, handlers, services, repositories | ✅ Working |
| `apps/simulator` | `simulator` | Virtual device emitting telemetry on an interval | ✅ Working |

The boundaries for a processing pipeline (`crates/telemetry`) and protocol
adapters (`crates/transport`) were removed until they have a real consumer;
per the vision, a crate only gains content when a real need exists.

## Repository tree

```text
.
├── Cargo.toml                  # workspace manifest (resolver 2, edition 2024)
├── apps/
│   ├── server/                 # Axum API
│   └── simulator/              # virtual device
├── crates/
│   ├── core/                   # domain (telemetry_core)
│   ├── telemetry/              # reserved: pipeline
│   └── transport/              # reserved: protocols
├── migrations/                 # SQLx migrations (applied at startup)
├── docker/                     # docker-compose.yml (PostgreSQL 17)
├── docs/                       # architecture, vision, analysis
├── scripts/                    # development tooling (WIP)
└── .github/                    # issue/PR templates
```

## Design philosophy

- **Domain-first.** All knowledge about devices, metrics, events, and alerts
  lives in `telemetry_core` with no framework dependencies. Axum, SQLx, and
  Tokio are adapters, not identity.
- **Contracts over features.** Extending the platform means implementing a
  defined contract — a repository, a transport, a device type — never patching
  an existing layer.
- **Honesty over aspiration.** Docs describe what exists; the roadmap describes
  what will exist.
- **Hardware independence.** The simulator speaks the same contract as future
  hardware, so devices are always optional.

## Quick start

Prerequisites: Docker, Rust 1.85+ (edition 2024).

```bash
# 1. Start PostgreSQL 17 (host port 5439)
docker compose -f docker/docker-compose.yml up -d

# 2. Configure environment
cp .env.example .env
#    edit .env: set DATABASE_URL and POSTGRES_* for your local setup

# 3. Run the server (migrations apply automatically at startup)
cargo run -p server

# 4. Run the simulator (registers SIMULATOR-001, sends telemetry every second)
cargo run -p simulator
```

End-to-end demo (after ~5 seconds of simulator data):

```bash
# Device registration (the simulator does this automatically on startup)
curl http://127.0.0.1:3000/api/v1/devices/SIMULATOR-001
# → {"id":"1269f242-1183-48d8-924b-4102cc47e944","code":"SIMULATOR-001",
#    "name":"Simulator Device","status":"ONLINE","device_type":"SIMULATOR"}

# Query the 5 newest samples
curl "http://127.0.0.1:3000/api/v1/telemetry?device_id=1269f242-1183-48d8-924b-4102cc47e944&limit=5"
# → {"device_id":"1269f242-1183-48d8-924b-4102cc47e944","count":5,
#    "samples":[{"key":"battery","value":99.91,"unit":"percent","recorded_at":"2026-08-11T17:02:44.003843Z"}, ...]}
```

Your device id will differ — grab it from the output of the devices curl, then
use it in the telemetry query. The response is newest-first, `limit` is clamped
to 1–1000 (default 100).

## Configuration

Environment variables, read from `.env` via `dotenvy`:

| Variable | Default | Description |
|---|---|---|
| `APP_NAME` | `telemetry-hub` | Service name reported by the API |
| `APP_HOST` | `0.0.0.0` | Bind address |
| `APP_PORT` | `3000` | Bind port |
| `APP_ENV` | `development` | Environment label |
| `DATABASE_URL` | — | PostgreSQL connection string (`postgres://user:pass@host:port/db`) |
| `POSTGRES_DB` / `POSTGRES_USER` / `POSTGRES_PASSWORD` | — | Database provisioning for Docker Compose |
| `SIMULATOR_INTERVAL_MS` | `1000` | Simulator emit interval |

## Development workflow

- Server, simulator, and database run from the workspace root — see
  [CONTRIBUTING.md](CONTRIBUTING.md) for the full engineering handbook.
- Before every PR: `cargo fmt`, `cargo clippy --workspace -- -D warnings`,
  `cargo test --workspace`.
- Migrations are SQLx files in `migrations/`; they run at server startup.
  Append new numbered files — never edit an applied migration.

## Documentation

| Area | Document |
|---|---|
| Product vision | [`docs/PRODUCT_VISION.md`](docs/PRODUCT_VISION.md) |
| Architecture | [`docs/software-architecture.md`](docs/software-architecture.md) |
| Repository analysis | [`docs/PROJECT_ANALYSIS.md`](docs/PROJECT_ANALYSIS.md) |
| Roadmap | [`ROADMAP.md`](ROADMAP.md) |
| Development | [`CONTRIBUTING.md`](CONTRIBUTING.md) |
| Community | [`SUPPORT.md`](SUPPORT.md), [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) |
| Security | [`SECURITY.md`](SECURITY.md) |
| Database / Simulator / API / Deployment | planned — tracked in [ROADMAP.md](ROADMAP.md) |

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) —
contributions are judged on architectural discipline first, feature count
second. All interactions are governed by the
[Code of Conduct](CODE_OF_CONDUCT.md).

## Security

Found a vulnerability? Do **not** open a public issue. Report it privately per
[SECURITY.md](SECURITY.md) — or email the maintainer directly at
adibayuluthfiansyah@gmail.com.

## License

Licensed under the [MIT License](LICENSE). Copyright © 2026 Adibayu Luthfiansyah.
