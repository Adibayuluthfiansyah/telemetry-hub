"use client";

import { useState, useRef, useEffect, useCallback } from "react";
import type { EventEnvelope, TelemetryQueryResponse } from "@/lib/types";
import { parseEvent } from "@/lib/types";

export type ConnectionState = "connecting" | "live" | "offline";

export interface MetricStat {
  value: number;
  unit: string;
  deviceId: string;
  updatedAt: string;
}

export interface SeriesPoint {
  t: number;
  value: number;
}

export const SERIES_CAP = 120;
const REST_SEED_LIMIT = 360;

function resolveApiBase(wsUrl: string): string {
  const fromEnv = process.env.NEXT_PUBLIC_API_URL;
  if (fromEnv) return fromEnv.replace(/\/$/, "");
  return wsUrl.replace(/^ws/, "http").replace(/\/stream$/, "");
}

export function useTelemetryStream() {
  const [connection, setConnection] = useState<ConnectionState>("connecting");
  const [lastError, setLastError] = useState<string | null>(null);
  const [reconnectCount, setReconnectCount] = useState(0);
  const [events, setEvents] = useState<EventEnvelope[]>([]);
  const [stats, setStats] = useState<Record<string, MetricStat>>({});
  const [devices, setDevices] = useState<string[]>([]);
  const [series, setSeries] = useState<Record<string, SeriesPoint[]>>({});
  const socketRef = useRef<WebSocket | null>(null);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const seenEventIdsRef = useRef<Set<string>>(new Set());
  const seededDeviceRef = useRef<string | null>(null);

  const connectRef = useRef<() => void>(() => {});

  const scheduleReconnect = useCallback(() => {
    if (timerRef.current !== null) return;
    timerRef.current = setTimeout(() => {
      timerRef.current = null;
      if (document.hidden) return;
      connectRef.current();
      setReconnectCount((c) => c + 1);
    }, 2000);
  }, []);

  const connect = useCallback(() => {
    const wsUrl = process.env.NEXT_PUBLIC_WS_URL ?? "ws://localhost:3000/api/v1/stream";
    const ws = new WebSocket(wsUrl);
    socketRef.current = ws;

    ws.onopen = () => {
      setConnection("live");
      setLastError(null);
    };

    ws.onmessage = (raw) => {
      let parsed: unknown;
      try {
        parsed = JSON.parse(raw.data);
      } catch {
        return;
      }
      const event = parseEvent(parsed);
      if (!event) return;
      if (seenEventIdsRef.current.has(event.event_id)) return;
      seenEventIdsRef.current.add(event.event_id);
      if (seenEventIdsRef.current.size > 1000) {
        const first = seenEventIdsRef.current.values().next().value;
        if (first) seenEventIdsRef.current.delete(first);
      }

      setEvents((prev) => [event, ...prev].slice(0, 50));
      if (event.event_type === "TELEMETRY_RECEIVED" && event.payload) {
        const { metrics } = event.payload;
        const now = event.created_at;
        const t = Date.parse(now);
        setStats((prev) => {
          const next = { ...prev };
          for (const m of metrics) {
            next[m.key] = {
              value: m.value,
              unit: m.unit,
              deviceId: event.device_id,
              updatedAt: now,
            };
          }
          return next;
        });
        if (Number.isFinite(t)) {
          setSeries((prev) => {
            const next = { ...prev };
            for (const m of metrics) {
              if (!Number.isFinite(m.value)) continue;
              const arr = next[m.key] ?? [];
              const last = arr.length > 0 ? arr[arr.length - 1].t : undefined;
              if (last !== undefined && t <= last) continue;
              next[m.key] = [...arr, { t, value: m.value }].slice(-SERIES_CAP);
            }
            return next;
          });
        }
      }
      setDevices((prev) => (prev.includes(event.device_id) ? prev : [...prev, event.device_id]));
    };

    ws.onerror = () => {
      setLastError("WebSocket error");
    };

    ws.onclose = () => {
      setConnection("offline");
      if (!document.hidden) scheduleReconnect();
    };
  }, [scheduleReconnect]);

  useEffect(() => {
    connectRef.current = connect;
  }, [connect]);

  useEffect(() => {
    const target = devices[0];
    if (!target) return;
    if (seededDeviceRef.current === target) return;
    if (seededDeviceRef.current !== null) setSeries({});
    seededDeviceRef.current = target;
    const wsUrl =
      process.env.NEXT_PUBLIC_WS_URL ?? "ws://localhost:3000/api/v1/stream";
    const apiBase = resolveApiBase(wsUrl);
    let cancelled = false;
    const seed = async () => {
      try {
        const res = await fetch(
          `${apiBase}/telemetry?device_id=${encodeURIComponent(target)}&limit=${REST_SEED_LIMIT}`,
        );
        if (!res.ok || cancelled) return;
        const data = (await res.json()) as TelemetryQueryResponse;
        if (!Array.isArray(data.samples) || cancelled) return;
        const grouped: Record<string, SeriesPoint[]> = {};
        for (const s of data.samples) {
          const t = Date.parse(s.recorded_at);
          if (!Number.isFinite(t) || !Number.isFinite(s.value)) continue;
          (grouped[s.key] ??= []).push({ t, value: s.value });
        }
        setSeries((prev) => {
          const next: Record<string, SeriesPoint[]> = { ...prev };
          for (const [key, pts] of Object.entries(grouped)) {
            pts.sort((a, b) => a.t - b.t);
            const seen = new Set((next[key] ?? []).map((p) => p.t));
            const merged = [...(next[key] ?? [])];
            for (const p of pts) {
              if (seen.has(p.t)) continue;
              seen.add(p.t);
              merged.push(p);
            }
            merged.sort((a, b) => a.t - b.t);
            next[key] = merged.slice(-SERIES_CAP);
          }
          return next;
        });
      } catch {
        seededDeviceRef.current = null;
      }
    };
    void seed();
    return () => {
      cancelled = true;
    };
  }, [devices]);

  useEffect(() => {
    connect();
    const handleVisibility = () => {
      if (!document.hidden) {
        const socket = socketRef.current;
        const dead = !socket || socket.readyState >= WebSocket.CLOSING;
        if (dead && timerRef.current === null) scheduleReconnect();
      }
    };
    document.addEventListener("visibilitychange", handleVisibility);
    return () => {
      document.removeEventListener("visibilitychange", handleVisibility);
      if (timerRef.current !== null) {
        clearTimeout(timerRef.current);
        timerRef.current = null;
      }
      const socket = socketRef.current;
      if (
        socket &&
        (socket.readyState === WebSocket.OPEN || socket.readyState === WebSocket.CONNECTING)
      ) {
        socket.close();
      }
      socketRef.current = null;
    };
  }, [connect, scheduleReconnect]);

  return { connection, lastError, reconnectCount, events, stats, devices, series };
}
