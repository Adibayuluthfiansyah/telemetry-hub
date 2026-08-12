Issue #17 — chore: add one-command development stack (label: enhancement)
## Goal / M2 exit criterion

`./scripts/dev.sh` reproduces the whole development stack with **one command**
and cleans up cleanly on Ctrl-C. This issue is complete only when that
criterion is demonstrated end-to-end, not merely when the script exists.

## Current state

- `scripts/` exists but is empty.
- Stack today: `docker compose -f docker/docker-compose.yml up -d` (PostgreSQL
  on host port 5439) → `cargo run -p server` (migrations auto-apply) →
  `cargo run -p simulator`.

## Requirements for `scripts/dev.sh`

- Start PostgreSQL via the existing compose file.
- Wait for PostgreSQL readiness properly (e.g. `pg_isready` via `docker exec`,
  or a compose healthcheck) — **blind fixed sleeps are not acceptable as the
  only readiness mechanism**.
- Start the server; wait until `GET /api/v1/health` responds before starting
  the simulator.
- Start the simulator.
- Handle Ctrl-C/termination gracefully: forward signals to children, tear down
  server + simulator, leave **no orphan processes**; re-running the script
  must be idempotent.
- Use existing env/config (`.env`, `.env.example`); no new external
  dependency for the script beyond what the dev environment already has
  (bash, docker, cargo).
- Do not change server or simulator behavior to fit the script.

## Implementation order

1. Audit `docker/docker-compose.yml` (service name, ports, healthcheck, env).
2. Audit current startup commands and env requirements.
3. Design process lifecycle (background children + `trap`, or equivalent —
   no process-manager dependency unless unavoidable).
4. Implement `scripts/dev.sh` and make it executable.
5. Verify:
   - one command brings up DB + server + simulator;
   - telemetry actually arrives (check via `GET /api/v1/telemetry` or row
     count in PostgreSQL);
   - Ctrl-C tears down every process the script started (`pgrep` check);
   - running it a second time works identically.

## Out of scope

Server/simulator behavior changes, deployment scripts, CI, issues #4/#15/#16/#18.

## Acceptance criteria

- [ ] `./scripts/dev.sh` starts the whole stack with one command
- [ ] readiness is gated (Postgres ready → server ready → simulator), no blind sleep as sole mechanism
- [ ] Ctrl-C cleans up all child processes; no orphans
- [ ] re-run is idempotent
- [ ] telemetry provably written to PostgreSQL in the final verification
- [ ] gates green

## Quality gates

`cargo fmt --all -- --check` · `cargo build --workspace` ·
`cargo test --workspace` · `cargo clippy --workspace --all-targets -- -D warnings` ·
`git diff --check`

## Estimate

M
