# penta

`penta` is a deterministic, headless simulator for two-player Old School
Magic: The Gathering, built for writing AI bots against.

**Want to write a bot?** You can drive the engine from Python, C, C++, or
Rust, play against the included bot algorithms, and train against self-play.
The instructions are in [BOTS.md](BOTS.md).

## Format

The engine targets **Eternal Central Old School 93/94** as one fixed format.
That means:

- Alpha, Beta, Unlimited, Collector's Edition, International Collector's
  Edition, Arabian Nights, Antiquities, Revised, Legends, The Dark, Fallen
  Empires, and the three 1994 promotional cards
- the Eternal Central banned and restricted list
- 60-card minimum decks and sideboards of up to 15 cards
- the London mulligan
- current Magic rules except where Eternal Central explicitly differs,
  notably mana burn

Paper-only policies such as which physical reprints are acceptable have no
meaning in the simulator.

The canonical format reference is [Eternal Central's 93/94 rules][ec-rules].

The format is intentionally not configurable. If another Old School variant
is added later, it should be a distinct format implementation rather than a
bag of switches spread throughout the engine.

## Engine principles

- Game state changes only through explicit actions.
- All randomness comes from a recorded seed and a versioned PRNG.
- Cards have stable numeric definition IDs and per-game instance IDs.
- A player's observation cannot expose an opponent's hidden cards.
- Legal actions are enumerated by the engine.
- The core crate has no UI, network, async runtime, or training dependencies.

See [the engine design notes](docs/engine.md) for state-machine invariants and
extension boundaries.

## Current scope

The engine currently supports:

- deck validation and seeded, reproducible setup
- hidden-information-safe observations and deterministic legal actions
- the priority-bearing turn skeleton, active player, and priority passing
- the stack and last-in-first-out spell resolution
- basic and nonbasic land plays, five-color and colorless mana sources, and EC
  phase-boundary mana burn
- player damage, concession, and empty-library loss conditions
- public battlefield, graveyard, and stack observations
- an authoritative event log for replay and debugging consumers
- London mulligans and player-selected cleanup discards
- staged attacker and blocker declaration, player-selected combat damage
  assignment, and trample
- summoning sickness, haste, temporary modifiers, marked damage, and death
- colored and colorless mana, generic and variable-X costs, and mana burn
- multi-target spells, copy retargeting, activated and triggered choices, and
  restricted untaps
- staged public/private decisions with bounded multi-selection, cancellation,
  bot preferences, and continuations across costs and effect resolution
- functional behavior metadata for all 107 catalog cards, with engine support
  for the shared mana, removal, discard, draw, tutor, and global-effect package
- ten fixed 60-card archetype decks with 15-card sideboards
- a small bot API with seeded random and card-aware handcrafted policies

The event log is intentionally omniscient and must not be passed directly to a
bot; bots consume `PlayerObservation`.

Complex choices are represented by `DecisionObservation`. The engine exposes
the prompt, visibility, selectable objects, selection bounds, and legal
`ChooseDecision` actions without exposing private options to the other player.
Recall, Balance, Demonic Tutor, Time Vault, and Sylvan Library all use this
shared path.

The POC is playable end to end, but it is not yet a general implementation of
the Comprehensive Rules. Fireball supports its multi-target additional cost
and even damage division, Fork can choose new targets for its copy, and combat
damage uses the current player-selected assignment rules. Non-mana activated
abilities generally resolve atomically, while Chaos Orb uses the stack so its
source can be removed in response. Simple upkeep/entry triggers still resolve
atomically. These constraints are explicit extension points rather than silent
support for cards outside the POC.

## Built-in decks

The proof of concept contains sixteen powered EC archetypes:

- `poc::goblins()` is a tribal aggro deck built around Goblin King, Goblin
  Grenade, Goblin Balloon Brigade, and Goblins of the Flarg.
- `poc::sligh()` is a curve-based aggro/burn deck with Ironclaw Orcs, Ball
  Lightning, Granite Gargoyle, Dragon Whelp, and direct damage.
- `poc::artifacts()` is Atog Smash, using Atog, Orcish Mechanics, Black Vise,
  Ankh of Mishra, Copper Tablet, and fast artifact mana.
- `poc::robots()` uses Mana Vault to accelerate Juggernaut, Su-Chi, and
  Triskelion, backed by Atog and red removal.
- `poc::the_deck()` is the format's namesake control strategy: Counterspell,
  Mana Drain, Swords to Plowshares, Balance, Demonic Tutor, Jayemdae Tome, and
  the format's restricted card-draw suite.
- `poc::mono_black()` combines Dark Ritual, Hypnotic Specter, Hymn to Tourach,
  Sinkhole, and Juzam Djinn.
- `poc::white_weenie()` curves Savannah Lions and Icatian Javelineers into
  Crusade and Armageddon.
- `poc::erhnamgeddon()` pairs Birds of Paradise and Erhnam Djinn with white
  removal and Armageddon.
- `poc::counterburn()` is blue-red tempo with Serendib Efreet, permission,
  Psionic Blast, and burn.
- `poc::lions_dib()` is the blue-white Savannah Lions/Serendib Efreet tempo
  shell.
- `poc::bwr_aggro()` is a black-white-red knight and burn aggro shell.
- `poc::gr_aggro()` is a green-red creature deck built around Kird Ape,
  Argothian Pixies, mana Elves, and pump spells.
- `poc::troll_disk()` is black-red Sedge Troll control with Nevinyrral's Disk
  and land destruction.
- `poc::jeskai_aggro()` is a blue-white-red tempo deck with burn and
  permission.
- `poc::lions_dib_bolt()` is the Lion/Dib shell with a dedicated Bolt package.

Their cores are based on the [TC Decks Goblins aggregate][goblins-data], the
[Wak-Wak Sligh archetype guide][sligh-guide], a representative
[EC Atog Smash list][atog-list], and the [TC Decks Artifact Aggro
aggregate][robots-data]. The Robots list stays mono-red for this implementation
slice while preserving the archetype's fast-mana and large-artifact core.
The control shell follows the recurring core in the [TC Decks The Deck
aggregate][the-deck-data].

The decks use some combination of Mishra's Factory, Strip Mine, Black Lotus,
Mox Ruby, Wheel of Fortune, Chaos Orb, and Sol Ring. The artifact deck also
uses the off-color Moxen; every Mox now produces its printed color.

EC Chaos Orb normally uses a physical dexterity flip. The headless simulator
instead treats a resolving Orb activation as a deterministic successful flip
against the chosen permanent. The activation uses the stack, and removing the
Orb before resolution nullifies the flip. This keeps seeded games reproducible
and makes the format playable without modeling a human motor skill.

The expanded corpus now includes the recurring top-table cards that make those
decks distinct: Copy Artifact, Tetravus, Icy Manipulator, Relic Barrier, The
Abyss, Sedge Troll, Stone Rain, Kird Ape, Scryb Sprites, Llanowar Elves, Giant
Growth, Berserk, Pendelhaven, Moat, Wrath of God, Dust to Dust, Hurkyl's
Recall, Energy Flux, and City in a Bottle. It is still intentionally based on
cards in actual archetypes rather than every card legal in the format.

## Bot policies

Bots implement the `Policy` trait by choosing one of the legal actions in a
hidden-information-safe `PlayerObservation`. `play_game` drives two policies
until the game ends or a caller-provided action limit is reached.

The built-in `RandomPolicy` samples uniformly from non-concession actions with
a seeded PRNG. `HandcraftedPolicy` is a deterministic baseline with simple
mulligan, casting, targeting, combat, mana, and card-specific heuristics. It is
deliberately inspectable rather than sophisticated.

Run the reproducible, seat-swapped sanity gauntlet with:

```sh
cargo run --release --bin policy_sanity
```

The gauntlet uses mirror matches for all ten built-in decks, isolating policy
quality from deck strength.

Run the broader rules audit with:

```sh
cargo run --release --bin rules_audit
```

This plays Random/Random and both seatings of Handcrafted/Random across every
built-in deck matchup and 100 seeds. After every action it checks that both
players agree on public state, hidden hand sizes remain consistent, legal
actions are unique, the correct player owns the decision, and completed games
expose no further actions. Pass a different seed count as the final argument
when doing a longer soak run, for example
`cargo run --release --bin rules_audit -- 1000`.

## Web interface

The `web/` application runs the same Rust engine in the browser through the
small `wasm/` adapter. Generated bindings are built locally under
`web/app/wasm`, which is ignored by Git, so Vite can bundle the module and its
binary together. The browser submits an index from the engine's current
legal-action list for ordinary actions. Generic decisions submit a decision ID
and option IDs through the same WASM facade; the browser never reconstructs or
mutates game rules in TypeScript.

Build fresh browser bindings after changing the Rust API:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
./scripts/build-wasm.sh
```

Then start the interface:

```sh
cd web
pnpm install
pnpm dev
```

The `dev`, `build`, and `test` workflows rebuild the ignored WASM bindings
before they run.

```sh
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

[ec-rules]: https://www.eternalcentral.com/9394rules/
[goblins-data]: https://www.tcdecks.net/archetype.php?archetype=Goblins&format=Old+School&src=all
[sligh-guide]: https://www.wak-wak.se/9394decks/sligh
[atog-list]: https://tappedout.net/mtg-decks/atog-smash-9394-1/
[robots-data]: https://www.tcdecks.net/archetype.php?archetype=Artifact+Aggro&format=Old+School&src=all
[the-deck-data]: https://www.tcdecks.net/archetype.php?archetype=The+Deck&format=Old+School&src=all
