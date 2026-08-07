"use client";

export type CardArtMode = "off" | "scryfall";

const scryfallIdPattern =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/;

const scryfallArtUrl = (id: string) =>
  `https://cards.scryfall.io/art/front/${id[0]}/${id[1]}/${id}.webp`;

export function CardArt({
  mode = "off",
  cardKind,
  scryfallId,
}: {
  mode?: CardArtMode;
  cardKind: string;
  scryfallId: string;
}) {
  const artUrl =
    mode === "scryfall" && scryfallIdPattern.test(scryfallId)
      ? scryfallArtUrl(scryfallId)
      : null;

  return (
    <span className="card-art" aria-hidden="true">
      <i>{cardKind.includes("land") ? "▲" : cardKind.includes("artifact") ? "◇" : "●"}</i>
      {artUrl && (
        // eslint-disable-next-line @next/next/no-img-element
        <img
          src={artUrl}
          alt=""
          draggable={false}
          loading="lazy"
          decoding="async"
          onError={(event) => {
            event.currentTarget.hidden = true;
          }}
        />
      )}
    </span>
  );
}
