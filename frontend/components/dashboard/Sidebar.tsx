"use client";

import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";

interface SidebarProps {
  className?: string;
}

const navItems = [
  { id: "overview", label: "OVERVIEW", icon: "dashboard", active: false },
  { id: "live", label: "LIVE STREAM", icon: "podcasts", active: true },
  { id: "api", label: "API", icon: "api", active: false },
  { id: "database", label: "DB", icon: "database", active: false },
  { id: "ws", label: "WS", icon: "settings_ethernet", active: false },
] as const;

export function Sidebar({ className }: SidebarProps) {
  return (
    <nav
      className={cn(
        "bg-surface-container-low border-r border-border flat no-shadows flex flex-col h-full w-20 md:w-24 py-4 items-center shrink-0 z-20",
        className,
      )}
      aria-label="Main navigation"
    >
      <div className="mb-8 flex flex-col items-center">
        <div className="w-10 h-10 bg-surface-container flex items-center justify-center border border-border mb-2 rounded-none">
          <span className="material-symbols-outlined text-primary text-xl">memory</span>
        </div>
        <div className="font-label-caps text-[11px] leading-4 font-bold text-on-surface text-center whitespace-nowrap">
          NODE_01
        </div>
        <div className="font-data-mono text-[9px] leading-3 text-primary mt-1 whitespace-nowrap">
          OPERATIONAL
        </div>
      </div>

      <div className="flex-1 w-full flex flex-col items-center space-y-4">
        {navItems.slice(0, 2).map((item) => (
          <Button
            key={item.id}
            variant="ghost"
            className={cn(
              "group h-auto w-full flex flex-col items-center py-3 text-on-surface-variant hover:text-on-surface hover:bg-surface-container transition-colors border-l-2",
              item.active
                ? "text-primary border-primary bg-surface-container-high"
                : "border-transparent",
            )}
            aria-current={item.active ? "page" : undefined}
          >
            <span className="material-symbols-outlined mb-1 group-hover:scale-110 transition-transform">
              {item.icon}
            </span>
            <span className="font-label-caps text-[9px] leading-3 mt-1 hidden md:block whitespace-nowrap">
              {item.label}
            </span>
          </Button>
        ))}
      </div>

      <div className="w-full flex flex-col items-center space-y-4 mt-auto pt-4 border-t border-border">
        {navItems.slice(2).map((item) => (
          <Button
            key={item.id}
            variant="ghost"
            className="group w-full flex flex-col items-center py-2 text-on-surface-variant hover:text-on-surface hover:bg-surface-container transition-colors border-l-2 border-transparent"
            title={item.label}
          >
            <span className="material-symbols-outlined group-hover:scale-110 transition-transform text-sm">
              {item.icon}
            </span>
            <span className="font-label-caps text-[8px] leading-[10px] mt-1 hidden md:block whitespace-nowrap">
              {item.label}
            </span>
          </Button>
        ))}
      </div>
    </nav>
  );
}
