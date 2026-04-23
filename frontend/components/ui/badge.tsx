import { cva, type VariantProps } from "class-variance-authority";

import { cn } from "@/lib/utils";

const badgeVariants = cva(
  "inline-flex items-center rounded-full px-3 py-1 text-[11px] font-semibold uppercase tracking-[0.12em]",
  {
    variants: {
      variant: {
        secondary: "bg-secondary text-secondary-foreground",
        success: "bg-accent text-accent-foreground",
        outline: "border bg-white/70 text-foreground"
      }
    },
    defaultVariants: {
      variant: "secondary"
    }
  }
);

export function Badge({
  variant,
  className,
  children
}: {
  variant?: VariantProps<typeof badgeVariants>["variant"];
  className?: string;
  children: React.ReactNode;
}) {
  return (
    <span
      className={cn(badgeVariants({ variant }), className)}
    >
      {children}
    </span>
  );
}
