import type { PaginatedResponse } from "@/lib/types/common";

export type Event = {
  id: string;
  title: string;
  slug: string;
  event_type: string;
  status: string;
  template_id: string | null;
  event_date: string | null;
  timezone: string;
  venue_name: string | null;
  venue_address: string | null;
  created_at: string;
  updated_at: string;
};

export type EventCreateRequest = {
  title: string;
  slug?: string;
  event_type: string;
  template_id?: string | null;
  event_date?: string | null;
  timezone?: string | null;
  venue_name?: string | null;
  venue_address?: string | null;
};

export type EventUpdateRequest = Partial<EventCreateRequest>;

export type EventsResponse = PaginatedResponse<Event>;
