"use client";

import { useEffect, useMemo, useRef, useState } from "react";
import Link from "next/link";
import { ExternalLink, LoaderCircle, MapPinned } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { DEFAULT_TIMEZONE, EVENT_TYPE_OPTIONS } from "@/lib/constants";
import { searchAddressSuggestions, type AddressSuggestion } from "@/lib/places";
import { getTimezoneLabel, getTimezoneOptions } from "@/lib/timezones";
import type { Event, EventCreateRequest, EventUpdateRequest, Template } from "@/lib/types";
import { buildYandexMapsSearchUrl } from "@/lib/utils";

type EventFormValues = {
  title: string;
  event_type: string;
  template_id: string;
  event_date: string;
  timezone: string;
  venue_name: string;
  venue_address: string;
};

type EventFormProps = {
  mode: "create" | "edit";
  templates: Template[];
  event?: Event | null;
  initialTemplateId?: string | null;
  isSubmitting?: boolean;
  onSubmit: (payload: EventCreateRequest | EventUpdateRequest) => Promise<void> | void;
};

export function EventForm({
  mode,
  templates,
  event,
  initialTemplateId,
  isSubmitting = false,
  onSubmit
}: EventFormProps) {
  const timezoneFieldRef = useRef<HTMLDivElement | null>(null);
  const timezoneListRef = useRef<HTMLDivElement | null>(null);
  const addressFieldRef = useRef<HTMLDivElement | null>(null);
  const addressListRef = useRef<HTMLDivElement | null>(null);
  const resolvedInitialTemplateId =
    mode === "create" &&
    initialTemplateId &&
    templates.some((template) => template.id === initialTemplateId)
      ? initialTemplateId
      : null;
  const availableTimezones = useMemo(() => {
    const baseOptions = getTimezoneOptions();
    const currentTimezone = event?.timezone;

    if (
      currentTimezone &&
      !baseOptions.some((timezoneOption) => timezoneOption.value === currentTimezone)
    ) {
      return [
        {
          label: getTimezoneLabel(currentTimezone),
          value: currentTimezone,
          offsetMinutes: 0,
          searchText: `${getTimezoneLabel(currentTimezone)} ${currentTimezone}`.toLowerCase()
        },
        ...baseOptions
      ];
    }

    return baseOptions;
  }, [event?.timezone]);

  const timezoneLabelByValue = useMemo(
    () =>
      new Map(
        availableTimezones.map((timezoneOption) => [timezoneOption.value, timezoneOption.label])
      ),
    [availableTimezones]
  );
  const timezoneValueByLabel = useMemo(
    () =>
      new Map(
        availableTimezones.map((timezoneOption) => [timezoneOption.label, timezoneOption.value])
      ),
    [availableTimezones]
  );

  const initialValues = useMemo<EventFormValues>(
    () => ({
      title: event?.title ?? "",
      event_type: event?.event_type ?? EVENT_TYPE_OPTIONS[0],
      template_id: event?.template_id ?? resolvedInitialTemplateId ?? "",
      event_date: event?.event_date ? toDateTimeLocal(event.event_date) : "",
      timezone:
        timezoneLabelByValue.get(event?.timezone ?? DEFAULT_TIMEZONE) ??
        event?.timezone ??
        getTimezoneLabel(DEFAULT_TIMEZONE),
      venue_name: event?.venue_name ?? "",
      venue_address: event?.venue_address ?? ""
    }),
    [event, resolvedInitialTemplateId, timezoneLabelByValue]
  );

  const [values, setValues] = useState<EventFormValues>(initialValues);
  const [timezoneDropdownOpen, setTimezoneDropdownOpen] = useState(false);
  const [highlightedTimezoneIndex, setHighlightedTimezoneIndex] = useState(0);
  const [addressDropdownOpen, setAddressDropdownOpen] = useState(false);
  const [addressSuggestions, setAddressSuggestions] = useState<AddressSuggestion[]>([]);
  const [addressSuggestionsLoading, setAddressSuggestionsLoading] = useState(false);
  const [addressSuggestionsError, setAddressSuggestionsError] = useState<string | null>(null);
  const [highlightedAddressIndex, setHighlightedAddressIndex] = useState(0);
  const yandexMapsUrl = useMemo(
    () => buildYandexMapsSearchUrl(values.venue_address),
    [values.venue_address]
  );
  const selectedTemplate = useMemo(
    () => templates.find((template) => template.id === values.template_id) ?? null,
    [templates, values.template_id]
  );

  useEffect(() => {
    setValues(initialValues);
  }, [initialValues]);

  useEffect(() => {
    setHighlightedTimezoneIndex(0);
  }, [values.timezone, timezoneDropdownOpen]);

  useEffect(() => {
    setHighlightedAddressIndex(0);
  }, [values.venue_address, addressDropdownOpen]);

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (
        timezoneFieldRef.current &&
        event.target instanceof Node &&
        !timezoneFieldRef.current.contains(event.target)
      ) {
        setTimezoneDropdownOpen(false);
      }
      if (
        addressFieldRef.current &&
        event.target instanceof Node &&
        !addressFieldRef.current.contains(event.target)
      ) {
        setAddressDropdownOpen(false);
      }
    }

    document.addEventListener("mousedown", handleClickOutside);

    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, []);

  useEffect(() => {
    const query = values.venue_address.trim();

    if (query.length < 3) {
      setAddressSuggestions([]);
      setAddressSuggestionsError(null);
      setAddressSuggestionsLoading(false);
      return;
    }

    const timeoutId = window.setTimeout(async () => {
      setAddressSuggestionsLoading(true);
      setAddressSuggestionsError(null);
      try {
        const suggestions = await searchAddressSuggestions(query);
        setAddressSuggestions(suggestions);
      } catch (error) {
        setAddressSuggestions([]);
        setAddressSuggestionsError(
          error instanceof Error ? error.message : "Не удалось загрузить подсказки адреса"
        );
      } finally {
        setAddressSuggestionsLoading(false);
      }
    }, 300);

    return () => {
      window.clearTimeout(timeoutId);
    };
  }, [values.venue_address]);

  const filteredTimezones = useMemo(() => {
    const query = values.timezone.trim().toLowerCase();

    if (!query) {
      return availableTimezones.slice(0, 30);
    }

    return availableTimezones
      .filter((timezoneOption) => timezoneOption.searchText.includes(query))
      .slice(0, 30);
  }, [availableTimezones, values.timezone]);

  useEffect(() => {
    if (!timezoneDropdownOpen || !timezoneListRef.current) {
      return;
    }

    const activeElement = timezoneListRef.current.querySelector<HTMLElement>(
      `[data-timezone-index="${highlightedTimezoneIndex}"]`
    );
    activeElement?.scrollIntoView({ block: "nearest" });
  }, [highlightedTimezoneIndex, timezoneDropdownOpen]);

  useEffect(() => {
    if (!addressDropdownOpen || !addressListRef.current) {
      return;
    }

    const activeElement = addressListRef.current.querySelector<HTMLElement>(
      `[data-address-index="${highlightedAddressIndex}"]`
    );
    activeElement?.scrollIntoView({ block: "nearest" });
  }, [highlightedAddressIndex, addressDropdownOpen]);

  function selectTimezone(label: string) {
    setValues((current) => ({ ...current, timezone: label }));
    setTimezoneDropdownOpen(false);
  }

  function selectAddress(suggestion: AddressSuggestion) {
    setValues((current) => ({ ...current, venue_address: suggestion.fullAddress }));
    setAddressDropdownOpen(false);
  }

  async function handleSubmit(eventForm: React.FormEvent<HTMLFormElement>) {
    eventForm.preventDefault();

    await onSubmit({
      title: values.title.trim(),
      event_type: values.event_type,
      template_id: values.template_id || null,
      event_date: values.event_date ? new Date(values.event_date).toISOString() : null,
      timezone:
        timezoneValueByLabel.get(values.timezone.trim()) ??
        values.timezone.trim() ??
        DEFAULT_TIMEZONE,
      venue_name: values.venue_name.trim() || null,
      venue_address: values.venue_address.trim() || null
    });
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>{mode === "create" ? "Create event" : "Event details"}</CardTitle>
      </CardHeader>
      <CardContent>
        <form className="grid gap-5 md:grid-cols-2" onSubmit={handleSubmit}>
          {mode === "create" && selectedTemplate ? (
            <div className="md:col-span-2 rounded-[1.5rem] border bg-secondary/40 px-5 py-4">
              <div className="text-sm font-semibold text-foreground">
                Выбран шаблон: {selectedTemplate.name}
              </div>
              <p className="mt-1 text-sm text-muted-foreground">
                Категория: {selectedTemplate.category}. При желании можно сменить шаблон ниже до
                создания события.
              </p>
            </div>
          ) : null}
          <Field className="md:col-span-2" label="Title" htmlFor="event-title">
            <Input
              id="event-title"
              value={values.title}
              onChange={(event) => setValues((current) => ({ ...current, title: event.target.value }))}
              placeholder="Anna & Mark wedding"
              required
            />
          </Field>
          <Field label="Event type" htmlFor="event-type">
            <select
              id="event-type"
              value={values.event_type}
              onChange={(event) => setValues((current) => ({ ...current, event_type: event.target.value }))}
              className="flex h-12 w-full rounded-[1.2rem] border bg-background px-4 text-sm outline-none transition focus-visible:ring-2 focus-visible:ring-ring"
            >
              {EVENT_TYPE_OPTIONS.map((option) => (
                <option key={option} value={option}>
                  {option}
                </option>
              ))}
            </select>
          </Field>
          <Field label="Template" htmlFor="event-template">
            <select
              id="event-template"
              value={values.template_id}
              onChange={(event) => setValues((current) => ({ ...current, template_id: event.target.value }))}
              className="flex h-12 w-full rounded-[1.2rem] border bg-background px-4 text-sm outline-none transition focus-visible:ring-2 focus-visible:ring-ring"
            >
              <option value="">No template</option>
              {templates.map((template) => (
                <option key={template.id} value={template.id}>
                  {template.name}
                </option>
              ))}
            </select>
          </Field>
          <Field label="Event date" htmlFor="event-date">
            <Input
              id="event-date"
              type="datetime-local"
              value={values.event_date}
              onChange={(event) => setValues((current) => ({ ...current, event_date: event.target.value }))}
            />
          </Field>
          <Field label="Timezone" htmlFor="event-timezone">
            <div className="relative" ref={timezoneFieldRef}>
              <Input
                id="event-timezone"
                value={values.timezone}
                onFocus={() => setTimezoneDropdownOpen(true)}
                onChange={(event) => {
                  setValues((current) => ({ ...current, timezone: event.target.value }));
                  setTimezoneDropdownOpen(true);
                }}
                onKeyDown={(event) => {
                  if (!timezoneDropdownOpen && (event.key === "ArrowDown" || event.key === "ArrowUp")) {
                    setTimezoneDropdownOpen(true);
                    return;
                  }

                  if (!filteredTimezones.length) {
                    if (event.key === "Escape") {
                      setTimezoneDropdownOpen(false);
                    }
                    return;
                  }

                  if (event.key === "ArrowDown") {
                    event.preventDefault();
                    setHighlightedTimezoneIndex((current) =>
                      Math.min(current + 1, filteredTimezones.length - 1)
                    );
                  }

                  if (event.key === "ArrowUp") {
                    event.preventDefault();
                    setHighlightedTimezoneIndex((current) => Math.max(current - 1, 0));
                  }

                  if (event.key === "Enter" && timezoneDropdownOpen) {
                    event.preventDefault();
                    const activeTimezone = filteredTimezones[highlightedTimezoneIndex];
                    if (activeTimezone) {
                      selectTimezone(activeTimezone.label);
                    }
                  }

                  if (event.key === "Escape") {
                    setTimezoneDropdownOpen(false);
                  }
                }}
                placeholder="Найдите город: Москва, Amsterdam, New York"
                autoComplete="off"
              />
              {timezoneDropdownOpen ? (
                <div
                  ref={timezoneListRef}
                  className="absolute z-20 mt-2 max-h-72 w-full overflow-auto rounded-[1.2rem] border bg-card p-2 shadow-xl"
                >
                  {filteredTimezones.length ? (
                    filteredTimezones.map((timezone, index) => (
                      <button
                        key={timezone.value}
                        type="button"
                        data-timezone-index={index}
                        className={`flex w-full items-center justify-between rounded-2xl px-3 py-3 text-left text-sm transition ${
                          highlightedTimezoneIndex === index ? "bg-secondary/70" : "hover:bg-secondary/60"
                        }`}
                        onMouseEnter={() => setHighlightedTimezoneIndex(index)}
                        onMouseDown={(event) => {
                          event.preventDefault();
                          selectTimezone(timezone.label);
                        }}
                      >
                        <span>{timezone.label}</span>
                        <span className="ml-4 text-xs text-muted-foreground">{timezone.value}</span>
                      </button>
                    ))
                  ) : (
                    <div className="px-3 py-4 text-sm text-muted-foreground">
                      Ничего не найдено. Попробуйте искать по-русски, по-английски или по `UTC`.
                    </div>
                  )}
                </div>
              ) : null}
            </div>
          </Field>
          <Field label="Название площадки" htmlFor="event-venue-name">
            <Input
              id="event-venue-name"
              value={values.venue_name}
              onChange={(event) => setValues((current) => ({ ...current, venue_name: event.target.value }))}
              placeholder="Grand Hall"
            />
          </Field>
          <Field label="Место проведения" htmlFor="event-venue-address">
            <div className="space-y-3">
              <div className="relative" ref={addressFieldRef}>
                <Input
                  id="event-venue-address"
                  type="search"
                  value={values.venue_address}
                  onFocus={() => setAddressDropdownOpen(true)}
                  onChange={(event) => {
                    setValues((current) => ({ ...current, venue_address: event.target.value }));
                    setAddressDropdownOpen(true);
                  }}
                  onKeyDown={(event) => {
                    if (!addressDropdownOpen && (event.key === "ArrowDown" || event.key === "ArrowUp")) {
                      setAddressDropdownOpen(true);
                      return;
                    }

                    if (!addressSuggestions.length) {
                      if (event.key === "Escape") {
                        setAddressDropdownOpen(false);
                      }
                      return;
                    }

                    if (event.key === "ArrowDown") {
                      event.preventDefault();
                      setHighlightedAddressIndex((current) =>
                        Math.min(current + 1, addressSuggestions.length - 1)
                      );
                    }

                    if (event.key === "ArrowUp") {
                      event.preventDefault();
                      setHighlightedAddressIndex((current) => Math.max(current - 1, 0));
                    }

                    if (event.key === "Enter" && addressDropdownOpen) {
                      event.preventDefault();
                      const activeSuggestion = addressSuggestions[highlightedAddressIndex];
                      if (activeSuggestion) {
                        selectAddress(activeSuggestion);
                      }
                    }

                    if (event.key === "Escape") {
                      setAddressDropdownOpen(false);
                    }
                  }}
                  placeholder="Введите адрес или название места, например: Тверская 7, Москва или Grand Hall"
                  autoComplete="street-address"
                />
                {addressDropdownOpen ? (
                  <div
                    ref={addressListRef}
                    className="absolute z-20 mt-2 max-h-72 w-full overflow-auto rounded-[1.2rem] border bg-card p-2 shadow-xl"
                  >
                    {addressSuggestionsLoading ? (
                      <div className="flex items-center gap-2 px-3 py-4 text-sm text-muted-foreground">
                        <LoaderCircle className="size-4 animate-spin" />
                        Ищем места и адреса...
                      </div>
                    ) : null}
                    {!addressSuggestionsLoading && addressSuggestions.length ? (
                      addressSuggestions.map((suggestion, index) => (
                        <button
                          key={suggestion.id}
                          type="button"
                          data-address-index={index}
                          className={`flex w-full flex-col rounded-2xl px-3 py-3 text-left text-sm transition ${
                            highlightedAddressIndex === index ? "bg-secondary/70" : "hover:bg-secondary/60"
                          }`}
                          onMouseEnter={() => setHighlightedAddressIndex(index)}
                          onMouseDown={(event) => {
                            event.preventDefault();
                            selectAddress(suggestion);
                          }}
                        >
                          <span className="font-medium">{suggestion.title}</span>
                          <span className="mt-1 text-xs text-muted-foreground">
                            {suggestion.subtitle || suggestion.fullAddress}
                          </span>
                        </button>
                      ))
                    ) : null}
                    {!addressSuggestionsLoading && !addressSuggestions.length && values.venue_address.trim().length >= 3 ? (
                      <div className="px-3 py-4 text-sm text-muted-foreground">
                        {addressSuggestionsError ?? "Подсказки не найдены. Можно сохранить место проведения вручную."}
                      </div>
                    ) : null}
                  </div>
                ) : null}
              </div>
              {yandexMapsUrl ? (
                <div className="rounded-2xl border bg-secondary/35 px-4 py-3">
                  <div className="mb-2 flex items-center gap-2 text-sm font-medium">
                    <MapPinned className="size-4 text-primary" />
                    Проверить место на карте
                  </div>
                  <p className="mb-3 text-sm text-muted-foreground">
                    {values.venue_address}
                  </p>
                  <Button variant="secondary" asChild>
                    <Link href={yandexMapsUrl} target="_blank" rel="noreferrer">
                      Открыть в Яндекс Картах
                      <ExternalLink className="size-4" />
                    </Link>
                  </Button>
                </div>
              ) : null}
            </div>
          </Field>
          <div className="md:col-span-2 flex justify-end">
            <Button type="submit" disabled={isSubmitting}>
              {isSubmitting ? "Saving..." : mode === "create" ? "Create event" : "Save changes"}
            </Button>
          </div>
        </form>
      </CardContent>
    </Card>
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

function toDateTimeLocal(value: string) {
  return new Date(value).toISOString().slice(0, 16);
}
