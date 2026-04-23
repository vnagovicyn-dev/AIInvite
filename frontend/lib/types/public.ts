export type PublicInviteGuestContext = {
  id: string;
  invite_token: string;
  full_name: string;
  group_name: string | null;
  plus_one_allowed: boolean;
  is_child: boolean;
  vip: boolean;
};

export type PublicInviteSection = {
  id: string;
  section_type: string;
  position: number;
  is_enabled: boolean;
  title: string | null;
  content: Record<string, unknown>;
};

export type PublicInviteRsvpQuestion = {
  id: string;
  position: number;
  code: string;
  label: string;
  question_type: string;
  required: boolean;
  options: unknown;
};

export type PublicInvitePage = {
  id: string;
  title: string;
  slug: string;
  event_type: string;
  status: string;
  event_date: string | null;
  timezone: string;
  venue_name: string | null;
  venue_address: string | null;
  sections: PublicInviteSection[];
  rsvp: {
    title: string;
    description: string | null;
    deadline_at: string | null;
    settings: Record<string, unknown>;
    questions: PublicInviteRsvpQuestion[];
  };
  guest: PublicInviteGuestContext | null;
};
