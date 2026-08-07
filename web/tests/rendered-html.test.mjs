import assert from "node:assert/strict";
import test from "node:test";

async function render() {
  const workerUrl = new URL("../dist/server/index.js", import.meta.url);
  workerUrl.searchParams.set("test", `${process.pid}-${Date.now()}`);
  const { default: worker } = await import(workerUrl.href);
  return worker.fetch(
    new Request("http://localhost/", { headers: { accept: "text/html" } }),
    { ASSETS: { fetch: async () => new Response("Not found", { status: 404 }) } },
    { waitUntil() {}, passThroughOnException() {} },
  );
}

test("server renders the Penta application shell", async () => {
  const response = await render();
  assert.equal(response.status, 200);
  assert.match(response.headers.get("content-type") ?? "", /^text\/html\b/i);

  const html = await response.text();
  assert.match(html, /<title>Penta/);
  // The masthead moved onto the table, which only exists once the engine has
  // dealt a game, so the shell is the deck picker and the loading state.
  assert.match(html, /Choose your deck/);
  assert.match(html, /Old School 93\/94/);
  assert.match(html, /ISD–RTR Standard/);
  assert.match(html, /Waking the Rust engine/);
  assert.doesNotMatch(html, /codex-preview|react-loading-skeleton/i);
});
