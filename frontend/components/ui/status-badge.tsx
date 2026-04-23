import { Badge } from "@/components/ui/badge";

export function StatusBadge({ status }: { status: string }) {
  const normalized = status.toLowerCase();
  const variant = normalized === "published" || normalized === "active" || normalized === "enabled" || normalized === "vip"
    ? "success"
    : "secondary";

  return <Badge variant={variant}>{status}</Badge>;
}
