import Image from "next/image";
import Link from "next/link";
import { ArrowRight } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import type { Template } from "@/lib/types";

export function TemplateCard({ template }: { template: Template }) {
  const categoryLabel = getCategoryLabel(template.category);
  const summary = getTemplateSummary(template.category);
  const imageSrc = getTemplateImageSrc(template.code, template.category);
  const accentClass = getTemplateAccentClass(template.category);

  return (
    <Card className="homepage-surface group h-full overflow-hidden border transition duration-200 hover:-translate-y-0.5 hover:shadow-[0_22px_50px_rgba(15,23,42,0.08)]">
      <div className={`relative aspect-[4/3] border-b border-[var(--homepage-border)] ${accentClass}`}>
        <Image
          src={imageSrc}
          alt={`Шаблон ${template.name}`}
          fill
          sizes="(max-width: 768px) 100vw, (max-width: 1280px) 50vw, 33vw"
          className="object-cover object-center transition duration-500 group-hover:scale-[1.03]"
        />
        <div className="absolute inset-0 bg-[linear-gradient(180deg,rgba(17,24,39,0.02)_0%,rgba(17,24,39,0.04)_42%,rgba(17,24,39,0.16)_100%)]" />
        <div className="absolute left-4 top-4 flex gap-2">
          <Badge variant="outline" className="border-[rgba(99,102,241,0.18)] bg-white/92 text-[var(--homepage-accent)]">
            {categoryLabel}
          </Badge>
        </div>
        <div className="absolute inset-x-4 bottom-4 rounded-[1.3rem] bg-white/88 px-4 py-3 backdrop-blur">
          <div className="text-[11px] font-semibold uppercase tracking-[0.16em] text-muted-foreground">
            Готовый стиль
          </div>
          <div className="mt-1 text-[15px] font-medium leading-6 text-foreground">{summary}</div>
        </div>
      </div>
      <CardHeader className="gap-2 pb-3">
        <CardTitle className="homepage-card-title text-[1.18rem]">{template.name}</CardTitle>
        <p className="text-[15px] leading-6 text-muted-foreground">{summary}</p>
      </CardHeader>
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

function getTemplateImageSrc(code: string, category: string) {
  const byCode: Record<string, string> = {
    "wedding-classic": "/images/home/template-wedding-classic.webp",
    "wedding-modern": "/images/home/template-wedding-modern.webp",
    "birthday-confetti": "/images/home/template-birthday-confetti.webp",
    "corporate-clean": "/images/home/template-corporate-clean.webp",
    "corporate-bold": "/images/home/template-corporate-bold.webp",
    "baby-soft": "/images/home/template-baby-soft.webp"
  };

  const byCategory: Record<string, string> = {
    wedding: "/images/home/template-wedding-classic.webp",
    birthday: "/images/home/template-birthday-confetti.webp",
    anniversary: "/images/home/template-wedding-modern.webp",
    baby: "/images/home/template-baby-soft.webp",
    corporate: "/images/home/template-corporate-clean.webp"
  };

  return byCode[code] ?? byCategory[category] ?? "/images/home/template-wedding-classic.webp";
}

function getTemplateAccentClass(category: string) {
  const accents: Record<string, string> = {
    wedding: "bg-[var(--homepage-soft-lilac)]",
    birthday: "bg-[var(--homepage-soft-warm)]",
    anniversary: "bg-[var(--homepage-soft-lilac)]",
    baby: "bg-[rgba(242,154,122,0.1)]",
    corporate: "bg-[rgba(99,102,241,0.08)]"
  };

  return accents[category] ?? "bg-[var(--homepage-soft-lilac)]";
}
