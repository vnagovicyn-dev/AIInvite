import Image from "next/image";
import Link from "next/link";
import {
  CalendarDays,
  CheckCircle2,
  ChevronRight,
  HeartHandshake,
  Link2,
  ShieldCheck,
  Smartphone,
  Users
} from "lucide-react";

import { FeatureGrid } from "@/components/public/feature-grid";
import { LandingHero } from "@/components/public/landing-hero";
import { TemplateGrid } from "@/components/public/template-grid";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { templatesApi } from "@/lib/api/templates";

export const dynamic = "force-dynamic";

export default async function HomePage() {
  const templates = await templatesApi.list("?page=1&per_page=6&is_active=true");

  return (
    <div className="space-y-24 pb-12">
      <LandingHero templatesCount={templates.total} />

      <section className="space-y-6" id="events">
        <div className="space-y-3">
          <p className="homepage-section-kicker">
            Форматы событий
          </p>
          <h2 className="homepage-section-title">
            Подходит для самых важных событий
          </h2>
          <p className="homepage-section-copy">
            Выберите формат, который подходит именно вам, и начните с готовой основы.
          </p>
        </div>
        <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
          {eventUseCases.map((item) => (
            <Link
              key={item.title}
              href={item.href}
              className="homepage-surface group rounded-[1.5rem] border p-5 shadow-soft transition hover:-translate-y-0.5 hover:border-[rgba(99,102,241,0.4)] hover:bg-white"
            >
              <div className="flex items-start justify-between gap-4">
                <div>
                  <div className="homepage-card-title text-[1.15rem]">{item.title}</div>
                  <p className="homepage-card-copy mt-2">{item.description}</p>
                </div>
                <ChevronRight className="mt-1 size-4 text-muted-foreground transition group-hover:text-foreground" />
              </div>
            </Link>
          ))}
        </div>
      </section>

      <section className="homepage-surface rounded-[2rem] border px-6 py-8 sm:px-8 sm:py-9" id="how-it-works">
        <div className="space-y-3">
          <p className="homepage-section-kicker">
            Как это работает
          </p>
          <h2 className="homepage-section-title">
            Создайте приглашение за 3 шага
          </h2>
        </div>
        <div className="grid gap-4 lg:grid-cols-3">
          {steps.map((step, index) => (
            <Card key={step.title} className="h-full border bg-white shadow-[0_14px_36px_rgba(17,24,39,0.05)]">
              <CardHeader className="space-y-4">
                <div className="inline-flex size-11 items-center justify-center rounded-2xl bg-[rgba(99,102,241,0.14)] text-[var(--homepage-accent)]">
                  <span className="text-lg font-semibold">{index + 1}</span>
                </div>
                <CardTitle className="homepage-card-title text-[1.2rem]">{step.title}</CardTitle>
              </CardHeader>
              <CardContent className="homepage-card-copy">
                {step.description}
              </CardContent>
            </Card>
          ))}
        </div>
      </section>

      <FeatureGrid />

      <section aria-label="Визуальный баннер AI Invite">
        <div className="homepage-surface relative overflow-hidden rounded-[2.2rem] border shadow-soft">
          <div className="relative aspect-[21/9] min-h-[16rem] sm:min-h-[19rem] lg:min-h-[23rem]">
            <Image
              src="/images/home/banner-invitation-experience.webp"
              alt="AI Invite помогает собрать красивое приглашение и ответы гостей в одном сервисе"
              fill
              sizes="100vw"
              className="object-cover object-center"
            />
            <div className="absolute inset-0 bg-[linear-gradient(90deg,rgba(17,24,39,0.18)_0%,rgba(17,24,39,0.04)_34%,rgba(17,24,39,0.08)_100%)]" />
            <div className="absolute inset-x-5 bottom-5 max-w-[34rem] rounded-[1.5rem] bg-white/84 px-5 py-4 shadow-[0_18px_44px_rgba(17,24,39,0.08)] ring-1 ring-white/70 backdrop-blur sm:inset-x-7 sm:bottom-7 sm:px-6 sm:py-5">
              <div className="homepage-section-kicker">AI Invite</div>
              <div className="mt-2 font-[family-name:var(--font-heading)] text-[1.55rem] font-bold leading-[1.1] tracking-[-0.03em] text-foreground sm:text-[1.9rem]">
                Приглашение, гости и ответы гостей складываются в один спокойный рабочий процесс
              </div>
            </div>
          </div>
        </div>
      </section>

      <section className="homepage-surface-lilac space-y-6 rounded-[2.2rem] border px-6 py-8 sm:px-8 sm:py-9" id="templates">
        <div className="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
          <div className="space-y-3">
            <p className="homepage-section-kicker">
              Шаблоны
            </p>
            <h2 className="homepage-section-title">
              Выберите шаблон для своего события
            </h2>
            <p className="homepage-section-copy">
              Начните с готового стиля и адаптируйте его под свой формат события.
            </p>
          </div>
          <Button asChild variant="secondary" className="homepage-button-label homepage-secondary-button">
            <Link href="/templates">Смотреть все шаблоны</Link>
          </Button>
        </div>
        <TemplateGrid templates={templates.items} compact />
      </section>

      <section className="grid gap-6 lg:grid-cols-[1.15fr_0.85fr]" id="demo">
        <Card className="homepage-surface-lilac overflow-hidden border shadow-soft">
          <CardHeader className="space-y-3">
            <div className="homepage-section-kicker">
              Демо
            </div>
            <CardTitle className="homepage-section-title">
              Так выглядит готовая страница приглашения
            </CardTitle>
            <p className="homepage-section-copy max-w-xl">
              Откройте демо и посмотрите, как гости увидят ваше приглашение.
            </p>
          </CardHeader>
          <CardContent className="grid gap-5" id="demo-preview">
            <div className="overflow-hidden rounded-[1.8rem] border border-[rgba(99,102,241,0.14)] bg-white">
              <div className="relative aspect-[16/10]">
                <Image
                  src="/images/home/demo-invite-showcase.webp"
                  alt="Пример готовой страницы приглашения AI Invite"
                  fill
                  sizes="(max-width: 1024px) 100vw, 52vw"
                  className="object-cover object-top"
                />
                <div className="absolute inset-0 bg-[linear-gradient(180deg,rgba(17,24,39,0.02)_0%,rgba(17,24,39,0.08)_50%,rgba(17,24,39,0.16)_100%)]" />
                <div className="absolute inset-x-4 bottom-4 rounded-[1.4rem] bg-white/88 px-4 py-3 shadow-[0_18px_44px_rgba(17,24,39,0.08)] ring-1 ring-white/75 backdrop-blur sm:inset-x-5 sm:bottom-5 sm:px-5 sm:py-4">
                  <div className="text-[11px] font-semibold uppercase tracking-[0.16em] text-muted-foreground">
                    Готовая страница
                  </div>
                  <div className="mt-2 font-[family-name:var(--font-heading)] text-[1.4rem] font-bold leading-[1.08] tracking-[-0.03em] text-foreground sm:text-[1.7rem]">
                    Свадьба Анны и Игоря
                  </div>
                  <div className="mt-3 flex flex-wrap gap-2 text-[13px] text-muted-foreground">
                    <span className="rounded-full bg-[var(--homepage-soft-lilac)] px-3 py-1.5">20 августа 2026</span>
                    <span className="rounded-full bg-[var(--homepage-soft-warm)] px-3 py-1.5">Амстердам</span>
                    <span className="rounded-full bg-white px-3 py-1.5 ring-1 ring-[var(--homepage-border)]">Grand Hall</span>
                  </div>
                </div>
              </div>
            </div>
            <div className="grid gap-3 md:grid-cols-2">
              <DemoLine
                icon={CalendarDays}
                title="Программа дня"
                text="Гости видят расписание, место проведения и важные детали."
              />
              <DemoLine
                icon={Users}
                title="Список гостей и ответы"
                text="Организатор следит за подтверждениями участия в кабинете."
              />
            </div>
          </CardContent>
        </Card>

        <Card className="homepage-surface-warm border shadow-soft">
          <CardContent className="flex h-full flex-col justify-between gap-7 px-6 py-6">
            <div className="space-y-4">
              <div className="homepage-warm-chip inline-flex rounded-full px-4 py-1 text-[15px] font-semibold">
                Демо-поток
              </div>
              <div className="space-y-3">
                <div className="font-[family-name:var(--font-heading)] text-[2rem] font-bold leading-[1.08] tracking-[-0.03em]">
                  От выбора шаблона до готовой ссылки для гостей
                </div>
                <p className="homepage-card-copy max-w-sm">
                  Один экран показывает готовую страницу, второй — как организатор видит ответы гостей.
                </p>
              </div>
            </div>
            <div className="grid gap-3">
              <CompactDemoCard
                icon={HeartHandshake}
                title="Подтверждение участия"
                text="Гости отвечают по ссылке без лишних действий."
              />
              <CompactDemoCard
                icon={Link2}
                title="Публикация по ссылке"
                text="Приглашение легко отправить в мессенджер или почту."
              />
            </div>
            <div className="flex flex-wrap gap-3 pt-1">
              <Button asChild className="homepage-button-label homepage-primary-button">
                <Link href="#demo-preview">Открыть демо</Link>
              </Button>
              <Button asChild variant="secondary" className="homepage-button-label homepage-secondary-button">
                <Link href="/templates">Посмотреть шаблоны</Link>
              </Button>
            </div>
          </CardContent>
        </Card>
      </section>

      <section className="space-y-6">
        <div className="space-y-3">
          <p className="homepage-section-kicker">
            Почему это удобно
          </p>
          <h2 className="homepage-section-title">
            Понятно организатору, удобно гостям
          </h2>
        </div>
        <div className="grid gap-4 md:grid-cols-2 xl:grid-cols-4">
          {trustItems.map((item) => (
            <Card key={item.title} className="homepage-surface h-full border shadow-[0_12px_30px_rgba(17,24,39,0.04)]">
              <CardHeader className="space-y-4">
                <div className="mb-4 inline-flex size-11 items-center justify-center rounded-2xl bg-[var(--homepage-soft-warm)] text-[#c86f4e]">
                  <item.icon className="size-5" />
                </div>
                <CardTitle className="homepage-card-title">{item.title}</CardTitle>
              </CardHeader>
              <CardContent className="homepage-card-copy">
                {item.description}
              </CardContent>
            </Card>
          ))}
        </div>
      </section>

      <section className="space-y-6">
        <div className="space-y-3">
          <p className="homepage-section-kicker">
            Частые вопросы
          </p>
          <h2 className="homepage-section-title">Частые вопросы</h2>
        </div>
        <div className="grid gap-3">
          {faqItems.map((item) => (
            <details
              key={item.question}
              className="homepage-surface group rounded-[1.5rem] border px-5 py-5 shadow-[0_10px_28px_rgba(17,24,39,0.04)]"
            >
              <summary className="flex cursor-pointer list-none items-center justify-between gap-4 text-[1.05rem] font-semibold leading-7 text-foreground">
                {item.question}
                <ChevronRight className="size-4 shrink-0 text-muted-foreground transition group-open:rotate-90" />
              </summary>
              <p className="homepage-card-copy mt-4 max-w-3xl">
                {item.answer}
              </p>
            </details>
          ))}
        </div>
      </section>

      <section>
        <Card className="homepage-surface-lilac overflow-hidden border">
          <CardContent className="flex flex-col gap-6 px-6 py-10 text-center sm:px-10">
            <div className="space-y-3">
              <div className="homepage-warm-chip inline-flex self-center rounded-full px-4 py-1 text-[15px] font-semibold">
                Готовы попробовать?
              </div>
              <h2 className="font-[family-name:var(--font-heading)] text-[2.6rem] font-bold leading-[1.08] tracking-[-0.035em] sm:text-[3rem]">
                Создайте своё первое приглашение уже сегодня
              </h2>
              <p className="homepage-section-copy mx-auto">
                Выберите шаблон, настройте страницу события и начните собирать ответы гостей в
                одном сервисе.
              </p>
            </div>
            <div className="flex flex-wrap justify-center gap-3">
              <Button asChild size="lg" className="homepage-button-label homepage-primary-button">
                <Link href="/register">Создать приглашение</Link>
              </Button>
              <Button asChild variant="secondary" size="lg" className="homepage-button-label homepage-secondary-button">
                <Link href="/templates">Посмотреть шаблоны</Link>
              </Button>
            </div>
          </CardContent>
        </Card>
      </section>
    </div>
  );
}

