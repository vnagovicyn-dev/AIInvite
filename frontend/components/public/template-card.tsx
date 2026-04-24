import Link from "next/link";
import { ArrowRight } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import type { Template } from "@/lib/types";

export function TemplateCard({ template }: { template: Template }) {
  const categoryLabel = getCategoryLabel(template.category);
  const summary = getTemplateSummary(template.category);

  return (
    <Card className="homepage-surface h-full overflow-hidden border">
      <div className="relative aspect-[4/3] border-b border-[var(--homepage-border)] bg-[var(--homepage-soft-lilac)]">
        {template.preview_url ? (
          <div
            className="absolute inset-0 bg-cover bg-center"
            style={{ backgroundImage: `url(${template.preview_url})` }}
          />
        ) : null}
        <div className="absolute inset-0 bg-gradient-to-b from-transparent via-transparent to-black/10" />
        <div className="absolute left-4 top-4 flex gap-2">
          <Badge variant="outline" className="border-[rgba(99,102,241,0.18)] bg-white/92 text-[var(--homepage-accent)]">
            {categoryLabel}
          </Badge>
        </div>
        <div className="absolute inset-x-4 bottom-4 rounded-[1.3rem] bg-white/86 px-4 py-3 backdrop-blur">
          <div className="text-[11px] font-semibold uppercase tracking-[0.16em] text-muted-foreground">
            Готовый стиль
          </div>
          <div className="mt-1 text-[15px] font-medium leading-6 text-foreground">{summary}</div>
        </div>
      </div>
      <CardHeader className="gap-3">
        <CardTitle className="homepage-card-title text-[1.18rem]">{template.name}</CardTitle>
        <p className="homepage-card-copy">
          {summary}
        </p>
      </CardHeader>
      <CardContent className="text-muted-foreground">
        <div className="homepage-card-copy rounded-2xl bg-[var(--homepage-soft-warm)] px-4 py-3">
          Подходит для событий, где важно быстро собрать красивую страницу и отправить её гостям.
        </div>
      </CardContent>
      <CardFooter>
        <Button asChild className="homepage-button-label homepage-primary-button w-full justify-between">
          <Link href={`/templates/${template.id}`}>
            Выбрать шаблон
            <ArrowRight className="size-4" />
          </Link>
        </Button>
      </CardFooter>
    </Card>
  );
}

function getCategoryLabel(category: string) {
  const labels: Record<string, string> = {
    wedding: "Свадьба",
    birthday: "День рождения",
    anniversary: "Юбилей",
    baby: "Детский праздник",
    corporate: "Корпоративное событие"
  };

  return labels[category] ?? "Другое событие";
}

function getTemplateSummary(category: string) {
  const summaries: Record<string, string> = {
    wedding: "Для свадебного приглашения с программой, локацией и ответами гостей",
    birthday: "Для тёплого приглашения на день рождения с деталями праздника",
    anniversary: "Для семейного или личного юбилея с важными деталями события",
    baby: "Для детского праздника, baby shower или семейной встречи",
    corporate: "Для корпоративного события, презентации или камерной встречи"
  };

  return summaries[category] ?? "Для события, которое хочется оформить аккуратно и понятно";
}
