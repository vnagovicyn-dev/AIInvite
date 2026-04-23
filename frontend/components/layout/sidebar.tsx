"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import {
  Camera,
  CalendarFold,
  LayoutPanelTop,
  LayoutDashboard,
  MessageCircleHeart,
  Sparkles,
  Users
} from "lucide-react";

import { cn } from "@/lib/utils";

const items = [
  { href: "/dashboard", label: "Dashboard", icon: LayoutDashboard },
  { href: "/events/new", label: "New event", icon: CalendarFold },
  { href: "/dashboard#guests", label: "Guests", icon: Users },
  { href: "/dashboard#rsvp", label: "RSVP", icon: MessageCircleHeart }
];

const upcomingItems = [
  { label: "Media", icon: Camera },
  { label: "AI", icon: Sparkles },
  { label: "Seating", icon: LayoutPanelTop }
];

export function Sidebar() {
  const pathname = usePathname();

  return (
    <aside className="hidden w-[286px] shrink-0 rounded-[2rem] border border-slate-900/80 bg-slate-950/96 px-4 py-5 text-slate-50 shadow-[0_24px_60px_rgba(15,23,42,0.28)] lg:flex lg:flex-col">
      <div className="mb-8 px-3">
        <div className="font-[family-name:var(--font-heading)] text-2xl">AIInvite</div>
        <p className="mt-2 text-sm leading-6 text-slate-300">
          Кабинет организатора для событий, гостей, RSVP и страниц приглашений.
        </p>
      </div>
      <nav className="space-y-1">
        {items.map(({ href, label, icon: Icon }) => {
          const active = pathname === href || pathname.startsWith(`${href}/`);

          return (
            <Link
              key={href}
              href={href}
              className={cn(
                "flex items-center gap-3 rounded-2xl px-3 py-3 text-sm transition-all duration-200",
                active
                  ? "bg-white text-slate-950 shadow-[0_10px_30px_rgba(255,255,255,0.14)]"
                  : "text-slate-300 hover:bg-white/10 hover:text-white"
              )}
            >
              <Icon className="size-4" />
              <span>{label}</span>
            </Link>
          );
        })}
      </nav>
      <div className="mt-8 px-3">
        <p className="mb-3 text-xs font-semibold uppercase tracking-[0.18em] text-slate-400">
          Скоро
        </p>
        <div className="space-y-1">
          {upcomingItems.map(({ label, icon: Icon }) => (
            <div
              key={label}
              className="flex items-center gap-3 rounded-2xl border border-white/5 bg-white/[0.03] px-3 py-3 text-sm text-slate-500"
            >
              <Icon className="size-4" />
              <span>{label}</span>
            </div>
          ))}
        </div>
      </div>
      <div className="mt-auto rounded-[1.4rem] border border-white/10 bg-white/10 p-4 text-sm leading-6 text-slate-300">
        Здесь уже собраны основные рабочие сценарии продукта. Media, AI и Seating можно будет
        аккуратно добавить позже, не ломая текущий кабинет.
      </div>
    </aside>
  );
}
