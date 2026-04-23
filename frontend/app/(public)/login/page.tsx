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

export default function LoginPage() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const { hydrated, isAuthenticated, login } = useAuth();
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
      const response = await authApi.login({ email, password });
      login(response.access_token, response.user, redirectTo);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to log in");
    } finally {
      setIsSubmitting(false);
    }
  }

  return (
    <AuthFormShell
      eyebrow="Welcome back"
      title="Log in to your workspace"
      description="Token storage, redirect handling and auth fetch wrapper are already prepared."
    >
      <Card>
        <CardHeader>
          <CardTitle>Log in</CardTitle>
        </CardHeader>
        <CardContent className="space-y-5">
          <form className="space-y-4" onSubmit={handleSubmit}>
            <div className="space-y-2">
              <Label htmlFor="login-email">Email</Label>
              <Input
                id="login-email"
                type="email"
                placeholder="you@example.com"
                value={email}
                onChange={(event) => setEmail(event.target.value)}
                required
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="login-password">Password</Label>
              <Input
                id="login-password"
                type="password"
                placeholder="Enter your password"
                value={password}
                onChange={(event) => setPassword(event.target.value)}
                required
              />
            </div>
            <div className="flex flex-wrap gap-3">
              <Button type="submit" disabled={isSubmitting}>
                {isSubmitting ? "Logging in..." : "Log in"}
              </Button>
              <Button variant="secondary" asChild>
                <Link
                  href={
                    redirectTo !== "/dashboard"
                      ? `/register?next=${encodeURIComponent(redirectTo)}`
                      : "/register"
                  }
                >
                  Create account
                </Link>
              </Button>
            </div>
          </form>
          {error ? <ErrorState title="Login failed" description={error} /> : null}
          <div className="rounded-2xl bg-secondary/50 px-4 py-3 text-sm text-muted-foreground">
            Protected routes in the app shell already redirect here when access token is missing.
          </div>
        </CardContent>
        <CardFooter className="text-xs text-muted-foreground">
          Use any backend-backed account from `/api/auth/register`.
        </CardFooter>
      </Card>
    </AuthFormShell>
  );
}
