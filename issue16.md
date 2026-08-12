Issue #16 — feat: add structured request tracing (label: enhancement)
## Goal

Operational request logs showing method, path, status, and duration — built on
the existing `tower-http` `TraceLayer`, no new dependencies.

## Current state

- `apps/server/src/main.rs` already applies `TraceLayer::new_for_http()` with
  `tracing_subscriber` fmt + `EnvFilter("server=debug,tower_http=debug")`.
- Logs exist but use default span behavior (limited visibility into
  per-request method/path/status/duration).
- `main.rs` also prints startup info via `println!`.

## Scope

- Keep `TraceLayer`/`tower-http` as-is.
- Add useful structured fields per request: HTTP method, URI/path (the
  matched route if available), response status, request duration/latency.
- Convert server-side `println!` operational output to `tracing` where it
  adds observability (startup banner may stay as a banner or move to tracing).
- Output stays readable in development (text fmt; `EnvFilter` already env-driven).
- No new dependencies, no router/handler changes.

## Implementation order

1. Audit tracing setup in `main.rs`, `app.rs`, `router.rs`.
2. Inspect the current default `TraceLayer` span output.
3. Add custom span/response hooks (`make_span_with` / `on_response`) emitting
   method, route/path, status, latency.
4. `cargo build` + `cargo clippy -D warnings`.
5. Run the server, hit every endpoint, and verify log lines show
   method/path/status/duration — paste real output in the PR.

## Out of scope

Simulator logging, JSON/structured log output formats, metrics collection,
issue #4, #15, #17, #18.

## Acceptance criteria

- [ ] log lines show method, path (or route), status, and duration per request
- [ ] `TraceLayer` retained; no new dependencies
- [ ] gates green
- [ ] real log output from a running server attached to the PR

## Quality gates

`cargo fmt --all -- --check` · `cargo build --workspace` ·
`cargo test --workspace` · `cargo clippy --workspace --all-targets -- -D warnings` ·
`git diff --check`

## Estimate

S
