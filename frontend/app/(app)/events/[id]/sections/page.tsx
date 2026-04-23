"use client";

import { useEffect, useMemo, useState } from "react";
import Link from "next/link";
import { useParams } from "next/navigation";
import { Layers, Plus } from "lucide-react";

import { ConfirmDialog } from "@/components/cabinet/confirm-dialog";
import { SectionCard } from "@/components/cabinet/section-card";
import { SectionForm } from "@/components/cabinet/section-form";
import { PageHeader } from "@/components/layout/page-header";
import { EmptyState } from "@/components/states/empty-state";
import { ErrorState } from "@/components/states/error-state";
import { LoadingState } from "@/components/states/loading-state";
import { Button } from "@/components/ui/button";
import { InfoCard } from "@/components/ui/info-card";
import { useApiState } from "@/hooks/use-api-state";
import { eventsApi } from "@/lib/api/events";
import { sectionsApi } from "@/lib/api/sections";
import type { CreateSectionRequest, Event, PageSection, UpdateSectionRequest } from "@/lib/types";

export default function EventSectionsPage() {
  const params = useParams<{ id: string }>();
  const eventId = params.id;
  const eventState = useApiState<Event | null>(null);
  const sectionsState = useApiState<PageSection[] | null>(null);
  const [showCreate, setShowCreate] = useState(false);
  const [editingSection, setEditingSection] = useState<PageSection | null>(null);
  const [submitting, setSubmitting] = useState(false);
  const [deleteTarget, setDeleteTarget] = useState<PageSection | null>(null);

  async function loadData() {
    await Promise.all([
      eventState.run(async () => eventsApi.getById(eventId)),
      sectionsState.run(async () => {
        const response = await sectionsApi.list(eventId);
        return response.items;
      })
    ]);
  }

  useEffect(() => {
    void loadData();
  }, [eventId]);

  const sections = useMemo(
    () => [...(sectionsState.data ?? [])].sort((left, right) => left.position - right.position),
    [sectionsState.data]
  );

  async function handleCreate(payload: CreateSectionRequest | UpdateSectionRequest) {
    setSubmitting(true);
    try {
      await sectionsApi.create(eventId, payload as CreateSectionRequest);
      setShowCreate(false);
      await refreshSections();
    } finally {
      setSubmitting(false);
    }
  }

  async function handleUpdate(payload: CreateSectionRequest | UpdateSectionRequest) {
    if (!editingSection) {
      return;
    }

    setSubmitting(true);
    try {
      await sectionsApi.update(eventId, editingSection.id, payload as UpdateSectionRequest);
      setEditingSection(null);
      await refreshSections();
    } finally {
      setSubmitting(false);
    }
  }

  async function refreshSections() {
    await sectionsState.run(async () => {
      const response = await sectionsApi.list(eventId);
      return response.items;
    });
  }

  async function reorderSections(index: number, direction: -1 | 1) {
    const nextIndex = index + direction;
    if (nextIndex < 0 || nextIndex >= sections.length) {
      return;
    }

    const nextIds = sections.map((section) => section.id);
    const [current] = nextIds.splice(index, 1);
    nextIds.splice(nextIndex, 0, current);

    await sectionsApi.reorder(eventId, { section_ids: nextIds });
    await refreshSections();
  }

  async function deleteSection() {
    if (!deleteTarget) {
      return;
    }

    setSubmitting(true);
    try {
      await sectionsApi.remove(eventId, deleteTarget.id);
      setDeleteTarget(null);
      await refreshSections();
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="space-y-6">
      <PageHeader
        eyebrow="Sections"
        title={eventState.data ? `${eventState.data.title} sections` : "Event sections"}
        description="Собирайте invite page из секций, меняйте порядок блоков и редактируйте JSON-контент без лишней магии."
        action={
          <div className="flex flex-wrap gap-3">
            <Button variant="secondary" asChild>
              <Link href={`/events/${eventId}`}>Back to event</Link>
            </Button>
            <Button onClick={() => setShowCreate((current) => !current)}>
              <Plus className="size-4" />
              Add section
            </Button>
          </div>
        }
      />

      {eventState.isLoading || sectionsState.isLoading ? (
        <LoadingState label="Loading event sections..." />
      ) : null}

      {eventState.error || sectionsState.error ? (
        <ErrorState
          title="Could not load sections"
          description={eventState.error ?? sectionsState.error ?? "Unexpected error"}
          onRetry={() => {
            void loadData();
          }}
        />
      ) : null}

      {showCreate ? (
        <SectionForm mode="create" isSubmitting={submitting} onCancel={() => setShowCreate(false)} onSubmit={handleCreate} />
      ) : null}

      {editingSection ? (
        <SectionForm
          mode="edit"
          section={editingSection}
          isSubmitting={submitting}
          onCancel={() => setEditingSection(null)}
          onSubmit={handleUpdate}
        />
      ) : null}

      {sections.length > 0 ? (
        <div className="grid gap-4 md:grid-cols-3">
          <InfoCard label="Всего секций" value={String(sections.length)} />
          <InfoCard
            label="Enabled"
            value={String(sections.filter((section) => section.is_enabled).length)}
          />
          <InfoCard
            label="Next step"
            value="Проверьте preview"
            hint="После правок откройте preview и убедитесь, что порядок блоков читается хорошо."
          />
        </div>
      ) : null}

      {!eventState.isLoading && !sectionsState.isLoading && !eventState.error && !sectionsState.error && sections.length === 0 ? (
        <EmptyState
          icon={Layers}
          title="No sections yet"
          description="Start with a hero section, then add program, location, FAQ and RSVP blocks."
          action={
            <Button onClick={() => setShowCreate(true)}>
              <Plus className="size-4" />
              Add first section
            </Button>
          }
        />
      ) : null}

      <div className="grid gap-4">
        {sections.map((section, index) => (
          <SectionCard
            key={section.id}
            section={section}
            canMoveUp={index > 0}
            canMoveDown={index < sections.length - 1}
            onMoveUp={() => {
              void reorderSections(index, -1);
            }}
            onMoveDown={() => {
              void reorderSections(index, 1);
            }}
            onEdit={() => setEditingSection(section)}
            onDelete={() => setDeleteTarget(section)}
          />
        ))}
      </div>

      <ConfirmDialog
        open={Boolean(deleteTarget)}
        title="Delete section?"
        description={`This will remove ${deleteTarget?.title ?? deleteTarget?.section_type ?? "the section"} from the page.`}
        confirmLabel="Delete"
        tone="danger"
        isLoading={submitting}
        onConfirm={() => {
          void deleteSection();
        }}
        onCancel={() => setDeleteTarget(null)}
      />
    </div>
  );
}
