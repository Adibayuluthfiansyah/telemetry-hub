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
    <div className="flex h-screen bg-background overflow-hidden">
      <Sidebar />
      <div className="flex-1 flex flex-col overflow-hidden relative">
        <TopBar connection={connection} />
        <main className="flex-1 overflow-y-auto p-[8px] flex flex-col gap-[8px]">
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
  );
}