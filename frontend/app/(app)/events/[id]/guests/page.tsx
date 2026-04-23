"use client";

import { useEffect, useMemo, useState } from "react";
import Link from "next/link";
import { useParams } from "next/navigation";
import { Plus, Users } from "lucide-react";

import { ConfirmDialog } from "@/components/cabinet/confirm-dialog";
import { CsvImportPanel } from "@/components/cabinet/csv-import-panel";
import { GuestFormDialog } from "@/components/cabinet/guest-form-dialog";
import { GuestTable } from "@/components/cabinet/guest-table";
import { PageHeader } from "@/components/layout/page-header";
import { EmptyState } from "@/components/states/empty-state";
import { ErrorState } from "@/components/states/error-state";
import { LoadingState } from "@/components/states/loading-state";
import { Button } from "@/components/ui/button";
import { InfoCard } from "@/components/ui/info-card";
import { Input } from "@/components/ui/input";
import { useApiState } from "@/hooks/use-api-state";
import { eventsApi } from "@/lib/api/events";
import { guestsApi } from "@/lib/api/guests";
import type { Event, Guest, GuestCreateRequest, GuestImportSummary, GuestUpdateRequest } from "@/lib/types";

export default function EventGuestsPage() {
  const params = useParams<{ id: string }>();
  const eventId = params.id;
  const eventState = useApiState<Event | null>(null);
  const guestsState = useApiState<Guest[] | null>(null);
  const [search, setSearch] = useState("");
  const [vipOnly, setVipOnly] = useState(false);
  const [dialogOpen, setDialogOpen] = useState(false);
  const [editingGuest, setEditingGuest] = useState<Guest | null>(null);
  const [deleteTarget, setDeleteTarget] = useState<Guest | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [importSummary, setImportSummary] = useState<GuestImportSummary | null>(null);

  async function loadData() {
    await Promise.all([eventState.run(async () => eventsApi.getById(eventId)), loadGuests()]);
  }

  async function loadGuests() {
    const query = new URLSearchParams({ page: "1", per_page: "100" });
    if (search.trim()) {
      query.set("search", search.trim());
    }
    if (vipOnly) {
      query.set("vip", "true");
    }

    await guestsState.run(async () => {
      const response = await guestsApi.list(eventId, `?${query.toString()}`);
      return response.items;
    });
  }

  useEffect(() => {
    void loadData();
  }, [eventId]);

  async function handleSubmit(payload: GuestCreateRequest | GuestUpdateRequest) {
    setIsSubmitting(true);
    try {
      if (editingGuest) {
        await guestsApi.update(eventId, editingGuest.id, payload as GuestUpdateRequest);
      } else {
        await guestsApi.create(eventId, payload as GuestCreateRequest);
      }
      setDialogOpen(false);
      setEditingGuest(null);
      await loadGuests();
    } finally {
      setIsSubmitting(false);
    }
  }

  async function handleDelete() {
    if (!deleteTarget) {
      return;
    }

    setIsSubmitting(true);
    try {
      await guestsApi.remove(eventId, deleteTarget.id);
      setDeleteTarget(null);
      await loadGuests();
    } finally {
      setIsSubmitting(false);
    }
  }

  async function handleImport(file: File) {
    setIsSubmitting(true);
    try {
      const summary = await guestsApi.importCsv(eventId, file);
      setImportSummary(summary);
      await loadGuests();
    } finally {
      setIsSubmitting(false);
    }
  }

  const guests = useMemo(() => guestsState.data ?? [], [guestsState.data]);

  return (
    <div className="space-y-6">
      <PageHeader
        eyebrow="Guests"
        title={eventState.data ? `${eventState.data.title} guests` : "Guest management"}
        description="Ведите список гостей, быстро находите нужных людей и импортируйте CSV без ручной рутины."
        action={
          <div className="flex flex-wrap gap-3">
            <Button variant="secondary" asChild>
              <Link href={`/events/${eventId}`}>Back to event</Link>
            </Button>
            <Button
              onClick={() => {
                setEditingGuest(null);
                setDialogOpen(true);
              }}
            >
              <Plus className="size-4" />
              Add guest
            </Button>
          </div>
        }
      />

      <div className="grid gap-4 xl:grid-cols-[1.3fr_0.7fr]">
        <div className="space-y-4">
          <div className="rounded-[1.5rem] border bg-white/92 px-5 py-5 shadow-[0_16px_34px_rgba(15,23,42,0.04)]">
            <div className="grid gap-3 md:grid-cols-[1fr_auto_auto]">
              <Input
                value={search}
                onChange={(event) => setSearch(event.target.value)}
                placeholder="Search by guest name"
              />
              <label className="flex items-center gap-3 rounded-full border px-4 py-2 text-sm">
                <input
                  type="checkbox"
                  checked={vipOnly}
                  onChange={(event) => setVipOnly(event.target.checked)}
                />
                VIP only
              </label>
              <Button variant="secondary" onClick={() => void loadGuests()}>
                Apply filters
              </Button>
            </div>
          </div>

          {eventState.isLoading || guestsState.isLoading ? <LoadingState label="Loading guests..." /> : null}

          {eventState.error || guestsState.error ? (
            <ErrorState
              title="Could not load guests"
              description={eventState.error ?? guestsState.error ?? "Unexpected error"}
              onRetry={() => {
                void loadData();
              }}
            />
          ) : null}

          {!eventState.isLoading && !guestsState.isLoading && !eventState.error && !guestsState.error && guests.length === 0 ? (
            <EmptyState
              icon={Users}
              title="No guests found"
              description="Add your first guest manually or import a CSV to start building the RSVP audience."
              action={
                <Button
                  onClick={() => {
                    setEditingGuest(null);
                    setDialogOpen(true);
                  }}
                >
                  <Plus className="size-4" />
                  Add first guest
                </Button>
              }
            />
          ) : null}

          {guests.length > 0 ? (
            <div className="grid gap-4 md:grid-cols-3">
              <InfoCard label="Guests" value={String(guests.length)} />
              <InfoCard label="VIP" value={String(guests.filter((guest) => guest.vip).length)} />
              <InfoCard
                label="Groups"
                value={String(new Set(guests.map((guest) => guest.group_name).filter(Boolean)).size)}
              />
            </div>
          ) : null}

          {guests.length > 0 ? (
            <GuestTable
              guests={guests}
              onEdit={(guest) => {
                setEditingGuest(guest);
                setDialogOpen(true);
              }}
              onDelete={(guest) => setDeleteTarget(guest)}
            />
          ) : null}
        </div>

        <CsvImportPanel
          isSubmitting={isSubmitting}
          summary={importSummary}
          onImport={(file) => {
            void handleImport(file);
          }}
        />
      </div>

      <GuestFormDialog
        open={dialogOpen}
        guest={editingGuest}
        isSubmitting={isSubmitting}
        onClose={() => {
          setDialogOpen(false);
          setEditingGuest(null);
        }}
        onSubmit={handleSubmit}
      />

      <ConfirmDialog
        open={Boolean(deleteTarget)}
        title="Delete guest?"
        description={`This will remove ${deleteTarget?.full_name ?? "this guest"} from the event guest list.`}
        confirmLabel="Delete"
        tone="danger"
        isLoading={isSubmitting}
        onConfirm={() => {
          void handleDelete();
        }}
        onCancel={() => setDeleteTarget(null)}
      />
    </div>
  );
}
