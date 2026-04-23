"use client";

import { useEffect } from "react";
import Link from "next/link";
import { useParams } from "next/navigation";
import { Eye } from "lucide-react";

import { PageHeader } from "@/components/layout/page-header";
import { ErrorState } from "@/components/states/error-state";
import { LoadingState } from "@/components/states/loading-state";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { StatusBadge } from "@/components/ui/status-badge";
import { useApiState } from "@/hooks/use-api-state";
import { eventsApi } from "@/lib/api/events";
import { rsvpApi } from "@/lib/api/rsvp";
import { sectionsApi } from "@/lib/api/sections";
import type { Event, PageSection, RsvpForm } from "@/lib/types";

export default function EventPreviewPage() {
  const params = useParams<{ id: string }>();
  const eventId = params.id;
  const eventState = useApiState<Event | null>(null);
  const sectionsState = useApiState<PageSection[] | null>(null);
  const rsvpState = useApiState<RsvpForm | null>(null);

  async function loadData() {
    await Promise.all([
      eventState.run(async () => eventsApi.getById(eventId)),
      sectionsState.run(async () => {
        const response = await sectionsApi.list(eventId);
        return response.items;
      }),
      rsvpState.run(async () => rsvpApi.getForm(eventId))
    ]);
  }

  useEffect(() => {
    void loadData();
  }, [eventId]);

  const sections = [...(sectionsState.data ?? [])].sort((left, right) => left.position - right.position);

  return (
    <div className="space-y-6">
      <PageHeader
        eyebrow="Preview"
        title={eventState.data ? `${eventState.data.title} preview` : "Invite preview"}
        description="Internal read-only preview built from the current private event data, even before publication."
        action={
          <div className="flex flex-wrap gap-3">
            <Button variant="secondary" asChild>
              <Link href={`/events/${eventId}`}>Back to event</Link>
            </Button>
            {eventState.data ? (
              <Button variant="outline" asChild>
                <Link href={`/invite/${eventState.data.slug}`}>Public route</Link>
              </Button>
            ) : null}
          </div>
        }
      />

      {eventState.isLoading || sectionsState.isLoading || rsvpState.isLoading ? (
        <LoadingState label="Preparing private preview..." />
      ) : null}

      {eventState.error || sectionsState.error || rsvpState.error ? (
        <ErrorState
          title="Could not prepare preview"
          description={eventState.error ?? sectionsState.error ?? rsvpState.error ?? "Unexpected error"}
          onRetry={() => {
            void loadData();
          }}
        />
      ) : null}

      {eventState.data ? (
        <Card className="overflow-hidden">
          <div className="bg-[linear-gradient(135deg,rgba(0,129,167,0.18),rgba(183,225,205,0.3))] px-6 py-10 sm:px-10">
            <div className="max-w-3xl space-y-4">
              <StatusBadge status={eventState.data.status} />
              <h1 className="font-[family-name:var(--font-heading)] text-4xl">
                {eventState.data.title}
              </h1>
              <p className="text-muted-foreground">
                {eventState.data.event_type} · {eventState.data.event_date ? new Date(eventState.data.event_date).toLocaleString() : "Date TBD"} · {eventState.data.timezone}
              </p>
              {eventState.data.venue_name || eventState.data.venue_address ? (
                <p className="text-sm text-muted-foreground">
                  {eventState.data.venue_name ?? "Venue"} · {eventState.data.venue_address ?? "Address TBD"}
                </p>
              ) : null}
            </div>
          </div>
          <CardContent className="space-y-6 pt-6">
            {sections.map((section) => (
              <Card key={section.id} className="bg-background shadow-none">
                <CardHeader>
                  <div className="flex items-center justify-between gap-3">
                    <div>
                      <StatusBadge status={`#${section.position}`} />
                      <CardTitle className="mt-3">{section.title ?? section.section_type}</CardTitle>
                    </div>
                    <StatusBadge status={section.is_enabled ? "enabled" : "disabled"} />
                  </div>
                </CardHeader>
                <CardContent>
                  <pre className="overflow-x-auto rounded-2xl bg-slate-950 p-4 text-xs text-slate-100">
                    {JSON.stringify(section.content, null, 2)}
                  </pre>
                </CardContent>
              </Card>
            ))}
            <Card className="bg-background shadow-none">
              <CardHeader>
                <div className="flex items-center gap-3">
                  <div className="inline-flex size-10 items-center justify-center rounded-2xl bg-secondary text-secondary-foreground">
                    <Eye className="size-4" />
                  </div>
                  <CardTitle>{rsvpState.data?.title ?? "RSVP"}</CardTitle>
                </div>
              </CardHeader>
              <CardContent className="space-y-4">
                {rsvpState.data?.description ? (
                  <p className="text-sm text-muted-foreground">{rsvpState.data.description}</p>
                ) : null}
                <div className="space-y-3">
                  {(rsvpState.data?.questions ?? []).map((question) => (
                    <div key={question.id} className="rounded-2xl bg-secondary/35 px-4 py-3">
                      <div className="font-medium">{question.label}</div>
                      <div className="text-sm text-muted-foreground">
                        {question.question_type} {question.required ? "· required" : ""}
                      </div>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          </CardContent>
        </Card>
      ) : null}
    </div>
  );
}
