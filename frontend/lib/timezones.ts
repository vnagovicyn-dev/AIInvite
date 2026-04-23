export type TimezoneOption = {
  label: string;
  value: string;
  offsetMinutes: number;
  searchText: string;
};

const FALLBACK_TIMEZONES = [
  "Europe/Amsterdam",
  "Europe/Berlin",
  "Europe/London",
  "Europe/Paris",
  "Europe/Madrid",
  "Europe/Rome",
  "Europe/Moscow",
  "America/New_York",
  "America/Chicago",
  "America/Denver",
  "America/Los_Angeles",
  "Asia/Dubai",
  "Asia/Tbilisi",
  "Asia/Almaty",
  "Asia/Bangkok",
  "Asia/Singapore",
  "Asia/Tokyo"
] as const;

let cachedTimezoneOptions: TimezoneOption[] | null = null;

export function getTimezoneOptions(): TimezoneOption[] {
  if (cachedTimezoneOptions) {
    return cachedTimezoneOptions;
  }

  const values = getSupportedTimezones();
  const options = values
    .map((value) => buildTimezoneOption(value))
    .sort((left, right) => {
      if (left.offsetMinutes !== right.offsetMinutes) {
        return left.offsetMinutes - right.offsetMinutes;
      }

      return left.label.localeCompare(right.label);
    });

  cachedTimezoneOptions = options;
  return options;
}

export function getTimezoneLabel(timezone: string) {
  return (
    getTimezoneOptions().find((timezoneOption) => timezoneOption.value === timezone)?.label ??
    timezone
  );
}

function getSupportedTimezones() {
  if (typeof Intl.supportedValuesOf === "function") {
    return Intl.supportedValuesOf("timeZone");
  }

  return [...FALLBACK_TIMEZONES];
}

function buildTimezoneOption(value: string): TimezoneOption {
  const offsetMinutes = getOffsetMinutes(value);
  const names = getTimezoneNames(value);

  return {
    label: `${names.primary} (UTC${formatOffset(offsetMinutes)})`,
    value,
    offsetMinutes,
    searchText: [
      names.primary,
      names.secondary,
      names.region,
      value,
      formatOffset(offsetMinutes),
      `utc${formatOffset(offsetMinutes)}`
    ]
      .filter(Boolean)
      .join(" ")
      .toLowerCase()
  };
}

function getOffsetMinutes(timeZone: string) {
  const formatter = new Intl.DateTimeFormat("en-US", {
    timeZone,
    timeZoneName: "longOffset",
    hour: "2-digit",
    minute: "2-digit"
  });
  const timeZoneName =
    formatter.formatToParts(new Date()).find((part) => part.type === "timeZoneName")?.value ??
    "GMT+00:00";

  return parseOffsetMinutes(timeZoneName);
}

function parseOffsetMinutes(offsetText: string) {
  const normalized = offsetText.replace("UTC", "GMT");
  if (normalized === "GMT") {
    return 0;
  }

  const match = normalized.match(/^GMT([+-])(\d{1,2})(?::?(\d{2}))?$/);
  if (!match) {
    return 0;
  }

  const sign = match[1] === "-" ? -1 : 1;
  const hours = Number(match[2]);
  const minutes = Number(match[3] ?? "0");

  return sign * (hours * 60 + minutes);
}

function formatOffset(totalMinutes: number) {
  const sign = totalMinutes < 0 ? "-" : "+";
  const absoluteMinutes = Math.abs(totalMinutes);
  const hours = String(Math.floor(absoluteMinutes / 60)).padStart(2, "0");
  const minutes = String(absoluteMinutes % 60).padStart(2, "0");

  return `${sign}${hours}:${minutes}`;
}

function getTimezoneNames(value: string) {
  const parts = value.split("/");
  const city = value.split("/").at(-1)?.replaceAll("_", " ") ?? value;
  const russian = cityNameMap[city];
  const region = parts.length > 1 ? parts[0] : "";

  return {
    primary: russian ?? city,
    secondary: russian ? city : "",
    region
  };
}

const cityNameMap: Record<string, string> = {
  Amsterdam: "Амстердам",
  Athens: "Афины",
  Auckland: "Окленд",
  Baghdad: "Багдад",
  Bangkok: "Бангкок",
  Baku: "Баку",
  Belgrade: "Белград",
  Berlin: "Берлин",
  Bogotá: "Богота",
  Brisbane: "Брисбен",
  Bucharest: "Бухарест",
  Budapest: "Будапешт",
  Cairo: "Каир",
  Chicago: "Чикаго",
  Colombo: "Коломбо",
  Copenhagen: "Копенгаген",
  Denver: "Денвер",
  Dubai: "Дубай",
  Dublin: "Дублин",
  Edmonton: "Эдмонтон",
  Havana: "Гавана",
  Helsinki: "Хельсинки",
  Honolulu: "Гонолулу",
  Istanbul: "Стамбул",
  Jakarta: "Джакарта",
  Jerusalem: "Иерусалим",
  Kabul: "Кабул",
  Kaliningrad: "Калининград",
  Karachi: "Карачи",
  Kathmandu: "Катманду",
  Khandyga: "Хандыга",
  Kirov: "Киров",
  Kyiv: "Киев",
  Lisbon: "Лиссабон",
  London: "Лондон",
  "Los Angeles": "Лос-Анджелес",
  Madrid: "Мадрид",
  Manila: "Манила",
  Magadan: "Магадан",
  Melbourne: "Мельбурн",
  "Mexico City": "Мехико",
  Minsk: "Минск",
  Moscow: "Москва",
  Nairobi: "Найроби",
  "New Delhi": "Нью-Дели",
  "New York": "Нью-Йорк",
  Novokuznetsk: "Новокузнецк",
  Novosibirsk: "Новосибирск",
  Omsk: "Омск",
  Oslo: "Осло",
  Paris: "Париж",
  Perth: "Перт",
  Prague: "Прага",
  Riga: "Рига",
  Rome: "Рим",
  Samara: "Самара",
  Sakhalin: "Сахалин",
  Saratov: "Саратов",
  Santiago: "Сантьяго",
  "São Paulo": "Сан-Паулу",
  Seoul: "Сеул",
  Shanghai: "Шанхай",
  Singapore: "Сингапур",
  Sofia: "София",
  Stockholm: "Стокгольм",
  Srednekolymsk: "Среднеколымск",
  Sydney: "Сидней",
  Tallinn: "Таллин",
  Tashkent: "Ташкент",
  Tbilisi: "Тбилиси",
  Tehran: "Тегеран",
  Tokyo: "Токио",
  Toronto: "Торонто",
  Tomsk: "Томск",
  Ulyanovsk: "Ульяновск",
  "Ust-Nera": "Усть-Нера",
  Vienna: "Вена",
  Vilnius: "Вильнюс",
  Vladivostok: "Владивосток",
  Volgograd: "Волгоград",
  Warsaw: "Варшава",
  Yakutsk: "Якутск",
  Yekaterinburg: "Екатеринбург",
  "Yuzhno-Sakhalinsk": "Южно-Сахалинск",
  Anadyr: "Анадырь",
  Barnaul: "Барнаул",
  Astrakhan: "Астрахань",
  Chita: "Чита",
  Irkutsk: "Иркутск",
  Krasnoyarsk: "Красноярск",
  Kamchatka: "Камчатка"
};
