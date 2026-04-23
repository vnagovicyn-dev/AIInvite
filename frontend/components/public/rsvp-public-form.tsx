"use client";

import { FormEvent, useMemo, useState } from "react";
import { ApiClientError } from "@/lib/api/client";
import { rsvpApi } from "@/lib/api/rsvp";
import type { PublicInvitePage, PublicInviteRsvpQuestion } from "@/lib/types";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { ErrorState } from "@/components/states/error-state";
import { SuccessState } from "@/components/public/success-state";

type AnswersState = Record<string, string | number | boolean | string[]>;

export function RsvpPublicForm({
  slug,
  invitePage
}: {
  slug: string;
  invitePage: PublicInvitePage;
}) {
  const [status, setStatus] = useState<"confirmed" | "declined" | "maybe">("confirmed");
  const [plusOneCount, setPlusOneCount] = useState<number>(0);
  const [answers, setAnswers] = useState<AnswersState>({});
  const [validationError, setValidationError] = useState<string | null>(null);
  const [submitError, setSubmitError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);

  const questions = invitePage.rsvp.questions ?? [];
  const guest = invitePage.guest;
  const canBringPlusOne = guest?.plus_one_allowed ?? false;
  const deadlineLabel = invitePage.rsvp.deadline_at
    ? new Date(invitePage.rsvp.deadline_at).toLocaleString("ru-RU")
    : null;

  const defaultDescription = useMemo(() => {
    if (guest?.full_name) {
      return `Мы сохранили персональный контекст для ${guest.full_name}. Заполните форму ниже, чтобы отправить ответ организатору.`;
    }

    return "Заполните короткую форму и отправьте свой ответ организатору события.";
  }, [guest]);

  if (isSuccess) {
    return (
      <SuccessState
        title="Ответ отправлен"
        description="Спасибо, ваш RSVP сохранён. Организатор увидит ответ в кабинете события."
        actionLabel="Вернуться к приглашению"
        actionHref={`/invite/${slug}`}
      />
    );
  }

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setValidationError(null);
    setSubmitError(null);

    const requiredQuestion = questions.find((question) => {
      if (!question.required) {
        return false;
      }

      const value = answers[question.code];
      if (Array.isArray(value)) {
        return value.length === 0;
      }

      if (typeof value === "boolean") {
        return false;
      }

      return value === undefined || value === null || String(value).trim().length === 0;
    });

    if (requiredQuestion) {
      setValidationError(`Заполните обязательное поле: ${requiredQuestion.label}`);
      return;
    }

    setIsSubmitting(true);

    try {
      await rsvpApi.submitPublic(slug, {
        status,
        plus_one_count: canBringPlusOne ? plusOneCount : 0,
        guest_id: guest?.id,
        answers
      });
      setIsSuccess(true);
    } catch (error) {
      if (error instanceof ApiClientError && error.status === 404) {
        setSubmitError("Приглашение недоступно для отправки RSVP.");
      } else {
        setSubmitError(error instanceof Error ? error.message : "Не удалось отправить RSVP.");
      }
    } finally {
      setIsSubmitting(false);
    }
  }

  return (
    <Card className="border-white/80 bg-white/96">
      <CardHeader className="space-y-3">
        <CardTitle>{invitePage.rsvp.title}</CardTitle>
        <p className="text-sm text-muted-foreground">
          {invitePage.rsvp.description || defaultDescription}
        </p>
        {deadlineLabel ? (
          <p className="text-xs font-medium uppercase tracking-[0.16em] text-muted-foreground">
            Ответить до {deadlineLabel}
          </p>
        ) : null}
      </CardHeader>
      <CardContent className="space-y-6">
        <form className="space-y-6" onSubmit={handleSubmit}>
          <div className="space-y-3">
            <Label>Ваш статус</Label>
            <div className="flex flex-wrap gap-2">
              {[
                { value: "confirmed", label: "Подтверждаю" },
                { value: "maybe", label: "Пока не уверен" },
                { value: "declined", label: "Не смогу прийти" }
              ].map((option) => (
                <Button
                  key={option.value}
                  type="button"
                  variant={status === option.value ? "default" : "secondary"}
                  onClick={() => setStatus(option.value as typeof status)}
                >
                  {option.label}
                </Button>
              ))}
            </div>
          </div>

          {canBringPlusOne ? (
            <div className="space-y-2 rounded-[1.4rem] border bg-secondary/28 p-4">
              <Label htmlFor="plus-one-count">Количество гостей с вами</Label>
              <Input
                id="plus-one-count"
                type="number"
                min={0}
                max={5}
                value={String(plusOneCount)}
                onChange={(event) => setPlusOneCount(Number(event.target.value || 0))}
              />
            </div>
          ) : null}

          {questions.map((question) => (
            <QuestionField
              key={question.id}
              question={question}
              value={answers[question.code]}
              onChange={(value) =>
                setAnswers((current) => ({
                  ...current,
                  [question.code]: value
                }))
              }
            />
          ))}

          {validationError ? (
            <ErrorState title="Проверьте форму" description={validationError} />
          ) : null}
          {submitError ? <ErrorState title="RSVP не отправлен" description={submitError} /> : null}

          <Button type="submit" size="lg" disabled={isSubmitting}>
            {isSubmitting ? "Отправляем..." : "Отправить RSVP"}
          </Button>
        </form>
      </CardContent>
    </Card>
  );
}

