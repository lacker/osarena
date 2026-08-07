# Writing an AI bot for penta

penta is a deterministic engine for Eternal Central Old School 93/94 Magic.
This guide is for writing a program that plays it: from Python, C, C++, or
Rust, against the included bots or against itself.

A bot is a function from an **observation** (your seat's view of the game,
as JSON) to an **action index** (a position in that observation's
`legalActions` array). The engine validates every index against the legal
list, so an illegal move cannot even be expressed. Everything else —
mulligans, mana payment, combat — arrives as entries in that same list.

The included opponents:

- `random` — picks uniformly among legal actions. The sanity check: if your
  bot cannot beat noise, something is wrong.
- `handcrafted` — a rules-based policy that plays lands on curve, attacks,
  blocks, and answers threats. The first real milestone.

For scale: the engine plays a full `handcrafted` vs `random` game in about
five milliseconds, and the Python loop below runs ~15 games/second single
threaded. Training-scale rollouts are practical on a laptop.

## Quick start: Python

Requires Python 3.9+ and a [Rust toolchain](https://rustup.rs). From the
repository root:

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
    choice = actions[1]["index"]     # your bot goes here; [0] is Concede
    game.act(choice)
print(game.result())                       # "p1", "p2", or "draw"
```

A complete bot that plays lands, casts its biggest spells, and attacks —
and beats `random` 100 games out of 100 — is in
[`examples/python/first_bot.py`](examples/python/first_bot.py). Copy it next
to your built `penta.so` and run it.

The module surface:

| call | meaning |
| --- | --- |
| `penta.Game(p1_deck, p2_deck, opponent=, opponent_seat=, seed=)` | start a game; `opponent` is `"handcrafted"`, `"random"`, or `"external"` |
| `game.observe(seat=None)` | one seat's observation as JSON (default: the seat that must act) |
| `game.act(index)` | play one entry from `legalActions` |
| `game.choose_decision([ids])` | answer a multi-pick decision explicitly (see below) |
| `game.decision_seat()` | `"p1"` / `"p2"` / `None` when the game is over |
| `game.result()` | `None`, `"p1"`, `"p2"`, or `"draw"` |
| `penta.catalog()` | every card definition, as JSON |
| `penta.deck_names()` | the built-in decks |
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
config, `penta_observe_json` returns an observation, `penta_act` takes an
index. `penta_legal_action_count` lets a minimal client act without parsing
JSON at all. From C++, wrap the header and parse observations with any JSON
library (e.g. nlohmann/json). Anything else with a C FFI — Julia, Go, C# —
can consume the same library.

## Quick start: Rust

The engine is an ordinary crate. Depend on it by path (or git) and use the
same facade the bindings use:

```rust
use penta::protocol::{BotGame, Opponent};
use penta::PlayerId;

let mut game = BotGame::new("Sligh", "The Deck", Opponent::Handcrafted, PlayerId::Two, 42)?;
while game.result().is_none() {
    let observation = game.observe_json(game.decision_seat().unwrap());
    game.act(1)?; // your bot's index here; index 0 is always Concede
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
| `seat` | whose view this is: `"p1"` or `"p2"` |
| `pregame` | true while mulligans are being settled |
| `turn`, `activeSeat`, `prioritySeat`, `step` | where the game is; `step` is one of `Upkeep`, `Draw`, `PrecombatMain`, `BeginningOfCombat`, `DeclareAttackers`, `DeclareBlockers`, `CombatDamage`, `EndOfCombat`, `PostcombatMain`, `End`, `Cleanup` |
| `life`, `manaPools`, `librarySizes` | two-element arrays, indexed p1 then p2 |
| `hand` | your cards: `{instance, definition, name}` |
| `opponentHandSize` | their hand as a count — never the cards |
| `battlefield` | every permanent: `{instance, definition, name, controller, tapped, power, toughness, damage, attacking, blocking, flying, canAttack, enteredThisTurn}` |
| `stack` | pending spells and abilities, bottom to top, with `targets` and `x` |
| `graveyards`, `exiles` | public zones, both players |
| `decision` | a pending choice (see below), or null |
| `result` | null while running, else `{winner, reason}` |
| `legalActions` | what you can do, each with an `index` |

Cards are referenced two ways: `instance` identifies one physical card in
this game (the second Mountain is a different instance from the first);
`definition` identifies the card *kind*, and is the key into
`penta.catalog()`, which carries names, mana costs, power/toughness, and
rules text. Fetch the catalog once at startup.

### Actions

Every entry in `legalActions` has an `index` (what you pass to `act`) and a
`type` naming the engine action, plus fields saying what it operates on:

`KeepHand`, `TakeMulligan`, `BottomCards`, `PlayLand`, `CastSpell` (with
`targets`, `sacrifices`, and `x` already filled in — one entry per legal
targeting), `ActivateAbility`, `ActivateManaAbility`, `PayLifeForMana`,
`DeclareAttacker`, `FinishDeclaringAttackers`, `DeclareBlocker`,
`FinishDeclaringBlockers`, `AssignCombatDamage`, `DiscardCards`,
`ChooseUntap`, `ChooseDecision`, `CancelDecision`, `PassPriority`,
`Concede`.

Three things worth knowing:

- **`legalActions[0]` is always `Concede`.** A "grab the first action"
  starter bot resigns on the spot: take `actions[1]`, or filter out
  `type == "Concede"`, until your bot has real preferences.
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

The same decks, seed, and action sequence always produce the identical
game, byte for byte — replays, regression tests, and reproducible training
episodes are free. `(engine version, seed, decks, action list)` is a
complete record of a game.

Rules behavior is part of the API: pin `engine_version()` alongside any
trained weights, since a rules fix can change what a trained policy sees.
`protocol_version()` covers the JSON shapes themselves and changes rarely.

## What the engine covers, honestly

This is Old School 93/94 with a curated pool of roughly 100 implemented
cards — `penta.catalog()` is the authoritative list, including each card's
implemented rules text. The fifteen built-in decks are constructed entirely
from that pool. The Eternal Central banned/restricted list is enforced.

What this is *not*: a complete Magic rules engine. Cards outside the pool
do not exist here; custom decks beyond the built-in fifteen are not yet
exposed through the protocol. Interactions are implemented to the depth the
pool requires and are covered by the engine's test suite and long random
self-play sweeps — but a trained bot will probe every edge, and if you find
behavior that contradicts the printed cards, that is a bug worth filing.

## Where this is going

The protocol you train against locally is the protocol a future tournament
server will speak over a socket: same observations, same indices, with the
authoritative engine on the server and your bot dialing in from your own
hardware. Nothing about a bot written today needs to change for that.
