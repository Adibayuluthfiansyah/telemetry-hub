export interface EventEnvelope {
  event_id: string;
  device_id: string;
  created_at: string;
  payload: { metrics: TelemetryMetric[] } | null;
  event_type: EventType;
}

export type EventType =
  | "TELEMETRY_RECEIVED"
  | "DEVICE_CONNECTED"
  | "DEVICE_DISCONNECTED"
  | "ALERT_RAISED";

export interface TelemetryMetric {
  key: string;
  value: number;
  unit: string;
}

function isEventType(value: string): value is EventType {
  return (
    value === "TELEMETRY_RECEIVED" ||
    value === "DEVICE_CONNECTED" ||
    value === "DEVICE_DISCONNECTED" ||
    value === "ALERT_RAISED"
  );
}

export function parseEvent(value: unknown): EventEnvelope | null {
  if (typeof value !== "object" || value === null) return null;
  const raw = value as Record<string, unknown>;
  if (!("event_type" in raw)) return null;
  if (typeof raw.event_type !== "string") return null;
  if (typeof raw.event_id !== "string") return null;
  if (typeof raw.device_id !== "string") return null;
  if (typeof raw.created_at !== "string") return null;
  if (!isEventType(raw.event_type)) return null;
  if (raw.payload !== null && typeof raw.payload !== "object") return null;
  return {
    event_id: raw.event_id,
    device_id: raw.device_id,
    created_at: raw.created_at,
    event_type: raw.event_type,
    payload: raw.payload as { metrics: TelemetryMetric[] } | null,
  };
}
