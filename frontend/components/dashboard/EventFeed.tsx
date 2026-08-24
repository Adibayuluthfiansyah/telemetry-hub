"use client";

import { cn } from "@/lib/utils";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { EventEnvelope } from "@/lib/types";
import { EventRow } from "./EventRow";
import { ConnectionState } from "@/hooks/use-telemetry-stream";

interface EventFeedProps {
  events: EventEnvelope[];
  connection: ConnectionState;
  className?: string;
}

export function EventFeed({ events, connection, className }: EventFeedProps) {
  const isLoading = connection !== "live";

  return (
    <Card
      className={cn(
        "flex flex-col h-full min-h-0 border-border [--card-spacing:0px]",
        className
      )}
    >
      <CardHeader className="border-b border-border p-3 bg-surface-container shrink-0">
        <CardTitle className="font-label-caps text-[11px] leading-4 text-on-surface">
          LIVE EVENT STREAM
        </CardTitle>
      </CardHeader>
      <CardContent className="flex-1 min-h-0 overflow-hidden p-0">
        <div className="h-full overflow-y-auto p-2 space-y-[1px] bg-border scrollbar-slim">
          {isLoading && events.length === 0 ? (
            [...Array(5)].map((_, i) => (
              <Skeleton key={i} className="h-20 w-full bg-surface-container-low" />
            ))
          ) : events.length === 0 ? (
            <div className="flex items-center justify-center h-32 bg-surface-container text-on-surface-variant font-data-mono text-[11px] leading-[18px]">
              No events received yet...
            </div>
          ) : (
            events.map((event, index) => (
              <EventRow key={`${event.event_id}-${index}`} event={event} />
            ))
          )}
        </div>
      </CardContent>
    </Card>
  );
}
