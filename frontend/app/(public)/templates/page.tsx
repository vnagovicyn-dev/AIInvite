import Link from "next/link";
import { PageHeader } from "@/components/layout/page-header";
import { TemplateGrid } from "@/components/public/template-grid";
import { EmptyState } from "@/components/states/empty-state";
import { Button } from "@/components/ui/button";
import { templatesApi } from "@/lib/api/templates";
import { LibraryBig } from "lucide-react";

export default async function TemplatesPage({
  searchParams
}: {
  searchParams: Promise<{ category?: string }>;
}) {
  const { category } = await searchParams;
  const query = new URLSearchParams({ page: "1", per_page: "24", is_active: "true" });
  if (category) {
    query.set("category", category);
  }

  const [templates, categories] = await Promise.all([
    templatesApi.list(`?${query.toString()}`),
    templatesApi.categories()
  ]);

  return (
    <div className="space-y-6">
      <PageHeader
        eyebrow="Каталог"
        title="Шаблоны"
        description="Выберите стиль для события, отфильтруйте каталог по категории и откройте подходящий шаблон."
      />

      <div className="rounded-[1.6rem] border border-white/75 bg-white/88 p-4">
        <div className="mb-3 text-[11px] font-semibold uppercase tracking-[0.16em] text-muted-foreground">
          Категории
        </div>
        <div className="flex flex-wrap gap-2">
          <Button asChild variant={!category ? "default" : "secondary"} size="sm" className="homepage-button-label">
            <Link href="/templates">Все</Link>
          </Button>
          {categories.map((item) => (
            <Button
              key={item.category}
              asChild
              variant={category === item.category ? "default" : "secondary"}
              size="sm"
              className="homepage-button-label"
            >
              <Link href={`/templates?category=${encodeURIComponent(item.category)}`}>
                {getCategoryLabel(item.category)}
              </Link>
            </Button>
          ))}
        </div>
      </div>

      {templates.items.length === 0 ? (
        <EmptyState
          icon={LibraryBig}
          title="Шаблоны не найдены"
          description="Попробуйте снять фильтр или вернитесь позже, когда в каталоге появятся новые шаблоны."
        />
      ) : (
        <TemplateGrid templates={templates.items} />
      )}
    </div>
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
