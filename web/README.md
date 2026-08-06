# Penta web client

This directory contains the local browser client for the Rust game engine. It
uses React, vinext, and a generated WebAssembly bridge; no account or database
is required to play a local game.

## Prerequisites

- Node.js `>=22.13.0`
- Rust and the `wasm32-unknown-unknown` target
- `wasm-bindgen` on `PATH`

## Quick start

```bash
pnpm install
pnpm run dev
```

Then open `http://localhost:3000`. The client defaults to The Deck versus
Goblins, and all game state stays in the browser. Development, production
builds, and tests regenerate the Git-ignored WASM bindings automatically.

## Checks

From the repository root, run:

```bash
./scripts/check-all.sh
```

That formats, lints, and tests both Rust crates, rebuilds the WASM artifact,
builds the client, and runs the browser-facing tests. The shorter commands are
available from this directory as `pnpm lint`, `pnpm build`, and `pnpm test`.

## Deploying

The client deploys to Cloudflare Workers. `worker/index.ts` is the entry point
and `vite.config.ts` declares the Worker; the build writes a ready Wrangler
config to `dist/server/wrangler.json`.

```bash
pnpm run deploy
```

That rebuilds the WASM artifact and the client, then publishes the `penta`
Worker. It needs a Cloudflare account — `npx wrangler login` once, and
`npx wrangler whoami` to check which account is active. The game runs entirely
in the browser, so there are no D1, R2, or other storage bindings to provision,
and local development needs no Cloudflare credentials at all.
