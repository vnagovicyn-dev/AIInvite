import Link from "next/link";
import { ArrowRight, Eye } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import type { Template } from "@/lib/types";

export function TemplateCard({ template }: { template: Template }) {
  return (
    <Card className="h-full overflow-hidden border-white/75 bg-white/92">
      <div className="relative aspect-[4/3] border-b bg-gradient-to-br from-secondary via-background to-accent/25">
        {template.preview_url ? (
          <div
            className="absolute inset-0 bg-cover bg-center"
            style={{ backgroundImage: `url(${template.preview_url})` }}
          />
        ) : null}
        <div className="absolute inset-0 bg-gradient-to-b from-transparent via-transparent to-black/10" />
        <div className="absolute left-4 top-4 flex gap-2">
          <Badge variant={template.is_active ? "success" : "outline"}>
            {template.is_active ? "active" : "inactive"}
          </Badge>
          <Badge variant="outline" className="bg-white/85">
            {template.category}
          </Badge>
        </div>
      </div>
      <CardHeader className="gap-3">
        <CardTitle>{template.name}</CardTitle>
        <p className="text-sm leading-6 text-muted-foreground">
          Шаблон для публичной страницы приглашения, который можно адаптировать под своё событие.
        </p>
      </CardHeader>
      <CardContent className="text-sm text-muted-foreground">
        <div className="flex items-center justify-between rounded-2xl bg-secondary/50 px-4 py-3">
          <span className="flex items-center gap-2">
            <Eye className="size-4" />
            Preview
          </span>
          <span>{template.code}</span>
        </div>
      </CardContent>
      <CardFooter>
        <Button asChild className="w-full justify-between">
          <Link href={`/templates/${template.id}`}>
            Открыть шаблон
            <ArrowRight className="size-4" />
          </Link>
        </Button>
      </CardFooter>
    </Card>
  );
}
