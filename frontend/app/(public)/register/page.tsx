"use client";

import { FormEvent, useEffect, useState } from "react";
import Link from "next/link";
import { useRouter, useSearchParams } from "next/navigation";

import { AuthFormShell } from "@/components/auth/auth-form-shell";
import { ErrorState } from "@/components/states/error-state";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useAuth } from "@/hooks/use-auth";
import { authApi } from "@/lib/api/auth";

export default function RegisterPage() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const { hydrated, isAuthenticated, login } = useAuth();
  const [fullName, setFullName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const redirectTo = searchParams.get("next") ?? "/dashboard";

  useEffect(() => {
    if (hydrated && isAuthenticated) {
      router.replace(redirectTo);
    }
  }, [hydrated, isAuthenticated, redirectTo, router]);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setError(null);
    setIsSubmitting(true);

    try {
      await authApi.register({
        email,
        password,
        full_name: fullName
      });

      const response = await authApi.login({ email, password });
      login(response.access_token, response.user, redirectTo);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to create account");
    } finally {
      setIsSubmitting(false);
    }
  }

  return (
    <AuthFormShell
      eyebrow="Get started"
      title="Create your organizer account"
      description="Registration page scaffold is ready for form fields, validation and auth bootstrap."
    >
      <Card>
        <CardHeader>
          <CardTitle>Create account</CardTitle>
        </CardHeader>
        <CardContent className="space-y-5">
          <form className="space-y-4" onSubmit={handleSubmit}>
            <div className="space-y-2">
              <Label htmlFor="register-full-name">Full name</Label>
              <Input
                id="register-full-name"
                type="text"
                placeholder="Anna Smith"
                value={fullName}
                onChange={(event) => setFullName(event.target.value)}
                required
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="register-email">Email</Label>
              <Input
                id="register-email"
                type="email"
                placeholder="you@example.com"
                value={email}
                onChange={(event) => setEmail(event.target.value)}
                required
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="register-password">Password</Label>
              <Input
                id="register-password"
                type="password"
                placeholder="Create a secure password"
                value={password}
                onChange={(event) => setPassword(event.target.value)}
                minLength={8}
                required
              />
            </div>
            <div className="flex flex-wrap gap-3">
              <Button type="submit" disabled={isSubmitting}>
                {isSubmitting ? "Creating account..." : "Create account"}
              </Button>
              <Button variant="secondary" asChild>
                <Link
                  href={
                    redirectTo !== "/dashboard"
                      ? `/login?next=${encodeURIComponent(redirectTo)}`
                      : "/login"
                  }
                >
                  Already have an account?
                </Link>
              </Button>
            </div>
          </form>
          {error ? <ErrorState title="Registration failed" description={error} /> : null}
          <div className="rounded-2xl bg-secondary/50 px-4 py-3 text-sm text-muted-foreground">
            Successful registration immediately signs the organizer into the private workspace.
          </div>
        </CardContent>
        <CardFooter className="text-xs text-muted-foreground">
          This screen is already wired to the Rust backend auth endpoints.
        </CardFooter>
      </Card>
    </AuthFormShell>
  );
}
