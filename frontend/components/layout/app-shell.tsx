"use client";

import { Sidebar } from "@/components/layout/sidebar";
import { Topbar } from "@/components/layout/topbar";

export function AppShell({ children }: { children: React.ReactNode }) {
  return (
    <div className="min-h-screen bg-[linear-gradient(180deg,rgba(255,255,255,0.52),rgba(255,255,255,0))]">
      <div className="mx-auto flex min-h-screen max-w-[1620px] gap-5 px-3 py-3 sm:px-5 sm:py-5 lg:px-7">
        <Sidebar />
        <div className="flex min-h-[calc(100vh-1.5rem)] flex-1 flex-col rounded-[2rem] border border-white/75 bg-card/92 shadow-[0_24px_60px_rgba(15,23,42,0.07)] backdrop-blur">
          <Topbar />
          <div className="flex-1 px-4 py-5 sm:px-7 sm:py-7 lg:px-9">{children}</div>
        </div>
      </div>
    </div>
  );
}
