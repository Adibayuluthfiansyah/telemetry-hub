# PRODUCT_VISION

> The identity of Telemetry Hub, written from the perspective of the project
> founder and software architect. This document is the decision filter for
> every future architectural choice and every public document. It is not
> marketing copy; it is engineering intent.

## 1. Identity — What Telemetry Hub Is

Telemetry Hub is a **device-agnostic telemetry platform whose development never
requires a device**.

The platform's founding stance: a telemetry source is defined by the *shape of
its data*, not by its hardware. Any thing that can emit a metric —
`{key, value, unit}` — is a first-class citizen, whether it is a software
simulator, an ESP32, an Arduino, or something invented tomorrow. The simulator
is not a demo of the platform; it is the platform's first device, and its
presence in the architecture is the guarantee that every later hardware device
inherits a fully developed backend.

The project is simultaneously two things, and both are intentional:

1. A **learning platform** — a production-inspired codebase where Rust
   developers can study and practice clean architecture, domain modeling, and
   trait-based extensibility on a real problem, not a tutorial CRUD app.
2. A **reference implementation** — a canonical example of how to structure a
   real-time telemetry backend in Rust: domain crate, application boundaries,
   swappable persistence, protocol-independent transport.

## 2. The Problem It Solves

Two failures are common in the IoT/telemetry software space:

- **Hardware-gated development.** Backend work waits for hardware; hardware
  waits for backend work. Teams stall on integration instead of building.
- **Tutorial-grade outcomes.** Most learning backends are toy CRUD applications
  that teach frameworks, not architecture. Engineers who finish them cannot
  build production systems — they can only build the same tutorial again.

Telemetry Hub attacks both: it decouples software development from hardware
availability, and it commits to production-shaped architecture while remaining
honest about its learning origin.

## 3. Why an Open-Source Platform Instead of "Just Another Axum Backend"

A private Axum backend would satisfy the author. Telemetry Hub must be open
source because its core promises are public goods:

1. **The platform's value is its architecture, and architecture only compounds
   when shared.** The trait boundaries, domain contracts, and module structure
   are the product's real output. An open repository turns that output into
   infrastructure other Rust developers can study, reuse, and challenge.
2. **Hardware independence requires many devices, and many devices require many
   contributors.** A closed project reaches exactly one device ecosystem. An
   open project accumulates device integrations, transport protocols, and
   storage backends from the community.
3. **The learning promise is fulfilled only in public.** The project's stated
   philosophy is "learn by building the real thing." That promise is kept when
   reviewers, issue discussions, and PRs document *why* decisions were made.
4. **Auditability.** A telemetry platform will eventually move real data. Trust
   for a data-handling system comes from open review of its persistence and
   privacy behavior, not from marketing.

## 4. Target Audience

- **Primary — Rust developers learning backend/IoT architecture.** People who
  want to see clean architecture applied to a real-time, data-intensive domain,
  with swappable persistence and transport.
- **Secondary — IoT students and makers.** Learners who need a backend for
  their ESP32/Arduino projects without building one from scratch; they read the
  ingestion contract, point their device at it, and get a dashboard.
- **Tertiary — Contributors.** Rust engineers who want to extend a real
  platform (new transports, new storage, new device protocols) inside
  disciplined boundaries.

## 5. Long-Term Vision

Telemetry Hub evolves along a single axis, in phases:

**Phase A — Trustworthy foundation (today).** Postgres persistence, a complete
ingestion path (device registration → telemetry intake → storage → query), an
honest REST API, and a simulator that speaks the same contract as real hardware.

**Phase B — Real-time spine.** WebSocket streaming driven by the event model
(`DeviceConnected`, `DeviceDisconnected`, `TelemetryReceived`, `AlertRaised`).
The dashboard consumes the stream, not the database. Events become the backbone
that decouples ingestion from presentation.

