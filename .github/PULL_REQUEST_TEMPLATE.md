<!--
Thank you for contributing to Telemetry Hub.

Before opening, the ground rules in CONTRIBUTING.md must pass:

  cargo fmt --all -- --check
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace

This template mirrors the architecture review checklist in CONTRIBUTING.md.
Fill it out honestly; the reviewer will verify.
-->

## What

<!-- The problem this PR solves, in one or two sentences. -->

## Why

<!-- Evidence: issue number, log output, reproduction, or measurement. -->

## How

<!-- The approach, and — critically — WHICH SEAM the change touches:

  - domain (crates/core)
  - transport (REST/WebSocket/MQTT adapter)
  - service (generic over repository traits)
  - repository (trait or Postgres implementation)
  - handler/DTO (HTTP layer)
  - documentation
-->

Seam: `...`

## Checklist

- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [ ] `cargo test --workspace` passes
- [ ] No framework imports in `crates/core`
- [ ] Metrics remain generic (`key`/`value`/`unit`); no device-specific fields
      hardcoded in domain types
- [ ] Storage/transport work goes through existing traits or seams
- [ ] Parameterized SQL only; no string interpolation
- [ ] DB→domain conversion returns `Result`; no new panics/`.expect()`
- [ ] Docs updated in this PR; nothing documented that does not exist
- [ ] CHANGELOG entry added if user-visible

<!--
Optional, for the reviewer:

- [ ] Unit tests added for new domain logic / conversions
- [ ] Repository behavior tested against real PostgreSQL
-->
