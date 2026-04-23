import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import type { EditableQuestion } from "@/components/cabinet/rsvp-question-editor";

type RsvpFormPreviewProps = {
  title: string;
  description: string;
  deadlineAt: string;
  questions: EditableQuestion[];
};

export function RsvpFormPreview({
  title,
  description,
  deadlineAt,
  questions
}: RsvpFormPreviewProps) {
  return (
    <Card className="border-white/75 bg-white/95">
      <CardHeader>
        <CardTitle>{title || "RSVP preview"}</CardTitle>
        {description ? <p className="text-sm text-muted-foreground">{description}</p> : null}
        {deadlineAt ? (
          <p className="text-xs uppercase tracking-[0.16em] text-muted-foreground">
            Deadline · {new Date(deadlineAt).toLocaleString()}
          </p>
        ) : null}
      </CardHeader>
      <CardContent className="space-y-4">
        {questions.length ? (
          questions.map((question, index) => (
            <div key={question.id} className="rounded-[1.4rem] border bg-secondary/30 px-4 py-4">
              <div className="mb-2 text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">
                Question {index + 1}
              </div>
              <div className="font-medium">{question.label || "Untitled question"}</div>
              <div className="mt-1 text-sm text-muted-foreground">
                {question.question_type} {question.required ? "· required" : ""}
              </div>
              {Array.isArray(question.options) && question.options.length ? (
                <div className="mt-3 flex flex-wrap gap-2">
                  {question.options.map((option) => (
                    <span key={String(option)} className="rounded-full bg-background px-3 py-1 text-xs">
                      {String(option)}
                    </span>
                  ))}
                </div>
              ) : null}
            </div>
          ))
        ) : (
          <div className="rounded-2xl border border-dashed px-4 py-6 text-sm text-muted-foreground">
            Add RSVP questions to preview the public form.
          </div>
        )}
      </CardContent>
    </Card>
  );
}
