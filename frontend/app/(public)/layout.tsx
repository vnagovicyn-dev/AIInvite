import { PublicTopbar } from "@/components/layout/public-topbar";

export default function PublicLayout({
  children
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <div className="min-h-screen">
      <PublicTopbar />
      <main className="mx-auto flex min-h-[calc(100vh-72px)] w-full max-w-7xl flex-col px-4 pb-12 pt-6 sm:px-6 lg:px-8">
        {children}
      </main>
    </div>
  );
}
