import type { PaginatedResponse } from "@/lib/types/common";

export type Guest = {
  id: string;
  event_id: string;
  invite_token: string;
  full_name: string;
  phone: string | null;
  email: string | null;
  group_name: string | null;
  tags: string[];
  plus_one_allowed: boolean;
  is_child: boolean;
  vip: boolean;
  notes: string | null;
  created_at: string;
};

export type GuestCreateRequest = {
  full_name: string;
  phone?: string | null;
  email?: string | null;
  group_name?: string | null;
  tags?: string[];
  plus_one_allowed?: boolean;
  is_child?: boolean;
  vip?: boolean;
  notes?: string | null;
};

export type GuestUpdateRequest = Partial<GuestCreateRequest>;

export type GuestImportSummary = {
  imported_count: number;
  skipped_count: number;
  errors: string[];
};

export type GuestsResponse = PaginatedResponse<Guest>;
