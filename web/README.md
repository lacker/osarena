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

The primary checkout uses `http://localhost:3000`; linked Git worktrees each
receive a stable, distinct port so their development servers can run together.
The server prints its URL at startup, and `pnpm run dev:url` reports it without
starting the server. A linked worktree keeps its assignment in the ignored
`web/.dev-port` file. The client defaults to The Deck versus Goblins, and all
game state stays in the browser. Development, production builds, and tests keep
the Git-ignored WASM bindings current automatically. Cargo checks incrementally,
and unchanged bindings skip `wasm-bindgen`.

## Checks

From the repository root, use the canonical `Makefile` task catalog and select
the smallest relevant suite while iterating:

```bash
make lint-web
make typecheck-web
make test-web-wasm-casting PATTERN='auto-pass'
make test-web-render
```

The fast WASM tests are organized into `contract`, `casting`, `combat`, `pacing`,
and `state` domains. Each has a discoverable target such as
`make test-web-wasm-contract`; `make test-web-wasm` runs all five. These targets
do not build the production application. Run the simulation-heavy sweeps with
`make test-web-wasm-slow`, or use `make test-web-full` for every web test.
`PATTERN` can narrow any domain or aggregate target by test name.

`make check-fast` is the broad development checkpoint, while `make check` is
the complete engine and web gate for stable, PR-ready work.

From this directory, `pnpm run test:fast`, `pnpm run test:wasm`,
`pnpm run test:wasm:slow`, and `pnpm run test:render` provide targeted aliases.
The fast alias also discovers standalone Node tests that do not need a
production build.
`pnpm run typecheck` checks the application through the root orchestration.
`pnpm test` and `pnpm run test:all` retain the complete web gate. `pnpm lint`
and `pnpm build` remain available directly.

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
