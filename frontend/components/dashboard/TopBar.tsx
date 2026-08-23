"use client";

import { cn } from "@/lib/utils";
import { StatusIndicator } from "./StatusIndicator";
import { ConnectionState } from "@/hooks/use-telemetry-stream";

interface TopBarProps {
  connection: ConnectionState;
  className?: string;
}

export function TopBar({ connection, className }: TopBarProps) {
  const statusLabel = connection === "live" ? "STREAM LIVE" : connection.toUpperCase();
  const statusType = connection === "live" ? "live" : "connecting";

  return (
    <header className={cn(
      "bg-background border-b border-border flat no-shadows flex justify-between items-center w-full px-[16px] h-12 shrink-0 z-10",
      className
    )}>
      <div className="flex items-center gap-4">
        <div className="font-headline-md text-headline-md font-bold text-primary tracking-tight">
          TELEMETRY HUB
        </div>
      </div>

      <div className="flex items-center gap-4">
        <div className="font-data-mono text-[11px] text-primary flex items-center gap-2">
          <StatusIndicator status={statusType} size="sm" showLabel={false} />
          <span>{statusLabel}</span>
        </div>
      </div>
    </header>
  );
}