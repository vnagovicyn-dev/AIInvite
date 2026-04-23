export type PageSection = {
  id: string;
  event_id: string;
  section_type: string;
  position: number;
  is_enabled: boolean;
  title: string | null;
  content: Record<string, unknown>;
  created_at: string;
  updated_at: string;
};

export type PageSectionsResponse = {
  items: PageSection[];
};

export type CreateSectionRequest = {
  section_type: string;
  title?: string | null;
  content: Record<string, unknown>;
  is_enabled?: boolean;
};

export type UpdateSectionRequest = {
  title?: string | null;
  content?: Record<string, unknown> | null;
  is_enabled?: boolean;
};

export type ReorderSectionsRequest = {
  section_ids: string[];
};
