"use client";

import { useEffect } from "react";
import Link from "next/link";
import { useRouter } from "next/navigation";

import { EventForm } from "@/components/cabinet/event-form";
import { PageHeader } from "@/components/layout/page-header";
import { ErrorState } from "@/components/states/error-state";
import { LoadingState } from "@/components/states/loading-state";
import { Button } from "@/components/ui/button";
import { useApiState } from "@/hooks/use-api-state";
import { eventsApi } from "@/lib/api/events";
import { templatesApi } from "@/lib/api/templates";
import type { EventCreateRequest, Template, TemplatesResponse } from "@/lib/types";

type EventCreatePageClientProps = {
  initialTemplateId: string | null;
};

export function EventCreatePageClient({
  initialTemplateId
}: EventCreatePageClientProps) {
  const router = useRouter();
  const templatesState = useApiState<Template[] | null>(null);
  const createState = useApiState<unknown>(null);

  useEffect(() => {
    void templatesState.run(async () => {
      const response: TemplatesResponse = await templatesApi.list("?page=1&per_page=100&is_active=true");
      return response.items;
    });
  }, []);

  async function handleSubmit(payload: EventCreateRequest | Partial<EventCreateRequest>) {
    await createState.run(async () => {
      const event = await eventsApi.create(payload as EventCreateRequest);
      router.push(`/events/${event.id}`);
      return event;
    });
  }

  return (
    <div className="space-y-6">
      <PageHeader
        eyebrow="Events"
        title="Create event"
        description="Start a new invitation flow with the core event settings and an optional template."
        action={
          <Button variant="secondary" asChild>
            <Link href="/dashboard">Back to dashboard</Link>
          </Button>
        }
      />

      {templatesState.isLoading ? <LoadingState label="Loading templates..." /> : null}
      {templatesState.error ? (
        <ErrorState
          title="Could not load templates"
          description={templatesState.error}
          onRetry={() => {
            void templatesState.run(async () => {
              const response = await templatesApi.list("?page=1&per_page=100&is_active=true");
              return response.items;
            });
          }}
        />
      ) : null}
      {createState.error ? (
        <ErrorState title="Could not create event" description={createState.error} />
      ) : null}

      {!templatesState.isLoading && !templatesState.error ? (
        <EventForm
          mode="create"
          templates={templatesState.data ?? []}
          initialTemplateId={initialTemplateId}
          isSubmitting={createState.isLoading}
          onSubmit={handleSubmit}
        />
      ) : null}
    </div>
  );
}
