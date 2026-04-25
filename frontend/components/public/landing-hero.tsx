import Image from "next/image";
import Link from "next/link";
import {
  ArrowRight,
  CheckCircle2,
  Clock3,
  Users,
  XCircle
} from "lucide-react";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export function LandingHero({
  templatesCount
}: {
  templatesCount: number;
}) {
  return (
    <section className="homepage-surface relative overflow-hidden rounded-[2.5rem] border px-8 py-9 shadow-soft sm:px-12 sm:py-12 lg:px-14 lg:py-14">
      <div className="absolute inset-0 bg-hero-grid bg-[size:28px_28px] opacity-[0.12]" />
      <div className="absolute right-12 top-10 h-32 w-32 rounded-full bg-[rgba(99,102,241,0.12)] blur-3xl" />
      <div className="absolute bottom-6 left-10 h-24 w-24 rounded-full bg-[rgba(242,154,122,0.12)] blur-3xl" />
      <div className="relative grid gap-10 lg:grid-cols-[1.04fr_0.96fr] lg:items-center">
        <div className="space-y-8">
          <p className="homepage-warm-chip inline-flex rounded-full px-4 py-1.5 text-[13px] font-semibold">
            Онлайн-приглашения для важных событий
          </p>
          <div className="max-w-[40rem] space-y-6">
            <h1 className="max-w-[14ch] font-[family-name:var(--font-heading)] text-[2.15rem] font-extrabold leading-[1.04] tracking-[-0.04em] sm:text-[2.75rem] lg:text-[3.3rem]">
              Создайте красивое приглашение и собирайте ответы гостей в одном сервисе
            </h1>
            <p className="max-w-[36rem] text-[1.04rem] font-medium leading-8 text-muted-foreground sm:text-[1.12rem] sm:leading-8">
              Выберите шаблон, настройте страницу события, добавьте гостей и откройте
              подтверждение участия по ссылке — без таблиц, переписок и лишней операционки.
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
            <HeroChip label="Получайте ответы гостей" />
          </div>
        </div>

        <div className="relative lg:pl-6">
          <div className="pointer-events-none absolute -left-2 top-10 hidden h-24 w-24 rounded-full bg-[rgba(99,102,241,0.12)] blur-2xl lg:block" />
          <div className="pointer-events-none absolute right-4 top-2 hidden h-20 w-20 rounded-full bg-[rgba(242,154,122,0.14)] blur-2xl lg:block" />

          <Card className="homepage-surface relative overflow-hidden border shadow-[0_28px_70px_rgba(17,24,39,0.08)]">
            <div className="relative aspect-[4/4.6] overflow-hidden sm:aspect-[4/4.2] lg:aspect-[4/4.45]">
              <Image
                src={HERO_IMAGE_SRC}
                alt="Пример страницы онлайн-приглашения AI Invite"
                fill
                priority
                sizes="(max-width: 1024px) 100vw, 42vw"
                className="object-cover object-top"
              />
              <div className="absolute inset-0 bg-[linear-gradient(180deg,rgba(252,251,248,0.02)_0%,rgba(252,251,248,0.08)_30%,rgba(252,251,248,0.78)_100%)]" />
              <div className="absolute inset-x-4 top-4 flex items-start justify-between gap-4 sm:inset-x-5 sm:top-5">
                <div className="rounded-full bg-white/92 px-4 py-2 text-[11px] font-semibold uppercase tracking-[0.16em] text-[var(--homepage-accent)] shadow-[0_10px_30px_rgba(17,24,39,0.08)] ring-1 ring-white/80 backdrop-blur">
                  Страница приглашения
                </div>
                <div className="hidden rounded-full bg-[rgba(255,255,255,0.92)] px-3 py-1.5 text-[12px] font-semibold text-muted-foreground shadow-[0_10px_20px_rgba(17,24,39,0.06)] ring-1 ring-white/80 sm:block">
                  {templatesCount}+ готовых стилей
                </div>
              </div>

              <div className="absolute inset-x-4 bottom-4 rounded-[1.7rem] bg-white/88 p-4 shadow-[0_18px_44px_rgba(17,24,39,0.08)] ring-1 ring-white/75 backdrop-blur sm:inset-x-5 sm:bottom-5 sm:p-5">
                <div className="text-[11px] font-semibold uppercase tracking-[0.16em] text-muted-foreground">
                  Готовое приглашение
                </div>
                <CardTitle className="mt-2 text-[1.4rem] sm:text-[1.55rem]">
                  Свадьба Анны и Игоря
                </CardTitle>
                <div className="mt-3 flex flex-wrap gap-2 text-[13px] text-muted-foreground">
                  <span className="rounded-full bg-[var(--homepage-soft-lilac)] px-3 py-1.5">
                    20 августа 2026
                  </span>
                  <span className="rounded-full bg-[var(--homepage-soft-warm)] px-3 py-1.5">
                    Grand Hall, Амстердам
                  </span>
                  <span className="rounded-full bg-white px-3 py-1.5 ring-1 ring-[var(--homepage-border)]">
                    Подтверждение участия подключено
                  </span>
                </div>
              </div>
            </div>
          </Card>

          <Card className="homepage-surface-warm relative mt-4 overflow-hidden border shadow-[0_20px_44px_rgba(17,24,39,0.08)] lg:absolute lg:-right-5 lg:top-12 lg:mt-0 lg:w-[18.5rem]">
            <CardHeader className="space-y-2 pb-3">
              <div className="flex items-center justify-between gap-3">
                <div className="text-[15px] font-semibold text-foreground">Ответы гостей</div>
              </div>
            </CardHeader>
            <CardContent className="space-y-3 pt-0">
              <ResponseRow
                icon={CheckCircle2}
                label="Подтвердили"
                value="24"
                tone="bg-[rgba(99,102,241,0.12)] text-[var(--homepage-accent)]"
              />
              <ResponseRow
                icon={Clock3}
                label="Пока не ответили"
                value="6"
                tone="bg-white text-[#7c5d48]"
              />
              <ResponseRow
                icon={XCircle}
                label="Не смогут"
                value="3"
                tone="bg-[rgba(242,154,122,0.16)] text-[#c86f4e]"
              />
            </CardContent>
          </Card>

          <Card className="homepage-surface relative mt-4 overflow-hidden border shadow-[0_18px_40px_rgba(17,24,39,0.06)] lg:absolute lg:-left-1 lg:bottom-8 lg:mt-0 lg:w-[15.5rem]">
            <CardContent className="space-y-4 p-5">
              <div className="flex items-center gap-2 text-[15px] font-semibold text-foreground">
                <Users className="size-4 text-[var(--homepage-accent)]" />
                Гости
              </div>
              <div className="space-y-3">
                <AudienceRow label="Семья" value="12 гостей" />
                <AudienceRow label="Друзья" value="18 гостей" />
                <AudienceRow label="VIP" value="4 гостя" />
              </div>
            </CardContent>
          </Card>
        </div>
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

function ResponseRow({
  icon: Icon,
  label,
  value,
  tone
}: {
  icon: React.ComponentType<{ className?: string }>;
  label: string;
  value: string;
  tone: string;
}) {
  return (
    <div className="flex items-center justify-between rounded-[1.2rem] bg-white/72 px-4 py-3">
      <div className="flex items-center gap-3">
        <div className={`inline-flex size-9 items-center justify-center rounded-2xl ${tone}`}>
          <Icon className="size-4" />
        </div>
        <div>
          <div className="text-[15px] font-semibold text-foreground">{label}</div>
          <div className="mt-1 text-[13px] font-medium text-muted-foreground">{value} гостей</div>
        </div>
      </div>
      <div className="size-2.5 rounded-full bg-[rgba(99,102,241,0.45)]" />
    </div>
  );
}

function AudienceRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex items-center justify-between rounded-[1.25rem] bg-[var(--homepage-soft-lilac)] px-4 py-3">
      <span className="text-[15px] font-semibold text-foreground">{label}</span>
      <span className="text-[14px] font-medium text-muted-foreground">{value}</span>
    </div>
  );
}

const HERO_IMAGE_SRC = "/images/home/hero-invite-platform-premium.webp.png";
