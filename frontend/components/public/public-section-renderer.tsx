import Link from "next/link";
import { Gift, ImageIcon, PlayCircle } from "lucide-react";

import { FaqSection } from "@/components/public/faq-section";
import { LocationSection } from "@/components/public/location-section";
import { ProgramSection } from "@/components/public/program-section";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import type { PublicInviteSection } from "@/lib/types";

export function PublicSectionRenderer({
  section,
  eventTitle,
  venueName,
  venueAddress,
  rsvpHref
}: {
  section: PublicInviteSection;
  eventTitle: string;
  venueName?: string | null;
  venueAddress?: string | null;
  rsvpHref: string;
}) {
  if (!section.is_enabled) {
    return null;
  }

  switch (section.section_type) {
    case "hero":
      return <HeroSection section={section} eventTitle={eventTitle} />;
    case "story":
      return <StorySection section={section} />;
    case "program":
      return (
        <ProgramSection
          title={section.title}
          items={readObjectArray(section.content.items).map((item) => ({
            time: readString(item.time) ?? undefined,
            title: readString(item.title) ?? undefined,
            description: readString(item.description) ?? undefined
          }))}
        />
      );
    case "location":
      return (
        <LocationSection
          title={section.title}
          venueName={venueName}
          venueAddress={venueAddress}
          content={section.content}
        />
      );
    case "faq":
      return (
        <FaqSection
          title={section.title}
          items={readObjectArray(section.content.items).map((item) => ({
            question: readString(item.question) ?? undefined,
            answer: readString(item.answer) ?? undefined
          }))}
        />
      );
    case "dress_code":
      return <SimpleTextSection title={section.title || "Дресс-код"} section={section} />;
    case "gift":
      return <GiftSection section={section} />;
    case "gallery":
      return <PlaceholderSection icon={ImageIcon} title={section.title || "Галерея"} section={section} />;
    case "video":
      return <PlaceholderSection icon={PlayCircle} title={section.title || "Видео"} section={section} />;
    case "rsvp":
      return <RsvpSection title={section.title} section={section} rsvpHref={rsvpHref} />;
    default:
      return <SimpleTextSection title={section.title || section.section_type} section={section} />;
  }
}

function HeroSection({
  section,
  eventTitle
}: {
  section: PublicInviteSection;
  eventTitle: string;
}) {
  const headline =
    readString(section.content.headline) ||
    readString(section.content.title) ||
    section.title ||
    eventTitle;
  const subheadline =
    readString(section.content.subheadline) ||
    readString(section.content.description) ||
    null;

  return (
    <section className="relative overflow-hidden rounded-[2.2rem] border border-white/70 bg-white/92 p-8 text-center shadow-soft sm:p-12">
      <div className="absolute inset-0 bg-hero-grid bg-[size:26px_26px] opacity-20" />
      <div className="relative mx-auto max-w-3xl space-y-4">
        <p className="text-sm font-semibold uppercase tracking-[0.2em] text-muted-foreground">
          Invitation
        </p>
        <h2 className="font-[family-name:var(--font-heading)] text-4xl leading-tight sm:text-5xl">
          {headline}
        </h2>
        {subheadline ? <p className="text-lg text-muted-foreground">{subheadline}</p> : null}
      </div>
    </section>
  );
}

function StorySection({ section }: { section: PublicInviteSection }) {
  const paragraphs = readStringArray(section.content.paragraphs);
  const description =
    readString(section.content.story) ||
    readString(section.content.description) ||
    null;

  if (!section.title && paragraphs.length === 0 && !description) {
    return null;
  }

  return (
    <Card className="border-white/80 bg-white/92">
      <CardHeader>
        <CardTitle>{section.title || "История"}</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4 text-base leading-8 text-muted-foreground">
        {description ? <p>{description}</p> : null}
        {paragraphs.map((paragraph, index) => (
          <p key={`${paragraph}-${index}`}>{paragraph}</p>
        ))}
      </CardContent>
    </Card>
  );
}

