import { ArrowDown, ArrowUp, GripVertical, Pencil, Trash2 } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { StatusBadge } from "@/components/ui/status-badge";
import type { PageSection } from "@/lib/types";

type SectionCardProps = {
  section: PageSection;
  canMoveUp: boolean;
  canMoveDown: boolean;
  onMoveUp: () => void;
  onMoveDown: () => void;
  onEdit: () => void;
  onDelete: () => void;
};

export function SectionCard({
  section,
  canMoveUp,
  canMoveDown,
  onMoveUp,
  onMoveDown,
  onEdit,
  onDelete
}: SectionCardProps) {
  const previewLines = JSON.stringify(section.content, null, 2).split("\n").slice(0, 7).join("\n");

  return (
    <Card className="overflow-hidden border-white/75 bg-white/95">
      <div className="h-1.5 bg-gradient-to-r from-primary/35 via-secondary to-accent/70" />
      <CardHeader className="gap-4">
        <div className="flex flex-wrap items-start justify-between gap-3">
          <div className="space-y-2">
            <div className="flex items-center gap-2">
              <Badge variant="secondary">#{section.position}</Badge>
              <StatusBadge status={section.is_enabled ? "enabled" : "disabled"} />
              <Badge variant="outline">{section.section_type}</Badge>
            </div>
            <CardTitle>{section.title ?? section.section_type}</CardTitle>
            <p className="text-sm text-muted-foreground">
              Контент-блок страницы приглашения. JSON можно править вручную в editor форме.
            </p>
          </div>
          <div className="flex flex-wrap gap-2">
            <div className="hidden items-center rounded-full border bg-secondary/45 px-3 py-2 text-xs font-semibold uppercase tracking-[0.14em] text-muted-foreground sm:flex">
              <GripVertical className="mr-2 size-3.5" />
              Reorder
            </div>
            <Button size="icon" variant="ghost" onClick={onMoveUp} disabled={!canMoveUp}>
              <ArrowUp className="size-4" />
            </Button>
            <Button size="icon" variant="ghost" onClick={onMoveDown} disabled={!canMoveDown}>
              <ArrowDown className="size-4" />
            </Button>
            <Button size="icon" variant="ghost" onClick={onEdit}>
              <Pencil className="size-4" />
            </Button>
            <Button size="icon" variant="ghost" onClick={onDelete}>
              <Trash2 className="size-4" />
            </Button>
          </div>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="rounded-2xl bg-secondary/35 px-4 py-3 text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">
          JSON preview
        </div>
        <pre className="overflow-x-auto rounded-[1.3rem] bg-slate-950 p-4 text-xs leading-6 text-slate-100">
          {previewLines}
        </pre>
      </CardContent>
    </Card>
  );
}
