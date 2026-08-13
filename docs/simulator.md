# Simulator

The simulator is the platform's **first device** — not a demo. Per the vision,
anything that emits `{key, value, unit}` metrics is a first-class citizen, and
the simulator exercises the exact same contract a future ESP32 or Arduino
will use. See `apps/simulator/src` for the implementation; this document
describes its behavior **as it exists today**.

## Running

```bash
# from the workspace root, with the server already running:
cargo run -p simulator

# or: the whole stack with one command
./scripts/dev.sh
```

The simulator talks to the server at `SIMULATOR_SERVER_URL`. On `Ctrl-C` it
prints `Shutting down simulator` and exits (graceful shutdown via
`tokio::signal::ctrl_c`).

## Configuration

Environment variables, read from `.env` via `dotenvy` (`apps/simulator/src/config/env.rs`):

| Variable | Default | Effect |
|---|---|---|
| `SIMULATOR_INTERVAL_MS` | `1000` | Emit interval (one telemetry batch per tick) |
| `SIMULATOR_SERVER_URL` | `http://localhost:3000` | Base URL of the API server |
| `SIMULATOR_DEVICE_CODE` | `SIMULATOR-001` | Device code used for registration |
| `SIMULATOR_DEVICE_NAME` | `Simulator Device` | Device name used for registration |

## Startup sequence

1. Load config (env with defaults above).
2. Register the device: `POST /api/v1/devices` with
   `{"code", "name", "device_type": "SIMULATOR"}`.
   - `201 Created` → ready to send.
   - `409 Conflict` → the device already exists; **accepted**, sending starts
     anyway. Re-running the simulator is always safe.
3. Send one telemetry batch immediately, then keep sending on every interval
   tick.

## Generated metrics

Each tick produces three metrics in one `POST /api/v1/telemetry` payload:

```json
{
  "device_code": "SIMULATOR-001",
  "metrics": [
    { "key": "temperature", "value": 27.4,  "unit": "celsius" },
    { "key": "humidity",    "value": 61.2,  "unit": "percent" },
    { "key": "battery",     "value": 99.98, "unit": "percent" }
  ]
}
```

Behavior of the generator (`apps/simulator/src/generator/mod.rs`):

| Metric | Range | Behavior |
|---|---|---|
| `temperature` (celsius) | 20.0–35.0 | Sinusoidal wave — phase advances by 0.1 per tick |
| `humidity` (percent) | 40.0–80.0 | Same sinusoidal phase |
| `battery` (percent) | 100 → 20 | Drains 0.01 per tick, **resets to 100 when it drops below 20** |

## Behavior on failures

Honest current state — **there is no retry/backoff**. On a failed send the
simulator logs the error and the interval loop simply continues:

| Failure | Behavior |
|---|---|
| `404` from `POST /api/v1/telemetry` (device unknown) | Re-registers the device once, then resumes sending; a failed re-register is logged |
| Network/connection error | Logged to stderr (`Connection error: ...`), loop continues |
| Any other non-201 server status | Logged to stderr (`Server returned error: <status>`), loop continues |

## Data flow

```
simulator ──POST /api/v1/telemetry──▶ server ──INSERT──▶ PostgreSQL
                                                          │
    ──GET /api/v1/telemetry?device_id=…&limit=5───────────┘
```

Read telemetry back with the server's query endpoint:

```bash
curl "http://127.0.0.1:3000/api/v1/telemetry?device_id=<DEVICE_ID>&limit=5"
```

(newest-first, `limit` clamped to 1–1000). The device id comes from
`GET /api/v1/devices/SIMULATOR-001`.