const eventUseCases = [
  {
    title: "Свадьба",
    description: "Для приглашения с программой дня, местом проведения и сбором ответов гостей.",
    href: "/templates?category=wedding"
  },
  {
    title: "День рождения",
    description: "Для камерного праздника, вечеринки или семейной встречи по ссылке.",
    href: "/templates?category=birthday"
  },
  {
    title: "Юбилей",
    description: "Для события, где важно собрать всех близких и не потерять ответы гостей.",
    href: "/templates?category=anniversary"
  },
  {
    title: "Детский праздник",
    description: "Для праздника с удобной страницей события и быстрым подтверждением участия.",
    href: "/templates?category=baby"
  },
  {
    title: "Корпоративное событие",
    description: "Для деловой встречи, презентации или внутреннего мероприятия команды.",
    href: "/templates?category=corporate"
  },
  {
    title: "Другое событие",
    description: "Если нужен аккуратный онлайн-формат приглашения для любого другого повода.",
    href: "/templates"
  }
] as const;

const steps = [
  {
    title: "Выберите шаблон",
    description: "Подберите стиль, который подходит вашему событию."
  },
  {
    title: "Заполните страницу события",
    description: "Добавьте дату, место, программу, важные детали и вопросы для гостей."
  },
  {
    title: "Опубликуйте и собирайте ответы",
    description: "Отправьте ссылку гостям и отслеживайте подтверждение участия в кабинете."
  }
] as const;

