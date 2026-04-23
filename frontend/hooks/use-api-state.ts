"use client";

import { useCallback, useState } from "react";

export function useApiState<T>(initialData: T | null = null) {
  const [data, setData] = useState<T | null>(initialData);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const run = useCallback(async (request: () => Promise<T>) => {
    setIsLoading(true);
    setError(null);
    try {
      const next = await request();
      setData(next);
      return next;
    } catch (err) {
      const message = err instanceof Error ? err.message : "Unexpected error";
      setError(message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  return { data, setData, isLoading, error, run };
}