function SimpleTextSection({
  title,
  section
}: {
  title: string;
  section: PublicInviteSection;
}) {
  const paragraphs = readStringArray(section.content.paragraphs);
  const text =
    readString(section.content.text) ||
    readString(section.content.description) ||
    readString(section.content.content) ||
    null;

  if (!title && paragraphs.length === 0 && !text) {
    return null;
  }

  return (
    <Card className="border-white/80 bg-white/92">
      <CardHeader>
        <CardTitle>{title}</CardTitle>
      </CardHeader>
      <CardContent className="space-y-3 text-sm leading-7 text-muted-foreground">
        {text ? <p>{text}</p> : null}
        {paragraphs.map((paragraph, index) => (
          <p key={`${paragraph}-${index}`}>{paragraph}</p>
        ))}
      </CardContent>
    </Card>
  );
}

function GiftSection({ section }: { section: PublicInviteSection }) {
  const title = section.title || "Подарки";
  const text =
    readString(section.content.text) ||
    readString(section.content.description) ||
    null;

  if (!text && readObjectArray(section.content.items).length === 0) {
    return (
      <Card className="border-white/80 bg-white/92">
        <CardHeader>
          <CardTitle>{title}</CardTitle>
        </CardHeader>
        <CardContent className="text-sm text-muted-foreground">
          Детали по подаркам будут добавлены позже.
        </CardContent>
      </Card>
    );
  }

  return (
    <Card className="border-white/80 bg-white/92">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Gift className="size-5 text-muted-foreground" />
          {title}
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-3 text-sm text-muted-foreground">
        {text ? <p>{text}</p> : null}
        {readObjectArray(section.content.items).map((item, index) => (
          <div key={index} className="rounded-2xl bg-secondary/45 px-4 py-3">
            <div className="font-medium text-foreground">
              {readString(item.title) || "Подсказка"}
            </div>
            {readString(item.description) ? (
              <p className="mt-1">{readString(item.description)}</p>
            ) : null}
          </div>
        ))}
      </CardContent>
    </Card>
  );
}

function PlaceholderSection({
  icon: Icon,
  title,
  section
}: {
  icon: React.ComponentType<{ className?: string }>;
  title: string;
  section: PublicInviteSection;
}) {
  const text =
    readString(section.content.text) ||
    readString(section.content.description) ||
    "Медиа-блок будет расширен на следующем этапе.";

  return (
    <Card className="border-white/80 bg-white/90">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Icon className="size-5 text-muted-foreground" />
          {title}
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="flex aspect-[16/8] items-center justify-center rounded-[1.6rem] border border-dashed bg-secondary/35 text-sm text-muted-foreground">
          Placeholder
        </div>
        <p className="text-sm text-muted-foreground">{text}</p>
      </CardContent>
    </Card>
  );
}

function RsvpSection({
  title,
  section,
  rsvpHref
}: {
  title?: string | null;
  section: PublicInviteSection;
  rsvpHref: string;
}) {
  const text =
    readString(section.content.text) ||
    readString(section.content.description) ||
    "Подтвердите участие через короткую форму RSVP.";

  return (
    <Card className="border-accent/40 bg-gradient-to-br from-accent/45 via-white to-secondary/60">
      <CardHeader>
        <CardTitle>{title || "RSVP"}</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <p className="text-sm text-muted-foreground">{text}</p>
        <Button asChild>
          <Link href={rsvpHref}>Открыть форму RSVP</Link>
        </Button>
      </CardContent>
    </Card>
  );
}

function readString(value: unknown) {
  return typeof value === "string" && value.trim().length > 0 ? value.trim() : null;
}

function readStringArray(value: unknown) {
  if (!Array.isArray(value)) {
    return [];
  }

  return value.filter((item): item is string => typeof item === "string" && item.trim().length > 0);
}

function readObjectArray(value: unknown) {
  if (!Array.isArray(value)) {
    return [];
  }

  return value.filter((item): item is Record<string, unknown> => Boolean(item) && typeof item === "object");
}
