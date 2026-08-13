# Database

This document describes the database **as it exists today** — PostgreSQL 17,
managed by Docker Compose, migrated by SQLx at server startup. Nothing here
describes unshipped schema or future features.

## Development setup

PostgreSQL runs in a Docker container defined by
[`docker/docker-compose.yml`](../docker/docker-compose.yml):

- service `postgres`, container name `telemetry-postgres`
- host port **5439** → container port 5432
- data persisted in the `postgres-data` volume
- credentials and database names come from `.env` (the compose file reads
  `POSTGRES_DB`, `POSTGRES_USER`, `POSTGRES_PASSWORD`)

Start it with:

```bash
docker compose -f docker/docker-compose.yml up -d
```

The container is long-lived (`restart: unless-stopped`) — it is **not** torn
down when you stop the application stack.

## Connection configuration

Both connection strings live in `.env`, loaded via `dotenvy`:

| Variable | Used for | Example |
|---|---|---|
| `DATABASE_URL` | Server (runtime) | `postgres://postgres:postgres@localhost:5439/telemetry` |
| `TEST_DATABASE_URL` | Integration tests in `apps/server/tests` | `postgres://postgres:yourpassword@localhost:5439/telemetry_test` |

Development data goes to the `telemetry` database. The test suite connects
through `TEST_DATABASE_URL` (see `apps/server/tests/common.rs`) against a
separate `telemetry_test` database so tests never touch development data.

The server fails fast at startup if it cannot connect to `DATABASE_URL` or
cannot run migrations.

## Migrations

Migrations are SQLX files in [`migrations/`](../migrations/) at the repository
root. The server applies **any pending migrations at every startup** via
`sqlx::migrate!("../../migrations")` (`apps/server/src/database/migration.rs`,
invoked in `apps/server/src/main.rs`) — there is no manual migration step.

Shipped migrations:

| File | What it does |
|---|---|
| `20260804201459_init_schema.sql` | Creates `devices` |
| `20260806161032_add_device_type_to_devices.sql` | Adds `device_type` to `devices` |
| `20260809134820_create_telemetry.sql` | Creates `telemetry`, FK, and the `(device_id, recorded_at)` index |

SQLx records each applied migration in the `_sqlx_migrations` table. Verify:

```bash
psql "$DATABASE_URL" -c "SELECT version, description, success FROM _sqlx_migrations ORDER BY version;"
```

**Rules:** append new numbered files when the schema changes; never edit or
delete an applied migration.

## Schema

### `devices`

One row per registered device.

| Column | Type | Constraints / default |
|---|---|---|
| `id` | `uuid` | PRIMARY KEY |
| `code` | `varchar(50)` | NOT NULL, UNIQUE (`devices_code_key`) |
| `name` | `varchar(100)` | NOT NULL |
| `status` | `varchar(20)` | NOT NULL, default `'OFFLINE'` |
| `created_at` | `timestamptz` | NOT NULL, default `now()` |
| `updated_at` | `timestamptz` | NOT NULL, default `now()` |
| `device_type` | `varchar(50)` | NOT NULL, default `'SIMULATOR'` |

### `telemetry`

One row per telemetry sample. Referenced by `fk_telemetry_device`
→ `devices(id)`.

| Column | Type | Constraints / default |
|---|---|---|
| `id` | `uuid` | PRIMARY KEY |
| `device_id` | `uuid` | NOT NULL, FK → `devices(id)` |
| `key` | `varchar(100)` | NOT NULL |
| `value` | `double precision` | NOT NULL |
| `unit` | `varchar(50)` | NOT NULL |
| `recorded_at` | `timestamptz` | NOT NULL |

Indexes:

- `telemetry_pkey` — primary key on `id`
- `idx_telemetry_device_recorded` — `(device_id, recorded_at)`, backs the
  newest-first `GET /telemetry` query

## Quick verification

```bash
psql "$DATABASE_URL" -c "\d devices"
psql "$DATABASE_URL" -c "\d telemetry"
psql "$DATABASE_URL" -c "SELECT count(*) FROM telemetry;"
```

With the simulator running, the `telemetry` count grows by one row per metric
(three per second by default) — see [`simulator.md`](simulator.md).