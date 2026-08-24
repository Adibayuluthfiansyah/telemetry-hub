"use client";

export type TelemetryUnit =
  | "celsius"
  | "fahrenheit"
  | "percent"
  | "volts"
  | "amperes"
  | "hertz"
  | "bytes"
  | "pascal"
  | "meter"
  | "second"
  | string;

export interface FormatValueOptions {
  decimals?: number;
  showUnit?: boolean;
  locale?: string;
}

export function formatValue(
  value: number,
  unit: TelemetryUnit = "",
  options: FormatValueOptions = {},
): string {
  const { decimals, showUnit = true, locale = "en-US" } = options;

  if (!Number.isFinite(value)) return "-";

  const unitLower = unit.toLowerCase();
  const resolvedDecimals = decimals ?? resolveDecimals(unitLower);
  const formatted = new Intl.NumberFormat(locale, {
    minimumFractionDigits: resolvedDecimals,
    maximumFractionDigits: resolvedDecimals,
  }).format(value);

  if (!showUnit) return formatted;

  const label = formatUnitLabel(unitLower);
  return label ? `${formatted} ${label}` : formatted;
}

function resolveDecimals(unit: string): number {
  switch (unit) {
    case "percent":
      return 0;
    case "celsius":
    case "fahrenheit":
      return 1;
    case "volts":
    case "amperes":
      return 2;
    case "hertz":
      return 0;
    case "bytes":
      return 1;
    default:
      return 1;
  }
}

export function formatUnitLabel(unit: TelemetryUnit): string {
  const unitLower = unit.toLowerCase();
  const labels: Record<string, string> = {
    celsius: "°C",
    fahrenheit: "°F",
    percent: "%",
    volts: "V",
    amperes: "A",
    hertz: "Hz",
    bytes: "B",
    pascal: "Pa",
    meter: "m",
    second: "s",
  };
  return labels[unitLower] ?? unit;
}

export function formatRelativeTime(
  isoString: string,
  options: { locale?: string; maxAgeDays?: number } = {},
): string {
  const { locale = "en-US", maxAgeDays = 30 } = options;

  const date = new Date(isoString);
  if (isNaN(date.getTime())) return "—";

  const now = Date.now();
  const diffMs = now - date.getTime();
  const diffSec = Math.floor(diffMs / 1000);
  const diffMin = Math.floor(diffSec / 60);
  const diffHour = Math.floor(diffMin / 60);
  const diffDay = Math.floor(diffHour / 24);

  if (diffDay >= maxAgeDays) {
    return date.toLocaleDateString(locale, {
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  const rtf = new Intl.RelativeTimeFormat(locale, { numeric: "auto" });
  if (diffSec < 30) return rtf.format(-diffSec, "second");
  if (diffMin < 60) return rtf.format(-diffMin, "minute");
  if (diffHour < 24) return rtf.format(-diffHour, "hour");
  return rtf.format(-diffDay, "day");
}

export function formatAbsoluteTime(
  isoString: string,
  options: { locale?: string; includeSeconds?: boolean } = {},
): string {
  const { locale = "en-US", includeSeconds = false } = options;
  const date = new Date(isoString);
  if (isNaN(date.getTime())) return "—";
  return date.toLocaleTimeString(locale, {
    hour: "2-digit",
    minute: "2-digit",
    second: includeSeconds ? "2-digit" : undefined,
  });
}

/** Format ISO timestamp to UTC mission-clock style: "00:12:43.102 UTC". */
export function formatUtcTimestamp(
  isoString: string,
  options: { showMs?: boolean } = {},
): string {
  const { showMs = true } = options;
  const date = new Date(isoString);
  if (isNaN(date.getTime())) return "—";

  const pad = (n: number, width = 2) => String(n).padStart(width, "0");
  const hh = pad(date.getUTCHours());
  const mm = pad(date.getUTCMinutes());
  const ss = pad(date.getUTCSeconds());
  const ms = showMs ? `.${pad(date.getUTCMilliseconds(), 3)}` : "";
  return `${hh}:${mm}:${ss}${ms} UTC`;
}

export function formatBytes(bytes: number, decimals = 1): string {
  if (!Number.isFinite(bytes) || bytes < 0) return "—";
  if (bytes === 0) return "0 B";

  const units = ["B", "KB", "MB", "GB", "TB", "PB"];
  const base = 1024;
  const exp = Math.floor(Math.log(bytes) / Math.log(base));
  const value = bytes / Math.pow(base, exp);
  return `${value.toFixed(decimals)} ${units[exp]}`;
}

export function formatDuration(ms: number): string {
  if (!Number.isFinite(ms) || ms < 0) return "—";
  if (ms < 1000) return `${Math.round(ms)}ms`;
  const sec = Math.floor(ms / 1000);
  if (sec < 60) return `${sec}s`;
  const min = Math.floor(sec / 60);
  if (min < 60) return `${min}m ${sec % 60}s`;
  const hour = Math.floor(min / 60);
  return `${hour}h ${min % 60}m`;
}
