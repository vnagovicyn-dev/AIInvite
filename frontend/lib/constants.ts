export const APP_NAME = "AIInvite";
export const API_BASE_URL =
  process.env.NEXT_PUBLIC_API_BASE_URL ?? "http://127.0.0.1:8080";

export const EVENT_TYPE_OPTIONS = [
  "wedding",
  "birthday",
  "corporate",
  "baby",
  "anniversary"
] as const;

export const SECTION_TYPE_OPTIONS = [
  "hero",
  "story",
  "program",
  "location",
  "dress_code",
  "faq",
  "gift",
  "gallery",
  "video",
  "rsvp"
] as const;

export const RSVP_QUESTION_TYPE_OPTIONS = [
  "text",
  "textarea",
  "select",
  "multiselect",
  "boolean",
  "number"
] as const;

export const EVENT_STATUS_OPTIONS = ["all", "draft", "published"] as const;

export const DEFAULT_TIMEZONE = "Europe/Amsterdam";
