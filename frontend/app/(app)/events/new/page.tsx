import { EventCreatePageClient } from "@/components/cabinet/event-create-page-client";

export default async function EventCreatePage({
  searchParams
}: {
  searchParams: Promise<{ templateId?: string }>;
}) {
  const { templateId } = await searchParams;

  return <EventCreatePageClient initialTemplateId={templateId ?? null} />;
}
