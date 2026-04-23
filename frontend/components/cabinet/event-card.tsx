import Link from "next/link";
import { ArrowRight, Clock3, MapPin, Sparkles } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { StatusBadge } from "@/components/ui/status-badge";
import type { Event } from "@/lib/types";

export function EventCard({ event }: { event: Event }) {
  return (
    <Card className="h-full overflow-hidden border-white/80 bg-white/95">
      <div className="h-1.5 bg-gradient-to-r from-primary/65 via-accent to-secondary" />
      <CardHeader className="gap-4">
        <div className="flex flex-wrap items-start justify-between gap-3">
          <div className="space-y-2">
            <CardTitle>{event.title}</CardTitle>
            <p className="text-sm text-muted-foreground">
              {event.event_type} · {event.slug}
            </p>
          </div>
          <StatusBadge status={event.status} />
        </div>
      </CardHeader>
      <CardContent className="space-y-5 text-sm text-muted-foreground">
        <div className="grid gap-3 sm:grid-cols-2">
          <InfoLine
            label="Updated"
            value={new Date(event.updated_at).toLocaleDateString()}
            icon={Clock3}
          />
          <InfoLine label="Timezone" value={event.timezone} />
          <InfoLine label="Date" value={event.event_date ? new Date(event.event_date).toLocaleString() : "Not set"} />
          <InfoLine label="Venue" value={event.venue_name ?? event.venue_address ?? "Not set"} icon={MapPin} />
        </div>
        <div className="rounded-[1.3rem] bg-gradient-to-r from-secondary/55 to-accent/35 px-4 py-3 text-sm">
          <div className="mb-1 flex items-center gap-2 font-medium text-foreground">
            <Sparkles className="size-4 text-primary" />
            Быстрый next step
          </div>
          <div className="text-muted-foreground">
            {event.status === "published"
              ? "Проверьте публичную страницу и ответы RSVP."
              : "Дополните секции и опубликуйте событие, когда всё будет готово."}
          </div>
        </div>
        <Button asChild className="w-full justify-between">
          <Link href={`/events/${event.id}`}>
            Открыть событие
            <ArrowRight className="size-4" />
          </Link>
        </Button>
      </CardContent>
    </Card>
  );
}

function InfoLine({
  label,
  value,
  icon: Icon
}: {
  label: string;
  value: string;
  icon?: React.ComponentType<{ className?: string }>;
}) {
  return (
    <div className="rounded-2xl bg-secondary/45 px-4 py-3">
      <div className="mb-1 flex items-center gap-2 text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">
        {Icon ? <Icon className="size-3.5" /> : null}
        {label}
      </div>
      <div className="text-sm leading-6 text-foreground">{value}</div>
    </div>
  );
}
