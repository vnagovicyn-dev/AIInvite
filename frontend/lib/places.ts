export type AddressSuggestion = {
  id: string;
  title: string;
  subtitle: string;
  fullAddress: string;
  lat?: string;
  lon?: string;
};

type NominatimItem = {
  place_id: number;
  display_name: string;
  lat?: string;
  lon?: string;
  address?: {
    road?: string;
    pedestrian?: string;
    house_number?: string;
    city?: string;
    town?: string;
    village?: string;
    state?: string;
    country?: string;
  };
};

export async function searchAddressSuggestions(query: string): Promise<AddressSuggestion[]> {
  const normalizedQuery = query.trim();
  if (normalizedQuery.length < 3) {
    return [];
  }

  const params = new URLSearchParams({
    q: normalizedQuery,
    format: "jsonv2",
    addressdetails: "1",
    limit: "6",
    "accept-language": "ru,en"
  });

  const response = await fetch(`https://nominatim.openstreetmap.org/search?${params.toString()}`, {
    headers: {
      Accept: "application/json"
    }
  });

  if (!response.ok) {
    throw new Error("Не удалось получить подсказки адреса");
  }

  const payload = (await response.json()) as NominatimItem[];

  return payload.map((item) => {
    const road = item.address?.road ?? item.address?.pedestrian ?? "";
    const houseNumber = item.address?.house_number ?? "";
    const city =
      item.address?.city ?? item.address?.town ?? item.address?.village ?? item.address?.state ?? "";
    const country = item.address?.country ?? "";

    const title = [road, houseNumber].filter(Boolean).join(" ").trim() || item.display_name;
    const subtitle = [city, country].filter(Boolean).join(", ");

    return {
      id: String(item.place_id),
      title,
      subtitle,
      fullAddress: item.display_name,
      lat: item.lat,
      lon: item.lon
    };
  });
}
