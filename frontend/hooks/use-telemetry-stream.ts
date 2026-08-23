"use client";

import { useState, useRef, useEffect, useCallback } from "react";
import type { EventEnvelope } from "@/lib/types";
import { parseEvent } from "@/lib/types";

export type ConnectionState = "connecting" | "live" | "offline";

export interface MetricStat {
  value: number;
  unit: string;
  deviceId: string;
  updatedAt: string;
}

export function useTelemetryStream() {
  const [connection, setConnection] = useState<ConnectionState>("connecting");
  const [lastError, setLastError] = useState<string | null>(null);
  const [reconnectCount, setReconnectCount] = useState(0);
  const [events, setEvents] = useState<EventEnvelope[]>([]);
  const [stats, setStats] = useState<Record<string, MetricStat>>({});
  const [devices, setDevices] = useState<string[]>([]);
  const socketRef = useRef<WebSocket | null>(null);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const seenEventIdsRef = useRef<Set<string>>(new Set());

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

  return { connection, lastError, reconnectCount, events, stats, devices };
}
