export function LoadingState({ label = "Loading..." }: { label?: string }) {
  return (
    <div className="rounded-[1.6rem] border bg-white/92 px-6 py-10 shadow-[0_18px_40px_rgba(15,23,42,0.05)]">
      <div className="animate-pulse space-y-4">
        <div className="h-4 w-28 rounded-full bg-muted" />
        <div className="h-10 w-72 rounded-full bg-muted" />
        <div className="h-28 rounded-3xl bg-muted" />
      </div>
      <p className="mt-4 text-sm text-muted-foreground">{label}</p>
    </div>
  );
}
