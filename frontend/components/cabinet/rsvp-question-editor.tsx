"use client";

import { Trash2 } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { RSVP_QUESTION_TYPE_OPTIONS } from "@/lib/constants";
import type { RsvpFormUpdateRequest, RsvpQuestion } from "@/lib/types";

export type EditableQuestion = RsvpFormUpdateRequest["questions"][number] & {
  id: string;
};

type RsvpQuestionEditorProps = {
  index: number;
  question: EditableQuestion;
  onChange: (question: EditableQuestion) => void;
  onRemove: () => void;
};

export function RsvpQuestionEditor({
  index,
  question,
  onChange,
  onRemove
}: RsvpQuestionEditorProps) {
  const optionsText = Array.isArray(question.options) ? question.options.join("\n") : "";
  const usesOptions = question.question_type === "select" || question.question_type === "multiselect";

  return (
    <div className="rounded-[1.5rem] border bg-white/94 p-4 shadow-[0_16px_34px_rgba(15,23,42,0.04)]">
      <div className="mb-4 flex items-center justify-between gap-3">
        <div>
          <p className="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">
            Question {index + 1}
          </p>
          <h3 className="text-base font-semibold">{question.label || "Untitled question"}</h3>
          <div className="mt-2 flex flex-wrap gap-2">
            <Badge variant="outline">{question.question_type}</Badge>
            {question.required ? <Badge variant="success">required</Badge> : null}
          </div>
        </div>
        <Button size="icon" variant="ghost" onClick={onRemove}>
          <Trash2 className="size-4" />
        </Button>
      </div>
      <div className="grid gap-4 md:grid-cols-2">
        <Field label="Code" htmlFor={`rsvp-code-${question.id}`}>
          <Input
            id={`rsvp-code-${question.id}`}
            value={question.code}
            onChange={(event) => onChange({ ...question, code: event.target.value })}
            placeholder="attendance"
          />
        </Field>
        <Field label="Type" htmlFor={`rsvp-type-${question.id}`}>
          <select
            id={`rsvp-type-${question.id}`}
            value={question.question_type}
            onChange={(event) =>
              onChange({
                ...question,
                question_type: event.target.value,
                options:
                  event.target.value === "select" || event.target.value === "multiselect"
                    ? Array.isArray(question.options)
                      ? question.options
                      : []
                    : []
              })
            }
            className="flex h-12 w-full rounded-[1.2rem] border bg-background px-4 text-sm outline-none transition focus-visible:ring-2 focus-visible:ring-ring"
          >
            {RSVP_QUESTION_TYPE_OPTIONS.map((option) => (
              <option key={option} value={option}>
                {option}
              </option>
            ))}
          </select>
        </Field>
        <Field label="Label" htmlFor={`rsvp-label-${question.id}`} className="md:col-span-2">
          <Input
            id={`rsvp-label-${question.id}`}
            value={question.label}
            onChange={(event) => onChange({ ...question, label: event.target.value })}
            placeholder="Will you attend?"
          />
        </Field>
        {usesOptions ? (
          <Field label="Options" htmlFor={`rsvp-options-${question.id}`} className="md:col-span-2">
            <Textarea
              id={`rsvp-options-${question.id}`}
              className="min-h-[110px]"
              value={optionsText}
              onChange={(event) =>
                onChange({
                  ...question,
                  options: event.target.value
                    .split("\n")
                    .map((value) => value.trim())
                    .filter(Boolean)
                })
              }
              placeholder={"yes\nno\nmaybe"}
            />
            <p className="mt-2 text-xs text-muted-foreground">
              Каждый вариант с новой строки. Такой формат проще всего поддерживать для MVP-builder.
            </p>
          </Field>
        ) : null}
        <label className="md:col-span-2 flex items-center gap-3 rounded-2xl border bg-white/75 px-4 py-3 text-sm">
          <input
            type="checkbox"
            checked={Boolean(question.required)}
            onChange={(event) => onChange({ ...question, required: event.target.checked })}
            className="size-4 rounded border-border"
          />
          Required question
        </label>
      </div>
    </div>
  );
}

function Field({
  children,
  className,
  htmlFor,
  label
}: {
  children: React.ReactNode;
  className?: string;
  htmlFor: string;
  label: string;
}) {
  return (
    <div className={className}>
      <Label htmlFor={htmlFor}>{label}</Label>
      <div className="mt-2">{children}</div>
    </div>
  );
}

export function editableQuestionFromApi(question: RsvpQuestion): EditableQuestion {
  return {
    id: question.id,
    code: question.code,
    label: question.label,
    question_type: question.question_type,
    required: question.required,
    options: Array.isArray(question.options) ? question.options : []
  };
}
