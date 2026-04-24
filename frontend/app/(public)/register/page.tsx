import { RegisterPageClient } from "@/components/auth/register-page-client";

export default async function RegisterPage({
  searchParams
}: {
  searchParams: Promise<{ next?: string }>;
}) {
  const { next } = await searchParams;

  return <RegisterPageClient redirectTo={next ?? "/dashboard"} />;
}
