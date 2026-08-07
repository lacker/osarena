# Writing an AI bot for penta

penta is a deterministic engine for two-player constructed Magic. It currently
ships Eternal Central Old School 93/94 and the final pre-Theros ISD–RTR
Standard format. This guide is for writing a program that plays it: from
Python, C, C++, or Rust, against the included bots or against itself.

This guide describes protocol 2, shipped by engine version 0.3.0. Old School
remains the default for compatibility; new integrations should record and pass
an explicit format slug with each game.

A bot is a function from an **observation** (your seat's view of the game,
as JSON) to an **action index** (a position in that observation's
`legalActions` array). The engine validates every index against the legal
list, so an illegal move cannot even be expressed. Everything else —
mulligans, mana payment, combat — arrives as entries in that same list.

The included opponents:

- `random` — picks uniformly among legal actions. The sanity check: if your
  bot cannot beat noise, something is wrong. It plays a real, witless game
  rather than resigning, because nothing a bot can choose ends the game on
  the spot.
- `handcrafted` — a rules-based policy that plays lands on curve, attacks,
  blocks, and answers threats. The first real milestone.

For scale: the engine plays a full `handcrafted` vs `random` game in about
five milliseconds, and the Python loop below runs ~15 games/second single
threaded. Training-scale rollouts are practical on a laptop.

## Quick start: Python

