# Roadmap

The roadmap is the project's **intent**, not its advertising. It derives from
[`docs/PRODUCT_VISION.md`](docs/PRODUCT_VISION.md) (the identity) and
[`docs/PROJECT_ANALYSIS.md`](docs/PROJECT_ANALYSIS.md) (the measured state of
the codebase). Phases land in order; each phase is done when its exit criteria
are met, not when its checklist feels complete.

Legend: ✅ done · 🔨 in progress · ⏳ planned · 🚫 explicitly out of scope

## M0 — Stabilize the foundation

Goal: the repo is trustworthy before it gains features.

- ✅ LICENSE (MIT)
- ✅ Community health files (README, CONTRIBUTING, SECURITY, SUPPORT,
  CODE_OF_CONDUCT, templates)
- ✅ Server binds `APP_HOST`/`APP_PORT` from configuration
- ✅ Docker Compose passes `POSTGRES_*` correctly (env override bug fixed)
- ⏳ CI gate: `fmt` + `clippy -D warnings` + `test` on every PR
- ⏳ Resolve 23 pre-existing compiler warnings
- ⏳ Replace `.expect()` with `Result` in DB→domain mapping
- ⏳ Delete or document the reserved placeholder crates policy

**Exit criteria:** clean build with zero warnings; CI green; every doc
describes only what exists.

## M1 — Complete the ingestion path

Goal: a telemetry sample can travel device → API → database → query.

- ✅ `telemetry` table migration (+ `(device_id, recorded_at)` index)
- ✅ `TelemetryRecord` model and `PostgresTelemetryRepository` implementing
  `TelemetryRepository`
- ✅ `DeviceService::create_device` and the telemetry service layer
- ✅ Routes: `POST /devices`, `POST /telemetry` using the existing DTOs
- ⏳ DB-aware `/health` (readiness, not just liveness)
- ✅ Error-response contract and request validation
- ✅ Telemetry query endpoint: `GET /telemetry` (data can be read back)

**Exit criteria:** end-to-end flow works from curl against a fresh database.

## M2 — Simulator and observability

Goal: the platform's first device is real, and the platform is observable.

- ✅ Real simulator: interval-driven (`SIMULATOR_INTERVAL_MS`), speaking the
  same contract as future hardware (vision principle: the simulator is
  first-class, never a demo path)
- ⏳ Seed script and development tooling in `scripts/`
- ⏳ Structured tracing middleware
- ⏳ Documentation: `docs/database.md`, `docs/simulator.md`

**Exit criteria:** `cargo run -p simulator` populates the database; a single
command reproduces the whole stack.

## M3 — Real-time spine

Goal: consumers stop polling; they subscribe.

- ⏳ WebSocket transport — a new `crates/transport` crate, re-created when
  this lands (the boundary's first real consumer)
- ⏳ Event model wired end-to-end (`TelemetryReceived`, `DeviceConnected`,
  `DeviceDisconnected`, `AlertRaised`)
- ⏳ Dashboard (Next.js, `frontend/`) consuming the stream, not the database
- ⏳ Documentation: `docs/api.md` (OpenAPI)

**Exit criteria:** a browser shows live telemetry pushed over WebSocket.

## M4 — Hardening and first release

Goal: `v0.1.0` is a release people can depend on.

- ⏳ Unit and integration tests (domain, mapping, repositories, handlers)
- ⏳ API documentation published
- ⏳ CHANGELOG entries for every user-visible change since M0
- ⏳ First tagged release `v0.1.0` with release notes
- ⏳ Documentation: `docs/deployment.md`

**Exit criteria:** the release gate in CONTRIBUTING passes without
exceptions.

## M5+ — Platform maturity

Goal: the extensibility axes from the vision start paying off.

- ⏳ MQTT transport (protocol independence — P4)
- ⏳ Physical devices: ESP32/Arduino as first-class `DeviceType`s
- ⏳ Alerting pipeline — a new `crates/telemetry` crate, re-created when this
  lands (severity computed, never carried)
- ⏳ Timescale-aware storage behind the existing repository traits
- ⏳ Authentication & authorization (single-tenant era ends deliberately)

## Non-goals

From the vision (§10) — deliberately out of scope and rejected if they arrive
early:

- 🚫 Kubernetes / cloud deployment before M4
- 🚫 Multi-tenancy, billing, commercial SaaS
- 🚫 HA clustering and sub-millisecond latency guarantees
- 🚫 Data analytics/BI

## Phase bookkeeping

The phases above map to the numbered phases in
[`docs/software-architecture.md`](docs/software-architecture.md):
M0–M2 ≈ Phases 1–5, M3 ≈ Phase 6, M4 ≈ Phase 7, M5+ ≈ Phase 8. When the two
documents disagree, this roadmap is the current intent and the architecture doc
is updated in the same change.