const trustItems = [
  {
    title: "Открывается по ссылке без лишних действий",
    description: "Гостям не нужно ничего устанавливать: страница доступна сразу в браузере.",
    icon: Link2
  },
  {
    title: "Подходит для телефона и компьютера",
    description: "Приглашение удобно читать и открывать на любом устройстве.",
    icon: Smartphone
  },
  {
    title: "Помогает не терять ответы гостей",
    description: "Подтверждения участия и ответы на вопросы собираются в одном месте.",
    icon: CheckCircle2
  },
  {
    title: "Собирает всё событие в одном месте",
    description: "Страница события, гости, ответы и публикация по ссылке работают вместе.",
    icon: ShieldCheck
  }
] as const;

const faqItems = [
  {
    question: "Сколько времени занимает создание приглашения?",
    answer: "Обычно первое приглашение можно собрать за несколько минут."
  },
  {
    question: "Можно ли отправить приглашение по ссылке?",
    answer: "Да, страницу можно открыть на любом устройстве и отправить гостям в мессенджере."
  },
  {
    question: "Можно ли собирать ответы гостей?",
    answer: "Да, сервис помогает отслеживать подтверждение участия и ответы на вопросы."
  },
  {
    question: "Подходит ли сервис для свадьбы и дня рождения?",
    answer: "Да, AI Invite подходит для частных и праздничных событий разных форматов."
  },
  {
    question: "Нужно ли устанавливать приложение?",
    answer: "Нет, приглашение открывается в браузере."
  }
] as const;

