"use client";

import { useEffect, useState } from "react";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import type { Guest, GuestCreateRequest, GuestUpdateRequest } from "@/lib/types";

type GuestFormDialogProps = {
  open: boolean;
  guest?: Guest | null;
  isSubmitting?: boolean;
  onClose: () => void;
  onSubmit: (payload: GuestCreateRequest | GuestUpdateRequest) => Promise<void> | void;
};

export function GuestFormDialog({
  open,
  guest,
  isSubmitting = false,
  onClose,
  onSubmit
}: GuestFormDialogProps) {
  const [fullName, setFullName] = useState("");
  const [phone, setPhone] = useState("");
  const [email, setEmail] = useState("");
  const [groupName, setGroupName] = useState("");
  const [tags, setTags] = useState("");
  const [plusOneAllowed, setPlusOneAllowed] = useState(false);
  const [isChild, setIsChild] = useState(false);
  const [vip, setVip] = useState(false);
  const [notes, setNotes] = useState("");

  useEffect(() => {
    setFullName(guest?.full_name ?? "");
    setPhone(guest?.phone ?? "");
    setEmail(guest?.email ?? "");
    setGroupName(guest?.group_name ?? "");
    setTags((guest?.tags ?? []).join(", "));
    setPlusOneAllowed(guest?.plus_one_allowed ?? false);
    setIsChild(guest?.is_child ?? false);
    setVip(guest?.vip ?? false);
    setNotes(guest?.notes ?? "");
  }, [guest, open]);

  if (!open) {
    return null;
  }

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();

    await onSubmit({
      full_name: fullName.trim(),
      phone: phone.trim() || null,
      email: email.trim() || null,
      group_name: groupName.trim() || null,
      tags: tags
        .split(",")
        .map((value) => value.trim())
        .filter(Boolean),
      plus_one_allowed: plusOneAllowed,
      is_child: isChild,
      vip,
      notes: notes.trim() || null
    });
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/35 p-4 backdrop-blur-sm">
      <Card className="w-full max-w-2xl border-white/75 bg-white/96 shadow-[0_30px_70px_rgba(15,23,42,0.18)]">
        <CardHeader>
          <CardTitle>{guest ? "Edit guest" : "Add guest"}</CardTitle>
        </CardHeader>
        <CardContent>
          <form className="grid gap-4 md:grid-cols-2" onSubmit={handleSubmit}>
            <Field label="Full name" htmlFor="guest-full-name" className="md:col-span-2">
              <Input
                id="guest-full-name"
                value={fullName}
                onChange={(event) => setFullName(event.target.value)}
                required
              />
            </Field>
            <Field label="Phone" htmlFor="guest-phone">
              <Input id="guest-phone" value={phone} onChange={(event) => setPhone(event.target.value)} />
            </Field>
            <Field label="Email" htmlFor="guest-email">
              <Input id="guest-email" value={email} onChange={(event) => setEmail(event.target.value)} />
            </Field>
            <Field label="Group" htmlFor="guest-group">
              <Input id="guest-group" value={groupName} onChange={(event) => setGroupName(event.target.value)} />
            </Field>
            <Field label="Tags" htmlFor="guest-tags">
              <Input
                id="guest-tags"
                value={tags}
                onChange={(event) => setTags(event.target.value)}
                placeholder="family, vip"
              />
            </Field>
            <Field label="Notes" htmlFor="guest-notes" className="md:col-span-2">
              <Textarea
                id="guest-notes"
                className="min-h-[110px]"
                value={notes}
                onChange={(event) => setNotes(event.target.value)}
                placeholder="Например: сидит рядом с семьёй, требуется детское меню"
              />
            </Field>
            <div className="md:col-span-2 grid gap-3 sm:grid-cols-3">
              <CheckLine checked={plusOneAllowed} label="Plus one allowed" onChange={setPlusOneAllowed} />
              <CheckLine checked={isChild} label="Child guest" onChange={setIsChild} />
              <CheckLine checked={vip} label="VIP guest" onChange={setVip} />
            </div>
            <div className="md:col-span-2 flex flex-wrap justify-end gap-3">
              <Button type="button" variant="secondary" onClick={onClose}>
                Cancel
              </Button>
              <Button type="submit" disabled={isSubmitting}>
                {isSubmitting ? "Saving..." : guest ? "Save guest" : "Add guest"}
              </Button>
            </div>
          </form>
        </CardContent>
      </Card>
    </div>
  );
}

function Field({
  children,
  className,
  htmlFor,
  label
}: {
  children: React.ReactNode;
  className?: string;
  htmlFor: string;
  label: string;
}) {
  return (
    <div className={className}>
      <Label htmlFor={htmlFor}>{label}</Label>
      <div className="mt-2">{children}</div>
    </div>
  );
}

function CheckLine({
  checked,
  label,
  onChange
}: {
  checked: boolean;
  label: string;
  onChange: (checked: boolean) => void;
}) {
  return (
    <label className="flex items-center gap-3 rounded-2xl border bg-white/75 px-4 py-3 text-sm">
      <input
        type="checkbox"
        checked={checked}
        onChange={(event) => onChange(event.target.checked)}
        className="size-4 rounded border-border"
      />
      {label}
    </label>
  );
}
