import assert from "node:assert/strict";
import { after, before, test } from "node:test";
import { fileURLToPath } from "node:url";
import reactPlugin from "@vitejs/plugin-react";
import React from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { createServer } from "vite";

const webRoot = fileURLToPath(new URL("..", import.meta.url));
const scryfallId = "f594b7aa-d44e-47c4-989b-565f881e25f1";
const expectedArtUrl =
  `https://cards.scryfall.io/art/front/f/5/${scryfallId}.webp`;

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

test("interactive card art renders the Scryfall art image", () => {
  const html = renderCardArt({
    mode: "scryfall",
    cardKind: "artifact",
    scryfallId,
  });

  assert.match(html, /<i>◇<\/i>/);
  assert.match(html, /<img\b/i);
  assert.match(html, new RegExp(`src="${expectedArtUrl}"`));
  assert.match(html, /alt="" draggable="false" loading="lazy" decoding="async"/);
});

test("invalid Scryfall IDs do not create image elements", () => {
  const html = renderCardArt({
    mode: "scryfall",
    cardKind: "land",
    scryfallId: "not-a-scryfall-id",
  });

  assert.match(html, /<i>▲<\/i>/);
  assert.doesNotMatch(html, /<img\b/i);
  assert.doesNotMatch(html, /cards\.scryfall\.io/i);
});
