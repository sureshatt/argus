import { twMerge } from 'tailwind-merge'

export function cn(a: string, b: string): string {
    return twMerge(a, b)
}

const vendorMap: Record<string, { name: string; type: "phone" | "laptop" | "desktop" | "tv" }> = {
  // Phones
  "f4:5c:89": { name: "Apple", type: "phone" },
  "28:cf:e9": { name: "Apple", type: "phone" },
  "ec:9b:f3": { name: "Samsung", type: "phone" },

  // Laptops/Desktops
  "00:1b:63": { name: "Dell", type: "laptop" },
  "3c:07:54": { name: "Apple", type: "laptop" },
  "b8:27:eb": { name: "Raspberry Pi", type: "desktop" },

  // TVs
  "cc:fa:00": { name: "LG", type: "tv" },
  "00:18:6b": { name: "Samsung", type: "tv" },
};

export function getDeviceTypeFromMac(mac: string): "phone" | "laptop" | "desktop" | "tv" {
  const normalized = mac.toLowerCase().slice(0, 8);

  // Check vendor map
  const vendor = vendorMap[normalized];
  if (vendor) return vendor.type;

  // Randomized MACs → likely phones
  const firstByte = parseInt(mac.split(":")[0], 16);
  const isLAA = (firstByte & 0b00000010) === 0b00000010;
  if (isLAA) return "phone";

  return "desktop"; // fallback
}