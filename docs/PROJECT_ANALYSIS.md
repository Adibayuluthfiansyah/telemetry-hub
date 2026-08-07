# PROJECT_ANALYSIS

> Technical maintainer's review of the repository, performed before the first
> public release. This document is a **snapshot in time** (v0.1.0, pre-release).
> It is the source of truth for what the codebase was at analysis time; where it
> disagrees with the code, the code is the authority — file an issue.

## 1. Executive Summary

Telemetry Hub is a Rust workspace (edition 2024, resolver 2) aiming to become a
modular, hardware-independent real-time telemetry platform: virtual device
simulators emit sensor data, an Axum backend ingests and persists it, a
dashboard visualizes it.

At the time of analysis the repository is at a **very early scaffolding stage**
(roughly Phase 2 of the 8-phase roadmap). Of 6 workspace members, only 2 contain
meaningful code (`crates/core`, `apps/server`). The server compiles, exposes
exactly one route (`GET /health`), and contains a layered skeleton
(handlers → services → repository traits → Postgres implementation) that is
roughly half-built: one repository impl is complete, one is an empty struct,
two service files are empty or `todo!()`, and all DTOs are wired to nothing.
There are no real tests, no CI, and the simulator is a `Hello, world!` stub.

The architectural direction — Clean Architecture with domain-first design and
repository traits — is sound. The main risks are premature modularization
(3 empty placeholder crates), documentation that overpromises, and small
correctness issues (hardcoded bind address, panic-based DB mapping, a compose
env override bug) that compound as the codebase grows.

## 2. Project Vision

See `docs/PRODUCT_VISION.md` — this document's companion. In short: a telemetry
platform where **the backend is fully developable before any physical hardware
exists**, with transport and persistence swappable behind clean boundaries.

## 3. Project Goals

Explicit (from README/docs): real-time telemetry processing, virtual device
simulator, REST API, WebSocket streaming, live dashboard, modular architecture,
hardware independence.

Implied from code: Clean Architecture layering with swappable persistence
(repository traits) and shared domain types; PostgreSQL persistence via SQLx
migrations.

**Observed contradiction (now resolved in the roadmap):** the v1 scope in the
original architecture doc said database persistence was "excluded" and
in-memory state "included"; the code went the opposite way (Postgres today, no
in-memory store). `ROADMAP.md` and the docs reflect the implemented direction:
Postgres first, in-memory/streaming layers later.

## 4. Current Development Stage

- Git history: workspace init → backend-server (DB + health) → domain-layer
  (domain models + record mapping) → query-layer WIP.
- Roughly Phase 2-3 of the roadmap: domain exists; API is a skeleton (`/health`
  only); simulator is a stub; WebSocket, dashboard, and the telemetry table do
  not exist.

## 5. Architecture Analysis

```
apps/simulator ──(future HTTP)──▶ apps/server (Axum)
                                     │
                        ┌────────────┴────────────┐
                  handlers (HTTP)            (future) WebSocket
                        │
                   services (generic over repo trait)
                        │
              repositories: trait + postgres impl
                        │
                   sqlx PgPool → Postgres 17
                        │
              crates/core (domain: Device, Telemetry, Alert, Event, Metric)
```

- `crates/core` — domain entities + enums, no framework dependencies. Correct
  direction.
- `config/` — env-based configuration.
- `state/` — `AppState { config, db }` injected into the router.
- `handlers/` — HTTP layer; currently only `health`.
- `services/` — generic services over repository traits.
- `repositories/` — trait definitions + `postgres/` implementations + record
  models.
- `dto/` — request/response types (defined, not yet wired to handlers).

**Data flow today:** `GET /health` → static JSON. There is no end-to-end
telemetry flow anywhere: no route, no service, no telemetry table, no simulator.

**Dependency direction is clean:** handlers → services → repository traits ←
postgres impls; everything → `telemetry_core`.

**Where layering breaks down:**

1. `DeviceRecord::from` uses `.expect()` on enum string conversion — a
   corrupted row panics the request thread. Conversion must return `Result`.
2. Enum columns are `VARCHAR` + hand-written `TryFrom<&str>`; no DB-level
   constraint.
3. `Telemetry` domain shape is incoherent (fixed sensor fields + generic
   metrics — see §9).
4. `common`, `telemetry`, `transport` exist with zero functionality —
   modularization by aspiration (see §6).

## 6. Workspace Analysis

`Cargo.toml`: workspace, resolver 2, 6 members. `Cargo.lock` committed (correct
for a binary workspace).

