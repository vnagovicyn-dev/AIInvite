import { apiRequest } from "@/lib/api/client";
import type {
  Guest,
  GuestCreateRequest,
  GuestImportSummary,
  GuestsResponse,
  GuestUpdateRequest
} from "@/lib/types";

export const guestsApi = {
  list(eventId: string, searchParams = "") {
    return apiRequest<GuestsResponse>(`/api/events/${eventId}/guests${searchParams}`, {
      auth: true
    });
  },
  create(eventId: string, payload: GuestCreateRequest) {
    return apiRequest<Guest>(`/api/events/${eventId}/guests`, {
      method: "POST",
      auth: true,
      body: JSON.stringify(payload)
    });
  },
  getById(eventId: string, guestId: string) {
    return apiRequest<Guest>(`/api/events/${eventId}/guests/${guestId}`, { auth: true });
  },
  update(eventId: string, guestId: string, payload: GuestUpdateRequest) {
    return apiRequest<Guest>(`/api/events/${eventId}/guests/${guestId}`, {
      method: "PATCH",
      auth: true,
      body: JSON.stringify(payload)
    });
  },
  remove(eventId: string, guestId: string) {
    return apiRequest<void>(`/api/events/${eventId}/guests/${guestId}`, {
      method: "DELETE",
      auth: true
    });
  },
  importCsv(eventId: string, file: File) {
    const formData = new FormData();
    formData.append("file", file);

    return apiRequest<GuestImportSummary>(`/api/events/${eventId}/guests/import`, {
      method: "POST",
      auth: true,
      body: formData
    });
  }
};
