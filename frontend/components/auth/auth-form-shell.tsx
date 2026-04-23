type AuthFormShellProps = {
  eyebrow: string;
  title: string;
  description: string;
  children: React.ReactNode;
};

export function AuthFormShell({
  eyebrow,
  title,
  description,
  children
}: AuthFormShellProps) {
  return (
    <div className="mx-auto grid w-full max-w-6xl gap-8 lg:grid-cols-[0.95fr_1.05fr] lg:items-start">
      <div className="rounded-[2rem] border bg-slate-950 px-8 py-10 text-white shadow-soft">
        <p className="text-sm font-semibold uppercase tracking-[0.18em] text-slate-300">{eyebrow}</p>
        <h1 className="mt-4 font-[family-name:var(--font-heading)] text-4xl leading-tight">{title}</h1>
        <p className="mt-4 max-w-lg text-slate-300">{description}</p>
      </div>
      <div>{children}</div>
    </div>
  );
}
