import { ProtectedAppShell } from "@/components/auth/protected-app-shell";

export default function PrivateLayout({
  children
}: Readonly<{ children: React.ReactNode }>) {
  return <ProtectedAppShell>{children}</ProtectedAppShell>;
}
