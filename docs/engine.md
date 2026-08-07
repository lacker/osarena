# Engine design

## API boundary

`Game` is the authoritative state machine. Consumers do not mutate zones,
life, mana, priority, or the stack. They ask for `legal_actions(player)` and
submit one of those values to `apply(player, action)`. `apply` checks legality
again so stale bot decisions fail without changing state. For a generic
`DecisionObservation`, `legal_actions` returns a compact `ChooseDecision`
marker; callers select option IDs from the observation and use
`is_legal_action`/`apply` for validation without expanding every combination.

Bots receive `PlayerObservation`, which contains that player's hand and only
counts for an opponent's hidden zones. `GameEvent` is an omniscient debugging
and replay stream; it is not a bot observation.

A bot runner asks `decision_player()` who must act, observes that player, and
submits one of the observation's legal actions:

```rust
while let Some(player) = game.decision_player() {
    let observation = game.observe(player);
    let action = bots[player.index()].choose_action(&observation);
    game.apply(player, action)?;
}
```

The decision player is normally the player with priority, but differs during
mulligans, blocker declaration, restricted untaps, cleanup discards, and
triggered or combat-damage choices.

## Identities and zones

A `CardDefinitionId` identifies a kind of card in the catalog. A
`CardInstanceId` identifies one card object during a game. Moving a card
between library, hand, stack, battlefield, and graveyard preserves its
instance ID and owner. Permanents separately track their controller.

## Priority and atomic actions

Exactly one player has priority while a game is running. Concession is always
legal; other actions are generated only for the priority player.

- A non-pass action resets the consecutive-pass count.
- The first priority pass gives priority to the opponent.
- Two passes with a nonempty stack resolve its top object.
- Two passes with an empty stack advance the turn step.
- After a resolution or step change, the active player receives priority.

Mana abilities resolve immediately and do not use the stack. Spell actions
consider both floating mana and usable untapped mana sources. Applying a spell
action deterministically activates only the additional sources needed to pay
its cost, preferring colorless sources for generic costs and avoiding excess
production where possible. The read-only `mana_sources_for_action` helper
exposes that payment preview to UI clients without cloning a complete game
state. Explicit mana actions remain legal for callers that intentionally want
to float mana. Chaos Orb's non-mana activated ability uses the stack and is
identified separately from spells in `StackObservation`; its chosen permanent
is exposed as a choice rather than a target.

Attacker and blocker declaration are staged to keep legal-action generation
linear rather than enumerating exponential subsets. No player receives
priority until the declaring player submits the corresponding finish action.
When an attacker is blocked by multiple creatures, its controller explicitly
divides its damage among them. A trampling attacker can also assign damage to
the defending player once lethal damage has been assigned to every blocker.
This follows the current rules, which removed combat damage assignment order
in the [Foundations rules update][foundations-update].

Spell actions carry a list of targets. Fireball enumerates affordable,
distinct target combinations, charges one additional generic mana for each
target beyond the first, and divides X evenly on resolution. After Fork
resolves, its controller chooses legal targets for the copy or keeps the
original targets. Spell actions also carry explicit sacrifices for additional
costs such as Goblin Grenade.

## Determinism and replay

All random choices use the engine-owned, versioned PRNG. A dependency upgrade
therefore cannot change the meaning of an existing seed. A replay can be
reconstructed from the format/card version, decks, seed, and submitted action
sequence. Events provide a convenient derived trace for debugging and UI use.

## Card behavior

Each built-in card is declared once in its first-printing set module. Its
`CardRecord` keeps identity and `CardRules` together: name, cost, type, rules
text, creature stats, and traits can all be understood at the card's
declaration. The runtime `CardDefinition` carries those rules directly.

Executable game effects are still selected by the closed `CardBehavior` enum,
which is safe to serialize and also provides a compatibility lookup for copied
or temporary card behavior. Unsupported cards can exist in other catalogs and
hidden zones but do not generate cast actions. This makes partial coverage
explicit and keeps arbitrary card code out of serialized game state.

As the corpus grows, behavior should be factored into reusable primitives
(damage, draw, destroy, continuous restrictions, triggers) rather than one
large bespoke function per printed card.

## Rules boundary

The format is Eternal Central 93/94: current Magic rules plus the EC
exceptions, notably phase-boundary mana burn. The POC implements London
mulligans, priority-bearing turn steps, cleanup, combat, fifteen fixed powered
decks, and its 128-card corpus.

It deliberately remains narrower than the full Comprehensive Rules. Fireball
and Fork expose their full targeting decisions, and attackers expose current
combat damage assignment decisions. Simple non-mana abilities and triggers
generally resolve atomically. Chaos Orb's activation uses the stack and
deterministically destroys its chosen permanent rather than simulating EC's
physical card flip; removing the Orb before resolution nullifies the ability.
Colored sources pay their printed colors, dual lands expose both choices, and
flexible sources such as Black Lotus and Fellwar Stone are considered when the
engine checks or automatically pays a cost. Red Elemental Blast can counter
blue spells or destroy blue permanents.

[foundations-update]: https://magic.wizards.com/en/news/announcements/foundations-update-bulletin
