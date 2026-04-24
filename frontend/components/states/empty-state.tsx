import { LucideIcon } from "lucide-react";

type EmptyStateProps = {
  icon: LucideIcon;
  title: string;
  description: string;
  action?: React.ReactNode;
};

export function EmptyState({ icon: Icon, title, description, action }: EmptyStateProps) {
  return (
    <div className="rounded-[1.7rem] border border-dashed bg-white/86 px-6 py-14 text-center shadow-[0_18px_40px_rgba(15,23,42,0.05)]">
      <div className="mx-auto mb-5 inline-flex size-14 items-center justify-center rounded-3xl bg-secondary text-secondary-foreground">
        <Icon className="size-6" />
      </div>
      <h3 className="font-[family-name:var(--font-heading)] text-[1.55rem] font-bold leading-[1.15] tracking-[-0.025em]">
        {title}
      </h3>
      <p className="mx-auto mt-3 max-w-xl text-[1rem] leading-7 text-muted-foreground">{description}</p>
      {action ? <div className="mt-6">{action}</div> : null}
    </div>
  );
}
