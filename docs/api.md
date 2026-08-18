# Telemetry Hub API

HTTP + WebSocket reference for the `apps/server` crate.

- Base URL: `http://<APP_HOST>:<APP_PORT>` (env-driven, no defaults; see `.env.example` — development: `0.0.0.0:3000`)
- Content type: `application/json` (requests and responses)
- Auth: none (not yet implemented — do not document it as a feature)
- Versioning: prefix `/api/v1`. All endpoints live under this prefix

## Error format

Every non-2xx response is JSON:

```json
{
  "success": false,
  "message": "human readable reason"
}
```

| Status | Meaning |
|---|---|
| 400 | Invalid JSON payload or invalid query parameters |
| 404 | Resource not found (device code, device id) |
| 409 | Conflict (device code already exists) |
| 500 | Internal error (database failure, etc.) |
| 503 | Service unavailable (database health check timed out/failed) |

## Endpoints

### `GET /api/v1/health`

Readiness check: verifies the database answers `SELECT 1` within 2 seconds.

**200:**

```json
{
  "status": "ok",
  "service": "telemetry-hub",
  "database": "up"
}
```

**503:** standard error body.

### `POST /api/v1/devices`

Register a device. Emits a `DEVICE_CONNECTED` event on the WebSocket stream (201).

Request body:

```json
{
  "code": "sensor-01",
  "name": "Hall sensor",
  "device_type": "ESP32"
}
```

`device_type` is one of `SIMULATOR`, `ESP32`, `ARDUINO`.

**201 Created:**

```json
{
  "id": "7c9e6679-7425-40de-944b-e07fc1f90ae7",
  "code": "sensor-01",
  "name": "Hall sensor",
  "status": "ONLINE",
  "device_type": "ESP32"
}
```

**400 / 409 / 500:** standard error body. 409 when the code is already registered.

### `GET /api/v1/devices/{code}`

Fetch a device by its unique code.

**200:**

```json
{
  "id": "7c9e6679-7425-40de-944b-e07fc1f90ae7",
  "code": "sensor-01",
  "name": "Hall sensor",
  "status": "ONLINE",
  "device_type": "ESP32"
}
```

**404:** device not found.

### `GET /api/v1/telemetry`

Query stored telemetry for a device.

Query parameters:

| Param | Type | Required | Notes |
|---|---|---|---|
| `device_id` | UUID | yes | Target device |
| `limit` | integer | no | Default `100`; silently clamped to `1..=1000` |

**200:**

```json
{
  "device_id": "7c9e6679-7425-40de-944b-e07fc1f90ae7",
  "count": 1,
  "samples": [
    {
      "key": "cpu",
      "value": 42.5,
      "unit": "percent",
      "recorded_at": "2026-08-18T14:30:00Z"
    }
  ]
}
```

**400** (missing/invalid `device_id`), **404** (device not found).

### `POST /api/v1/telemetry`

Insert a telemetry sample for a device (by code). Emits a `TELEMETRY_RECEIVED`
event on the WebSocket stream (201).

Request body:

```json
{
  "device_code": "sensor-01",
  "metrics": [
    {
      "key": "cpu",
      "value": 42.5,
      "unit": "percent"
    }
  ]
}
```

**201 Created:**

```json
{
  "success": true,
  "message": "Telemetry created successfully"
}
```

**400** (empty `metrics` — `"Metrics cannot be empty"`, or invalid JSON),
**404** (unknown `device_code`), **500**.

## `GET /api/v1/stream` — WebSocket

Live event stream. This is a WebSocket endpoint, **not** an HTTP endpoint; it is
documented here (and referenced from `openapi.yaml` via an `x-websocket`
extension, since OpenAPI has no native WebSocket path type).

- URL: `ws://<APP_HOST>:<APP_PORT>/api/v1/stream`
- Query (optional): `?device_id=<uuid>` — only events for that device are sent
- Each connection subscribes independently; messages are UTF-8 JSON text frames

### Event envelope

```json
{
  "event_id": "8a2b9c1d-...",
  "event_type": "TELEMETRY_RECEIVED",
  "device_id": "7c9e6679-7425-40de-944b-e07fc1f90ae7",
  "created_at": "2026-08-18T14:30:00Z",
  "payload": null
}
```

| Field | Type | Notes |
|---|---|---|
| `event_id` | UUID | Unique event id |
| `event_type` | string | `TELEMETRY_RECEIVED`, `DEVICE_CONNECTED`, `DEVICE_DISCONNECTED`, `ALERT_RAISED` |
| `device_id` | UUID | Source device |
| `created_at` | RFC3339 UTC | When the event occurred |
| `payload` | object/null | Optional structured payload (unused today) |

### Emitted events (today)

| Event | Trigger | Timestamp |
|---|---|---|
| `DEVICE_CONNECTED` | Successful `POST /api/v1/devices` (201) | Registration time |
| `TELEMETRY_RECEIVED` | Successful `POST /api/v1/telemetry` (201) | Recording time |

### Honest limitations

- `DEVICE_DISCONNECTED` and `ALERT_RAISED` are part of the domain model but are
  **not emitted**: heartbeat/offline detection and the alerting pipeline do not
  exist yet. Do not write consumers that depend on them.
- **Live-only, no replay:** a connection receives events that occur after it
  connects. There is no snapshot or history on subscribe.
- **Best-effort delivery:** a client that falls behind the channel buffer is
  skipped (broadcast `Lagged`) — missing events are not re-sent. The stream is
  not a durable queue.

### Lifecycle

- The server closes the connection on shutdown or when the client sends a close
  frame.
- No ping/pong heartbeat is implemented; a silent, half-open connection is only
  noticed when a send fails.

## OpenAPI

Machine-readable spec: [`openapi.yaml`](openapi.yaml).
