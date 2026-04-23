import type { PaginatedResponse } from "@/lib/types/common";

export type RsvpQuestion = {
  id: string;
  position: number;
  code: string;
  label: string;
  question_type: string;
  required: boolean;
  options: unknown;
};

export type RsvpForm = {
  id: string;
  event_id: string;
  title: string;
  description: string | null;
  deadline_at: string | null;
  settings: Record<string, unknown>;
  questions: RsvpQuestion[];
};

export type RsvpFormUpdateRequest = {
  title?: string;
  description?: string | null;
  deadline_at?: string | null;
  settings?: Record<string, unknown>;
  questions: Array<{
    code: string;
    label: string;
    question_type: string;
    required?: boolean;
    options?: unknown;
  }>;
};

export type RsvpResponse = {
  id: string;
  event_id: string;
  guest_id: string | null;
  status: string;
  plus_one_count: number;
  answers: Record<string, unknown>;
  submitted_at: string | null;
  created_at: string;
};

export type RsvpResponsesResponse = PaginatedResponse<RsvpResponse> & {
  aggregates: {
    total: number;
    confirmed: number;
    declined: number;
    maybe: number;
    pending: number;
  };
};

export type PublicRsvpSubmitRequest = {
  status: "confirmed" | "declined" | "maybe";
  plus_one_count?: number;
  guest_id?: string;
  answers: Record<string, unknown>;
};

export type PublicRsvpSubmitResponse = {
  id: string;
  status: string;
  submitted_at: string;
};
