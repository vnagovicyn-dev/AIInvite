import { apiRequest } from "@/lib/api/client";
import type {
  PublicRsvpSubmitRequest,
  PublicRsvpSubmitResponse,
  RsvpForm,
  RsvpFormUpdateRequest,
  RsvpResponse,
  RsvpResponsesResponse
} from "@/lib/types";

export const rsvpApi = {
  getForm(eventId: string) {
    return apiRequest<RsvpForm>(`/api/events/${eventId}/rsvp-form`, { auth: true });
  },
  updateForm(eventId: string, payload: RsvpFormUpdateRequest) {
    return apiRequest<RsvpForm>(`/api/events/${eventId}/rsvp-form`, {
      method: "PUT",
      auth: true,
      body: JSON.stringify(payload)
    });
  },
  listResponses(eventId: string, searchParams = "") {
    return apiRequest<RsvpResponsesResponse>(`/api/events/${eventId}/rsvp-responses${searchParams}`, {
      auth: true
    });
  },
  getResponse(eventId: string, responseId: string) {
    return apiRequest<RsvpResponse>(`/api/events/${eventId}/rsvp-responses/${responseId}`, {
      auth: true
    });
  },
  submitPublic(slug: string, payload: PublicRsvpSubmitRequest) {
    return apiRequest<PublicRsvpSubmitResponse>(`/api/public/${slug}/rsvp`, {
      method: "POST",
      body: JSON.stringify(payload)
    });
  }
};
