import Link from "next/link";
import {
  CalendarDays,
  CheckCircle2,
  ChevronRight,
  HeartHandshake,
  LibraryBig,
  Link2,
  PartyPopper,
  ShieldCheck,
  Smartphone,
  Sparkles,
  Users,
  WandSparkles
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
    <div className="space-y-16 pb-10">
      <LandingHero templatesCount={templates.total} />

      <section className="space-y-5" id="events">
        <div className="space-y-2">
          <p className="text-sm font-semibold uppercase tracking-[0.18em] text-muted-foreground">
            Форматы событий
          </p>
          <h2 className="font-[family-name:var(--font-heading)] text-3xl">
            Подходит для самых важных событий
          </h2>
          <p className="max-w-2xl text-sm leading-6 text-muted-foreground">
            Выберите формат, который подходит именно вам, и начните с готовой основы.
          </p>
        </div>
        <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
          {eventUseCases.map((item) => (
            <Link
              key={item.title}
              href={item.href}
              className="group rounded-[1.5rem] border border-white/75 bg-white/88 p-5 shadow-soft transition hover:-translate-y-0.5 hover:border-accent/60 hover:bg-white"
            >
              <div className="flex items-start justify-between gap-4">
                <div>
                  <div className="text-lg font-semibold text-foreground">{item.title}</div>
                  <p className="mt-2 text-sm leading-6 text-muted-foreground">{item.description}</p>
                </div>
                <ChevronRight className="mt-1 size-4 text-muted-foreground transition group-hover:text-foreground" />
              </div>
            </Link>
          ))}
        </div>
      </section>

      <section className="space-y-5" id="how-it-works">
        <div className="space-y-2">
          <p className="text-sm font-semibold uppercase tracking-[0.18em] text-muted-foreground">
            Как это работает
          </p>
          <h2 className="font-[family-name:var(--font-heading)] text-3xl">
            Создайте приглашение за 3 шага
          </h2>
        </div>
        <div className="grid gap-4 lg:grid-cols-3">
          {steps.map((step, index) => (
            <Card key={step.title} className="border-white/75 bg-white/92">
              <CardHeader className="space-y-4">
                <div className="inline-flex size-11 items-center justify-center rounded-2xl bg-accent text-accent-foreground">
                  <span className="text-lg font-semibold">{index + 1}</span>
                </div>
                <CardTitle>{step.title}</CardTitle>
              </CardHeader>
              <CardContent className="text-sm leading-6 text-muted-foreground">
                {step.description}
              </CardContent>
            </Card>
          ))}
        </div>
      </section>

      <FeatureGrid />

      <section className="space-y-5" id="templates">
        <div className="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
          <div className="space-y-2">
            <p className="text-sm font-semibold uppercase tracking-[0.18em] text-muted-foreground">
              Шаблоны
            </p>
            <h2 className="font-[family-name:var(--font-heading)] text-3xl">
              Выберите шаблон для своего события
            </h2>
            <p className="max-w-2xl text-sm leading-6 text-muted-foreground">
              Начните с готового стиля и адаптируйте его под свой формат события.
            </p>
          </div>
          <Button asChild variant="secondary">
            <Link href="/templates">Смотреть все шаблоны</Link>
          </Button>
        </div>
        <TemplateGrid templates={templates.items} compact />
      </section>

      <section className="grid gap-6 lg:grid-cols-[1.05fr_0.95fr]" id="demo">
        <Card className="overflow-hidden border-white/75 bg-white/94">
          <CardHeader className="space-y-3">
            <div className="text-sm font-semibold uppercase tracking-[0.18em] text-muted-foreground">
              Демо
            </div>
            <CardTitle className="font-[family-name:var(--font-heading)] text-3xl">
              Так выглядит готовая страница приглашения
            </CardTitle>
            <p className="max-w-xl text-sm leading-6 text-muted-foreground">
              Откройте демо и посмотрите, как гости увидят ваше приглашение.
            </p>
          </CardHeader>
          <CardContent className="grid gap-4" id="demo-preview">
            <div className="rounded-[1.8rem] border border-border/70 bg-gradient-to-br from-white via-secondary/45 to-accent/30 p-6">
              <div className="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">
                Страница приглашения
              </div>
              <div className="mt-3 font-[family-name:var(--font-heading)] text-3xl text-foreground">
                Свадьба Анны и Игоря
              </div>
              <div className="mt-3 flex flex-wrap gap-2 text-sm text-muted-foreground">
                <span className="rounded-full bg-white/85 px-3 py-1">20 августа 2026</span>
                <span className="rounded-full bg-white/85 px-3 py-1">Амстердам</span>
                <span className="rounded-full bg-white/85 px-3 py-1">Grand Hall</span>
              </div>
              <div className="mt-5 grid gap-3">
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
                <DemoLine
                  icon={HeartHandshake}
                  title="Форма ответа"
                  text="Гости отвечают на приглашение прямо по ссылке."
                />
              </div>
            </div>
          </CardContent>
        </Card>

        <Card className="border-accent/40 bg-gradient-to-br from-white via-white to-accent/25">
          <CardContent className="flex h-full flex-col justify-between gap-6 px-6 py-6">
            <div className="space-y-4">
              <div className="inline-flex rounded-full bg-white/85 px-4 py-1 text-sm font-semibold text-foreground">
                Пример пользовательского сценария
              </div>
              <div className="space-y-3">
                <div className="text-2xl font-[family-name:var(--font-heading)]">
                  От выбора шаблона до готовой ссылки для гостей
                </div>
                <p className="text-sm leading-6 text-muted-foreground">
                  В демо-блоке мы показываем, как будет выглядеть опубликованное приглашение:
                  спокойная страница события, важные детали и понятное подтверждение участия.
                </p>
              </div>
            </div>
            <div className="flex flex-wrap gap-3">
              <Button asChild>
                <Link href="#demo-preview">Открыть демо</Link>
              </Button>
              <Button asChild variant="secondary">
                <Link href="/templates">Посмотреть шаблоны</Link>
              </Button>
            </div>
          </CardContent>
        </Card>
      </section>

      <section className="space-y-5">
        <div className="space-y-2">
          <p className="text-sm font-semibold uppercase tracking-[0.18em] text-muted-foreground">
            Почему это удобно
          </p>
          <h2 className="font-[family-name:var(--font-heading)] text-3xl">
            Понятно организатору, удобно гостям
          </h2>
        </div>
        <div className="grid gap-4 md:grid-cols-2 xl:grid-cols-4">
          {trustItems.map((item) => (
            <Card key={item.title} className="h-full border-white/75 bg-white/92">
              <CardHeader>
                <div className="mb-4 inline-flex size-11 items-center justify-center rounded-2xl bg-secondary text-foreground">
                  <item.icon className="size-5" />
                </div>
                <CardTitle className="text-lg">{item.title}</CardTitle>
              </CardHeader>
              <CardContent className="text-sm leading-6 text-muted-foreground">
                {item.description}
              </CardContent>
            </Card>
          ))}
        </div>
      </section>

      <section className="space-y-5">
        <div className="space-y-2">
          <p className="text-sm font-semibold uppercase tracking-[0.18em] text-muted-foreground">
            Частые вопросы
          </p>
          <h2 className="font-[family-name:var(--font-heading)] text-3xl">Частые вопросы</h2>
        </div>
        <div className="grid gap-3">
          {faqItems.map((item) => (
            <details
              key={item.question}
              className="group rounded-[1.5rem] border border-white/75 bg-white/92 px-5 py-5 shadow-soft"
            >
              <summary className="flex cursor-pointer list-none items-center justify-between gap-4 font-semibold text-foreground">
                {item.question}
                <ChevronRight className="size-4 shrink-0 text-muted-foreground transition group-open:rotate-90" />
              </summary>
              <p className="mt-4 max-w-3xl text-sm leading-6 text-muted-foreground">
                {item.answer}
              </p>
            </details>
          ))}
        </div>
      </section>

      <section>
        <Card className="overflow-hidden border-accent/45 bg-gradient-to-br from-white via-white to-accent/30">
          <CardContent className="flex flex-col gap-6 px-6 py-10 text-center sm:px-10">
            <div className="space-y-3">
              <div className="inline-flex self-center rounded-full bg-white/85 px-4 py-1 text-sm font-semibold text-foreground">
                Готовы попробовать?
              </div>
              <h2 className="font-[family-name:var(--font-heading)] text-4xl leading-tight">
                Создайте своё первое приглашение уже сегодня
              </h2>
              <p className="mx-auto max-w-2xl text-sm leading-6 text-muted-foreground">
                Выберите шаблон, настройте страницу события и начните собирать ответы гостей в
                одном сервисе.
              </p>
            </div>
            <div className="flex flex-wrap justify-center gap-3">
              <Button asChild size="lg">
                <Link href="/register">Создать приглашение</Link>
              </Button>
              <Button asChild variant="secondary" size="lg">
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
    <div className="flex items-start gap-3 rounded-[1.3rem] bg-white/85 px-4 py-3">
      <div className="mt-0.5 inline-flex size-9 items-center justify-center rounded-2xl bg-secondary text-foreground">
        <Icon className="size-4" />
      </div>
      <div>
        <div className="font-semibold text-foreground">{title}</div>
        <p className="mt-1 text-sm leading-6 text-muted-foreground">{text}</p>
      </div>
    </div>
  );
}
