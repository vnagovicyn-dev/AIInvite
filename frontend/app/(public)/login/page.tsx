import { LoginPageClient } from "@/components/auth/login-page-client";

export default async function LoginPage({
  searchParams
}: {
  searchParams: Promise<{ next?: string }>;
}) {
  const { next } = await searchParams;

  return <LoginPageClient redirectTo={next ?? "/dashboard"} />;
}
