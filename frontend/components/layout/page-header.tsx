type PageHeaderProps = {
  eyebrow?: string;
  title: string;
  description?: string;
  action?: React.ReactNode;
};

export function PageHeader({ eyebrow, title, description, action }: PageHeaderProps) {
  return (
    <div className="flex flex-col gap-5 rounded-[1.7rem] border border-white/75 bg-white/88 px-5 py-6 sm:flex-row sm:items-end sm:justify-between sm:px-6 sm:py-7">
      <div className="space-y-3">
        {eyebrow ? (
          <p className="text-xs font-semibold uppercase tracking-[0.2em] text-primary">{eyebrow}</p>
        ) : null}
        <div className="space-y-2">
          <h1 className="font-[family-name:var(--font-heading)] text-3xl leading-tight sm:text-[2.1rem]">
            {title}
          </h1>
          {description ? <p className="max-w-3xl text-sm leading-6 text-muted-foreground">{description}</p> : null}
        </div>
      </div>
      {action ? <div className="shrink-0">{action}</div> : null}
    </div>
  );
}
