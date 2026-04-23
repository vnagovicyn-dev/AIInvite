import { HeartHandshake } from "lucide-react";

import { RsvpPublicForm } from "@/components/public/rsvp-public-form";
import { EmptyState } from "@/components/states/empty-state";
import { ApiClientError } from "@/lib/api/client";
import { publicApi } from "@/lib/api/public";

export default async function InviteRsvpPage({
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

    return (
      <div className="mx-auto w-full max-w-3xl space-y-6 pb-10">
        <section className="rounded-[2rem] border border-white/75 bg-white/90 px-6 py-8 text-center shadow-soft">
          <p className="text-sm font-semibold uppercase tracking-[0.18em] text-muted-foreground">
            RSVP
          </p>
          <h1 className="font-[family-name:var(--font-heading)] text-4xl">{invitePage.title}</h1>
          <p className="mx-auto max-w-2xl text-sm leading-6 text-muted-foreground">
            {invitePage.guest
              ? `Форма ответа для ${invitePage.guest.full_name}`
              : "Публичная форма подтверждения участия"}
          </p>
        </section>
        <RsvpPublicForm slug={slug} invitePage={invitePage} />
      </div>
    );
  } catch (error) {
    if (error instanceof ApiClientError && error.status === 404) {
      return (
        <EmptyState
          icon={HeartHandshake}
          title="RSVP недоступен"
          description="Событие не опубликовано, не найдено или форма ответа уже закрыта."
        />
      );
    }

    throw error;
  }
}
