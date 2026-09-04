# Software Architecture

## Project

**Telemetry Hub**

A modular real-time telemetry platform built with Rust for collecting, processing, and visualizing telemetry data from virtual devices and future physical IoT devices.

---

# Vision

Build a modern telemetry platform that is modular, scalable, and hardware-independent during development.

---

# Scope

## Included (v1)

* Virtual Device Simulator (first-class `DeviceType::Simulator` — same contract as future hardware)
* Rust Backend (Axum, Clean Architecture: `telemetry_core` → services → repository traits → Postgres)
* PostgreSQL 17 + SQLx (migrations run at startup, `telemetry` + `devices` tables)
* REST API (`POST /devices`, `GET /devices/{code}`, `POST /telemetry`, `GET /telemetry`, `GET /health`)
* WebSocket live stream (`GET /api/v1/stream` — `TELEMETRY_RECEIVED` + `DEVICE_CONNECTED`, live-only broadcast 256)
* Live Dashboard (Next.js 16, App Router, Tailwind v4, shadcn/ui base-nova, dark-only, `RADIUS=0`)

## Excluded (v1)

* Authentication & Authorization (single-tenant era — M5+)
* MQTT transport (protocol independence — M5+)
* Physical IoT Devices (ESP32/Arduino — first-class `DeviceType`s in M5+)
* Alerting pipeline (`ALERT_RAISED` / `DEVICE_DISCONNECTED` not yet emitted — M5+)
* Kubernetes / Cloud Deployment (Non-goals until after `v0.1.0` — see `ROADMAP.md`)
* Multi-tenancy, Billing, HA clustering, BI analytics

---

# Architecture

```text
Virtual Device Simulator ──►  Rust Backend (Axum)
                               │
              ┌────────────────┼────────────────┐
              │                │                │
           Handlers        Services        Events
        (DTO, AppError)  (generic over   (Broadcast
              │           repo traits)    EventPublisher
              ▼                │          256)
          Services ───────────►│
              │                ▼
              │          Repository Traits
              │        (DeviceRepository,
              │         TelemetryRepository)
              │                │
              │                ▼
              │        Postgres Repository
              │        (SQLx, DeviceRecord/
              │         TelemetryRecord)
              │                │
              │                ▼
              │          PostgreSQL 17
              │         (5439→5432)
              │                │
              └────────────────┼────────────────┘
                               ▼
                      WebSocket ──► Next.js Dashboard
                      /api/v1/stream      (port 3001)
```

Dependencies point inward: `handlers → services → repository traits ← Postgres impl → domain`. `telemetry_core` and `telemetry_transport` have **zero framework dependencies**.

---

# Core Domains

## Device

Represents a telemetry source. Lives in `telemetry_core::Device` — zero framework deps.

| Field | Type | Description |
| ----- | ---- | ----------- |
| `id` | `Uuid` | Unique identifier |
| `code` | `String` | Unique human code (e.g., `SIMULATOR-001`) — unique index |
| `name` | `String` | Display name |
| `device_type` | `DeviceType` | `SIMULATOR` / `ESP32` / `ARDUINO` (`Display`/`TryFrom`/`serde`) |
| `status` | `DeviceStatus` | `ONLINE` / `OFFLINE` |
| `created_at` / `updated_at` | `DateTime<Utc>` | Lifecycle timestamps |

## Telemetry

Generic metric shape — any `{key, value, unit}` is first-class, not hardcoded `temperature`/`humidity`.

| Field | Type | Description |
| ----- | ---- | ----------- |
| `id` | `Uuid` | Sample id |
| `device_id` | `Uuid` | Source device |
| `metrics` | `Vec<Metric>` | `Metric { key: String, value: f64, unit: String }` |
| `recorded_at` | `DateTime<Utc>` | Ingestion time (`trunc_subsecs(3)` for PG millis) |

Query: `GET /telemetry?device_id=<uuid>&limit=<1..1000>` → `Sample { key, value, unit, recorded_at }` newest-first.

## Alert

Computed severity, never carried — `Alert { id, device_id, severity, message, created_at }` (`AlertSeverity` enum). `ALERT_RAISED` event not yet emitted (M5+ pipeline).

## Event

`Event { id, event_type, device_id, created_at }` + `EventEnvelope { event_id, event_type, device_id, created_at, payload }` (flat SCREAMING_SNAKE_CASE wire format). Today: `DEVICE_CONNECTED` (payload `null`) and `TELEMETRY_RECEIVED` (payload `{ metrics: [...] }`).

---

# Technology Stack

## Backend

* Rust 1.85+ (edition 2024, resolver 2)
* Tokio (async runtime)
* Axum (HTTP + WebSocket `WebSocketUpgrade`)
* Serde / Serde JSON (DTOs + wire `EventEnvelope`)
* SQLx 0.8 (Postgres 17, `PgPool`, `query_as`, migrations `sqlx::migrate!`)
* `tracing` + `TraceLayer` (structured request spans)
* `uuid` / `chrono` (domain primitives)
* `tokio::sync::broadcast` 256 (live event fan-out)

## Frontend

* Next.js 16.3 (App Router, Turbopack)
* React 19
* Tailwind CSS v4 (`@theme inline`, 24 surface tokens, `RADIUS=0`, dark-only)
* shadcn/ui base-nova (Button, Card, Badge, etc.)
* Geist / Inter / JetBrains Mono (`next/font`)
* Native browser WebSocket (`hooks/useTelemetryStream.ts` — dedup `seenEventIdsRef`, `limit 50` buffer, `document.hidden` pause)

## Development

* Cargo Workspace (`crates/core`, `crates/transport`, `apps/server`, `apps/simulator`, `frontend`)
* Docker + Compose v2 (Postgres 17 on `5439`)
* `scripts/dev.sh` (Postgres → Server `:3000` → Dashboard `:3001` → Simulator, health checks, `Ctrl-C` teardown)
* Git + GitHub Actions (CI: `fmt` + `clippy -D warnings` + `test` + frontend `lint`/`build`)

---

# Development Roadmap

Phases map to `ROADMAP.md` — single source of truth is `ROADMAP.md`; this section is a summary.

## Phase 1–5 — Foundation → Ingestion (M0–M2) ✅

Project setup, Clean Architecture seams, domain core, `telemetry` table + `PostgresTelemetryRepository`, `DeviceService`/`TelemetryService`, routes `POST /devices` / `POST /telemetry` / `GET /telemetry` / `GET /health`, simulator, tracing, `docs/database.md` + `docs/simulator.md`.

## Phase 6 — Real-time spine (M3) ✅

`crates/transport` (`EventPublisher`), `/api/v1/stream` WebSocket, `EventEnvelope` wired, dashboard (Next.js) consuming stream, `docs/api.md` + `openapi.yaml`.

## Phase 7 — Hardening and first release (M4) 🔨 → ✅ (docs close-out)

92 workspace tests (T1 domain 19, T2 transport+model 6, T3 PG 8, T4 service 7, T5 handler 4), CI frontend job, `docs/deployment.md`, `CHANGELOG.md` backfilled, `v0.1.0` next.

## Phase 8 — Platform maturity (M5+) ⏳

MQTT, physical `ESP32`/`ARDUINO`, alerting pipeline (`crates/telemetry`), Timescale-aware storage behind traits, auth & multi-tenancy.

---

# Design Principles

* Modular Architecture
* Separation of Concerns
* Clean Architecture
* Domain-First Design
* Extensibility
* Testability
* Hardware Independence
