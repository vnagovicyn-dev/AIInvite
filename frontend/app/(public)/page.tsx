import Link from "next/link";

import { FeatureGrid } from "@/components/public/feature-grid";
import { LandingHero } from "@/components/public/landing-hero";
import { TemplateGrid } from "@/components/public/template-grid";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { templatesApi } from "@/lib/api/templates";

export default async function HomePage() {
  const templates = await templatesApi.list("?page=1&per_page=6&is_active=true");

  return (
    <div className="space-y-12 pb-8">
      <LandingHero templatesCount={templates.total} />
      <FeatureGrid />

      <section className="space-y-5">
        <div className="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
          <div className="space-y-2">
            <p className="text-sm font-semibold uppercase tracking-[0.18em] text-muted-foreground">
              Каталог шаблонов
            </p>
            <h2 className="font-[family-name:var(--font-heading)] text-3xl">
              Выберите визуальное направление для события
            </h2>
          </div>
          <Button asChild variant="secondary">
            <Link href="/templates">Все шаблоны</Link>
          </Button>
        </div>
        <TemplateGrid templates={templates.items} compact />
      </section>

      <Card className="overflow-hidden border-white/75 bg-white/92">
        <CardHeader>
          <CardTitle>От landing page до опубликованного invite</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4 text-sm text-muted-foreground md:grid-cols-3">
          <div className="rounded-[1.45rem] bg-secondary/45 px-4 py-4 leading-6">
            1. Выбираете шаблон и создаёте событие в кабинете.
          </div>
          <div className="rounded-[1.45rem] bg-secondary/45 px-4 py-4 leading-6">
            2. Настраиваете секции страницы, список гостей и форму RSVP.
          </div>
          <div className="rounded-[1.45rem] bg-secondary/45 px-4 py-4 leading-6">
            3. Публикуете invite по slug и делитесь публичной страницей.
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
