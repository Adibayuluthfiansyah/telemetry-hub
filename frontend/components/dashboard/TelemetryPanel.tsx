"use client";

import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { MetricStat } from "@/hooks/use-telemetry-stream";
import { MetricReadout } from "./MetricReadout";
import { ConnectionState } from "@/hooks/use-telemetry-stream";
import { StatusIndicator } from "./StatusIndicator";
import { cn } from "@/lib/utils";

interface TelemetryPanelProps {
  metrics: [string, MetricStat][];
  connection: ConnectionState;
  targetDeviceId?: string;
  className?: string;
}

const metricLabelMap: Record<string, string> = {
  temperature: "TEMPERATURE",
  humidity: "HUMIDITY",
  battery: "BATTERY LEVEL",
};

const metricStatusColor: Record<string, "primary" | "secondary" | "default"> = {
  temperature: "primary",
  humidity: "secondary",
  battery: "primary",
};

const metricShowBar: Record<string, boolean> = {
  battery: true,
};

export function TelemetryPanel({ metrics, connection, targetDeviceId, className }: TelemetryPanelProps) {
  const isLoading = connection !== "live";
  const displayMetrics: [string, MetricStat][] = metrics.length > 0 ? metrics : [
    ["temperature", { value: 0, unit: "celsius", deviceId: "", updatedAt: "" }],
    ["humidity", { value: 0, unit: "percent", deviceId: "", updatedAt: "" }],
    ["battery", { value: 0, unit: "percent", deviceId: "", updatedAt: "" }],
  ];

  return (
    <Card className={cn("flex flex-col h-full min-h-0 border-border [--card-spacing:0px]", className)}>
      <CardHeader className="border-b border-border p-3 bg-surface-container shrink-0 flex items-center justify-between">
        <CardTitle className="font-label-caps text-[11px] leading-4 text-primary">LATEST TELEMETRY</CardTitle>
        <StatusIndicator status={connection === "live" ? "live" : "connecting"} size="sm" />
      </CardHeader>
      <CardContent className="flex-1 overflow-y-auto p-4 flex flex-col gap-4">
        {targetDeviceId ? (
          <div className="flex flex-col gap-1 border-b border-border pb-3">
            <div className="font-data-mono text-[10px] leading-3 text-on-surface-variant">TARGET ID</div>
            <div className="font-data-mono-lg text-[16px] leading-5 text-on-surface tracking-widest">
              {targetDeviceId}
            </div>
            <div className="flex items-center gap-2 mt-1">
              <StatusIndicator
                status={connection === "live" ? "live" : "connecting"}
                size="sm"
                showLabel
                label={connection === "live" ? "CONNECTED" : "CONNECTING"}
              />
            </div>
          </div>
        ) : null}
        <div className="flex flex-col gap-4">
          {displayMetrics.map(([metricKey, stat]) => {
            const label = metricLabelMap[metricKey] ?? metricKey.toUpperCase();
            const statusColor = metricStatusColor[metricKey] ?? "default";
            const showBar = metricShowBar[metricKey] ?? false;

            if (isLoading) {
              return (
                <Skeleton key={`skeleton-${metricKey}`} className="h-20 w-full bg-surface-container-low" />
              );
            }

            return (
              <MetricReadout
                key={metricKey}
                metricKey={metricKey}
                label={label}
                value={stat.value}
                unit={stat.unit}
                statusColor={statusColor}
                showBar={showBar}
              />
            );
          })}
        </div>
      </CardContent>
    </Card>
  );
}