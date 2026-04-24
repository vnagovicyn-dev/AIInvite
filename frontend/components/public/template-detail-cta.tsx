"use client";

import Link from "next/link";
import { useAuth } from "@/hooks/use-auth";
import { Button } from "@/components/ui/button";

export function TemplateDetailCta({
  templateId
}: {
  templateId: string;
}) {
  const { hydrated, isAuthenticated } = useAuth();
  const createHref = `/events/new?templateId=${encodeURIComponent(templateId)}`;
  const href = isAuthenticated
    ? createHref
    : `/register?next=${encodeURIComponent(createHref)}`;

  return (
    <div className="flex flex-wrap gap-3">
      <Button asChild size="lg" className="homepage-button-label">
        <Link href={href}>
          {hydrated && isAuthenticated ? "Использовать шаблон" : "Зарегистрироваться и использовать"}
        </Link>
      </Button>
      <Button asChild variant="secondary" size="lg" className="homepage-button-label">
        <Link href="/templates">Назад к каталогу</Link>
      </Button>
    </div>
  );
}
