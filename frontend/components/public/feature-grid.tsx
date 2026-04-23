import { CalendarHeart, LayoutTemplate, Mailbox, ShieldCheck } from "lucide-react";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

const features = [
  {
    title: "Шаблоны под разные события",
    description: "Каталог с готовыми концепциями для свадеб, дней рождения, baby shower и corporate invite.",
    icon: LayoutTemplate
  },
  {
    title: "Удобный кабинет организатора",
    description: "События, секции, гости и статус публикации собраны в одном месте без лишних переключений.",
    icon: CalendarHeart
  },
  {
    title: "RSVP и список гостей",
    description: "Публичная форма ответа и приватное управление гостями уже работают поверх одного backend flow.",
    icon: Mailbox
  },
  {
    title: "Архитектура под рост продукта",
    description: "Каркас уже подготовлен под media, AI suggestions, seating и дальнейшие улучшения.",
    icon: ShieldCheck
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
          Всё, что нужно для современного invite flow
        </h2>
      </div>
      <div className="grid gap-4 md:grid-cols-2 xl:grid-cols-4">
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
