import type { PaginatedResponse } from "@/lib/types/common";

export type Template = {
  id: string;
  code: string;
  name: string;
  category: string;
  preview_url: string | null;
  is_active: boolean;
  created_at: string;
};

export type TemplateCategory = {
  category: string;
};

export type TemplatesResponse = PaginatedResponse<Template>;
