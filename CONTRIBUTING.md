# Contributing to Telemetry Hub

## Start here

```bash
git clone https://github.com/Adibayuluthfiansyah/telemetry-hub
cd telemetry-hub && cp .env.example .env
./scripts/dev.sh    # PostgreSQL + API + simulator, one command
```

Then pick a
[`good first issue`](https://github.com/Adibayuluthfiansyah/telemetry-hub/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22),
say "I'd like to take this", and we'll figure it out together. A typo fix, a
test, or a doc improvement is a complete contribution.

Telemetry Hub is a learning-oriented, production-shaped telemetry platform.
Its most important output is its **architecture**: the boundaries, contracts,
and seams that let a device, a transport, or a storage engine join the platform
without a rewrite. When reviewing contributions, we care most about whether a
change respects the existing seams (domain vs. adapter, repository traits,
etc.) — code quality and test coverage matter too, but architectural fit is
what we'll usually discuss first in review. This isn't a bar you need to clear
alone: if you're unsure whether your approach fits, open a draft PR or an
issue early and we'll figure it out together.

Read [`docs/PRODUCT_VISION.md`](docs/PRODUCT_VISION.md) when you're ready for a
substantial design change — it is the decision filter for everything that gets
merged.
[`docs/PROJECT_ANALYSIS.md`](docs/PROJECT_ANALYSIS.md) documents the state of
the codebase at release-prep time and is the map of known debt.

New to the codebase? Small PRs are genuinely welcome — fixing a typo, adding a
test, improving an error message. You don't need to understand the full
architecture before your first contribution; reviewing existing code in
`crates/core` and `apps/server` is a good way to get oriented, and asking
questions in an issue before you start is always fine.

## Ground rules

Every PR must pass, before opening:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Additionally: documentation in the PR must match what the code does. The
project's core honesty principle — **docs never describe features that do not
exist** — is enforced at review.

## Local setup

Prerequisites: Docker, Rust 1.85+.

```bash
# 1. PostgreSQL 17 on host port 5439
docker compose -f docker/docker-compose.yml up -d

# 2. Environment
cp .env.example .env
#    set DATABASE_URL=postgres://postgres:postgres@localhost:5439/telemetry
#    and POSTGRES_DB/USER/PASSWORD to match

# 3. Run
cargo run -p server       # applies migrations at startup
cargo run -p simulator    # emits telemetry on SIMULATOR_INTERVAL_MS
```

Migrations apply automatically via `sqlx::migrate!`. For schema changes, add a
new numbered file under `migrations/` — **never edit an applied migration**.

## Workspace

| Path | Crate | What lives here |
|---|---|---|
| `crates/core/src/` | `telemetry_core` | Domain entities, enums, `Display`/`TryFrom` conversions. No framework imports, ever |
| `apps/server/src/config/` | `server` | Env-based configuration |
| `apps/server/src/state/` | `server` | `AppState`, injected into the router |
| `apps/server/src/handlers/` | `server` | HTTP layer, DTO marshalling |
| `apps/server/src/services/` | `server` | Generic services over repository traits |
| `apps/server/src/repositories/` | `server` | Traits; `postgres/` holds SQLx implementations and record mapping |
| `apps/server/src/dto/` | `server` | Request/response contracts |
| `apps/simulator/src/` | `simulator` | Virtual device |

> A `crates/telemetry` (processing pipeline) or `crates/transport` (protocol
> adapters) crate will be re-created when a real consumer lands (see
> ROADMAP M3); such future boundaries are not pre-allocated.

## Coding conventions

- **Edition 2024.** Workspace resolver 2.
- **Formatting is rustfmt's decision.** `cargo fmt --all` before committing;
  no local rustfmt overrides unless discussed.
- **Clippy must be clean with `-D warnings`.** A new warning in a PR is a
  review blocker.
- **Parameterized SQL only.** Every query uses SQLx bind parameters. String
  interpolation into SQL is rejected at review.
- **No panics on data conversion.** DB→domain mapping returns `Result`; a
  malformed row must fail gracefully, not crash a request thread. Do not
  introduce new `.expect()`/`.unwrap()` in library paths.
- **Domain types carry behavior, not frameworks.** New domain logic belongs in
  `telemetry_core` with only `chrono`/`uuid`-class dependencies.

## Tests

- `cargo test --workspace` must pass.
- New domain logic: unit tests alongside the code.
- New conversions (`From`, `TryFrom`, `Display`): test both directions and the
  error path.
- Repository work: test against a real PostgreSQL instance where feasible —
  the compose stack in `docker/` is the reference environment.
- Services: prefer fake repositories over mocks — services are generic over
  traits precisely so they can be tested alone (vision principle P7).

## SQLx migrations

- Files live in `migrations/`, named `<timestamp>_<description>.sql`.
- Append-only: once applied anywhere, a migration is immutable.
- `sqlx::migrate!("../../migrations")` runs at server startup — no separate
  step, but a working `DATABASE_URL` is required.

## Commit convention

- **Format:** imperative present tense, lowercase scope after a colon.
  - `feat: add device registration endpoint`
  - `fix: honor APP_HOST in server bind address`
  - `refactor: return Result from record mapping`
  - `docs: document repository contract`
  - `chore: bump sqlx to 0.9`
- Reference the issue when one exists: `fix: #42 ...`.
- One logical change per commit; one logical change per branch.

## Branch naming

Branch from `main`:

| Prefix | Use |
|---|---|
| `feat/` | New capability behind a seam |
| `fix/` | Defect correction |
| `refactor/` | Boundary or structure change without behavior change |
| `docs/` | Documentation only |
| `chore/` | Tooling, dependencies, maintenance |

## Pull request workflow

1. Open a **draft** PR early; mark it ready when the ground-rules gate passes.
2. Fill the PR description (template in `.github/PULL_REQUEST_TEMPLATE.md`):
   **What** (problem), **Why** (evidence: issue, log, trace), **How**
   (approach, trade-offs, which seam it touches), **Checklist**.
3. Small PRs are preferred — a small PR that adds business logic directly in a
   handler will likely need a round of revision to move that logic behind the
   right trait. That's a normal part of review, not a rejection. A larger PR
   that already follows the existing seams tends to move through review faster.
4. Review response is typically a few days. Expect questions about boundary
   placement and contract design; that is the review standard, not a blocker.

## Architecture review checklist

Maintainers review every PR against this list, in order of importance:

- [ ] **Boundary discipline** — the change stays inside its layer; no domain
      logic leaking into handlers, no SQL in services, no framework imports in
      `telemetry_core`.
- [ ] **Contract integrity** — metrics stay generic (`key`/`value`/`unit`);
      no device-specific fields hardcoded into domain types; new storage or
      transport work goes through existing traits/seams.
- [ ] **Event seam** — ingestion produces events; consumers subscribe. No new
      polling of the database where the event model is intended (P5).
- [ ] **State is derived** — status comes from lifecycle events; severity is
      computed, not carried by input (P6).
- [ ] **Honesty** — README/docs updated in the same PR; nothing documented
      that does not exist.
- [ ] **Safety** — parameterized SQL; `Result` over panics; no secrets, no
      `.env` committed.
- [ ] **Tests** — unit tests for domain logic; repository tests against real
      Postgres where feasible; services tested with fake repositories.

## Documentation policy

- Public behavior changes update the README in the same PR.
- Architecture changes update `docs/software-architecture.md` in the same PR.
- User-visible changes add a `CHANGELOG.md` entry (Keep a Changelog format).
- Product-intent changes update `docs/PRODUCT_VISION.md` — rarely, and only
  with maintainer discussion.

## Code of conduct

All interactions — issues, PRs, reviews, discussions — are governed by the
[Code of Conduct](CODE_OF_CONDUCT.md). By participating you agree to uphold
it. Violations can be reported privately to
adibayuluthfiansyah@gmail.com or via GitHub's report feature.

## License

By contributing you agree that your contributions are licensed under the
[MIT License](LICENSE), copyright © 2026 Adibayu Luthfiansyah.