function QuestionField({
  question,
  value,
  onChange
}: {
  question: PublicInviteRsvpQuestion;
  value: string | number | boolean | string[] | undefined;
  onChange: (value: string | number | boolean | string[]) => void;
}) {
  const options = normalizeOptions(question.options);

  if (question.question_type === "textarea") {
    return (
      <div className="space-y-2">
        <Label htmlFor={question.code}>
          {question.label}
          {question.required ? " *" : ""}
        </Label>
        <Textarea
          id={question.code}
          className="min-h-[140px]"
          value={typeof value === "string" ? value : ""}
          onChange={(event) => onChange(event.target.value)}
        />
      </div>
    );
  }

  if (question.question_type === "select") {
    return (
      <div className="space-y-2">
        <Label htmlFor={question.code}>
          {question.label}
          {question.required ? " *" : ""}
        </Label>
        <select
          id={question.code}
          className="flex h-12 w-full rounded-2xl border bg-background px-4 text-sm outline-none transition focus:border-primary focus:ring-2 focus:ring-primary/20"
          value={typeof value === "string" ? value : ""}
          onChange={(event) => onChange(event.target.value)}
        >
          <option value="">Выберите вариант</option>
          {options.map((option) => (
            <option key={option} value={option}>
              {option}
            </option>
          ))}
        </select>
      </div>
    );
  }

  if (question.question_type === "multiselect") {
    const selected = Array.isArray(value) ? value : [];

    return (
      <div className="space-y-3">
        <Label>
          {question.label}
          {question.required ? " *" : ""}
        </Label>
        <div className="grid gap-2">
          {options.map((option) => {
            const checked = selected.includes(option);

            return (
              <label
                key={option}
                className="flex items-center gap-3 rounded-2xl border bg-secondary/35 px-4 py-3 text-sm"
              >
                <input
                  type="checkbox"
                  checked={checked}
                  onChange={(event) => {
                    if (event.target.checked) {
                      onChange([...selected, option]);
                    } else {
                      onChange(selected.filter((item) => item !== option));
                    }
                  }}
                />
                <span>{option}</span>
              </label>
            );
          })}
        </div>
      </div>
    );
  }

  if (question.question_type === "boolean") {
    return (
      <div className="space-y-2">
        <Label>{question.label}</Label>
        <div className="flex flex-wrap gap-2">
          <Button
            type="button"
            variant={value === true ? "default" : "secondary"}
            onClick={() => onChange(true)}
          >
            Да
          </Button>
          <Button
            type="button"
            variant={value === false ? "default" : "secondary"}
            onClick={() => onChange(false)}
          >
            Нет
          </Button>
        </div>
      </div>
    );
  }

  if (question.question_type === "number") {
    return (
      <div className="space-y-2">
        <Label htmlFor={question.code}>
          {question.label}
          {question.required ? " *" : ""}
        </Label>
        <Input
          id={question.code}
          type="number"
          value={typeof value === "number" ? String(value) : ""}
          onChange={(event) => onChange(Number(event.target.value || 0))}
        />
      </div>
    );
  }

  return (
    <div className="space-y-2">
      <Label htmlFor={question.code}>
        {question.label}
        {question.required ? " *" : ""}
      </Label>
      <Input
        id={question.code}
        value={typeof value === "string" ? value : ""}
        onChange={(event) => onChange(event.target.value)}
      />
    </div>
  );
}

function normalizeOptions(options: unknown) {
  if (Array.isArray(options)) {
    return options.filter((option): option is string => typeof option === "string" && option.length > 0);
  }

  return [];
}