| Member | Package | Real content | Deps |
|---|---|---|---|
| `apps/server` | `server` | Layered skeleton | axum, tokio, sqlx, serde, chrono, uuid, anyhow, async-trait, dotenvy |
| `apps/simulator` | `simulator` | `Hello, world!` stub | none |
| `crates/core` | `telemetry_core` | Domain entities + enums | chrono, uuid |
| `crates/common` | `common` | boilerplate `add()` | none |
| `crates/telemetry` | `telemetry` | boilerplate `add()` | none |
| `crates/transport` | `transport` | boilerplate `add()` | none |

**Critique:**

- Three crates are cargo-new stubs. Recommendation: keep the boundaries but
  make them honest — a crate exists only when a consumer exists
  (PRODUCT_VISION principle P8). `telemetry`/`transport` are reserved for the
  pipeline and protocol work in M2-M5; `common` should be removed until it has
  real content.
- Package naming collision: `telemetry` (empty) vs `telemetry_core` (real).
- No `rustfmt.toml`, no workspace lint config. Edition 2024 used consistently.

## 7. Crate Responsibilities

- `telemetry_core` — pure domain types, no I/O, no framework.
- `common` — (reserved) shared utilities.
- `telemetry` — (reserved) processing pipeline: validation → enrichment →
  alerting → storage.
- `transport` — (reserved) protocol adapters: HTTP, WebSocket, future MQTT.
- `apps/server` — HTTP API + persistence.
- `apps/simulator` — generates fake telemetry on an interval.

## 8. Backend Architecture

### Config
`Config::load()` reads `APP_NAME`, `APP_HOST`, `APP_PORT`, `DATABASE_URL` via
dotenvy; all mandatory, panics on missing. No defaults, no validation, no test.

**Fixed during release prep:** `main.rs` previously hardcoded
`127.0.0.1:3000`; it now binds `config.app_host:config.app_port`.

### State
`AppState { config, db }` — Clone-able, correct for Axum. `app.rs` →
`router.rs` (`create_router().with_state(state)`) is idiomatic and testable.

### Router & handlers
Single route `GET /health`, static response. It does not check DB connectivity,
so it reports liveness, not readiness.

### Services
`DeviceService<R: DeviceRepository>` — generic over the repository trait
(unit-testable with a mock). `create_device` is `todo!()`.
`telemetry_service.rs` is empty.

### Repositories
- Traits: `DeviceRepository { save, find_by_code }`,
  `TelemetryRepository { save, find_latest }` — `Send + Sync`, `async_trait`.
- `PostresDeviceRepository` (typo in name, kept for now — rename before any
  external consumer) implements both trait methods with parameterized SQL.
- `PostgresTelemetryRepository` — empty struct, does not implement its trait.
- `models/telemetry_record.rs` — empty (no telemetry table yet).

### Why generics? Why traits? Why separate Postgres impl?
- `DeviceService<R>` + trait boundary: swap storage without touching service
  logic; unit-test services with mock repositories. The single best
  architectural decision in the repo.
- **Unrealized payoff:** nothing exercises these abstractions yet. The
  architecture is currently cost without benefit; benefit arrives when routes
  are wired.

## 9. Domain Model Analysis

- `Device` (id, code, name, status, device_type, created_at, updated_at) —
  clean; `is_online()`.
- Enums: `DeviceType { Simulator, Esp32, Arduino }`, `DeviceStatus { Online,
  Offline, Error }`, `AlertSeverity { Info, Warning, Critical }`, `EventType` —
  with `Display` + `TryFrom<&str>`. **Missing serde derives** — no domain type
  can be serialized to JSON, blocking API/WebSocket use.
- `Telemetry` — three design problems:
  1. Hardcoded `temperature`/`humidity` f32 fields *alongside* `Vec<Metric>`:
     the type wants to be both fixed schema and key-value map. Resolve toward
     generic metrics only (PRODUCT_VISION P1).
  2. `severity: AlertSeverity` on raw telemetry conflates observation with
     derived state (P6).
  3. `Alert.device_id: String` while `Event.device_id`/`Telemetry.device_id`
     are `Uuid`.
- `Event` is never referenced elsewhere — dead domain concept awaiting the
  streaming spine (M3).
- Domain types are `Clone + Debug` but not `PartialEq` — hampers tests.

## 10. Database Analysis

Migrations via `sqlx::migrate!`:

1. `init_schema` — `devices(id UUID PK, code VARCHAR(50) UNIQUE, name
   VARCHAR(100), status VARCHAR(20) DEFAULT 'OFFLINE', created_at, updated_at)`.
2. `add_device_type_to_devices` — `ALTER TABLE ... ADD COLUMN device_type
   VARCHAR(50) DEFAULT 'SIMULATOR'`.

**Critique:**

- Adding the column in a second migration is correct practice.
- **No `telemetry` table exists** — yet `TelemetryRepository` is already
  defined. Repository layer is ahead of the schema.
- Enum columns are `VARCHAR` + app-level conversion with `.expect()` panics.
- `updated_at` never refreshed after INSERT (no trigger).
- No indexes beyond `UNIQUE(code)`; `find_latest(device_id)` will need
  `(device_id, created_at DESC)`.
