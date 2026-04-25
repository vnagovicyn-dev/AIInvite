import { CalendarHeart, LayoutTemplate, Link2, Mailbox, Users } from "lucide-react";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

const features = [
  {
    title: "Страница события",
    description: "Красивое приглашение с датой, местом, программой и важными деталями.",
    icon: LayoutTemplate
  },
  {
    title: "Список гостей",
    description: "Храните гостей, группы и важные пометки в одном месте.",
    icon: Users
  },
  {
    title: "Подтверждение участия",
    description: "Собирайте ответы гостей без таблиц и переписок.",
    icon: Mailbox
  },
  {
    title: "Публикация по ссылке",
    description: "Делитесь приглашением в мессенджерах и по почте.",
    icon: Link2
  },
  {
    title: "Готовые шаблоны",
    description: "Начинайте не с пустого листа, а с готовой структуры.",
    icon: CalendarHeart
  }
];

export function FeatureGrid() {
  return (
    <section className="homepage-surface-warm rounded-[2rem] border px-6 py-8 sm:px-8 sm:py-9">
      <div className="space-y-6">
        <div className="space-y-3">
          <p className="homepage-section-kicker">
            Возможности
          </p>
          <h2 className="homepage-section-title">
            Всё, что нужно для приглашения и гостей
          </h2>
          <p className="homepage-section-copy max-w-2xl">
            Страница события, гости и сбор ответов гостей работают как единый аккуратный сценарий.
          </p>
        </div>
        <div className="grid gap-4 md:grid-cols-2 xl:grid-cols-5">
          {features.map(({ title, description, icon: Icon }) => (
            <Card key={title} className="h-full border bg-white/88 shadow-[0_14px_36px_rgba(17,24,39,0.05)]">
              <CardHeader className="space-y-4">
                <div className="inline-flex size-11 items-center justify-center rounded-2xl bg-[rgba(99,102,241,0.12)] text-[var(--homepage-accent)]">
                  <Icon className="size-5" />
                </div>
                <CardTitle className="homepage-card-title">{title}</CardTitle>
              </CardHeader>
              <CardContent className="homepage-card-copy">{description}</CardContent>
            </Card>
          ))}
        </div>
      </div>
    </section>
  );
}