Requires Python 3.9+ and [rustup](https://rustup.rs), which installs the
repository's pinned Rust version automatically. From the repository root:

```bash
cd bindings/penta-py
cargo build --release
cp target/release/libpenta.dylib penta.so   # Linux: cp target/release/libpenta.so penta.so
python3 -c "import penta; print(penta.engine_version())"
```

(With [maturin](https://maturin.rs) installed, `maturin develop --release`
does the copy for you and installs into your virtualenv.)

Then, in a file next to `penta.so`:

```python
import json
import penta

game = penta.Game("Sligh", "The Deck", opponent="handcrafted", seed=42)
while game.result() is None:
    observation = json.loads(game.observe())
    actions = observation["legalActions"]
    choice = actions[0]["index"]          # your bot's decision goes here
                                          # (nothing in the list resigns)
    game.act(choice)
print(game.result())                       # "p1", "p2", or "draw"
```

Old School remains the default for compatibility. Select Standard explicitly:

```python
game = penta.Game(
    "Briksza Naya Midrange",
    "Greer G/R Aggro",
    opponent="external",
    format="isd-rtr-standard",
    seed=42,
)
```

A complete bot that plays lands, casts its biggest spells, and attacks —
and beats `random` 100 games out of 100 — is in
[`examples/python/first_bot.py`](examples/python/first_bot.py). Copy it next
to your built `penta.so` and run it.

The module surface:

| call | meaning |
| --- | --- |
| `penta.Game(p1_deck, p2_deck, opponent=, opponent_seat=, seed=, format=)` | start a game; `format` defaults to `"old-school-93-94"` and `opponent` is `"handcrafted"`, `"random"`, or `"external"` |
| `game.observe(seat=None)` | one seat's observation as JSON (default: the seat that must act) |
| `game.act(index)` | play one entry from `legalActions` |
| `game.choose_decision([ids])` | answer a multi-pick decision explicitly (see below) |
| `game.decision_seat()` | `"p1"` / `"p2"` / `None` when the game is over |
| `game.result()` | `None`, `"p1"`, `"p2"`, or `"draw"` |
| `penta.catalog(format=)` | every canonical definition annotated with legality for the selected format, as JSON |
| `penta.deck_names(format=)` | the selected format's built-in decks |
| `penta.engine_version()`, `penta.protocol_version()` | pin these with your trained weights |

## Quick start: C and C++

```bash
cargo build --release -p penta-ffi
```

produces `target/release/libpenta_ffi.a` (and a shared library). Include
[`bindings/penta-ffi/include/penta.h`](bindings/penta-ffi/include/penta.h)
and link the library; the header documents every call and the ownership
rules. A complete program that plays full games through this interface is
[`bindings/penta-ffi/smoke.c`](bindings/penta-ffi/smoke.c):

```bash
cc mybot.c target/release/libpenta_ffi.a -I bindings/penta-ffi -o mybot
```

The C ABI is the same protocol with the same JSON: `penta_new` takes a
config, including an optional `"format"` slug; `penta_observe_json` returns an
observation; and `penta_act` takes an index. The original catalog and deck-name
functions remain Old School-compatible. New callers can use
`penta_catalog_json_for_format` and `penta_deck_names_for_format_json`.
`penta_legal_action_count` lets a minimal client act without parsing JSON at
all. From C++, wrap the header and parse observations with any JSON library
(e.g. nlohmann/json). Anything else with a C FFI — Julia, Go, C# — can consume
the same library.

## Quick start: Rust

The engine is an ordinary crate. Depend on it by path (or git) and use the
same facade the bindings use:

```rust
use penta::protocol::{BotGame, Opponent};
use penta::{Format, PlayerId};

let mut game = BotGame::new_with_format(
    Format::IsdRtrStandard,
    "Briksza Naya Midrange",
    "Greer G/R Aggro",
    Opponent::Handcrafted,
    PlayerId::Two,
    42,
)?;
while game.result().is_none() {
    let observation = game.observe_json(game.decision_seat().unwrap());
    game.act(0)?; // your bot's index here
}
```

Rust bots can also implement the `penta::Policy` trait directly and skip
JSON entirely; that is how the built-in bots are written.

## Running matches

`penta-match` pits the built-in policies against each other, alternating
seats, with deterministic seeds:

```bash
cargo run --release --bin penta-match -- \
    --p1 random --p2 handcrafted --deck1 Sligh --deck2 "The Deck" \
    --games 100 --seed 1
```

A deck of `Random` (the default) rotates through the built-in list. For
your own bot, the harness in `examples/python/first_bot.py` shows the
pattern: a seed loop, one `penta.Game` per seed, win counting.

## Self-play

`opponent="external"` disables the built-in opponent entirely: the game
stops at **every** decision, and `decision_seat()` tells you whose it is.
One loop drives both sides — your current model against a frozen
checkpoint, or against another author's bot:

```python
game = penta.Game("Goblins", "White Weenie", opponent="external", seed=7)
while game.result() is None:
    seat = game.decision_seat()
    observation = json.loads(game.observe(seat))
    bot = my_model if seat == "p1" else frozen_model
    game.act(bot(observation))
```

Observations are per-seat and redacted — `p1`'s observation never contains
`p2`'s hand — so neither side of a self-play loop can accidentally peek.

## The observation

`observe()` returns one JSON object. The essential fields:

| field | meaning |
| --- | --- |
| `format` | the rules/deck profile slug, such as `"old-school-93-94"` or `"isd-rtr-standard"` |
| `seat` | whose view this is: `"p1"` or `"p2"` |
| `pregame` | true while mulligans are being settled |
| `turn`, `activeSeat`, `prioritySeat`, `step` | where the game is; `step` is one of `Upkeep`, `Draw`, `PrecombatMain`, `BeginningOfCombat`, `DeclareAttackers`, `DeclareBlockers`, `CombatDamage`, `EndOfCombat`, `PostcombatMain`, `End`, `Cleanup` |
| `life`, `manaPools`, `librarySizes` | two-element arrays, indexed p1 then p2 |
| `hand` | your cards: `{instance, definition, name}` |
| `opponentHandSize` | their hand as a count — never the cards |
| `battlefield` | every permanent, including its current-zone object ID, canonical definition, and presented card-part ID |
| `stack` | pending spells and abilities, bottom to top, including each object's source and locked cast signature when applicable |
| `graveyards`, `exiles` | public zones, both players |
| `decision` | a pending choice (see below), or null |
| `result` | null while running, else `{winner, reason}` |
| `legalActions` | what you can do, each with an `index` |

Cards are referenced two ways: the object ID identifies one rules object in
its current zone, while `definition` identifies the canonical card kind and is
the key into `penta.catalog(format)`. A true zone change creates a new object
ID, so a Goblin Balloon Brigade card in hand, its spell on the stack, and its
permanent on the battlefield are distinct. Transforming, flipping, and phasing
do not create a new object. Physical-card lineage is private engine state and
never appears in a player's observation. Fetch the format's catalog once at
startup.

### Actions

Every entry in `legalActions` has an `index` (what you pass to `act`) and a
`type` naming the engine action, plus fields saying what it operates on:

`KeepHand`, `TakeMulligan`, `BottomCards`, `PlayLand` (with a
`playOptionId`), `CastSpell` (with the play option, ordered modes, cost
configuration, target slots, sacrifices, and X already filled in — one entry
per legal casting choice), `ActivateAbility`, `ActivateManaAbility`, `PayLifeForMana`,
`DeclareAttacker`, `FinishDeclaringAttackers`, `DeclareBlocker`,
`FinishDeclaringBlockers`, `AssignCombatDamage`, `DiscardCards`,
`ChooseUntap`, `ChooseDecision`, `CancelDecision`, `PassPriority`.

Three things worth knowing:

- **Nothing in the list loses on the spot.** Conceding is legal in every
  state of Magic, but it is strictly dominated for a bot — resigning can
  only lose a game that playing on might win — so it is not offered here at
  all. Picking blindly, by index or at random, makes a weak bot rather than
  an instant loss. (Humans concede through the browser client, which reads
  the engine's own action list.)

- **Mana is handled for you.** If a `CastSpell` appears in `legalActions`,
  you can afford it; playing it taps lands automatically. Tapping lands by
  hand (`ActivateManaAbility`) exists but is never required.
- **Costs and targets are enumerated.** A Lightning Bolt with three legal
  targets appears as three `CastSpell` entries. Your bot chooses among
  ready-made legal plays; it never constructs one.

### Decisions

Some card effects ask a question mid-resolution — "copy Chain Lightning?",
"choose a card to return". These arrive as a `decision` object (prompt,
options, `minimum`/`maximum` counts) *and* as `ChooseDecision` entries in
`legalActions`: a pick-exactly-one decision becomes one indexed action per
option, so an index-only bot handles it like anything else. For a
pick-several decision, `legalActions` carries one default selection (the
first `minimum` options) and `choose_decision([option_ids])` submits any
other selection you'd prefer.

## Determinism and versioning

The same engine version, format, decks, seed, and action sequence produce the
identical game, byte for byte — replays, regression tests, and reproducible
training episodes are free. `(engine version, format, seed, decks, action
list)` is a complete record of a game.

Two numbers describe what you trained against, and both are worth pinning
alongside your weights:

- `protocol_version()` covers the JSON shapes and the action space they
  describe. It bumps when a bot written against the old number could
  misread the new output — including a change to what appears in
  `legalActions`, since that shifts every index.
- `engine_version()` covers rules behavior, which is part of the contract
  too: a rules fix can change what a trained policy sees even when the
  shapes hold still.

[CHANGELOG.md](CHANGELOG.md) records what moved between versions and what a
bot has to do about it. Before 1.0, expect the action space to keep
settling — reading the `type` tags rather than hardcoding indices costs
nothing now and survives those changes.

## What the engine covers, honestly

Old School 93/94 has 128 cards with functional behavior and fifteen built-in
decks; the Eternal Central banned/restricted list and mana-burn exception are
enforced. ISD–RTR Standard adds the eight SCG Atlanta Top 8 decks and complete
declarative card records. Baseline creatures, mana, land entry, flash, and
combat metadata are active while specialized Standard card effects are being
implemented incrementally; metadata-only noncreature spells are withheld from
legal actions rather than resolving as silent no-ops. `penta.catalog(format)`
is the authoritative description of the selected format's support.

What this is *not*: a complete Magic rules engine. Cards outside the catalog
do not exist here, and custom decks beyond the twenty-three built-ins are not
yet exposed through the protocol. Interactions are implemented to the depth
the supported tranche requires and are covered by the engine's test suite and
long random self-play sweeps — but a trained bot will probe every edge, and if
you find behavior that contradicts the printed cards, that is a bug worth
filing.

## Where this is going

The protocol you train against locally is the protocol a future tournament
server will speak over a socket: same observations, same indices, with the
authoritative engine on the server and your bot dialing in from your own
hardware. Nothing about a bot written today needs to change for that.
