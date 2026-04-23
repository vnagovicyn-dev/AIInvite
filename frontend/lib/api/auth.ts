import { apiRequest } from "@/lib/api/client";
import type { AuthResponse, LoginRequest, RegisterRequest, User } from "@/lib/types";

export const authApi = {
  register(payload: RegisterRequest) {
    return apiRequest<User>("/api/auth/register", {
      method: "POST",
      body: JSON.stringify(payload)
    });
  },
  login(payload: LoginRequest) {
    return apiRequest<AuthResponse>("/api/auth/login", {
      method: "POST",
      body: JSON.stringify(payload)
    });
  },
  me() {
    return apiRequest<User>("/api/auth/me", {
      method: "GET",
      auth: true
    });
  }
};
