import Link from "next/link";
import { ArrowRight, CalendarCheck2, ListChecks, MapPinned, Users } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export function LandingHero({
  templatesCount
}: {
  templatesCount: number;
}) {
  return (
    <section className="homepage-surface relative overflow-hidden rounded-[2.3rem] border px-8 py-9 shadow-soft sm:px-12 sm:py-12 lg:px-14 lg:py-14">
      <div className="absolute inset-0 bg-hero-grid bg-[size:28px_28px] opacity-[0.12]" />
      <div className="absolute right-12 top-10 h-32 w-32 rounded-full bg-[rgba(99,102,241,0.12)] blur-3xl" />
      <div className="absolute bottom-6 left-10 h-24 w-24 rounded-full bg-[rgba(242,154,122,0.12)] blur-3xl" />
      <div className="relative grid gap-8 lg:grid-cols-[1.08fr_0.92fr] lg:items-center">
        <div className="space-y-7">
          <p className="homepage-warm-chip inline-flex rounded-full px-4 py-1.5 text-[13px] font-semibold">
            Онлайн-приглашение, гости и ответы в одном месте
          </p>
          <div className="space-y-5">
            <h1 className="max-w-4xl font-[family-name:var(--font-heading)] text-[3.45rem] font-extrabold leading-[1.02] tracking-[-0.045em] sm:text-[3.8rem] lg:text-[4rem]">
              Создавайте стильные онлайн-приглашения и собирайте ответы гостей в одном сервисе
            </h1>
            <p className="max-w-2xl text-[1.14rem] font-medium leading-8 text-muted-foreground sm:text-[1.2rem]">
              AI Invite помогает быстро собрать страницу события, добавить гостей, включить
              подтверждение участия и опубликовать приглашение по ссылке.
            </p>
          </div>
          <div className="flex flex-wrap gap-3">
            <Button asChild size="lg" className="homepage-button-label homepage-primary-button">
              <Link href="/register">
                Создать приглашение
                <ArrowRight className="size-4" />
              </Link>
            </Button>
            <Button asChild variant="secondary" size="lg" className="homepage-button-label homepage-secondary-button">
              <Link href="/templates">Посмотреть шаблоны</Link>
            </Button>
          </div>

          <div className="grid gap-3 pt-2 sm:grid-cols-2 xl:grid-cols-4">
            <HeroChip label="Выберите шаблон" />
            <HeroChip label="Настройте страницу события" />
            <HeroChip label="Добавьте гостей" />
            <HeroChip label="Собирайте подтверждения участия" />
          </div>
        </div>

        <Card className="homepage-surface-lilac overflow-hidden border">
          <CardHeader className="space-y-3 border-b border-[rgba(99,102,241,0.14)] bg-white/72">
            <div className="flex items-center justify-between gap-4">
              <div>
                <div className="text-xs font-semibold uppercase tracking-[0.18em] text-muted-foreground">
                  Пример рабочего пространства
                </div>
                <CardTitle className="mt-2">Организатор видит всё в одном кабинете</CardTitle>
              </div>
              <div className="rounded-full bg-[rgba(99,102,241,0.14)] px-3 py-1.5 text-[12px] font-semibold text-[var(--homepage-accent)]">
                {templatesCount}+ шаблонов
              </div>
            </div>
          </CardHeader>
          <CardContent className="grid gap-4 pt-6 text-[15px] text-muted-foreground">
            <div className="grid gap-3 sm:grid-cols-2">
              <PreviewMetric
                icon={CalendarCheck2}
                title="Страница события"
                value="Дата, место, программа и детали для гостей"
              />
              <PreviewMetric
                icon={Users}
                title="Список гостей"
                value="Группы, VIP-гости и важные пометки"
              />
            </div>

            <div className="rounded-[1.6rem] border border-[rgba(99,102,241,0.14)] bg-white/72 p-4">
              <div className="mb-3 flex items-center justify-between">
                <div className="text-sm font-semibold text-foreground">Ответы гостей</div>
                <div className="rounded-full bg-[rgba(242,154,122,0.12)] px-3 py-1 text-xs font-semibold text-[#c86f4e]">
                  Обновляется автоматически
                </div>
              </div>
              <div className="grid gap-3 sm:grid-cols-3">
                <ResponseStat label="Подтвердили" value="24" tone="bg-[rgba(99,102,241,0.12)]" />
                <ResponseStat label="Пока думают" value="6" tone="bg-[var(--homepage-soft-lilac)]" />
                <ResponseStat label="Не смогут" value="3" tone="bg-[var(--homepage-soft-warm)]" />
              </div>
            </div>

            <div className="rounded-[1.6rem] border border-[var(--homepage-border)] bg-white p-4">
              <div className="mb-3 flex items-center gap-2 font-semibold text-foreground">
                <ListChecks className="size-4 text-muted-foreground" />
                Что видит организатор
              </div>
              <div className="grid gap-3">
                <PreviewRow
                  label="Опубликовано"
                  value="Ссылка готова для отправки гостям"
                />
                <PreviewRow
                  label="Подтверждение участия"
                  value="Форма уже подключена к приглашению"
                />
                <PreviewRow
                  label="Место проведения"
                  value="Grand Hall, Амстердам"
                  icon={MapPinned}
                />
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    </section>
  );
}

function HeroChip({ label }: { label: string }) {
  return (
    <div className="rounded-[1.25rem] border border-[rgba(242,154,122,0.22)] bg-[rgba(242,154,122,0.08)] px-4 py-4 text-[15px] font-semibold leading-6 text-[var(--homepage-text)]">
      {label}
    </div>
  );
}

function PreviewMetric({
  icon: Icon,
  title,
  value
}: {
  icon: React.ComponentType<{ className?: string }>;
  title: string;
  value: string;
}) {
  return (
    <div className="rounded-[1.5rem] border border-border/70 bg-white/90 p-4">
      <div className="mb-3 inline-flex size-10 items-center justify-center rounded-2xl bg-secondary text-foreground">
        <Icon className="size-4" />
      </div>
      <div className="text-[15px] font-semibold text-foreground">{title}</div>
      <div className="mt-1 text-[15px] leading-7 text-muted-foreground">{value}</div>
    </div>
  );
}

function ResponseStat({
  label,
  value,
  tone
}: {
  label: string;
  value: string;
  tone: string;
}) {
  return (
    <div className={`rounded-[1.25rem] px-4 py-4 ${tone}`}>
      <div className="text-[11px] font-semibold uppercase tracking-[0.16em] text-muted-foreground">
        {label}
      </div>
      <div className="mt-2 text-[1.9rem] font-bold leading-none tracking-[-0.03em] text-foreground">{value}</div>
    </div>
  );
}

function PreviewRow({
  label,
  value,
  icon: Icon
}: {
  label: string;
  value: string;
  icon?: React.ComponentType<{ className?: string }>;
}) {
  return (
    <div className="flex items-start justify-between gap-4 rounded-[1.25rem] bg-secondary/38 px-4 py-3">
      <div className="flex items-center gap-3">
        {Icon ? (
          <div className="inline-flex size-8 items-center justify-center rounded-2xl bg-white text-foreground">
            <Icon className="size-4" />
          </div>
        ) : null}
        <span className="text-[15px] font-semibold text-foreground">{label}</span>
      </div>
      <span className="max-w-[220px] text-right text-[15px] leading-7 text-muted-foreground">
        {value}
      </span>
    </div>
  );
}
