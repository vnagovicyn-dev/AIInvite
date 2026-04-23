import Link from "next/link";

import { Button } from "@/components/ui/button";

export function PublicTopbar() {
  return (
    <header className="sticky top-0 z-40 border-b border-white/70 bg-background/80 backdrop-blur">
      <div className="mx-auto flex h-[76px] w-full max-w-7xl items-center justify-between px-4 sm:px-6 lg:px-8">
        <div className="flex items-center gap-4">
          <Link href="/" className="font-[family-name:var(--font-heading)] text-2xl font-semibold">
            AIInvite
          </Link>
          <div className="hidden rounded-full border bg-white/72 px-3 py-1.5 text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground lg:block">
            Invitation platform
          </div>
        </div>
        <nav className="hidden items-center gap-6 text-sm text-muted-foreground md:flex">
          <Link href="/">Главная</Link>
          <Link href="/templates">Шаблоны</Link>
        </nav>
        <div className="flex items-center gap-2">
          <Button asChild variant="ghost">
            <Link href="/login">Войти</Link>
          </Button>
          <Button asChild>
            <Link href="/register">Создать аккаунт</Link>
          </Button>
        </div>
      </div>
    </header>
  );
}
