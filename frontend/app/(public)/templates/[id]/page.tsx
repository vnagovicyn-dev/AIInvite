import { ApiClientError } from "@/lib/api/client";
import { templatesApi } from "@/lib/api/templates";
import { EmptyState } from "@/components/states/empty-state";
import { PageHeader } from "@/components/layout/page-header";
import { TemplateDetailCta } from "@/components/public/template-detail-cta";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Paintbrush2 } from "lucide-react";

export default async function TemplateDetailPage({
  params
}: {
  params: Promise<{ id: string }>;
}) {
  const { id } = await params;
  try {
    const template = await templatesApi.getById(id);

    return (
      <div className="space-y-6">
        <PageHeader
          eyebrow="Template detail"
          title={template.name}
          description="Используйте шаблон как стартовую точку для нового события и дальнейшей настройки invitation page."
        />
        <div className="grid gap-6 lg:grid-cols-[1.1fr_0.9fr]">
          <Card className="overflow-hidden border-white/75 bg-white/95">
            <div className="relative aspect-[4/3] border-b bg-gradient-to-br from-secondary via-background to-accent/25">
              {template.preview_url ? (
                <div
                  className="absolute inset-0 bg-cover bg-center"
                  style={{ backgroundImage: `url(${template.preview_url})` }}
                />
              ) : null}
            </div>
            <CardContent className="space-y-4 pt-6 text-sm text-muted-foreground">
              <div className="rounded-[1.35rem] bg-secondary/38 px-4 py-3">
                Шаблон показывает общее направление будущей invitation page и служит хорошей
                стартовой точкой для демо или реального проекта.
              </div>
              Preview info подготовлен под будущие медиа-ассеты. Уже сейчас этот шаблон можно
              выбрать как основу для нового invite flow.
            </CardContent>
          </Card>

          <Card className="border-white/75 bg-white/95">
            <CardHeader className="space-y-4">
              <div className="flex flex-wrap gap-2">
                <Badge variant="success">{template.is_active ? "active" : "inactive"}</Badge>
                <Badge variant="outline">{template.category}</Badge>
              </div>
              <CardTitle>О шаблоне</CardTitle>
            </CardHeader>
            <CardContent className="space-y-5 text-sm text-muted-foreground">
              <div className="rounded-[1.45rem] bg-secondary/45 px-4 py-4">
                <div className="mb-1 font-semibold text-foreground">Код шаблона</div>
                <div>{template.code}</div>
              </div>
              <div className="rounded-[1.45rem] bg-secondary/45 px-4 py-4">
                <div className="mb-1 font-semibold text-foreground">Категория</div>
                <div>{template.category}</div>
              </div>
              <div className="rounded-[1.45rem] bg-secondary/45 px-4 py-4">
                <div className="mb-1 flex items-center gap-2 font-semibold text-foreground">
                  <Paintbrush2 className="size-4" />
                  Применение
                </div>
                <p className="leading-6">
                  После выбора шаблона можно создать событие, настроить секции страницы и
                  опубликовать invite по публичному slug.
                </p>
              </div>
              <TemplateDetailCta templateId={template.id} />
            </CardContent>
          </Card>
        </div>
      </div>
    );
  } catch (error) {
    if (error instanceof ApiClientError && error.status === 404) {
      return (
        <div className="space-y-6">
          <PageHeader
            eyebrow="Template detail"
            title="Шаблон не найден"
            description="Возможно, он был удалён или ссылка устарела."
          />
          <EmptyState
            icon={Paintbrush2}
            title="Шаблон недоступен"
            description="Вернитесь в каталог и выберите другой вариант для создания события."
          />
        </div>
      );
    }

    throw error;
  }
}
