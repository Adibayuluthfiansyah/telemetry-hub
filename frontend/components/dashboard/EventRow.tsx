"use client";

import { cn } from "@/lib/utils";
import { EventEnvelope } from "@/lib/types";
import { formatUtcTimestamp } from "@/lib/format";
import { StatusIndicator } from "./StatusIndicator";

interface EventRowProps {
  event: EventEnvelope;
}

const eventTypeIcons: Record<string, string> = {
  TELEMETRY_RECEIVED: "sensors",
  DEVICE_CONNECTED: "radio_button_checked",
  DEVICE_DISCONNECTED: "radio_button_off",
  ALERT_RAISED: "warning",
};

const eventTypeLabels: Record<string, string> = {
  TELEMETRY_RECEIVED: "TELEMETRY RECEIVED",
  DEVICE_CONNECTED: "DEVICE CONNECTED",
  DEVICE_DISCONNECTED: "DEVICE DISCONNECTED",
  ALERT_RAISED: "ALERT RAISED",
};

const shortKeyMap: Record<string, string> = {
  temperature: "TEMP",
  humidity: "HUMID",
  battery: "BATT",
};

const unitSymbolMap: Record<string, string> = {
  celsius: "°C",
  fahrenheit: "°F",
  percent: "%",
};

function getEventVariant(event: EventEnvelope): {
  borderColor: string;
  textColor: string;
  status: "live" | "warning" | "error";
  iconColor: string;
} {
  if (event.event_type === "TELEMETRY_RECEIVED") {
    return { borderColor: "border-l-2 border-primary", textColor: "text-on-surface", status: "live", iconColor: "text-primary" };
  }
  if (event.event_type === "DEVICE_CONNECTED") {
    return { borderColor: "", textColor: "text-primary", status: "live", iconColor: "text-primary" };
  }
  if (event.event_type === "DEVICE_DISCONNECTED" || event.event_type === "ALERT_RAISED") {
    return { borderColor: "border-l-2 border-destructive", textColor: "text-destructive", status: "error", iconColor: "text-destructive" };
  }
  return { borderColor: "", textColor: "text-on-surface", status: "live", iconColor: "text-primary" };
}

function getPayloadPreview(event: EventEnvelope): React.ReactNode {
  if (event.event_type === "TELEMETRY_RECEIVED" && event.payload?.metrics) {
    return (
      <div className="flex flex-wrap gap-x-4 gap-y-1 mt-1 pl-6 font-data-mono text-[11px] leading-[18px]">
        {event.payload.metrics.map((metric, idx) => (
          <div key={idx} className="flex items-baseline gap-1">
            <span className="text-on-surface-variant">{shortKeyMap[metric.key] ?? metric.key.toUpperCase()}</span>
            <span className={cn("font-medium", metric.unit === "celsius" || metric.unit === "fahrenheit" ? "text-primary" : metric.unit === "percent" ? "text-secondary" : "text-on-surface")}>
              {typeof metric.value === "number" ? metric.value.toFixed(2) : metric.value}
              <span className="text-on-surface-variant text-[10px] ml-0.5">{unitSymbolMap[metric.unit] ?? metric.unit}</span>
            </span>
          </div>
        ))}
      </div>
    );
  }
  return null;
}

function getEventMessage(event: EventEnvelope): string {
  const payload = event.payload;
  if (event.event_type === "DEVICE_DISCONNECTED" && typeof payload === "object" && payload && "message" in payload) {
    return (payload as Record<string, unknown>).message as string;
  }
  if (event.event_type === "ALERT_RAISED" && typeof payload === "object" && payload && "message" in payload) {
    return (payload as Record<string, unknown>).message as string;
  }
  return "";
}

export function EventRow({ event }: EventRowProps) {
  const { borderColor, textColor, status, iconColor } = getEventVariant(event);
  const iconName = eventTypeIcons[event.event_type] || "terminal";
  const label = eventTypeLabels[event.event_type] || event.event_type;
  const message = getEventMessage(event);
  const timestamp = formatUtcTimestamp(event.created_at);

  return (
    <div className={cn(
      "bg-surface-container p-3 hover:bg-surface-container-low transition-colors group cursor-pointer flex flex-col gap-2",
      borderColor
    )}>
      <div className="flex items-center justify-between text-on-surface-variant">
        <span className="font-data-mono text-[11px] leading-[18px] whitespace-nowrap">{timestamp}</span>
        <span className="bg-surface-container-high px-1 font-data-mono text-[10px] leading-3 whitespace-nowrap">
          {event.device_id.slice(0, 8)}
        </span>
      </div>
      <div className={cn("flex items-center gap-2 font-medium", textColor)}>
        <span className={cn("material-symbols-outlined text-[14px]", iconColor)}>
          {iconName}
        </span>
        <StatusIndicator status={status} size="sm" showLabel={false} />
        <span className="font-label-caps text-[11px] leading-4">{label}</span>
      </div>
      {getPayloadPreview(event)}
      {message && (
        <div className={cn("pl-6 text-on-surface-variant opacity-80 font-data-mono text-[11px] leading-[18px]", textColor)}>
          {message}
        </div>
      )}
    </div>
  );
}
