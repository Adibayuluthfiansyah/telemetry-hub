"use client";

import { cn } from "@/lib/utils";
import { Card, CardContent } from "@/components/ui/card";
import { formatValue } from "@/lib/format";

interface MetricReadoutProps {
  metricKey: string;
  label: string;
  value: number;
  unit: string;
  statusColor?: "primary" | "secondary" | "default";
  showBar?: boolean;
  className?: string;
}

const statusColorMap = {
  primary: "bg-primary",
  secondary: "bg-secondary",
  default: "bg-on-surface",
} as const;

export function MetricReadout({
  metricKey,
  label,
  value,
  unit,
  statusColor = "default",
  showBar = false,
  className,
}: MetricReadoutProps) {
  const formattedValue = formatValue(value, unit, { showUnit: false });
  const unitLabel = unit === "celsius" ? "°C" : unit === "fahrenheit" ? "°F" : unit === "percent" ? "%" : unit;
  const barColor = statusColorMap[statusColor];

  return (
    <Card
      key={metricKey}
      className={cn(
        "bg-surface-container-lowest border-border p-3 flex flex-col gap-1",
        className
      )}
    >
      <CardContent className="flex flex-col gap-1 p-0">
        <div className="font-label-caps text-[9px] text-on-surface-variant uppercase tracking-wider">
          {label}
        </div>
        <div className="font-data-mono text-3xl font-light text-on-surface flex items-baseline gap-1">
          <span>{formattedValue}</span>
          <span className="text-sm text-on-surface-variant font-normal">{unitLabel}</span>
        </div>
        {showBar && (
          <div className="w-full h-1 bg-surface-variant mt-1">
            <div
              className={cn("h-full transition-all duration-300", barColor)}
              style={{ width: `${Math.min(Math.max(value, 0), 100)}%` }}
            />
          </div>
        )}
      </CardContent>
    </Card>
  );
}