function DemoLine({
  icon: Icon,
  title,
  text
}: {
  icon: React.ComponentType<{ className?: string }>;
  title: string;
  text: string;
}) {
  return (
    <div className="flex items-start gap-3 rounded-[1.3rem] bg-[var(--homepage-soft-lilac)] px-4 py-3">
      <div className="mt-0.5 inline-flex size-9 items-center justify-center rounded-2xl bg-[rgba(99,102,241,0.12)] text-[var(--homepage-accent)]">
        <Icon className="size-4" />
      </div>
      <div>
        <div className="text-[15px] font-semibold text-foreground">{title}</div>
        <p className="mt-1 text-[15px] leading-7 text-muted-foreground">{text}</p>
      </div>
    </div>
  );
}

function CompactDemoCard({
  icon: Icon,
  title,
  text
}: {
  icon: React.ComponentType<{ className?: string }>;
  title: string;
  text: string;
}) {
  return (
    <div className="rounded-[1.4rem] border border-white/75 bg-white/84 px-4 py-4 shadow-[0_12px_28px_rgba(17,24,39,0.05)]">
      <div className="flex items-start gap-3">
        <div className="inline-flex size-9 items-center justify-center rounded-2xl bg-[rgba(99,102,241,0.12)] text-[var(--homepage-accent)]">
          <Icon className="size-4" />
        </div>
        <div>
          <div className="text-[15px] font-semibold text-foreground">{title}</div>
          <p className="mt-1 text-[14px] leading-6 text-muted-foreground">{text}</p>
        </div>
      </div>
    </div>
  );
}
