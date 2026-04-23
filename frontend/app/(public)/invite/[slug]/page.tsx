import { ApiClientError } from "@/lib/api/client";
import { publicApi } from "@/lib/api/public";
import { PublicSectionRenderer } from "@/components/public/public-section-renderer";
import { EmptyState } from "@/components/states/empty-state";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Eye } from "lucide-react";
import Link from "next/link";

export default async function InvitePage({
  params,
  searchParams
}: {
  params: Promise<{ slug: string }>;
  searchParams: Promise<{ invite_token?: string }>;
}) {
  const { slug } = await params;
  const { invite_token: inviteToken } = await searchParams;

  try {
    const invitePage = await publicApi.getInvitePage(slug, inviteToken);
    const sections = [...invitePage.sections].sort((left, right) => left.position - right.position);
    const rsvpHref = inviteToken
      ? `/invite/${slug}/rsvp?invite_token=${encodeURIComponent(inviteToken)}`
      : `/invite/${slug}/rsvp`;
    const hasRsvpSection = sections.some((section) => section.section_type === "rsvp");

    return (
      <div className="mx-auto w-full max-w-4xl space-y-8 pb-10">
        <section className="rounded-[2.25rem] border border-white/75 bg-white/92 p-8 text-center shadow-soft sm:p-12">
          <div className="mx-auto max-w-3xl space-y-4">
            <div className="flex flex-wrap items-center justify-center gap-2">
              <Badge variant="success">{invitePage.status}</Badge>
              <Badge variant="outline">{invitePage.event_type}</Badge>
            </div>
            <h1 className="font-[family-name:var(--font-heading)] text-4xl leading-tight sm:text-5xl">
              {invitePage.title}
            </h1>
            <div className="flex flex-wrap items-center justify-center gap-3 text-sm text-muted-foreground">
              {invitePage.event_date ? (
                <span>{new Date(invitePage.event_date).toLocaleString("ru-RU")}</span>
              ) : null}
              <span>{invitePage.timezone}</span>
              {invitePage.venue_name || invitePage.venue_address ? (
                <span>{invitePage.venue_name ?? invitePage.venue_address}</span>
              ) : null}
            </div>
            {invitePage.guest ? (
              <div className="inline-flex rounded-full bg-secondary px-4 py-2 text-sm text-secondary-foreground">
                Приглашение для {invitePage.guest.full_name}
              </div>
            ) : null}
          </div>
        </section>

        {sections.map((section) => (
          <PublicSectionRenderer
            key={section.id}
            section={section}
            eventTitle={invitePage.title}
            venueName={invitePage.venue_name}
            venueAddress={invitePage.venue_address}
            rsvpHref={rsvpHref}
          />
        ))}

        {!hasRsvpSection && invitePage.rsvp.questions.length > 0 ? (
          <Card className="border-accent/40 bg-gradient-to-br from-accent/45 via-white to-secondary/55">
            <CardContent className="flex flex-col gap-4 px-6 py-6 sm:flex-row sm:items-center sm:justify-between">
              <div>
                <div className="text-lg font-semibold">Подтверждение участия</div>
                <p className="mt-1 text-sm text-muted-foreground">
                  Откройте форму RSVP и отправьте ответ организатору.
                </p>
              </div>
              <Button asChild>
                <Link href={rsvpHref}>Перейти к RSVP</Link>
              </Button>
            </CardContent>
          </Card>
        ) : null}
      </div>
    );
  } catch (error) {
    if (error instanceof ApiClientError && error.status === 404) {
      return (
        <div className="space-y-6">
          <EmptyState
            icon={Eye}
            title="Приглашение не найдено"
            description="Возможно, событие ещё не опубликовано или ссылка уже недействительна."
          />
        </div>
      );
    }

    throw error;
  }
}
