import { apiRequest } from "@/lib/api/client";
import type {
  CreateSectionRequest,
  PageSection,
  PageSectionsResponse,
  ReorderSectionsRequest,
  UpdateSectionRequest
} from "@/lib/types";

export const sectionsApi = {
  list(eventId: string) {
    return apiRequest<PageSectionsResponse>(`/api/events/${eventId}/sections`, { auth: true });
  },
  create(eventId: string, payload: CreateSectionRequest) {
    return apiRequest<PageSection>(`/api/events/${eventId}/sections`, {
      method: "POST",
      auth: true,
      body: JSON.stringify(payload)
    });
  },
  update(eventId: string, sectionId: string, payload: UpdateSectionRequest) {
    return apiRequest<PageSection>(`/api/events/${eventId}/sections/${sectionId}`, {
      method: "PATCH",
      auth: true,
      body: JSON.stringify(payload)
    });
  },
  remove(eventId: string, sectionId: string) {
    return apiRequest<void>(`/api/events/${eventId}/sections/${sectionId}`, {
      method: "DELETE",
      auth: true
    });
  },
  reorder(eventId: string, payload: ReorderSectionsRequest) {
    return apiRequest<PageSectionsResponse>(`/api/events/${eventId}/sections/reorder`, {
      method: "POST",
      auth: true,
      body: JSON.stringify(payload)
    });
  }
};
