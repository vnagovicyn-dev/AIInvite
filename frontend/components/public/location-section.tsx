import { MapPin } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { buildYandexMapsSearchUrl } from "@/lib/utils";

export function LocationSection({
  title,
  venueName,
  venueAddress,
  content
}: {
  title?: string | null;
  venueName?: string | null;
  venueAddress?: string | null;
  content: Record<string, unknown>;
}) {
  const name =
    readString(content.place_name) ||
    readString(content.venue_name) ||
    venueName ||
    null;
  const address =
    readString(content.address) ||
    readString(content.venue_address) ||
    venueAddress ||
    null;
  const note =
    readString(content.note) ||
    readString(content.description) ||
    null;

  if (!name && !address && !note) {
    return null;
  }

  const mapQuery = [name, address].filter(Boolean).join(", ");

  return (
    <Card className="border-white/80 bg-white/92">
      <CardHeader>
        <CardTitle>{title || "Место проведения"}</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="rounded-[1.7rem] bg-secondary/45 p-5">
          <div className="mb-3 inline-flex size-11 items-center justify-center rounded-2xl bg-background text-foreground">
            <MapPin className="size-5" />
          </div>
          {name ? <div className="text-lg font-semibold">{name}</div> : null}
          {address ? <p className="mt-2 text-sm text-muted-foreground">{address}</p> : null}
          {note ? <p className="mt-3 text-sm text-muted-foreground">{note}</p> : null}
        </div>
        {mapQuery ? (
          <Button asChild variant="secondary">
            <a href={buildYandexMapsSearchUrl(mapQuery)} target="_blank" rel="noreferrer">
              Открыть в Яндекс Картах
            </a>
          </Button>
        ) : null}
      </CardContent>
    </Card>
  );
}

function readString(value: unknown) {
  return typeof value === "string" && value.trim().length > 0 ? value : null;
}
