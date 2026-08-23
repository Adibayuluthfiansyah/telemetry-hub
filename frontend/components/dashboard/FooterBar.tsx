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
      "bg-surface-container-lowest border-t border-border flat no-shadows fixed bottom-0 left-0 w-full z-50 flex justify-between items-center px-[16px] h-8 pl-[64px] md:pl-[80px]",
      className
    )}>
      <div className="font-data-mono text-[10px] text-primary flex items-center gap-2">
        <StatusIndicator status={statusType} size="sm" showLabel={false} />
        <span>{statusLabel}</span>
      </div>
      <div className="flex items-center gap-6 font-data-mono text-[10px] text-on-surface-variant">
        <span>EVENTS {eventCount}</span>
        <span>DEVICES {deviceCount}</span>
        <span>BUFFER {bufferUsage.used}/{bufferUsage.total}</span>
      </div>
    </footer>
  );
}