# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- (nothing yet — next cycle starts here)

## [0.1.0] - 2026-09-03

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
- Health endpoint DB-aware readiness check: `GET /api/v1/health` (HTTP 200/503,
  DB probe with 2s timeout).
- Structured request tracing middleware (Axum `TraceLayer` — request spans +
  latency logging via `tracing`).
- One-command development stack: `./scripts/dev.sh` (Postgres → Server →
  Simulator with health checks and clean `Ctrl-C` teardown).
- WebSocket transport: `crates/transport` with `EventEnvelope` + `EventPublisher`
  traits (zero framework deps in `telemetry_core`/`telemetry_transport`).
- Live event stream endpoint: `GET /api/v1/stream` (WebSocket, live-only
  broadcast; `TELEMETRY_RECEIVED` and `DEVICE_CONNECTED` wired).
- Telemetry metrics wired into `TELEMETRY_RECEIVED` payload (`payload.metrics`).
- API reference: `docs/api.md` + hand-written `docs/openapi.yaml` (OpenAPI 3.1)
  for all REST + WebSocket endpoints.
- Next.js 16 dashboard scaffold (`frontend/`, App Router, Tailwind v4, Turbopack,
  port 3001) with shadcn/ui (base-nova, dark-only, `RADIUS=0`).
- TypeScript wire contract mirror (`lib/types.ts`: `EventEnvelope`, `EventType`
  union, `TelemetryMetric`, `parseEvent` guard) + `lib/utils.ts`.
- WebSocket stream hook: `hooks/useTelemetryStream.ts` (native browser WebSocket
  to `/api/v1/stream`, bounded 50-event buffer, per-metric stats, device
  registry, fixed 2s reconnect with `document.hidden` pause, StrictMode-safe
  deduplication via `seenEventIdsRef`).
- Dashboard design system (`frontend/DESIGN.md`): 24 surface tokens, operational
  colors, Geist/Inter/JetBrains Mono, sharp corners, 4px/8px spacing, tonal
  depth.
- Dashboard components: `StatusIndicator`, `KPICard`, `EventFeed` + `EventRow`,
  `MetricReadout` + `TelemetryPanel`, `Sidebar`, `TopBar`, `FooterBar`,
  `DashboardLayout` (WebSocket-driven, no polling).
- Format utilities (`lib/format.ts`): `formatValue`, `formatRelativeTime`,
  `formatAbsoluteTime`, `formatUtcTimestamp` (`00:12:43.102 UTC`), `formatBytes`,
  `formatDuration`, unit labels.
- Frontend dev integration: `scripts/dev.sh` now also starts the Next.js
  dashboard (port 3001 health check, `FRONTEND_PID` in cleanup).
- Frontend environment template: `frontend/.env.local.example` +
  `!.env.local.example` gitignore negation.
- Dashboard UI polish: sidebar Button height fix (`h-auto`), active variant
  alignment, typography scale made size-agnostic, console outer frame with
  dotted background.
- CI: frontend job (Node 20, `npm install`, lint, build) in
  `.github/workflows/ci.yml`.
- Deployment guide: `docs/deployment.md` (dev-only, ports 5439/3000/3001, env
  vars, one-command stack).
- Test hardening M4 — 48 → 92 workspace tests:
  - T1 domain core: `Display`/`TryFrom`/serde for enums, `Device`/`Alert`/`Event` (#52).
  - T2 transport & model mapping: `EventEnvelope` payload + error paths, `DeviceRecord`/`TelemetryRecord` (#53).
  - T3 Postgres repos: `find_by_id`, duplicate `code`, isolation, limits `0`/`-1`/`1000`, ordering (#54).
  - T4 service: `MockDeviceRepository::failing` + `MockTelemetryRepository` spy `last_limit` (`max(0)` vs `clamp(1,1000)`), limit-clamp and error bubbling (#55).
  - T5 handler: health `503` when pool closed, bad metric shape (`missing key` / `string value` → 400), stream `?device_id=invalid-uuid` → 400 (#56).

### Changed

- ROADMAP.md — M3 dashboard milestone marked ✅ (#25, #44); M4 hardening marked ✅ for tests, API docs, CHANGELOG, deployment (#51–#56).
- README.md — Quick start now mentions the dashboard (`http://localhost:3001`);
  Documentation table adds Dashboard row.
