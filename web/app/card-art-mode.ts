export type CardArtMode = "off" | "cropped" | "full";

const scryfallIdPattern =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/;

export const isScryfallId = (id: string) => scryfallIdPattern.test(id);
