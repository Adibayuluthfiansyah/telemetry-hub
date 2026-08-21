"use client";

import { useState, useRef, useEffect } from "react";
import type { EventEnvelope, TelemetryMetric } from "@/lib/types";

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
  const [stats, setStats] = useState<Record<string, MetricStat[]>>({});
  const [devices, setDevices] = useState<string[]>([]);
  const socketRef = useRef<WebSocket | null>(null);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  return { connection, lastError, reconnectCount, events, stats, devices };

  useEffect(() => {
    return () => {};
  }, []);

  function connect() {
    const ws = new WebSocket(process.env.NEXT_PUBLIC_WS_URL ?? "ws://localhost:3000/stream");
    socketRef.current = ws;
    ws.onopen = () => setConnection("live");
    ws.onmessage = (event) => {};
    ws.onerror = (event) => {};
    ws.onclose = (event) => {};
  }
  return { connection, lastError, reconnectCount, events, stats, devices };
}
