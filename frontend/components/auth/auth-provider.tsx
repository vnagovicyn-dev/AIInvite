"use client";

import { createContext, useCallback, useEffect, useMemo, useState } from "react";
import { usePathname, useRouter } from "next/navigation";

import { authApi } from "@/lib/api/auth";
import type { User } from "@/lib/types";
import { authStorage } from "@/lib/storage";

type AuthContextValue = {
  accessToken: string | null;
  user: User | null;
  hydrated: boolean;
  isAuthenticated: boolean;
  login: (accessToken: string, user: User, redirectTo?: string) => void;
  logout: () => void;
  setUser: (user: User | null) => void;
  refreshSession: () => Promise<User | null>;
};

export const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const router = useRouter();
  const pathname = usePathname();
  const [accessToken, setAccessToken] = useState<string | null>(null);
  const [user, setUser] = useState<User | null>(null);
  const [hydrated, setHydrated] = useState(false);

  const logout = useCallback(() => {
    authStorage.clear();
    setAccessToken(null);
    setUser(null);
    if (pathname.startsWith("/dashboard") || pathname.startsWith("/events")) {
      router.replace("/login");
    }
  }, [pathname, router]);

  const refreshSession = useCallback(async () => {
    const token = authStorage.accessToken();
    if (!token) {
      setAccessToken(null);
      setUser(null);
      return null;
    }

    try {
      const nextUser = await authApi.me();
      setAccessToken(token);
      setUser(nextUser);
      authStorage.patchUser(nextUser);
      return nextUser;
    } catch {
      authStorage.clear();
      setAccessToken(null);
      setUser(null);
      return null;
    }
  }, []);

  useEffect(() => {
    const snapshot = authStorage.read();
    setAccessToken(snapshot.accessToken);
    setUser(snapshot.user);
    if (!snapshot.accessToken) {
      setHydrated(true);
      return;
    }

    refreshSession().finally(() => {
      setHydrated(true);
    });
  }, [refreshSession]);

  const value = useMemo<AuthContextValue>(
    () => ({
      accessToken,
      user,
      hydrated,
      isAuthenticated: Boolean(accessToken),
      login: (nextToken, nextUser, redirectTo) => {
        authStorage.write({ accessToken: nextToken, user: nextUser });
        setAccessToken(nextToken);
        setUser(nextUser);
        router.push(redirectTo ?? "/dashboard");
      },
      logout,
      setUser: (nextUser) => {
        authStorage.patchUser(nextUser);
        setUser(nextUser);
      },
      refreshSession
    }),
    [accessToken, user, hydrated, router, logout, refreshSession]
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}