**Phase C — Protocol independence.** The transport layer grows MQTT alongside
REST/WebSocket. A device's protocol is a deployment detail, not an identity:
the same `Device` and `Metric` domain objects serve HTTP simulators and MQTT
hardware unchanged.

**Phase D — Physical devices.** ESP32 and Arduino integrations join as
first-class `DeviceType`s. By this point, hardware integration is a small step:
the backend was built against the contract, and the contract never changed.

**Phase E — Platform maturity.** Alerting pipelines (severity computed from
telemetry, not carried by it), retention policies, and scaled storage
(timeseries-aware persistence behind the existing repository traits).

The vision is not "the biggest telemetry system." The vision is: **the most
instructive, most extensible telemetry system** — the one where adding a
device, a protocol, or a storage engine is an event that happens inside clean
boundaries, not a rewrite.

## 6. Engineering Philosophy

1. **Domain-first, contract-driven.** Everything the platform knows about the
   world — devices, metrics, events, alerts — lives in one place
   (`telemetry_core`) with no framework dependencies. Framework details (Axum,
   sqlx, Tokio) are *adapters*, never the identity of the system.
2. **Learn by building the real thing.** No toy tiers: the simulator talks to
   the same API and persists through the same repository traits as a future
   ESP32. If a path exists only for demo purposes, it must not exist.
3. **Extensibility through boundaries, not features.** The way to extend the
   platform is to implement a defined contract — a new repository, a new
   transport, a new device type — not to modify existing layers. This is why
   the persistence layer is a trait (`DeviceRepository`,
   `TelemetryRepository`) and services are generic over it.
4. **Honesty over aspiration.** The platform's docs describe what exists, and
   its roadmap describes what will exist. The two never intentionally diverge.
5. **Simple in behavior, rigorous in structure.** v1 keeps features minimal
   (no auth, no MQTT, no clustering) so that the architecture can be correct.
   Depth is added by layers, not by scope.

## 7. Architectural Principles (binding for every decision)

- **P1 — The device contract is universal.** Telemetry is exchanged as generic
  metrics (`key`, `value`, `unit`) plus a device identity (`code`). No domain
  type may hardcode a sensor schema (temperature, humidity, rpm...) into its
  core structure — that is device knowledge, which belongs to devices, not the
  platform. Devices may be *typed* (`SIMULATOR`, `ESP32`, `ARDUINO`), but the
  platform must never depend on a type's fields.
- **P2 — The domain crate has zero framework dependencies.** `telemetry_core`
  imports nothing from the web, database, or async worlds. Any violation
  breaks the dependency rule.
- **P3 — Persistence is a contract, Postgres is an implementation.** All
  storage access flows through repository traits; services depend on traits,
  never on pools. Replacing Postgres with another store must be a new crate
  implementing existing traits.
- **P4 — Transport is a crate, protocol is a detail.** HTTP today, WebSocket
  and MQTT tomorrow — each is an adapter living behind the transport boundary,
  speaking the same domain events.
- **P5 — Events decouple producers from consumers.** Ingestion produces events
  (`TelemetryReceived`, `AlertRaised`); consumers (dashboard, alerting,
  storage) subscribe. The event model is the seam where real-time streaming
  grows.
- **P6 — State is explicit.** Device status is derived from life-cycle events
  (`DeviceConnected`/`DeviceDisconnected`), not guessed. Severity is computed
  by alerting, not carried by input.
- **P7 — Every layer is testable alone.** A service is testable with a fake
  repository; a repository is testable against a real database; a handler is
  testable against in-memory state. Boundaries that do not permit this are
  misdrawn.
- **P8 — Workspace boundaries reflect the architecture.** `core` = domain;
  `transport` = protocols; `telemetry` = processing pipeline; `common` =
  shared utilities; `apps/*` = executable compositions. A crate exists because
  the architecture needs the boundary — empty placeholders are reintroduced
  only when a real consumer exists.

## 8. Extensibility Model

The platform extends along four independent axes, each with a defined seam:

