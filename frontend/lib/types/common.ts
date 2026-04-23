export type ApiErrorPayload = {
  error: string;
};

export type PaginatedResponse<T> = {
  items: T[];
  page: number;
  per_page: number;
  total: number;
};
