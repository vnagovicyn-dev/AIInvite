import Link from "next/link";

import { Button } from "@/components/ui/button";

export function PublicTopbar() {
  return (
    <header className="sticky top-0 z-40 border-b border-[var(--homepage-border)] bg-[rgba(252,251,248,0.9)] backdrop-blur">
      <div className="mx-auto flex min-h-[76px] w-full max-w-7xl items-center justify-between gap-4 px-4 py-3 sm:px-6 lg:px-8">
        <div className="flex items-center gap-4">
          <Link href="/" className="font-[family-name:var(--font-heading)] text-[1.8rem] font-extrabold tracking-[-0.04em] text-[var(--homepage-text)]">
            AI Invite
          </Link>
          <div className="homepage-warm-chip hidden rounded-full px-3 py-1.5 text-[11px] font-semibold uppercase tracking-[0.16em] xl:block">
            Онлайн-приглашения
          </div>
        </div>

        <nav className="hidden items-center gap-6 text-[15px] font-medium lg:flex">
          <Link href="/" className="homepage-link">Главная</Link>
          <Link href="/templates" className="homepage-link">Шаблоны</Link>
          <Link href="/#how-it-works" className="homepage-link">Как это работает</Link>
          <Link href="/#demo" className="homepage-link">Демо</Link>
        </nav>

        <div className="hidden items-center gap-2 lg:flex">
          <Button asChild variant="ghost">
            <Link href="/login" className="homepage-link">Войти</Link>
          </Button>
          <Button asChild className="homepage-primary-button">
            <Link href="/register">Создать приглашение</Link>
          </Button>
        </div>

        <details className="relative lg:hidden">
          <summary className="flex cursor-pointer list-none items-center rounded-full border border-[var(--homepage-border)] bg-white px-4 py-2 text-[15px] font-medium text-[var(--homepage-text)]">
            Меню
          </summary>
          <div className="absolute right-0 top-[calc(100%+10px)] z-50 w-72 rounded-[1.4rem] border border-[var(--homepage-border)] bg-white p-3 shadow-soft">
            <nav className="grid gap-1 text-[15px]">
              <MobileNavLink href="/">Главная</MobileNavLink>
              <MobileNavLink href="/templates">Шаблоны</MobileNavLink>
              <MobileNavLink href="/#how-it-works">Как это работает</MobileNavLink>
              <MobileNavLink href="/#demo">Демо</MobileNavLink>
              <MobileNavLink href="/login">Войти</MobileNavLink>
              <div className="pt-2">
                <Button asChild className="homepage-primary-button w-full">
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
      className="rounded-2xl px-3 py-2.5 text-[15px] font-medium text-[var(--homepage-text)] transition hover:bg-[var(--homepage-soft-lilac)] hover:text-[var(--homepage-accent)]"
    >
      {children}
    </Link>
  );
}
