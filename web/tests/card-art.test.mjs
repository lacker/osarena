import assert from "node:assert/strict";
import { after, before, test } from "node:test";
import { fileURLToPath } from "node:url";
import reactPlugin from "@vitejs/plugin-react";
import React from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { createServer } from "vite";

const webRoot = fileURLToPath(new URL("..", import.meta.url));
const scryfallId = "f594b7aa-d44e-47c4-989b-565f881e25f1";
const expectedCroppedArtUrl =
  `https://cards.scryfall.io/art/front/f/5/${scryfallId}.webp`;
const expectedNormalArtUrl =
  `https://cards.scryfall.io/normal/front/f/5/${scryfallId}.jpg`;
const expectedLargeArtUrl =
  `https://cards.scryfall.io/large/front/f/5/${scryfallId}.jpg`;

let cardArt;
let vite;

before(async () => {
  vite = await createServer({
    appType: "custom",
    configFile: false,
    logLevel: "silent",
    optimizeDeps: { noDiscovery: true },
    plugins: [reactPlugin()],
    root: webRoot,
    server: { middlewareMode: true },
  });
  ({ CardArt: cardArt } = await vite.ssrLoadModule("/app/CardArt.tsx"));
});

after(async () => {
  await vite?.close();
});

const renderCardArt = (props) =>
  renderToStaticMarkup(React.createElement(cardArt, props));

test("card art is fail-closed when disabled", () => {
  const html = renderCardArt({ mode: "off", cardKind: "artifact", scryfallId });

  assert.match(html, /<i>◇<\/i>/);
  assert.doesNotMatch(html, /<img\b/i);
  assert.doesNotMatch(html, /cards\.scryfall\.io/i);
});

test("cropped card art renders the Scryfall art image", () => {
  const html = renderCardArt({
    mode: "cropped",
    cardKind: "artifact",
    scryfallId,
  });

  assert.match(html, /class="card-art"/);
  assert.doesNotMatch(html, /card-art-full/);
  assert.match(html, /<i>◇<\/i>/);
  assert.match(html, /<img\b/i);
  assert.match(html, new RegExp(`src="${expectedCroppedArtUrl}"`));
  assert.doesNotMatch(html, /srcSet=/);
  assert.doesNotMatch(html, /sizes=/);
  assert.match(html, /alt="" draggable="false" loading="lazy" decoding="async"/);
});

test("full card art lets the browser choose between normal and large images", () => {
  const html = renderCardArt({
    mode: "full",
    cardKind: "land",
    scryfallId,
  });

  assert.match(html, /class="card-art card-art-full"/);
  assert.match(html, /<i>▲<\/i>/);
  assert.match(html, /<img\b/i);
  assert.match(html, new RegExp(`src="${expectedNormalArtUrl}"`));
  assert.match(
    html,
    new RegExp(
      `srcSet="${expectedNormalArtUrl} 488w, ${expectedLargeArtUrl} 672w"`,
    ),
  );
  assert.match(html, /sizes="132px"/);
});

test("full card art advertises its rendered size", () => {
  const html = renderCardArt({
    mode: "full",
    cardKind: "artifact",
    scryfallId,
    fullImageSizes: "48px",
  });

  assert.match(html, /sizes="48px"/);
});

for (const mode of ["cropped", "full"]) {
  test(`invalid Scryfall IDs do not create ${mode} image elements`, () => {
    const html = renderCardArt({
      mode,
      cardKind: "land",
      scryfallId: "not-a-scryfall-id",
    });

    assert.match(html, /<i>▲<\/i>/);
    assert.doesNotMatch(html, /card-art-full/);
    assert.doesNotMatch(html, /<img\b/i);
    assert.doesNotMatch(html, /cards\.scryfall\.io/i);
  });
}

test("uppercase Scryfall IDs remain invalid", () => {
  const html = renderCardArt({
    mode: "full",
    cardKind: "artifact",
    scryfallId: scryfallId.toUpperCase(),
  });

  assert.doesNotMatch(html, /<img\b/i);
});
