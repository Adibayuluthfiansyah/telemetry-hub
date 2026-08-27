# Deployment

> **Dev-mode only — no production target yet.** This document describes how
> to run the stack locally. Production deployment (containers, TLS, scaling)
> is intentionally out of scope until `v0.1.0` — see `ROADMAP.md` Non-goals.

## Prerequisites

| Tool | Version | Note |
|---|---|---|
| Docker + Compose v2 | `docker --version` ≥ 20 | Postgres 17 |
| Rust | 1.85+ (edition 2024) | `rustup` stable |
| Node.js | 20+ | `node --version` |
| `DATABASE_URL` | — | `postgres://user:pass@host:port/db` |

Copy env files:

```bash
cp .env.example .env            # server / compose
# edit .env: DATABASE_URL + POSTGRES_DB / POSTGRES_USER / POSTGRES_PASSWORD
```

Frontend env is optional (defaults to `ws://localhost:3000/api/v1/stream`):

```bash
# frontend/.env.local (optional)
NEXT_PUBLIC_WS_URL=ws://localhost:3000/api/v1/stream
```

## Environment

| Variable | Default | Description |
|---|---|---|
| `APP_NAME` | `telemetry-hub` | Service name in logs |
| `APP_HOST` | `0.0.0.0` | Bind address |
| `APP_PORT` | `3000` | Bind port |
| `APP_ENV` | `development` | Environment label |
| `DATABASE_URL` | — | Postgres connection string |
| `POSTGRES_DB` / `POSTGRES_USER` / `POSTGRES_PASSWORD` | — | Compose provisioning |
| `SIMULATOR_INTERVAL_MS` | `1000` | Emit interval |
| `SIMULATOR_SERVER_URL` | `http://localhost:3000` | Server URL for simulator |
| `SIMULATOR_DEVICE_CODE` | `SIMULATOR-001` | Device code |
| `SIMULATOR_DEVICE_NAME` | `Simulator Device` | Device name |
| `NEXT_PUBLIC_WS_URL` | `ws://localhost:3000/api/v1/stream` | Dashboard stream URL |

## Architecture (local ports)

| Service | Port | Health |
|---|---|---|
| PostgreSQL 17 | `5439` (host) → `5432` (container) | `pg_isready` |
| API (Axum) | `3000` | `GET /api/v1/health` |
| Dashboard (Next.js) | `3001` | `GET /` |
| Simulator | — | emits to API |

`docker/docker-compose.yml` exposes only Postgres. API and dashboard run as
local processes.

## One-command stack

```bash
./scripts/dev.sh
# → Postgres (5439) → API (3000) → Dashboard (3001) → Simulator
# Ctrl-C tears down API, dashboard, and simulator; Postgres stays up (compose)
```

`dev.sh` waits for real readiness (`pg_isready` → `curl /api/v1/health` →
`curl http://localhost:3001`).

## Manual run (three terminals)

```bash
# Terminal 1: database + API
docker compose -f docker/docker-compose.yml up -d postgres
cargo run -p server          # migrations apply automatically at startup

# Terminal 2: dashboard
cd frontend && npm ci && npm run dev   # http://localhost:3001

# Terminal 3: simulator
cargo run -p simulator
```

## Production

Production deployment (image registry, TLS, scaling, secrets management) is
out of scope for `v0.1.0`. The current stack is intended for local
development and demonstration. When a production target lands, this document
will be expanded and `ROADMAP.md` M4 will be updated in the same PR.

## References

- `README.md` Quick start
- `CONTRIBUTING.md` (ground rules)
- `ROADMAP.md` M4
- `docs/api.md` + `docs/openapi.yaml`
