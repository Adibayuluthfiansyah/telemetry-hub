# Security Policy

Telemetry Hub is a pre-release (v0.1.0) platform that ingests and stores device
telemetry. Security is treated as a first-class concern from the first public
release onward, even while the platform is not yet production-ready.

## Supported versions

| Version | Supported |
|---|---|
| `main` (unreleased) | ✅ Best-effort fixes |
| `0.x` releases | ✅ Critical fixes only |
| pre-`0.1.0` | ❌ No releases exist yet |

Until the first tagged release, security fixes land on `main` only. Backports
are not performed before a release line exists.

## Reporting a vulnerability

**Do not open a public GitHub issue for a vulnerability.** Use one of these
private channels:

1. **Email the maintainer:** `adibayuluthfiansyah@gmail.com` — prefix the
   subject with `[SECURITY]`.
2. **GitHub security advisory:** repository → **Security** → **Report a
   vulnerability** (if enabled on the repository).

Include:

- Affected component, version, or commit hash
- Minimal steps to reproduce
- Impact assessment
- A proposed fix, if you have one

Never include credentials, tokens, or live `.env` contents in a report;
redact them first.

## What happens next

| Step | Timeline |
|---|---|
| Acknowledgment | Within 72 hours of a valid report |
| Triage: severity assessment + reproduction | Within 7 days |
| Fix prepared, tested, and released | Per severity below |

Fix targets once triaged:

| Severity | Target |
|---|---|
| Critical | 7 days |
| High | 14 days |
| Medium | 30 days |
| Low | Next milestone |

We practice **coordinated disclosure**: the reporter is credited (with consent)
in the advisory and is given a heads-up before public disclosure.

## Scope

In scope:

- The server binary (`apps/server`) and its HTTP API
- The domain and persistence layers (`crates/core`, repository implementations)
- SQLx migrations and database interaction
- Docker Compose configuration in `docker/`

Out of scope:

- The simulator (`apps/simulator`) — a development tool that emits fake data
- Future components that do not exist yet (WebSocket streaming, dashboard,
  MQTT transport)

## Security best practices

Baseline practices all contributions must preserve:

- **Parameterized SQL only.** All queries use SQLx bind parameters; string
  interpolation into SQL is rejected at review.
- **No secrets in the repository.** `.env` and `.env.*.local` are
  git-ignored; reviews check for accidental secrets.
- **Pinned lockfile.** `Cargo.lock` is committed for the application
  workspace; dependency updates land in dedicated, reviewed PRs.
- **No default credentials beyond development.** `postgres/postgres` defaults
  are for local Docker Compose only; production deployments must use their own
  credentials and restrict network exposure.

## Expectations for users

- This project is **not production-ready**. Do not operate it against
  untrusted networks or real critical infrastructure before `v1.0.0`.
- Run the database on a private network; never expose it to the public
  internet.
- Review changes between releases; pre-`v1.0.0` APIs can change without
  notice.
