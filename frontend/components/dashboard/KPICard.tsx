"use client";

import { cn } from "@/lib/utils";
import { Card, CardContent } from "@/components/ui/card";
import { StatusIndicator } from "./StatusIndicator";

interface KPICardProps {
  label: string;
  value: string | number;
  status?: "live" | "warning" | "error";
  icon?: React.ReactNode;
  className?: string;
}

const statusMap = {
  live: "live" as const,
  warning: "reconnecting" as const,
  error: "error" as const,
};

export function KPICard({ label, value, status = "live", icon, className }: KPICardProps) {
  const indicatorStatus = statusMap[status];

  return (
    <Card
      className={cn(
        "bg-surface-container hover:bg-surface-container-low transition-colors p-3 flex flex-col justify-between border-border [--card-spacing:0px]",
        className
      )}
    >
      <CardContent className="flex flex-col gap-2 p-0">
        <div className="font-label-caps text-[11px] leading-4 text-on-surface-variant">
          {label}
        </div>
        <div className="font-data-mono-lg text-[16px] leading-5 text-on-surface flex items-center gap-2">
          <StatusIndicator status={indicatorStatus} size="sm" />
          {icon}
          <span>{value}</span>
        </div>
      </CardContent>
    </Card>
  );
}