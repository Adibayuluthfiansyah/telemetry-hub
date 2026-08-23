"use client";

import { useTelemetryStream } from "@/hooks/use-telemetry-stream";
import { DashboardLayout } from "@/components/layout/DashboardLayout";
import { KPICard } from "@/components/dashboard/KPICard";
import { EventFeed } from "@/components/dashboard/EventFeed";
import { TelemetryPanel } from "@/components/dashboard/TelemetryPanel";
import { MetricStat } from "@/hooks/use-telemetry-stream";
import { ConnectionState } from "@/hooks/use-telemetry-stream";

export default function Page() {
  const { connection, events, stats, devices } = useTelemetryStream();

  const latestMetrics = Object.entries(stats) as [string, MetricStat][];
  const totalEvents = events.length;
  const activeDevices = devices.length;
  const bufferUsed = Math.min(events.length, 50);

  const getStreamStatus = (conn: ConnectionState): { label: string; status: "live" | "warning" | "error" } => {
    switch (conn) {
      case "live":
        return { label: "LIVE", status: "live" };
      case "connecting":
        return { label: "CONNECTING", status: "warning" };
      case "offline":
        return { label: "OFFLINE", status: "error" };
      default:
        return { label: String(conn).toUpperCase(), status: "warning" };
    }
  };

  const streamStatus = getStreamStatus(connection);

  return (
    <DashboardLayout
      connection={connection}
      eventCount={totalEvents}
      deviceCount={activeDevices}
      bufferUsage={{ used: bufferUsed, total: 50 }}
    >
      <div className="flex flex-col gap-[4px] px-[4px] py-2">
        <h1 className="font-headline-lg text-headline-lg text-on-surface uppercase tracking-wider">
          LIVE TELEMETRY OPERATIONS
        </h1>
        <p className="font-data-mono text-data-mono text-on-surface-variant">
          Real-time device telemetry stream
        </p>
      </div>

      <div className="grid grid-cols-3 gap-[1px] bg-border border-border">
        <KPICard label="ACTIVE DEVICES" value={activeDevices} status={streamStatus.status} />
        <KPICard label="EVENTS RECEIVED" value={totalEvents} status="live" />
        <KPICard label="STREAM STATUS" value={streamStatus.label} status={streamStatus.status} />
      </div>

      <div className="flex-1 grid grid-cols-1 lg:grid-cols-10 gap-[1px] bg-border border-border min-h-0">
        <div className="lg:col-span-7 min-h-0">
          <EventFeed events={events} connection={connection} />
        </div>
        <div className="lg:col-span-3 min-h-0">
          <TelemetryPanel
            metrics={latestMetrics}
            connection={connection}
            targetDeviceId={devices[0]}
          />
        </div>
      </div>
    </DashboardLayout>
  );
}