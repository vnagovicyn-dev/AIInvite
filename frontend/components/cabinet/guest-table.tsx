import { Pencil, Trash2 } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { StatusBadge } from "@/components/ui/status-badge";
import type { Guest } from "@/lib/types";

type GuestTableProps = {
  guests: Guest[];
  onEdit: (guest: Guest) => void;
  onDelete: (guest: Guest) => void;
};

export function GuestTable({ guests, onEdit, onDelete }: GuestTableProps) {
  return (
    <>
      <Card className="hidden overflow-hidden lg:block">
        <CardHeader>
          <CardTitle>Guest list</CardTitle>
        </CardHeader>
        <CardContent className="overflow-x-auto">
          <table className="w-full min-w-[780px] text-left text-sm">
            <thead className="text-xs uppercase tracking-[0.16em] text-muted-foreground">
              <tr>
                <th className="pb-3">Guest</th>
                <th className="pb-3">Group</th>
                <th className="pb-3">Tags</th>
                <th className="pb-3">Status</th>
                <th className="pb-3">Contact</th>
                <th className="pb-3 text-right">Actions</th>
              </tr>
            </thead>
            <tbody>
              {guests.map((guest) => (
                <tr key={guest.id} className="border-t border-border/70">
                  <td className="py-4">
                    <div className="font-medium">{guest.full_name}</div>
                    <div className="text-muted-foreground">{guest.notes ?? "Без заметок"}</div>
                  </td>
                  <td className="py-4">
                    {guest.group_name ? <Badge variant="outline">{guest.group_name}</Badge> : "—"}
                  </td>
                  <td className="py-4">
                    <div className="flex flex-wrap gap-2">
                      {guest.tags.length ? guest.tags.map((tag) => <Badge key={tag} variant="secondary">{tag}</Badge>) : "—"}
                    </div>
                  </td>
                  <td className="py-4">
                    <div className="flex flex-wrap gap-2">
                      {guest.vip ? <StatusBadge status="vip" /> : null}
                      {guest.plus_one_allowed ? <Badge variant="secondary">Plus one</Badge> : null}
                      {guest.is_child ? <Badge variant="secondary">Child</Badge> : null}
                    </div>
                  </td>
                  <td className="py-4 text-muted-foreground">
                    <div>{guest.email ?? "—"}</div>
                    <div>{guest.phone ?? "—"}</div>
                  </td>
                  <td className="py-4 text-right">
                    <div className="flex justify-end gap-2">
                      <Button size="icon" variant="ghost" onClick={() => onEdit(guest)}>
                        <Pencil className="size-4" />
                      </Button>
                      <Button size="icon" variant="ghost" onClick={() => onDelete(guest)}>
                        <Trash2 className="size-4" />
                      </Button>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </CardContent>
      </Card>
      <div className="grid gap-4 lg:hidden">
        {guests.map((guest) => (
          <Card key={guest.id} className="border-white/75 bg-white/95">
            <CardContent className="pt-6">
              <div className="flex items-start justify-between gap-3">
                <div>
                  <div className="font-medium">{guest.full_name}</div>
                  <div className="mt-1 text-sm text-muted-foreground">{guest.group_name ?? "No group"}</div>
                  <div className="mt-3 flex flex-wrap gap-2">
                    {guest.vip ? <StatusBadge status="vip" /> : null}
                    {guest.group_name ? <Badge variant="outline">{guest.group_name}</Badge> : null}
                  </div>
                </div>
                <div className="flex gap-2">
                  <Button size="icon" variant="ghost" onClick={() => onEdit(guest)}>
                    <Pencil className="size-4" />
                  </Button>
                  <Button size="icon" variant="ghost" onClick={() => onDelete(guest)}>
                    <Trash2 className="size-4" />
                  </Button>
                </div>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    </>
  );
}
