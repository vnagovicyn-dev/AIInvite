import type { Metadata } from "next";

import "@/app/globals.css";
import { AppProviders } from "@/lib/providers/app-providers";

export const metadata: Metadata = {
  title: "AIInvite",
  description: "Invitation platform frontend scaffold"
};

export default function RootLayout({
  children
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body className="min-h-screen font-[family-name:var(--font-body)]">
        <AppProviders>{children}</AppProviders>
      </body>
    </html>
  );
}
