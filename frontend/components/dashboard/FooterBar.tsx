"use client";

import { cn } from "@/lib/utils";
import { StatusIndicator } from "./StatusIndicator";
import { ConnectionState } from "@/hooks/use-telemetry-stream";

interface FooterBarProps {
  connection: ConnectionState;
  eventCount?: number;
  deviceCount?: number;
  bufferUsage?: { used: number; total: number };
  className?: string;
}

export function FooterBar({
  connection,
  eventCount = 0,
  deviceCount = 0,
  bufferUsage = { used: 0, total: 50 },
  className,
}: FooterBarProps) {
  const statusLabel = connection === "live" ? "STREAM LIVE" : "STREAM OFFLINE";
  const statusType = connection === "live" ? "live" : "offline";

  return (
    <footer className={cn(
      "shrink-0 w-full bg-surface-container-lowest border-t border-border flex justify-between items-center px-[16px] h-8",
      className
    )}>
      <div className="font-data-mono text-[10px] leading-3 text-primary flex items-center gap-2">
        <StatusIndicator status={statusType} size="sm" showLabel={false} />
        <span>{statusLabel}</span>
      </div>
      <div className="flex items-center gap-6 font-data-mono text-[10px] leading-3 text-on-surface-variant">
        <span>EVENTS {eventCount}</span>
        <span>DEVICES {deviceCount}</span>
        <span>BUFFER {bufferUsage.used}/{bufferUsage.total}</span>
      </div>
    </footer>
  );
}