"use client";

import { ReactNode } from "react";
import { Sidebar } from "../dashboard/Sidebar";
import { TopBar } from "../dashboard/TopBar";
import { FooterBar } from "../dashboard/FooterBar";
import { ConnectionState } from "@/hooks/use-telemetry-stream";

interface DashboardLayoutProps {
  children: ReactNode;
  connection: ConnectionState;
  eventCount?: number;
  deviceCount?: number;
  bufferUsage?: { used: number; total: number };
}

export function DashboardLayout({
  children,
  connection,
  eventCount = 0,
  deviceCount = 0,
  bufferUsage = { used: 0, total: 50 },
}: DashboardLayoutProps) {
  return (
    <div className="h-screen bg-dots bg-[#070a0b] p-3 flex flex-col gap-2 overflow-hidden">
      <div className="flex items-center gap-2 px-1 shrink-0">
        <span className="material-symbols-outlined text-primary text-[16px] leading-4">terminal</span>
        <span className="font-label-caps text-[11px] leading-4 text-on-surface">
          Telemetry Hub — Read-Only Console
        </span>
      </div>
      <div className="flex-1 min-h-0 flex bg-background border border-border overflow-hidden">
        <Sidebar />
        <div className="flex-1 flex flex-col overflow-hidden relative">
          <TopBar connection={connection} />
          <main className="flex-1 overflow-y-auto p-[8px] flex flex-col gap-[8px] scrollbar-slim">
            {children}
          </main>
          <FooterBar
            connection={connection}
            eventCount={eventCount}
            deviceCount={deviceCount}
            bufferUsage={bufferUsage}
          />
        </div>
      </div>
    </div>
  );
}
