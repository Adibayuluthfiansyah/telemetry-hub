"use client";

import { cn } from "@/lib/utils";

export type ConnectionStatus = "connecting" | "live" | "offline" | "reconnecting" | "error" | "warning";

interface StatusIndicatorProps {
  status: ConnectionStatus;
  showLabel?: boolean;
  label?: string;
  size?: "sm" | "md" | "lg";
  className?: string;
}

const statusConfig: Record<ConnectionStatus, { color: string; label: string; pulse?: boolean }> = {
  connecting: { color: "bg-secondary", label: "CONNECTING", pulse: true },
  live: { color: "bg-primary", label: "LIVE" },
  offline: { color: "bg-destructive", label: "OFFLINE" },
  reconnecting: { color: "bg-secondary", label: "RECONNECTING", pulse: true },
  error: { color: "bg-destructive", label: "ERROR", pulse: true },
  warning: { color: "bg-secondary", label: "WARNING", pulse: true },
};

const sizeClasses = {
  sm: "w-1.5 h-1.5",
  md: "w-2 h-2",
  lg: "w-3 h-3",
};

export function StatusIndicator({
  status,
  showLabel = false,
  label,
  size = "md",
  className,
}: StatusIndicatorProps) {
  const config = statusConfig[status];
  const displayLabel = label ?? config.label;

  return (
    <span className={cn("inline-flex items-center gap-1.5", className)}>
      <span
        className={cn(
          "status-dot rounded-full transition-colors",
          config.color,
          config.pulse && "animate-pulse",
          sizeClasses[size]
        )}
        aria-hidden="true"
      />
      {showLabel && (
        <span className={cn("font-label-caps text-label-caps", "text-on-surface-variant")}>
          {displayLabel}
        </span>
      )}
    </span>
  );
}