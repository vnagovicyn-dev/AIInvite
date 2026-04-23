import { Clock3 } from "lucide-react";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

type ProgramItem = {
  time?: string;
  title?: string;
  description?: string;
};

export function ProgramSection({
  title,
  items
}: {
  title?: string | null;
  items: ProgramItem[];
}) {
  if (items.length === 0) {
    return null;
  }

  return (
    <Card className="border-white/80 bg-white/92">
      <CardHeader>
        <CardTitle>{title || "Программа"}</CardTitle>
      </CardHeader>
      <CardContent className="grid gap-3">
        {items.map((item, index) => (
          <div
            key={`${item.time ?? "no-time"}-${index}`}
            className="grid gap-3 rounded-[1.5rem] border bg-secondary/35 px-4 py-4 sm:grid-cols-[120px_1fr]"
          >
            <div className="flex items-center gap-2 font-semibold text-foreground">
              <Clock3 className="size-4 text-muted-foreground" />
              {item.time ?? "Время уточняется"}
            </div>
            <div className="space-y-1">
              <div className="font-semibold">{item.title ?? "Событие"}</div>
              {item.description ? (
                <p className="text-sm text-muted-foreground">{item.description}</p>
              ) : null}
            </div>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}
