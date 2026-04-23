"use client";

import { useEffect, useMemo, useState } from "react";
import Link from "next/link";
import { useParams } from "next/navigation";
import { Plus, SquarePen } from "lucide-react";

import {
  editableQuestionFromApi,
  RsvpQuestionEditor,
  type EditableQuestion
} from "@/components/cabinet/rsvp-question-editor";
import { RsvpFormPreview } from "@/components/cabinet/rsvp-form-preview";
import { PageHeader } from "@/components/layout/page-header";
import { ErrorState } from "@/components/states/error-state";
import { LoadingState } from "@/components/states/loading-state";
import { Button } from "@/components/ui/button";
import { InfoCard } from "@/components/ui/info-card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { SectionBlock } from "@/components/ui/section-block";
import { Textarea } from "@/components/ui/textarea";
import { useApiState } from "@/hooks/use-api-state";
import { eventsApi } from "@/lib/api/events";
import { rsvpApi } from "@/lib/api/rsvp";
import type { Event, RsvpForm, RsvpResponsesResponse } from "@/lib/types";

export default function EventRsvpPage() {
  const params = useParams<{ id: string }>();
  const eventId = params.id;
  const eventState = useApiState<Event | null>(null);
  const formState = useApiState<RsvpForm | null>(null);
  const responsesState = useApiState<RsvpResponsesResponse | null>(null);
  const [title, setTitle] = useState("RSVP");
  const [description, setDescription] = useState("");
  const [deadlineAt, setDeadlineAt] = useState("");
  const [questions, setQuestions] = useState<EditableQuestion[]>([]);
  const [isSubmitting, setIsSubmitting] = useState(false);

  async function loadData() {
    await Promise.all([
      eventState.run(async () => eventsApi.getById(eventId)),
      loadForm(),
      responsesState.run(async () => rsvpApi.listResponses(eventId, "?page=1&per_page=20"))
    ]);
  }

  async function loadForm() {
    await formState.run(async () => rsvpApi.getForm(eventId));
  }

  useEffect(() => {
    void loadData();
  }, [eventId]);

  useEffect(() => {
    if (!formState.data) {
      return;
    }

    setTitle(formState.data.title);
    setDescription(formState.data.description ?? "");
    setDeadlineAt(formState.data.deadline_at ? new Date(formState.data.deadline_at).toISOString().slice(0, 16) : "");
    setQuestions(formState.data.questions.map(editableQuestionFromApi));
  }, [formState.data]);

  async function handleSave() {
    setIsSubmitting(true);
    try {
      const response = await rsvpApi.updateForm(eventId, {
        title,
        description: description || null,
        deadline_at: deadlineAt ? new Date(deadlineAt).toISOString() : null,
        settings: formState.data?.settings ?? {},
        questions: questions.map(({ code, label, question_type, required, options }) => ({
          code,
          label,
          question_type,
          required,
          options
        }))
      });
      formState.setData(response);
    } finally {
      setIsSubmitting(false);
    }
  }

  const aggregates = responsesState.data?.aggregates;
  const recentResponses = responsesState.data?.items ?? [];

  return (
    <div className="space-y-6">
      <PageHeader
        eyebrow="RSVP"
        title={eventState.data ? `${eventState.data.title} RSVP` : "RSVP builder"}
        description="Настройте метаданные формы, обновляйте список вопросов и следите за ответами в одном builder screen."
        action={
          <div className="flex flex-wrap gap-3">
            <Button variant="secondary" asChild>
              <Link href={`/events/${eventId}`}>Back to event</Link>
            </Button>
            <Button onClick={() => setQuestions((current) => [...current, createEmptyQuestion(current.length)])}>
              <Plus className="size-4" />
              Add question
            </Button>
          </div>
        }
      />

      {eventState.isLoading || formState.isLoading || responsesState.isLoading ? (
        <LoadingState label="Loading RSVP builder..." />
      ) : null}

      {eventState.error || formState.error || responsesState.error ? (
        <ErrorState
          title="Could not load RSVP data"
          description={eventState.error ?? formState.error ?? responsesState.error ?? "Unexpected error"}
          onRetry={() => {
            void loadData();
          }}
        />
      ) : null}

      <div className="grid gap-6 xl:grid-cols-[1.15fr_0.85fr]">
        <div className="space-y-6">
          <SectionBlock
            title="Form settings"
            description="Базовые тексты и дедлайн формы. Для MVP список вопросов полностью перезаписывается при сохранении."
          >
            <div className="grid gap-4 md:grid-cols-2">
              <Field label="Title" htmlFor="rsvp-title" className="md:col-span-2">
                <Input id="rsvp-title" value={title} onChange={(event) => setTitle(event.target.value)} />
              </Field>
              <Field label="Description" htmlFor="rsvp-description" className="md:col-span-2">
                <Textarea
                  id="rsvp-description"
                  className="min-h-[110px]"
                  value={description}
                  onChange={(event) => setDescription(event.target.value)}
                />
              </Field>
              <Field label="Deadline" htmlFor="rsvp-deadline">
                <Input
                  id="rsvp-deadline"
                  type="datetime-local"
                  value={deadlineAt}
                  onChange={(event) => setDeadlineAt(event.target.value)}
                />
              </Field>
            </div>
          </SectionBlock>

          <div className="space-y-4">
            {questions.map((question, index) => (
              <RsvpQuestionEditor
                key={question.id}
                index={index}
                question={question}
                onChange={(nextQuestion) =>
                  setQuestions((current) =>
                    current.map((item) => (item.id === nextQuestion.id ? nextQuestion : item))
                  )
                }
                onRemove={() =>
                  setQuestions((current) => current.filter((item) => item.id !== question.id))
                }
              />
            ))}
          </div>

          <div className="flex justify-end">
            <Button onClick={() => void handleSave()} disabled={isSubmitting}>
              <SquarePen className="size-4" />
              {isSubmitting ? "Saving form..." : "Save RSVP form"}
            </Button>
          </div>
        </div>

        <div className="space-y-6">
          <div className="grid gap-4 sm:grid-cols-2">
            <InfoCard label="Questions" value={String(questions.length)} />
            <InfoCard label="Required" value={String(questions.filter((question) => question.required).length)} />
          </div>

          <RsvpFormPreview
            title={title}
            description={description}
            deadlineAt={deadlineAt ? new Date(deadlineAt).toISOString() : ""}
            questions={questions}
          />

          <SectionBlock title="Response aggregates">
            <div className="grid gap-3 sm:grid-cols-2">
              <Stat label="Total guests" value={String(aggregates?.total ?? 0)} />
              <Stat label="Confirmed" value={String(aggregates?.confirmed ?? 0)} />
              <Stat label="Declined" value={String(aggregates?.declined ?? 0)} />
              <Stat label="Maybe" value={String(aggregates?.maybe ?? 0)} />
              <Stat label="Pending" value={String(aggregates?.pending ?? 0)} />
            </div>
          </SectionBlock>

          <SectionBlock title="Latest responses">
            <div className="space-y-3">
              {recentResponses.length ? (
                recentResponses.map((response) => (
                  <div key={response.id} className="rounded-2xl bg-secondary/35 px-4 py-3 text-sm">
                    <div className="font-medium">{response.status}</div>
                    <div className="text-muted-foreground">
                      Plus ones: {response.plus_one_count} · Submitted{" "}
                      {response.submitted_at ? new Date(response.submitted_at).toLocaleString() : "—"}
                    </div>
                  </div>
                ))
              ) : (
                <div className="rounded-2xl border border-dashed px-4 py-6 text-sm text-muted-foreground">
                  No RSVP responses yet.
                </div>
              )}
            </div>
          </SectionBlock>
        </div>
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

function Stat({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-2xl bg-secondary/45 px-4 py-3">
      <div className="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">
        {label}
      </div>
      <div className="mt-1 text-xl font-semibold">{value}</div>
    </div>
  );
}

function createEmptyQuestion(index: number): EditableQuestion {
  return {
    id: `draft-${Date.now()}-${index}`,
    code: `question_${index + 1}`,
    label: "",
    question_type: "text",
    required: false,
    options: []
  };
}
