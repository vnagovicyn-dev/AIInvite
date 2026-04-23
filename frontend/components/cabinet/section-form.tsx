"use client";

import { useEffect, useMemo, useState } from "react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { SectionBlock } from "@/components/ui/section-block";
import { Textarea } from "@/components/ui/textarea";
import { SECTION_TYPE_OPTIONS } from "@/lib/constants";
import type { CreateSectionRequest, PageSection, UpdateSectionRequest } from "@/lib/types";

type SectionFormProps = {
  mode: "create" | "edit";
  section?: PageSection | null;
  isSubmitting?: boolean;
  onCancel?: () => void;
  onSubmit: (payload: CreateSectionRequest | UpdateSectionRequest) => Promise<void> | void;
};

export function SectionForm({
  mode,
  section,
  isSubmitting = false,
  onCancel,
  onSubmit
}: SectionFormProps) {
  const initialContent = useMemo(
    () => JSON.stringify(section?.content ?? getDefaultContent(section?.section_type ?? "hero"), null, 2),
    [section]
  );

  const [sectionType, setSectionType] = useState(section?.section_type ?? "hero");
  const [title, setTitle] = useState(section?.title ?? "");
  const [contentText, setContentText] = useState(initialContent);
  const [isEnabled, setIsEnabled] = useState(section?.is_enabled ?? true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setSectionType(section?.section_type ?? "hero");
    setTitle(section?.title ?? "");
    setContentText(initialContent);
    setIsEnabled(section?.is_enabled ?? true);
    setError(null);
  }, [section, initialContent]);

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setError(null);

    try {
      const content = JSON.parse(contentText) as Record<string, unknown>;
      await onSubmit({
        ...(mode === "create" ? { section_type: sectionType } : {}),
        title: title.trim() || null,
        content,
        is_enabled: isEnabled
      });
    } catch {
      setError("Section content must be valid JSON.");
    }
  }

  return (
    <SectionBlock
      title={mode === "create" ? "Добавить секцию" : "Редактировать секцию"}
      description="Для MVP контент редактируется через JSON. Ниже уже есть стартовые структуры для hero, program и faq."
      className="border-dashed bg-white/92"
    >
        <form className="space-y-5" onSubmit={handleSubmit}>
          {mode === "create" ? (
            <div>
              <Label htmlFor="section-type">Section type</Label>
              <select
                id="section-type"
                value={sectionType}
                onChange={(event) => {
                  setSectionType(event.target.value);
                  if (!section) {
                    setContentText(JSON.stringify(getDefaultContent(event.target.value), null, 2));
                  }
                }}
                className="mt-2 flex h-12 w-full rounded-[1.2rem] border bg-background px-4 text-sm outline-none transition focus-visible:ring-2 focus-visible:ring-ring"
              >
                {SECTION_TYPE_OPTIONS.map((option) => (
                  <option key={option} value={option}>
                    {option}
                  </option>
                ))}
              </select>
            </div>
          ) : null}
          <div>
            <Label htmlFor="section-title">Title</Label>
            <Input
              id="section-title"
              className="mt-2"
              value={title}
              onChange={(event) => setTitle(event.target.value)}
              placeholder="Welcome section"
            />
          </div>
          <div className="rounded-2xl bg-secondary/45 px-4 py-3 text-sm text-muted-foreground">
            Совет: для `hero` используйте `headline` и `subheadline`, для `program` массив `items`,
            а для `faq` массив объектов с `question` и `answer`.
          </div>
          <div>
            <Label htmlFor="section-content">Content JSON</Label>
            <Textarea
              id="section-content"
              className="mt-2 min-h-[220px] font-mono text-xs"
              value={contentText}
              onChange={(event) => setContentText(event.target.value)}
              placeholder='{\n  "headline": "Ждём вас на празднике",\n  "subheadline": "Сохраните дату"\n}'
            />
          </div>
          <label className="flex items-center gap-3 rounded-2xl border bg-white/75 px-4 py-3 text-sm">
            <input
              type="checkbox"
              checked={isEnabled}
              onChange={(event) => setIsEnabled(event.target.checked)}
              className="size-4 rounded border-border"
            />
            Section is enabled
          </label>
          {error ? <p className="text-sm text-destructive">{error}</p> : null}
          <div className="flex flex-wrap justify-end gap-3">
            {onCancel ? (
              <Button type="button" variant="secondary" onClick={onCancel}>
                Отмена
              </Button>
            ) : null}
            <Button type="submit" disabled={isSubmitting}>
              {isSubmitting ? "Сохраняем..." : mode === "create" ? "Добавить секцию" : "Сохранить секцию"}
            </Button>
          </div>
        </form>
    </SectionBlock>
  );
}

function getDefaultContent(sectionType: string) {
  switch (sectionType) {
    case "program":
      return {
        items: [
          { time: "15:00", title: "Guests arrive", description: "Welcome drink and photos" }
        ]
      };
    case "faq":
      return {
        items: [
          { question: "Is there a dress code?", answer: "Cocktail attire in light tones." }
        ]
      };
    default:
      return {
        headline: "We would love to celebrate with you",
        subheadline: "Save the date"
      };
  }
}