| Axis | Seam | Example evolution |
|---|---|---|
| **Devices** | `DeviceType` enum + generic `Metric` contract | SIMULATOR → ESP32 → ARDUINO → community hardware |
| **Transports** | `transport` crate | REST → WebSocket → MQTT (each an adapter, domain unchanged) |
| **Persistence** | Repository traits + `Postgres*` impls | Postgres → Timescale/columnar store behind same traits |
| **Processing** | `telemetry` crate (validation → enrichment → alerting → storage) | v1 passthrough → v2 alert pipeline using `AlertSeverity` |

A contributor extends the platform by *implementing a contract*. New capability
goes in a new place, never as a change to existing layers. When a contribution
requires touching domain types to add device-specific fields, the contribution
has crossed the boundary and must be redesigned.

## 9. Scalability Direction

Honest position: v1 does not need distributed systems and will not pretend to
have them. The design targets **independent scalability of each layer**:

- **Ingestion scales horizontally by construction** — stateless HTTP handlers
  behind a load balancer; the pool is the only shared resource and is
  intentionally modest.
- **Storage scales vertically first** — the repository trait is the seam for
  timeseries-aware storage (e.g., TimescaleDB hypertables) when retention
  matters; the query surface (`find_latest`) is already shaped for
  recent-observation workloads.
- **Consumption scales by streaming, not polling** — the event model exists so
  that many WebSocket consumers fan out from a broadcast layer without
  hammering Postgres.
- **Simulators scale as load generators** — the same binary that produces one
  device's telemetry can produce thousands, making load testing part of the
  standard toolset.

The platform's scale story: **add devices → add stateless instances → add a
streaming layer → add timeseries storage**, in that order, each step behind a
boundary that already exists.

## 10. Non-Goals (explicit)

The following are deliberately out of scope and will be rejected if they arrive
early:

- **Authentication & authorization** (v1) — a single-tenant learning platform
  does not need it; premature auth complicates every other boundary.
- **MQTT** (v1) — protocol independence is structural; MQTT ships when the
  transport boundary has a second consumer to justify it.
- **Physical devices** (v1) — the platform's founding claim is that they are
  not needed for development; ESP32/Arduino arrive after the contract is proven
  by the simulator.
- **Kubernetes / cloud deployment** (v1) — a single Docker Compose environment
  until the platform earns complexity.
- **Multi-tenancy, billing, commercial SaaS** — Telemetry Hub is an open
  platform and a teaching infrastructure, not a business.
- **High-availability clustering and sub-millisecond latency guarantees** —
  consequences of later phases, never v1 requirements.
- **Data analytics/BI** — the platform stores recent observations
  (`find_latest`), not a warehouse.

Every non-goal is *deliberate*: each is excluded because shipping it early
would degrade the architecture the project exists to demonstrate.

## 11. North Star

> **A device joins Telemetry Hub by emitting a metric — and nothing else.**

The product is successful when:

1. Someone with zero experience in IoT can run the simulator, see telemetry
   persist, and watch a live dashboard — in under five minutes.
2. An ESP32 owner can integrate hardware by reading the ingestion contract,
   without asking the maintainers anything about the backend's internals.
3. A contributor can add a new storage engine or transport in an afternoon,
   touching only the adapter — never the domain or services.
4. The architecture itself remains the project's best documentation.

## 12. How This Document Governs Future Work

- **Docs must match code.** README, architecture docs, and CHANGELOG describe
  what exists; the roadmap describes what will exist. Contradiction between the
  two is a release-blocking defect.
- **PRs are judged against the axes, not the diff size.** A small PR that
  violates the device contract, the framework-free domain rule, or a trait
  boundary is worse than a large PR that honors them.
- **New boundaries beat new flags.** When a feature needs a mode/flag to
  coexist with the architecture, it is a sign the boundary is misdrawn.
- **The simulator stays first-class.** Any change that makes the simulator a
  second-class citizen (special-cased APIs, demo-only paths) violates the
  founding stance.
