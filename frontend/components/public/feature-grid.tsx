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
    <section className="space-y-5">
      <div className="space-y-2">
        <p className="text-sm font-semibold uppercase tracking-[0.18em] text-muted-foreground">
          Возможности
        </p>
        <h2 className="font-[family-name:var(--font-heading)] text-3xl">
          Всё, что нужно для приглашения и гостей
        </h2>
      </div>
      <div className="grid gap-4 md:grid-cols-2 xl:grid-cols-5">
        {features.map(({ title, description, icon: Icon }) => (
          <Card key={title} className="h-full border-white/70 bg-white/88">
            <CardHeader>
              <div className="mb-4 inline-flex size-11 items-center justify-center rounded-2xl bg-accent text-accent-foreground">
                <Icon className="size-5" />
              </div>
              <CardTitle>{title}</CardTitle>
            </CardHeader>
            <CardContent className="text-sm leading-6 text-muted-foreground">{description}</CardContent>
          </Card>
        ))}
      </div>
    </section>
  );
}
