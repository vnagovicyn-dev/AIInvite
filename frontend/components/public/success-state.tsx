import Link from "next/link";
import { CheckCircle2 } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";

export function SuccessState({
  title,
  description,
  actionLabel,
  actionHref
}: {
  title: string;
  description: string;
  actionLabel?: string;
  actionHref?: string;
}) {
  return (
    <Card className="border-accent/40 bg-gradient-to-br from-white to-accent/25 text-center">
      <CardContent className="px-6 py-12">
        <div className="mx-auto mb-5 inline-flex size-16 items-center justify-center rounded-full bg-accent text-accent-foreground">
          <CheckCircle2 className="size-7" />
        </div>
        <h2 className="font-[family-name:var(--font-heading)] text-3xl">{title}</h2>
        <p className="mx-auto mt-3 max-w-xl text-sm text-muted-foreground">{description}</p>
        {actionLabel && actionHref ? (
          <div className="mt-6">
            <Button asChild>
              <Link href={actionHref}>{actionLabel}</Link>
            </Button>
          </div>
        ) : null}
      </CardContent>
    </Card>
  );
}
