"use client";

import { useEffect } from "react";
import Link from "next/link";
import { useParams } from "next/navigation";
import { Eye, Files, MessageSquareMore, Send, Users } from "lucide-react";

import { EventForm } from "@/components/cabinet/event-form";
import { PageHeader } from "@/components/layout/page-header";
import { ErrorState } from "@/components/states/error-state";
import { LoadingState } from "@/components/states/loading-state";
import { Button } from "@/components/ui/button";
import { InfoCard } from "@/components/ui/info-card";
import { SectionBlock } from "@/components/ui/section-block";
import { StatusBadge } from "@/components/ui/status-badge";
import { useApiState } from "@/hooks/use-api-state";
import { eventsApi } from "@/lib/api/events";
import { templatesApi } from "@/lib/api/templates";
import type { Event, EventUpdateRequest, Template } from "@/lib/types";

export default function EventDetailPage() {
  const params = useParams<{ id: string }>();
  const eventId = params.id;
  const eventState = useApiState<Event | null>(null);
  const templatesState = useApiState<Template[] | null>(null);
  const submitState = useApiState<Event | null>(null);

  async function loadData() {
    await Promise.all([
      eventState.run(async () => eventsApi.getById(eventId)),
      templatesState.run(async () => {
        const response = await templatesApi.list("?page=1&per_page=100");
        return response.items;
      })
    ]);
  }

  useEffect(() => {
    void loadData();
  }, [eventId]);

  async function handleUpdate(payload: EventUpdateRequest) {
    const nextEvent = await submitState.run(async () => eventsApi.update(eventId, payload));
    eventState.setData(nextEvent);
  }

  async function handlePublish(nextStatus: "publish" | "unpublish") {
    const nextEvent = await submitState.run(async () =>
      nextStatus === "publish" ? eventsApi.publish(eventId) : eventsApi.unpublish(eventId)
    );
    eventState.setData(nextEvent);
  }

  const currentEvent = eventState.data;

  return (
    <div className="space-y-6">
      <PageHeader
        eyebrow="Event"
        title={currentEvent?.title ?? "Event overview"}
        description="Настройте основную информацию о событии, управляйте статусом публикации и переходите к секциям, гостям и RSVP."
        action={
          <div className="flex flex-wrap gap-3">
            <Button variant="secondary" asChild>
              <Link href="/dashboard">Dashboard</Link>
            </Button>
            {currentEvent?.status === "published" ? (
              <Button variant="outline" onClick={() => void handlePublish("unpublish")}>
                Unpublish
              </Button>
            ) : (
              <Button onClick={() => void handlePublish("publish")}>
                Publish
              </Button>
            )}
          </div>
        }
      />

      {eventState.isLoading || templatesState.isLoading ? (
        <LoadingState label="Loading event details..." />
      ) : null}

      {eventState.error || templatesState.error ? (
        <ErrorState
          title="Could not load event"
          description={eventState.error ?? templatesState.error ?? "Unexpected error"}
          onRetry={() => {
            void loadData();
          }}
        />
      ) : null}

      {submitState.error ? (
        <ErrorState title="Could not update event" description={submitState.error} />
      ) : null}

      {currentEvent ? (
        <div className="grid gap-6 xl:grid-cols-[1.1fr_0.9fr]">
          <div className="space-y-6">
            <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
              <InfoCard label="Status" value={currentEvent.status} hint="Текущее состояние публикации" />
              <InfoCard label="Type" value={currentEvent.event_type} />
              <InfoCard label="Timezone" value={currentEvent.timezone} />
              <InfoCard
                label="Date"
                value={
                  currentEvent.event_date
                    ? new Date(currentEvent.event_date).toLocaleDateString()
                    : "Not set"
                }
              />
            </div>
            <EventForm
              mode="edit"
              event={currentEvent}
              templates={templatesState.data ?? []}
              isSubmitting={submitState.isLoading}
              onSubmit={handleUpdate}
            />
          </div>

          <div className="space-y-6">
            <SectionBlock
              title="Workspace shortcuts"
              description="Быстрые переходы в ключевые рабочие разделы события."
              action={<StatusBadge status={currentEvent.status} />}
            >
              <div className="grid gap-3">
                <ShortcutLink href={`/events/${eventId}/sections`} icon={Files} label="Sections" />
                <ShortcutLink href={`/events/${eventId}/guests`} icon={Users} label="Guests" />
                <ShortcutLink href={`/events/${eventId}/rsvp`} icon={MessageSquareMore} label="RSVP" />
                <ShortcutLink href={`/events/${eventId}/preview`} icon={Eye} label="Preview" />
              </div>
            </SectionBlock>

            <SectionBlock
              title="Publication"
              description="Опубликованное событие становится доступно на public route по slug."
            >
              <div className="space-y-4 text-sm text-muted-foreground">
                <p>
                  Draft можно спокойно редактировать дальше. После публикации invite page и RSVP
                  становятся доступны по публичной ссылке.
                </p>
                <div className="rounded-2xl bg-secondary/45 px-4 py-4">
                  <div className="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">
                    Public slug
                  </div>
                  <div className="mt-1 break-all font-medium text-foreground">{currentEvent.slug}</div>
                </div>
                <Button
                  className="w-full"
                  variant={currentEvent.status === "published" ? "secondary" : "default"}
                  onClick={() => void handlePublish(currentEvent.status === "published" ? "unpublish" : "publish")}
                >
                  <Send className="size-4" />
                  {currentEvent.status === "published" ? "Switch back to draft" : "Publish event"}
                </Button>
              </div>
            </SectionBlock>
          </div>
        </div>
      ) : null}
    </div>
  );
}

function ShortcutLink({
  href,
  icon: Icon,
  label
}: {
  href: string;
  icon: React.ComponentType<{ className?: string }>;
  label: string;
}) {
  return (
    <Button variant="secondary" className="justify-start" asChild>
      <Link href={href}>
        <Icon className="size-4" />
        {label}
      </Link>
    </Button>
  );
}
