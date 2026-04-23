import { TemplateCard } from "@/components/public/template-card";
import type { Template } from "@/lib/types";

export function TemplateGrid({
  templates,
  compact = false
}: {
  templates: Template[];
  compact?: boolean;
}) {
  return (
    <div className={compact ? "grid gap-4 md:grid-cols-2 xl:grid-cols-3" : "grid gap-5 md:grid-cols-2 xl:grid-cols-3"}>
      {templates.map((template) => (
        <TemplateCard key={template.id} template={template} />
      ))}
    </div>
  );
}
