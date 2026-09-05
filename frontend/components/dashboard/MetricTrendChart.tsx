"use client";

import { useMemo } from "react";
import {
  CartesianGrid,
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { formatChartTime, formatValue } from "@/lib/format";
import { cn } from "@/lib/utils";
import type {
  ConnectionState,
  SeriesPoint,
} from "@/hooks/use-telemetry-stream";
import { StatusIndicator } from "./StatusIndicator";

interface MetricTrendChartProps {
  series: Record<string, SeriesPoint[]>;
  connection: ConnectionState;
  className?: string;
}

const CHART_KEYS = ["temperature", "humidity", "battery"] as const;
type ChartKey = (typeof CHART_KEYS)[number];

const CHART_COLORS: Record<ChartKey, string> = {
  temperature: "var(--chart-1)",
  humidity: "var(--chart-2)",
  battery: "var(--chart-3)",
};

const CHART_LABELS: Record<ChartKey, string> = {
  temperature: "TEMPERATURE",
  humidity: "HUMIDITY",
  battery: "BATTERY",
};

const CHART_UNITS: Record<ChartKey, string> = {
  temperature: "celsius",
  humidity: "percent",
  battery: "percent",
};

interface TrendRow {
  t: number;
  temperature?: number;
  humidity?: number;
  battery?: number;
}

interface TooltipEntry {
  dataKey?: string | number;
  value?: number | string;
  color?: string;
}

interface TrendTooltipProps {
  active?: boolean;
  payload?: TooltipEntry[];
  label?: number | string;
}

function TrendTooltip({ active, payload, label }: TrendTooltipProps) {
  if (!active || !payload || payload.length === 0) return null;
  const t = typeof label === "number" ? label : Number(label);
  return (
    <div className="bg-surface-container-lowest border border-border px-2 py-1.5 flex flex-col gap-1">
      <div className="font-data-mono text-[10px] leading-3 text-on-surface-variant">
        {formatChartTime(t)}
      </div>
      {payload.map((entry) => {
        const key = String(entry.dataKey ?? "");
        const unit =
          key === "temperature" || key === "humidity" || key === "battery"
            ? CHART_UNITS[key]
            : "";
        const value =
          typeof entry.value === "number"
            ? formatValue(entry.value, unit)
            : String(entry.value ?? "—");
        return (
          <div
            key={key}
            className="flex items-center gap-1.5 font-data-mono text-[11px] leading-4"
          >
            <span
              className="inline-block w-2 h-2 shrink-0"
              style={{ backgroundColor: entry.color }}
              aria-hidden="true"
            />
            <span className="text-on-surface-variant uppercase">{key}</span>
            <span className="text-on-surface ml-auto pl-3">{value}</span>
          </div>
        );
      })}
    </div>
  );
}

export function MetricTrendChart({
  series,
  connection,
  className,
}: MetricTrendChartProps) {
  const { rows, totalPoints } = useMemo(() => {
    const byTime = new Map<number, TrendRow>();
    let total = 0;
    for (const key of CHART_KEYS) {
      const pts = series[key] ?? [];
      total += pts.length;
      for (const p of pts) {
        let row = byTime.get(p.t);
        if (!row) {
          row = { t: p.t };
          byTime.set(p.t, row);
        }
        row[key] = p.value;
      }
    }
    const sorted = [...byTime.values()].sort((a, b) => a.t - b.t);
    return { rows: sorted, totalPoints: total };
  }, [series]);

  const latest = useMemo(() => {
    const out: Record<ChartKey, SeriesPoint | undefined> = {
      temperature: undefined,
      humidity: undefined,
      battery: undefined,
    };
    for (const key of CHART_KEYS) {
      const pts = series[key];
      out[key] = pts && pts.length > 0 ? pts[pts.length - 1] : undefined;
    }
    return out;
  }, [series]);

  const isLoading = connection !== "live" && totalPoints === 0;

  return (
    <Card
      className={cn(
        "flex flex-col min-h-0 border-border [--card-spacing:0px]",
        className,
      )}
    >
      <CardHeader className="border-b border-border p-3 bg-surface-container shrink-0 flex items-center justify-between">
        <CardTitle className="font-label-caps text-[11px] leading-4 text-primary">
          LIVE METRIC TREND
        </CardTitle>
        <StatusIndicator
          status={connection === "live" ? "live" : "connecting"}
          size="sm"
        />
      </CardHeader>
      <CardContent className="p-3 flex flex-col gap-2 min-h-0">
        <div className="flex items-center gap-4 flex-wrap">
          {CHART_KEYS.map((key) => {
            const point = latest[key];
            return (
              <div key={key} className="flex items-center gap-1.5">
                <span
                  className="inline-block w-2 h-2 shrink-0"
                  style={{ backgroundColor: CHART_COLORS[key] }}
                  aria-hidden="true"
                />
                <span className="font-label-caps text-[9px] leading-3 text-on-surface-variant">
                  {CHART_LABELS[key]}
                </span>
                <span className="font-data-mono text-[11px] leading-4 text-on-surface">
                  {point ? formatValue(point.value, CHART_UNITS[key]) : "—"}
                </span>
              </div>
            );
          })}
        </div>
        {isLoading ? (
          <Skeleton className="h-[220px] w-full bg-surface-container-low" />
        ) : totalPoints === 0 ? (
          <div className="h-[220px] w-full flex items-center justify-center border border-border bg-surface-container-lowest">
            <span className="font-label-caps text-[11px] leading-4 text-on-surface-variant">
              AWAITING FIRST SAMPLES
            </span>
          </div>
        ) : (
          <div className="h-[220px] w-full min-h-0">
            <ResponsiveContainer width="100%" height="100%">
              <LineChart
                data={rows}
                margin={{ top: 4, right: 8, bottom: 0, left: 0 }}
              >
                <CartesianGrid vertical={false} stroke="#283131" />
                <XAxis
                  type="number"
                  dataKey="t"
                  domain={["auto", "auto"]}
                  tickFormatter={(t: number) => formatChartTime(t)}
                  tick={{
                    fill: "#8D9996",
                    fontSize: 10,
                    fontFamily: "var(--font-mono)",
                  }}
                  axisLine={{ stroke: "#283131" }}
                  tickLine={{ stroke: "#283131" }}
                  minTickGap={48}
                />
                <YAxis
                  width={44}
                  domain={["auto", "auto"]}
                  tick={{
                    fill: "#8D9996",
                    fontSize: 10,
                    fontFamily: "var(--font-mono)",
                  }}
                  axisLine={{ stroke: "#283131" }}
                  tickLine={{ stroke: "#283131" }}
                />
                <Tooltip
                  content={<TrendTooltip />}
                  cursor={{ stroke: "#283131" }}
                />
                {CHART_KEYS.map((key) => (
                  <Line
                    key={key}
                    type="monotone"
                    dataKey={key}
                    stroke={CHART_COLORS[key]}
                    strokeWidth={1.5}
                    dot={false}
                    isAnimationActive={false}
                    connectNulls
                  />
                ))}
              </LineChart>
            </ResponsiveContainer>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
