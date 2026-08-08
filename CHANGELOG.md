# Changelog

Two numbers matter to a bot, and they move independently:

- **Protocol version** (`penta.protocol_version()`, `penta_protocol_version()`)
  covers the JSON shapes and the action space they describe. It bumps when a
  bot written against the old number could misread the new output.
- **Engine version** (`penta.engine_version()`, the crate version) covers
  rules behavior. It bumps for anything that changes what a policy sees,
  including rules fixes that leave the shapes alone.

Pin both alongside trained weights. Until 1.0 the engine version bumps its
minor for breaking changes, per Cargo's 0.x convention.

## Unreleased — protocol 6

### Changed

- Activated, mana, and triggered actions identify the exact printed,
  intrinsic, or granted ability that created them. Triggered abilities become
  independent stack objects with frozen source information and may be answered
  before they resolve; mana abilities remain immediate.
- Card rules text and implementation coverage now belong to ordered ability
  clauses. Card-level `Complete`, `Partial`, and `MetadataOnly` status is
  derived from those clauses, exposed as `implementationStatus`, and used by
  the browser's coverage messaging instead of the internal execution gate.
- Basic land subtypes grant distinct intrinsic mana abilities. Other mana
  producers declare their abilities beside the card, and each produced mana
  value retains its restrictions and spell/ability riders.
- Bespoke engine dispatch is now an optional `CardRules` hook. Declarative and
  metadata-only cards no longer require a `CardBehavior` identity.
- Catalog and browser hand JSON now serialize cards with no mana cost as
  `"manaCost": null`; a printed `{0}` remains a mana-cost object whose
  `generic` value is zero.

Protocol 4 added the selected ability origin to activated-action JSON.
Protocol 5 distinguishes no mana cost from a printed `{0}` cost.
Protocol 6 replaces public `effectStatus` metadata with clause-derived
`implementationStatus`; the former remains only an internal playability gate.

## 0.4.0 — protocol 2

### Fixed

- A library search may now fail to find. Searching a hidden zone never obliges
  the searcher to find anything (CR 701.19c), but Demonic Tutor demanded
  exactly one card, so a player holding a full library was forced to take one.
  Failing to find is distinct from cancelling: the spell resolved and the
  search happened, so the library is still shuffled — otherwise a player could
  tutor, decline, and read their own deck order off the top.
- A decision never asks for more cards than it offers. An empty library made
  Demonic Tutor demand one of zero options with no way to cancel, which left
  no legal action at all and deadlocked the game for every policy.

### Changed

- The bundled handcrafted policy takes as many options as a beneficial
  decision allows rather than the bare minimum, so it still finds a card when
  a search permits declining.

Protocol stays at 2: no JSON field was added, removed, or renamed. A bot that
reads a decision's `minimum` needs no change, but one that assumed a search
always yields a card will now see games where it does not.

## 0.3.0 — protocol 2

### Changed

- Games now select an explicit format. Existing constructors and catalog/deck
  helpers still default to Eternal Central Old School 93/94, while new
  format-aware entry points also expose ISD–RTR Standard.
- Runtime IDs now identify one game object in its current zone rather than a
  physical card for the whole game. A card in hand, the spell it becomes on
  the stack, and the permanent it becomes on the battlefield therefore have
  different IDs.
- `PlayLand` actions carry a play-option ID, and `CastSpell` actions carry
  structured play-option, mode, cost, X, and target-slot choices. Stack
  observations retain the resulting cast signature for spell-copy effects.
- Catalog and observation JSON expose structured card parts and the currently
  presented permanent part. These wire-shape and legal-action changes require
  protocol 2; clients should continue selecting actions by their `type` and
  other semantic fields rather than hardcoded indices.

### Added

- The final pre-Theros ISD–RTR Standard format profile and the eight decks from
  the September 2013 SCG Open Atlanta Top 8.
- Optional `format` arguments in the Python binding and protocol config JSON,
  plus format-aware catalog and deck-list helpers in the Python and C APIs.

## 0.2.0 — protocol 1

### Changed

- **Conceding is no longer a bot action.** It appeared in `legalActions` in
  every state, always at index 0, and is strictly dominated for a bot —
  resigning only loses a game that playing on might win. A bot that picked
  blindly or explored uniformly resigned on turn one, which made the
  `random` baseline meaningless to measure against. It is gone from the
  bot's list entirely, so **every index in `legalActions` shifts down by
  one**; a bot that hardcoded indices needs revisiting, one that reads the
  `type` tags does not. Humans still concede in the browser client, which
  reads the engine's own action list.

### Added

- Local matches between the built-in policies via `penta-match`.
- CI on every push and pull request, running the same two scripts as local
  development.
- `rust-toolchain.toml` pins the Rust version, components, and wasm target,
  so contributors, maintainers, and CI share one compiler.

## 0.1.0 — protocol 0

First release of the bot-facing surfaces: the `penta::protocol` module and
its canonical JSON, the Python bindings, the C ABI, self-play through an
external opponent, and [BOTS.md](BOTS.md).
