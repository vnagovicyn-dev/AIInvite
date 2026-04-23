import { AlertTriangle } from "lucide-react";

import { Button } from "@/components/ui/button";

type ErrorStateProps = {
  title?: string;
  description: string;
  onRetry?: () => void;
};

export function ErrorState({
  title = "Something went wrong",
  description,
  onRetry
}: ErrorStateProps) {
  return (
    <div className="rounded-[1.6rem] border bg-white/92 px-6 py-10 shadow-[0_18px_40px_rgba(15,23,42,0.05)]">
      <div className="mb-4 inline-flex size-12 items-center justify-center rounded-2xl bg-destructive/10 text-destructive">
        <AlertTriangle className="size-5" />
      </div>
      <h3 className="text-lg font-semibold">{title}</h3>
      <p className="mt-2 text-sm leading-6 text-muted-foreground">{description}</p>
      {onRetry ? (
        <Button className="mt-5" onClick={onRetry}>
          Retry
        </Button>
      ) : null}
    </div>
  );
}
