import Link from "next/link";
import { ArrowRight, CalendarDays, Sparkles, Users } from "lucide-react";

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
      <div className="relative grid gap-8 lg:grid-cols-[1.1fr_0.9fr] lg:items-end">
        <div className="space-y-6">
          <p className="inline-flex rounded-full bg-secondary px-4 py-1 text-sm font-semibold text-secondary-foreground">
            Приглашения, RSVP и гости в одном сервисе
          </p>
          <div className="space-y-4">
            <h1 className="max-w-3xl font-[family-name:var(--font-heading)] text-4xl leading-tight sm:text-5xl lg:text-6xl">
              Соберите красивую страницу события и управляйте приглашёнными без лишней
              операционки.
            </h1>
            <p className="max-w-2xl text-lg text-muted-foreground">
              Выбирайте шаблон, настраивайте секции, ведите список гостей и принимайте RSVP в
              аккуратном SaaS-интерфейсе.
            </p>
          </div>
          <div className="flex flex-wrap gap-3">
            <Button asChild size="lg">
              <Link href="/register">
                Создать событие
                <ArrowRight className="size-4" />
              </Link>
            </Button>
            <Button asChild variant="secondary" size="lg">
              <Link href="/templates">Посмотреть шаблоны</Link>
            </Button>
          </div>
          <div className="grid gap-3 pt-2 sm:grid-cols-3">
            <MiniMetric label="Setup" value="Templates + sections" />
            <MiniMetric label="Guests" value="CSV + tags + VIP" />
            <MiniMetric label="Publish" value="Public slug + RSVP" />
          </div>
        </div>

        <Card className="border-white/80 bg-card/95">
          <CardHeader>
            <CardTitle>Что уже готово</CardTitle>
          </CardHeader>
          <CardContent className="grid gap-3 text-sm text-muted-foreground">
            <StatLine icon={CalendarDays} label="События и страницы" value="Private cabinet" />
            <StatLine icon={Users} label="Гости и RSVP" value="Public + private flow" />
            <StatLine icon={Sparkles} label="Шаблоны в каталоге" value={`${templatesCount}+`} />
          </CardContent>
        </Card>
      </div>
    </section>
  );
}

function MiniMetric({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-[1.25rem] border border-white/65 bg-white/72 px-4 py-4">
      <div className="text-[11px] font-semibold uppercase tracking-[0.16em] text-muted-foreground">
        {label}
      </div>
      <div className="mt-2 text-sm font-semibold text-foreground">{value}</div>
    </div>
  );
}

function StatLine({
  icon: Icon,
  label,
  value
}: {
  icon: React.ComponentType<{ className?: string }>;
  label: string;
  value: string;
}) {
  return (
    <div className="flex items-center justify-between rounded-2xl bg-secondary/55 px-4 py-3">
      <div className="flex items-center gap-3">
        <div className="inline-flex size-9 items-center justify-center rounded-2xl bg-background text-foreground">
          <Icon className="size-4" />
        </div>
        <span>{label}</span>
      </div>
      <span className="font-semibold text-foreground">{value}</span>
    </div>
  );
}
