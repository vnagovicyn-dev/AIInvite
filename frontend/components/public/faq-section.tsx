import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

type FaqItem = {
  question?: string;
  answer?: string;
};

export function FaqSection({
  title,
  items
}: {
  title?: string | null;
  items: FaqItem[];
}) {
  if (items.length === 0) {
    return null;
  }

  return (
    <Card className="border-white/80 bg-white/92">
      <CardHeader>
        <CardTitle>{title || "Частые вопросы"}</CardTitle>
      </CardHeader>
      <CardContent className="grid gap-3">
        {items.map((item, index) => (
          <div key={`${item.question ?? "faq"}-${index}`} className="rounded-[1.5rem] border bg-background px-4 py-4">
            <div className="font-semibold">{item.question ?? "Вопрос"}</div>
            {item.answer ? <p className="mt-2 text-sm leading-6 text-muted-foreground">{item.answer}</p> : null}
          </div>
        ))}
      </CardContent>
    </Card>
  );
}