- Migration path `sqlx::migrate!("../../migrations")` is relative to
  `CARGO_MANIFEST_DIR` — works, but obscure.
- Docker Compose runs Postgres 17 on host port 5439. **Fixed during release
  prep:** the `environment:` block interpolated `${POSTGRES_*}` to empty
  strings (no `.env` in the run directory), overriding `env_file: ../.env`
  and causing a restart loop. Only `env_file` remains.

## 11. API Readiness

**Not ready.** One endpoint (`GET /health`, static). DTOs defined but unused.
No device CRUD, no ingestion, no error contract, no validation, no OpenAPI,
no CORS, no tracing middleware. The route→service→repository pipeline is a
skeleton.

## 12. Development Workflow

- Feature branches (`feat/*`) merged via GitHub PRs. Reasonable for one
  developer.
- Commit messages conventional-commit-ish but inconsistent.
- **No CI** — `cargo fmt`/`clippy`/`test` never gated. 23 warnings at analysis
  time. `scripts/` is empty.

## 13. Open Source Readiness

| Item | Status at analysis | Now |
|---|---|---|
| README | Claimed features that did not exist | Rewritten (honest) |
| LICENSE | Missing (claimed MIT) | Added |
| CONTRIBUTING | Missing | Added |
| SECURITY | Missing | Added |
| CODE_OF_CONDUCT | Missing | Added |
| CI | Missing | Planned (M0) |
| CHANGELOG | Missing | Added |
| ROADMAP | Scattered in docs | Dedicated `ROADMAP.md` |

## 14. Missing Components (priority-ranked)

**P0 — correctness/release blockers:** LICENSE file; hardcoded bind address
(fixed); compose env override (fixed); telemetry table + record +
`PostgresTelemetryRepository`; `Result`-based enum mapping.

**P1 — product vision:** wire device/telemetry routes; implement simulator;
complete service layer; DB-aware health check; CI gate.

**P2 — quality:** resolve placeholder crates; fix `Postres` typo; serde on
domain types; reconcile `Telemetry` shape; unit tests; honest README (done);
dev ergonomics (Makefile/justfile, seed script); `updated_at` handling;
`(device_id, created_at)` index; error-response contract; tracing.

**P3 — roadmap:** WebSocket streaming, dashboard, MQTT, auth, physical
devices, deployment.

## 15. Technical Debt

- `Telemetry` shape mismatch — cost of fixing grows with every consumer.
- Panic-based DB→domain mapping.
- Dead placeholder crates + boilerplate `add()` APIs.
- Half-plumbed `AppState`/`Config` (dead code warnings).
- 23 compiler warnings, no lint gate.
- `updated_at` never refreshed.
- Empty files (`telemetry_service.rs`, `telemetry_record.rs`).
- Typo `Postres` (cheap to rename now, expensive after release).

## 16. Recommended Repository Structure

```
telemetry-hub/
├── Cargo.toml            # workspace: apps/*, crates/*
├── apps/
│   ├── server/
│   └── simulator/
├── crates/
│   ├── core/             # sole active domain crate
│   ├── telemetry/        # reserved: pipeline (M2+)
│   └── transport/        # reserved: protocols (M3+)
├── migrations/
├── docker/
├── scripts/
├── docs/
├── .github/workflows/    # planned (M0)
└── community files
```

Rationale: monorepo with one lockfile and atomic PRs is right for this
project. Placeholder crates must not grow content until a consumer exists.

## 17. Proposed Documentation Structure

1. `README.md` — truthful front door; teaches the architecture.
2. `docs/software-architecture.md` — canonical architecture doc; must be
   updated in the same PR as architectural changes.
3. `docs/PRODUCT_VISION.md` — identity, philosophy, decision filter.
4. `ROADMAP.md` — phases, status, exit criteria.
5. `CHANGELOG.md` — Keep a Changelog.
6. `CONTRIBUTING.md` — engineering handbook.
7. `SECURITY.md`, `SUPPORT.md`, `CODE_OF_CONDUCT.md`, `.github/` templates.

## 18. Roadmap Proposal

- **M0 — Stabilize:** CI gate, LICENSE, debt fixes, honest docs.
- **M1 — Ingestion path:** telemetry table, repository impls, device +
  telemetry routes, DB-aware health.
- **M2 — Simulator & observability:** real simulator, seed tooling, tracing,
  error contract.
- **M3 — Real-time:** WebSocket stream, dashboard.
- **M4 — Hardening & release:** tests, API docs, first tag `v0.1.0`.
- **M5+ — Platform:** MQTT, physical devices, auth.

*(Mirrors `ROADMAP.md`; the ROADMAP is the living version of this section.)*
