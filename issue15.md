Issue #15 — feat: make health endpoint DB-aware (label: enhancement)
## Goal

`GET /api/v1/health` must report database readiness, not just liveness.

## Current state

- `apps/server/src/handlers/health.rs` returns a static
  `{"status":"ok","service":"telemetry-hub"}` — no DB connectivity check.
- `AppState` owns service instances built from a `PgPool`;
  `Config` loads `DATABASE_URL`; migrations auto-run at startup.

## Scope

- Health check performs a DB connectivity probe (e.g. `SELECT 1` via the
  existing pool, with a short timeout).
- Reachable → HTTP 200, body at least `{"status":"ok","database":"up"}`
  (keep the `service` field; README/curl examples reference it).
- Unavailable → HTTP 503 using the existing `AppError` contract
  (`{"success":false,"message":...}`).
- No changes to ingestion/query endpoints. No new dependencies.

## Implementation order

1. Audit `handlers/health.rs`, `state/app_state.rs`, `main.rs` (how the
   `PgPool` is created and owned).
2. Choose the smallest consistent probe. Note: `AppState` does not expose the
   pool directly — decide whether to keep the pool in `AppState` or probe via
   an existing service; state the trade-off in the PR.
3. Implement the handler (+ error mapping to 503).
4. Tests: healthy path using the existing `test_pool()` harness pattern.
   DB-failure path: only if deterministic without flakiness (e.g. unit-test
   the error mapper); otherwise document 503 manual verification in the PR.
5. Manual verify: server + Postgres up → `/health` 200; stop the Postgres
   container → 503; restart → 200.

## Out of scope

Issue #4 (`.expect()` cleanup, CI gate), tracing (#16), dev.sh (#17), docs
(#18), any API behavior change, any migration.

## Acceptance criteria

- [ ] 200 + `database:"up"` when reachable
- [ ] 503 + error contract when database is down
- [ ] automated healthy-path test (failure-path test if deterministic)
- [ ] all quality gates green
- [ ] manual verification (200 / 503 / 200) documented in the PR

## Quality gates

`cargo fmt --all -- --check` · `cargo build --workspace` ·
`cargo test --workspace` · `cargo clippy --workspace --all-targets -- -D warnings` ·
`git diff --check`

## Estimate

S
