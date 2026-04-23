import { apiRequest } from "@/lib/api/client";
import type { Event, EventCreateRequest, EventsResponse, EventUpdateRequest } from "@/lib/types";

export const eventsApi = {
  list(searchParams = "") {
    return apiRequest<EventsResponse>(`/api/events${searchParams}`, { auth: true });
  },
  create(payload: EventCreateRequest) {
    return apiRequest<Event>("/api/events", {
      method: "POST",
      auth: true,
      body: JSON.stringify(payload)
    });
  },
  getById(id: string) {
    return apiRequest<Event>(`/api/events/${id}`, { auth: true });
  },
  update(id: string, payload: EventUpdateRequest) {
    return apiRequest<Event>(`/api/events/${id}`, {
      method: "PATCH",
      auth: true,
      body: JSON.stringify(payload)
    });
  },
  remove(id: string) {
    return apiRequest<void>(`/api/events/${id}`, {
      method: "DELETE",
      auth: true
    });
  },
  publish(id: string) {
    return apiRequest<Event>(`/api/events/${id}/publish`, {
      method: "POST",
      auth: true
    });
  },
  unpublish(id: string) {
    return apiRequest<Event>(`/api/events/${id}/unpublish`, {
      method: "POST",
      auth: true
    });
  }
};
