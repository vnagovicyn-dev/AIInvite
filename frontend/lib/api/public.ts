import { apiRequest } from "@/lib/api/client";
import type { PublicInvitePage } from "@/lib/types";

export const publicApi = {
  getInvitePage(slug: string, inviteToken?: string) {
    const params = inviteToken ? `?invite_token=${encodeURIComponent(inviteToken)}` : "";
    return apiRequest<PublicInvitePage>(`/api/public/${slug}${params}`);
  }
};
