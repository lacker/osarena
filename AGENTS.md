# Repository instructions

## Optional reference material

Magic rules and card data may be available in an optional development cache
under Git's common directory. One cache is shared by every linked worktree in
the clone. Use `$query-magic-references` for efficient read-only access to its
generated Scryfall SQLite index and `$refresh-magic-references` to locate,
inspect, migrate, populate, or rebuild the cache.

Locating the cache, querying it, checking `status scryfall-index`, and viewing
`lock-status` are low-friction read operations; they do not justify a refresh.
Fetch, index, and migration commands mutate shared clone state and require
explicit human approval. Run them rarely: only when material is missing or
corrupt, the required database schema is unavailable, or the current task
genuinely requires fresher source data. Do not refresh merely because a new
worktree was created. If the cache is absent, stale for an irrelevant purpose,
or unavailable, continue with appropriate authoritative online sources.

Treat `refresh.lock` metadata as diagnostic information. The kernel lock is
authoritative; never delete or bypass the lock merely because its recorded
owner appears stale. Do not commit or ship downloaded reference payloads.

Treat both reference skills as maintained development tooling. When repeated
work exposes a missing field, relationship, index, or query pattern, update the
refresh builder, both skills, and the documented schema together, then rebuild
and validate the cache. Avoid expanding them for isolated one-off questions.

## Validation

Use the root `Makefile` as the canonical entry point; `make help` lists the
available suites.

- During implementation, run the narrowest target or filtered test that
  exercises the changed behavior. Do not run the full gate after every edit.
- Run `make check-fast` at coherent checkpoints. It covers formatting, lints,
  normal Rust tests, and the fast browser-facing WASM suite without a
  production web build or the simulation-heavy tests.
- Run the relevant slow target when a change affects simulation, policy,
  auto-pass, combat progression, or another behavior covered by that suite.
- Run `make check` once the change is stable and ready to push or open as a PR.
  If bindings changed, also run `make check-bindings`; `make ci` runs every
  repository gate.
- Do not rerun an unchanged passing suite unless later edits could affect it.
  In the handoff, list the exact targets run and call out any deferred gate.
- For UI changes, command-line checks do not replace the visual verification
  below.

Map changed paths to the narrowest useful target before broadening:

- `src/game/**`, `src/card/**`, and core rules: `make test-engine-unit
  FILTER=<name>` or `make test-engine-integration FILTER=<name>`.
- `src/policy.rs` and policy behavior: `make test-policy FILTER=<name>`; add
  `make test-rust-slow` when the simulation sweeps are relevant.
- `src/protocol.rs`: `make test-engine-unit FILTER=protocol`, then
  `make test-wasm-rust` plus the matching browser contract suite when the
  exposed bridge can change. For `wasm/**`, start with the latter two targets.
- `web/app/**`: `make lint-web`, `make typecheck-web`, and the matching WASM or
  render target, followed by the required browser verification for UI changes.
- `bindings/penta-ffi/**` or `bindings/penta-py/**`: use the corresponding
  `make check-bindings-*` target; use strict `make check-bindings` before handoff.
- `Makefile`, `scripts/**`, or `.github/workflows/**`: `make lint-infra` and
  exercise the changed orchestration target. For `.github/dependabot.yml`,
  validate against GitHub's Dependabot 2.0 schema. Run `make doctor` when
  prerequisites are in question.

## UI changes

For every change that can affect the web interface:

1. Start or restart the local server from the current working tree. Confirm that
   the worktree-specific URL from `cd web && pnpm run dev:url` is served by that
   process; do not accept a fallback port or assume an older server picked up
   the change.
2. Open the rendered application in a browser and inspect it visually. A
   successful build, DOM snapshot, or HTTP response is not sufficient.
3. Check at least a 1280×720 laptop viewport. Verify that important content is
   visible and readable, with no unintended clipping, overlap, off-screen
   controls, or inaccessible horizontal overflow.
4. Exercise enough UI state to display the changed component. For game-table
   changes, check cards in hand and cards on the battlefield when applicable.
5. Take a fresh screenshot after the final code change and inspect it before
   reporting completion.

Keep the verified local server running for the user unless they ask otherwise.
