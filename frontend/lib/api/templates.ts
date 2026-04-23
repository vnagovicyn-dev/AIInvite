import { apiRequest } from "@/lib/api/client";
import type { Template, TemplateCategory, TemplatesResponse } from "@/lib/types";

export const templatesApi = {
  list(searchParams = "") {
    return apiRequest<TemplatesResponse>(`/api/templates${searchParams}`);
  },
  getById(id: string) {
    return apiRequest<Template>(`/api/templates/${id}`);
  },
  categories() {
    return apiRequest<TemplateCategory[]>("/api/templates/categories");
  }
};
