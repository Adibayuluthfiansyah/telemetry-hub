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

  if (isLoading && events.length === 0) {
    return (
      <Card className={className}>
        <CardHeader className="border-b border-border p-3 bg-surface-container">
          <CardTitle className="font-label-caps text-label-caps text-on-surface">
            LIVE EVENT STREAM
          </CardTitle>
        </CardHeader>
        <CardContent className="p-2 flex-1 overflow-y-auto space-y-[1px] bg-border scrollbar-slim">
          {[...Array(5)].map((_, i) => (
            <Skeleton key={i} className="h-20 w-full bg-surface-container-low" />
          ))}
        </CardContent>
      </Card>
    );
  }

  return (
    <Card className={cn("flex flex-col h-full min-h-0", className)}>
      <CardHeader className="border-b border-border p-3 bg-surface-container shrink-0">
        <CardTitle className="font-label-caps text-label-caps text-on-surface">
          LIVE EVENT STREAM
        </CardTitle>
      </CardHeader>
      <CardContent className="p-2 flex-1 overflow-y-auto space-y-[1px] bg-border scrollbar-slim">
        {events.length === 0 ? (
          <div className="flex items-center justify-center h-32 text-on-surface-variant font-data-mono text-[11px]">
            No events received yet...
          </div>
        ) : (
          events.map((event, index) => (
            <EventRow key={`${event.event_id}-${index}`} event={event} />
          ))
        )}
      </CardContent>
    </Card>
  );
}