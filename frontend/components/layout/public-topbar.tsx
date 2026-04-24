import Link from "next/link";

import { Button } from "@/components/ui/button";

export function PublicTopbar() {
  return (
    <header className="sticky top-0 z-40 border-b border-white/70 bg-background/80 backdrop-blur">
      <div className="mx-auto flex min-h-[76px] w-full max-w-7xl items-center justify-between gap-4 px-4 py-3 sm:px-6 lg:px-8">
        <div className="flex items-center gap-4">
          <Link href="/" className="font-[family-name:var(--font-heading)] text-[1.8rem] font-extrabold tracking-[-0.04em]">
            AI Invite
          </Link>
          <div className="hidden rounded-full border bg-white/72 px-3 py-1.5 text-[11px] font-semibold uppercase tracking-[0.16em] text-muted-foreground xl:block">
            Онлайн-приглашения
          </div>
        </div>

        <nav className="hidden items-center gap-6 text-[15px] font-medium text-muted-foreground lg:flex">
          <Link href="/">Главная</Link>
          <Link href="/templates">Шаблоны</Link>
          <Link href="/#how-it-works">Как это работает</Link>
          <Link href="/#demo">Демо</Link>
        </nav>

        <div className="hidden items-center gap-2 lg:flex">
          <Button asChild variant="ghost">
            <Link href="/login">Войти</Link>
          </Button>
          <Button asChild>
            <Link href="/register">Создать приглашение</Link>
          </Button>
        </div>

        <details className="relative lg:hidden">
          <summary className="flex cursor-pointer list-none items-center rounded-full border bg-white/86 px-4 py-2 text-[15px] font-medium text-foreground">
            Меню
          </summary>
          <div className="absolute right-0 top-[calc(100%+10px)] z-50 w-72 rounded-[1.4rem] border border-white/80 bg-white/96 p-3 shadow-soft">
            <nav className="grid gap-1 text-[15px]">
              <MobileNavLink href="/">Главная</MobileNavLink>
              <MobileNavLink href="/templates">Шаблоны</MobileNavLink>
              <MobileNavLink href="/#how-it-works">Как это работает</MobileNavLink>
              <MobileNavLink href="/#demo">Демо</MobileNavLink>
              <MobileNavLink href="/login">Войти</MobileNavLink>
              <div className="pt-2">
                <Button asChild className="w-full">
                  <Link href="/register">Создать приглашение</Link>
                </Button>
              </div>
            </nav>
          </div>
        </details>
      </div>
    </header>
  );
}

function MobileNavLink({
  href,
  children
}: {
  href: string;
  children: React.ReactNode;
}) {
  return (
    <Link
      href={href}
      className="rounded-2xl px-3 py-2.5 text-[15px] font-medium text-foreground transition hover:bg-secondary/65"
    >
      {children}
    </Link>
  );
}
