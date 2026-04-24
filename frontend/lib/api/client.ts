import { INTERNAL_API_BASE_URL, PUBLIC_API_BASE_URL } from "@/lib/constants";
import { authStorage } from "@/lib/storage";
import type { ApiErrorPayload } from "@/lib/types";

type RequestOptions = RequestInit & {
  auth?: boolean;
};

export class ApiClientError extends Error {
  status: number;

  constructor(message: string, status: number) {
    super(message);
    this.name = "ApiClientError";
    this.status = status;
  }
}

function getApiBaseUrl(): string {
  return typeof window === "undefined" ? INTERNAL_API_BASE_URL : PUBLIC_API_BASE_URL;
}

export async function apiRequest<T>(path: string, options: RequestOptions = {}): Promise<T> {
  const headers = new Headers(options.headers);

  if (!headers.has("Content-Type") && !(options.body instanceof FormData)) {
    headers.set("Content-Type", "application/json");
  }

  if (options.auth) {
    const token = authStorage.accessToken();
    if (token) {
      headers.set("Authorization", `Bearer ${token}`);
    }
  }

  const response = await fetch(`${getApiBaseUrl()}${path}`, {
    ...options,
    headers,
    cache: "no-store"
  });

  if (!response.ok) {
    let message = "Request failed";
    try {
      const payload = (await response.json()) as ApiErrorPayload;
      message = payload.error ?? message;
    } catch {
      message = response.statusText || message;
    }

    if (response.status === 401) {
      authStorage.clear();
      if (typeof window !== "undefined" && !window.location.pathname.startsWith("/login")) {
        window.location.href = "/login";
      }
    }

    throw new ApiClientError(message, response.status);
  }

  if (response.status === 204) {
    return undefined as T;
  }

  return (await response.json()) as T;
}
