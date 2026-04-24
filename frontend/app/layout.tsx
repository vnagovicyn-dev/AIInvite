import type { Metadata } from "next";
import { Inter, Manrope } from "next/font/google";

import "@/app/globals.css";
import { AppProviders } from "@/lib/providers/app-providers";

const inter = Inter({
  subsets: ["latin", "cyrillic"],
  variable: "--font-body",
  display: "swap"
});

const manrope = Manrope({
  subsets: ["latin", "cyrillic"],
  variable: "--font-heading",
  display: "swap"
});

export const metadata: Metadata = {
  title: "AIInvite",
  description: "Invitation platform frontend scaffold"
};

export default function RootLayout({
  children
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="ru" className={`${inter.variable} ${manrope.variable}`}>
      <body className="min-h-screen font-[family-name:var(--font-body)]">
        <AppProviders>{children}</AppProviders>
      </body>
    </html>
  );
}
