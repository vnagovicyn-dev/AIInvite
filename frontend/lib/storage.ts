import type { User } from "@/lib/types";

const ACCESS_TOKEN_KEY = "aiinvite.access_token";
const USER_KEY = "aiinvite.user";

export const authStorage = {
  read() {
    if (typeof window === "undefined") {
      return { accessToken: null, user: null as User | null };
    }

    const accessToken = window.localStorage.getItem(ACCESS_TOKEN_KEY);
    const userJson = window.localStorage.getItem(USER_KEY);

    return {
      accessToken,
      user: userJson ? (JSON.parse(userJson) as User) : null
    };
  },
  write({ accessToken, user }: { accessToken: string; user: User }) {
    if (typeof window === "undefined") return;
    window.localStorage.setItem(ACCESS_TOKEN_KEY, accessToken);
    window.localStorage.setItem(USER_KEY, JSON.stringify(user));
  },
  clear() {
    if (typeof window === "undefined") return;
    window.localStorage.removeItem(ACCESS_TOKEN_KEY);
    window.localStorage.removeItem(USER_KEY);
  },
  patchUser(user: User | null) {
    if (typeof window === "undefined") return;
    if (user) {
      window.localStorage.setItem(USER_KEY, JSON.stringify(user));
    } else {
      window.localStorage.removeItem(USER_KEY);
    }
  },
  accessToken() {
    if (typeof window === "undefined") return null;
    return window.localStorage.getItem(ACCESS_TOKEN_KEY);
  }
};
