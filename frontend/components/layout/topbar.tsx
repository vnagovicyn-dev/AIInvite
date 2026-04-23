"use client";

import { LogOut, Search, ShieldCheck } from "lucide-react";

import { useAuth } from "@/hooks/use-auth";
import { Button } from "@/components/ui/button";

export function Topbar() {
  const { logout, isAuthenticated, user } = useAuth();

  return (
    <header className="flex h-[84px] items-center justify-between border-b border-white/70 px-4 sm:px-7">
      <div className="space-y-1">
        <p className="text-sm font-medium text-muted-foreground">Private workspace</p>
        <h2 className="font-[family-name:var(--font-heading)] text-[1.65rem]">Event builder</h2>
      </div>
      <div className="flex items-center gap-3">
        <div className="hidden items-center gap-2 rounded-full border bg-white/75 px-4 py-2 text-sm text-muted-foreground md:flex">
          <Search className="size-4" />
          Поиск появится позже
        </div>
        <div className="hidden items-center gap-2 rounded-full bg-accent px-3 py-2 text-sm text-accent-foreground md:flex">
          <ShieldCheck className="size-4" />
          {isAuthenticated ? "Авторизован" : "Гость"}
        </div>
        {user ? (
          <div className="hidden rounded-full border bg-white/78 px-4 py-2 text-sm text-muted-foreground lg:block">
            {user.full_name ?? user.email}
          </div>
        ) : null}
        <Button variant="ghost" size="icon" onClick={logout} aria-label="Logout">
          <LogOut className="size-4" />
        </Button>
      </div>
    </header>
  );
}
