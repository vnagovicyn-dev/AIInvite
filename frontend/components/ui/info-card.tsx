import { Card, CardContent } from "@/components/ui/card";

export function InfoCard({
  label,
  value,
  hint
}: {
  label: string;
  value: string;
  hint?: string;
}) {
  return (
    <Card className="h-full border-white/80 bg-white/92">
      <CardContent className="pt-6">
        <div className="text-[11px] font-semibold uppercase tracking-[0.16em] text-muted-foreground">
          {label}
        </div>
        <div className="mt-2 text-lg font-semibold leading-tight">{value}</div>
        {hint ? <div className="mt-2 text-sm text-muted-foreground">{hint}</div> : null}
      </CardContent>
    </Card>
  );
}
