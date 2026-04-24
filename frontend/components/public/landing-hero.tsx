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
    <section className="relative overflow-hidden rounded-[2.3rem] border border-white/70 bg-white/92 p-8 shadow-soft backdrop-blur sm:p-12 lg:p-14">
      <div className="absolute inset-0 bg-hero-grid bg-[size:28px_28px] opacity-35" />
      <div className="absolute right-0 top-0 h-48 w-48 rounded-full bg-accent/35 blur-3xl" />
      <div className="relative grid gap-8 lg:grid-cols-[1.08fr_0.92fr] lg:items-center">
        <div className="space-y-6">
          <p className="inline-flex rounded-full bg-secondary px-4 py-1 text-sm font-semibold text-secondary-foreground">
            Онлайн-приглашение, гости и ответы в одном месте
          </p>
          <div className="space-y-4">
            <h1 className="max-w-3xl font-[family-name:var(--font-heading)] text-4xl leading-tight sm:text-5xl lg:text-6xl">
              Создавайте стильные онлайн-приглашения и собирайте ответы гостей в одном сервисе
            </h1>
            <p className="max-w-2xl text-lg text-muted-foreground">
              AI Invite помогает быстро собрать страницу события, добавить гостей, включить
              подтверждение участия и опубликовать приглашение по ссылке.
            </p>
          </div>
          <div className="flex flex-wrap gap-3">
            <Button asChild size="lg">
              <Link href="/register">
                Создать приглашение
                <ArrowRight className="size-4" />
              </Link>
            </Button>
            <Button asChild variant="secondary" size="lg">
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

        <Card className="overflow-hidden border-white/80 bg-card/95">
          <CardHeader className="space-y-3 border-b border-border/60 bg-white/70">
            <div className="flex items-center justify-between gap-4">
              <div>
                <div className="text-xs font-semibold uppercase tracking-[0.18em] text-muted-foreground">
                  Пример рабочего пространства
                </div>
                <CardTitle className="mt-2">Организатор видит всё в одном кабинете</CardTitle>
              </div>
              <div className="rounded-full bg-accent px-3 py-1 text-xs font-semibold text-accent-foreground">
                {templatesCount}+ шаблонов
              </div>
            </div>
          </CardHeader>
          <CardContent className="grid gap-4 pt-6 text-sm text-muted-foreground">
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

            <div className="rounded-[1.6rem] border border-border/70 bg-secondary/35 p-4">
              <div className="mb-3 flex items-center justify-between">
                <div className="text-sm font-semibold text-foreground">Ответы гостей</div>
                <div className="rounded-full bg-white/80 px-3 py-1 text-xs font-semibold text-muted-foreground">
                  Обновляется автоматически
                </div>
              </div>
              <div className="grid gap-3 sm:grid-cols-3">
                <ResponseStat label="Подтвердили" value="24" tone="bg-accent/60" />
                <ResponseStat label="Пока думают" value="6" tone="bg-secondary/90" />
                <ResponseStat label="Не смогут" value="3" tone="bg-white/90" />
              </div>
            </div>

            <div className="rounded-[1.6rem] border border-border/70 bg-white/88 p-4">
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
    <div className="rounded-[1.25rem] border border-white/65 bg-white/72 px-4 py-4 text-sm font-semibold text-foreground">
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
      <div className="text-sm font-semibold text-foreground">{title}</div>
      <div className="mt-1 text-sm leading-6 text-muted-foreground">{value}</div>
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
      <div className="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">
        {label}
      </div>
      <div className="mt-2 text-2xl font-semibold text-foreground">{value}</div>
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
        <span className="font-medium text-foreground">{label}</span>
      </div>
      <span className="max-w-[220px] text-right text-sm leading-6 text-muted-foreground">
        {value}
      </span>
    </div>
  );
}
