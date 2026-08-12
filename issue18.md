Issue #18 — docs: document database and simulator (label: documentation)
## Goal

Document the **actual** database and simulator, and synchronize ROADMAP with
repository reality after #13, #15, #16, #17. Nothing invented, nothing future.

## Scope 1 — `docs/database.md` (actual state only)

- PostgreSQL development setup (compose, host port 5439, credentials via `.env`).
- Connection config (`DATABASE_URL`, `TEST_DATABASE_URL`).
- Migrations: the 3 shipped files, how they run (sqlx migrate at server
  startup), the `_sqlx_migrations` table.
- `devices` table (columns, `code` unique), `telemetry` table (columns,
  `fk_telemetry_device` FK → `devices(id)`, index
  `idx_telemetry_device_recorded (device_id, recorded_at)`).
- Development vs test database usage (`telemetry` vs `telemetry_test`).
- Do **not** document unshipped schema or future features.

## Scope 2 — `docs/simulator.md` (actual behavior only)

- Role: the simulator is a first-class, real telemetry producer (vision).
- Startup/config/env (`SIMULATOR_*` variables and defaults).
- Auto device registration; 409 tolerance (already-registered).
- Interval loop and generated metrics (temperature/humidity ranges, battery
  behavior).
- Connection-failure handling (current behavior: logged error, interval
  continues — state it honestly, no retry/backoff claims).
- 404 → re-register behavior.
- Ctrl-C graceful shutdown.
- Run command and the flow simulator → `POST /api/v1/telemetry` → PostgreSQL
  → `GET /api/v1/telemetry` (example only, no invented claims).

## Scope 3 — ROADMAP synchronization (facts only)

- M0: mark "Delete or document the reserved placeholder crates policy" ✅
  (completed via #13). Mark "Resolve 23 pre-existing compiler warnings" ✅
  with a note that remaining `.expect()`/CI work lives in issue #4; keep
  `.expect()` and CI items ⏳ until #4 closes.
- M1: "DB-aware `/health`" → ✅ (after #15).
- M2: tracing → ✅ (after #16); dev tooling → ✅ (after #17); docs → ✅
  (this issue).
- M3+: unchanged; confirm the transport/alerting wording says the crate is
  re-created when a real consumer lands.
- Do **not** touch historical snapshots for consistency:
  `docs/PROJECT_ANALYSIS.md`, `docs/IMPLEMENTATION_PLAN.md`,
  `docs/OPEN_SOURCE_READINESS_AUDIT.md`.
- If README Quick start drifted after #15–#17 (e.g. `/health` response
  shape), align it in this same PR (docs must describe what exists).

## Implementation order

1. Inspect `migrations/` and the live schema (read-only).
2. Write `docs/database.md`.
3. Inspect `apps/simulator/src` and `.env.example`.
4. Write `docs/simulator.md`.
5. Update ROADMAP per Scope 3; align README if needed.
6. Gates; one final end-to-end verification; attest results in the PR.

## Acceptance criteria

- [ ] `docs/database.md` exists and describes only the shipped schema
- [ ] `docs/simulator.md` exists and describes only real behavior
- [ ] ROADMAP flipped per facts; nothing invented; snapshot docs untouched
- [ ] gates green

## Quality gates

`cargo fmt --all -- --check` · `cargo build --workspace` ·
`cargo test --workspace` · `cargo clippy --workspace --all-targets -- -D warnings` ·
`git diff --check`

## Estimate

M
