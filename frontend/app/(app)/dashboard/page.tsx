"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { CalendarRange, Filter, Mailbox, PanelsTopLeft, Plus } from "lucide-react";

import { EventCard } from "@/components/cabinet/event-card";
import { PageHeader } from "@/components/layout/page-header";
import { EmptyState } from "@/components/states/empty-state";
import { ErrorState } from "@/components/states/error-state";
import { LoadingState } from "@/components/states/loading-state";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { useApiState } from "@/hooks/use-api-state";
import { useAuth } from "@/hooks/use-auth";
import { authApi } from "@/lib/api/auth";
import { eventsApi } from "@/lib/api/events";
import { EVENT_STATUS_OPTIONS } from "@/lib/constants";
import type { EventsResponse } from "@/lib/types";

const cards = [
  {
    title: "Events",
    description: "List and manage owned events from the authenticated workspace.",
    icon: CalendarRange
  },
  {
    title: "Guests",
    description: "Keep invitees, tags, VIP markers and CSV imports in one place.",
    icon: Mailbox
  },
  {
    title: "Page builder",
    description: "Compose public invitation pages from sections and RSVP blocks.",
    icon: PanelsTopLeft
  }
];

export default function DashboardPage() {
  const { user, setUser } = useAuth();
  const profileState = useApiState(user);
  const eventsState = useApiState<EventsResponse | null>(null);
  const [status, setStatus] = useState<(typeof EVENT_STATUS_OPTIONS)[number]>("all");

  async function loadEvents(nextStatus: (typeof EVENT_STATUS_OPTIONS)[number]) {
    const query = new URLSearchParams({ page: "1", per_page: "12" });
    if (nextStatus !== "all") {
      query.set("status", nextStatus);
    }

    await eventsState.run(async () => eventsApi.list(`?${query.toString()}`));
  }

  async function loadDashboard() {
    await profileState.run(async () => {
      const nextUser = await authApi.me();
      setUser(nextUser);
      return nextUser;
    });
    await loadEvents(status);
  }

  useEffect(() => {
    void loadDashboard();
  }, []);

  useEffect(() => {
    void loadEvents(status);
  }, [status]);

  const latestEvents = eventsState.data?.items ?? [];

  return (
    <div className="space-y-6">
      <PageHeader
        eyebrow="Workspace"
        title="Dashboard"
        description="Смотрите активные события, быстро переходите в нужный editor flow и держите publish status под контролем."
        action={
          <Button asChild>
            <Link href="/events/new">
              <Plus className="size-4" />
              New event
            </Link>
          </Button>
        }
      />
      {profileState.data ? (
        <Card className="border-white/75 bg-white/94">
          <CardHeader>
            <CardTitle>
              Hello, {profileState.data.full_name ?? profileState.data.email}
            </CardTitle>
          </CardHeader>
          <CardContent className="text-sm text-muted-foreground">
            Ваш кабинет готов к работе: создавайте события, собирайте приглашения и отслеживайте ответы гостей.
          </CardContent>
        </Card>
      ) : null}
      <Card className="border-white/75 bg-white/94">
        <CardContent className="flex flex-col gap-3 pt-6 md:flex-row md:items-center md:justify-between">
          <div className="flex items-center gap-2 text-sm text-muted-foreground">
            <Filter className="size-4" />
            Filter events by status
          </div>
          <div className="flex flex-wrap gap-2">
            {EVENT_STATUS_OPTIONS.map((option) => (
              <Button
                key={option}
                variant={status === option ? "default" : "secondary"}
                size="sm"
                onClick={() => setStatus(option)}
              >
                {option}
              </Button>
            ))}
          </div>
        </CardContent>
      </Card>
      <div className="grid gap-4 md:grid-cols-3">
        {cards.map(({ title, description, icon: Icon }) => (
          <Card key={title} className="border-white/75 bg-white/94">
            <CardHeader>
              <div className="mb-4 inline-flex size-10 items-center justify-center rounded-2xl bg-secondary text-secondary-foreground">
                <Icon className="size-5" />
              </div>
              <CardTitle>{title}</CardTitle>
            </CardHeader>
            <CardContent className="text-sm text-muted-foreground">{description}</CardContent>
          </Card>
        ))}
      </div>
      {eventsState.isLoading ? <LoadingState label="Loading your latest events..." /> : null}
      {eventsState.error ? (
        <ErrorState
          title="Could not load dashboard data"
          description={eventsState.error}
          onRetry={() => {
            void loadDashboard();
          }}
        />
      ) : null}
      {!eventsState.isLoading && !eventsState.error && latestEvents.length === 0 ? (
        <EmptyState
          icon={CalendarRange}
          title="No events yet"
          description="Create your first event to start building pages, guests and RSVP flows."
          action={
            <Button asChild>
              <Link href="/events/new">Create first event</Link>
            </Button>
          }
        />
      ) : null}
      {!eventsState.isLoading && !eventsState.error && latestEvents.length > 0 ? (
        <div className="grid gap-4 lg:grid-cols-[1.35fr_0.65fr]">
          <div className="grid gap-4 md:grid-cols-2">
            {latestEvents.map((event) => (
              <EventCard key={event.id} event={event} />
            ))}
          </div>
          <Card className="border-white/75 bg-white/94">
            <CardHeader>
              <CardTitle>Workspace stats</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4 text-sm">
              <StatLine label="Total events" value={String(eventsState.data?.total ?? 0)} />
              <StatLine
                label="Published"
                value={String(latestEvents.filter((event) => event.status === "published").length)}
              />
              <StatLine
                label="Draft"
                value={String(latestEvents.filter((event) => event.status === "draft").length)}
              />
            </CardContent>
          </Card>
        </div>
      ) : null}
    </div>
  );
}

function StatLine({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex items-center justify-between rounded-2xl bg-secondary/50 px-4 py-3">
      <span className="text-muted-foreground">{label}</span>
      <span className="font-semibold text-foreground">{value}</span>
    </div>
  );
}
