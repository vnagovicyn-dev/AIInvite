import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function buildYandexMapsSearchUrl(query: string) {
  const normalizedQuery = query.trim();
  if (!normalizedQuery) {
    return "";
  }

  return `https://yandex.ru/maps/?text=${encodeURIComponent(normalizedQuery)}`;
}
