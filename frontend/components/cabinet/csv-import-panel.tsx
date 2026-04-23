"use client";

import { useState } from "react";
import { Upload } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import type { GuestImportSummary } from "@/lib/types";

type CsvImportPanelProps = {
  isSubmitting?: boolean;
  summary?: GuestImportSummary | null;
  onImport: (file: File) => Promise<void> | void;
};

export function CsvImportPanel({
  isSubmitting = false,
  summary,
  onImport
}: CsvImportPanelProps) {
  const [file, setFile] = useState<File | null>(null);

  return (
    <Card className="border-white/75 bg-white/95">
      <CardHeader>
        <CardTitle>CSV import</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <p className="text-sm leading-6 text-muted-foreground">
          Upload a simple CSV with columns: <code>full_name</code>, <code>phone</code>, <code>email</code>, <code>group_name</code>, <code>vip</code>.
        </p>
        <label className="block rounded-[1.4rem] border border-dashed bg-secondary/35 px-4 py-5 text-sm text-muted-foreground">
          <div className="font-medium text-foreground">Выберите CSV файл</div>
          <div className="mt-1">Файл с гостями можно импортировать без ручного заполнения каждой записи.</div>
          <input
            type="file"
            accept=".csv,text/csv"
            onChange={(event) => setFile(event.target.files?.[0] ?? null)}
            className="mt-4 block w-full text-sm"
          />
        </label>
        <Button
          variant="secondary"
          disabled={!file || isSubmitting}
          onClick={() => {
            if (file) {
              void onImport(file);
            }
          }}
        >
          <Upload className="size-4" />
          {isSubmitting ? "Importing..." : "Import guests"}
        </Button>
        {summary ? (
          <div className="rounded-[1.4rem] bg-accent/45 p-4 text-sm">
            <div className="font-semibold text-foreground">Import summary</div>
            <div className="mt-2">Imported: {summary.imported_count}</div>
            <div>Skipped: {summary.skipped_count}</div>
            {summary.errors.length ? (
              <ul className="mt-3 space-y-1 text-destructive">
                {summary.errors.map((error) => (
                  <li key={error}>{error}</li>
                ))}
              </ul>
            ) : null}
          </div>
        ) : null}
      </CardContent>
    </Card>
  );
}
