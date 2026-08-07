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

A `CardDefinitionId` identifies one canonical card name and rules identity in
the catalog. Copy limits, banned and restricted lists, and executable behavior
all use that canonical identity. A `CardPrintingId` identifies one exact
set-and-variant printing of the definition. Multiple printing variants may
therefore share a set and canonical definition, which can represent different
basic-land art without duplicating gameplay rules.

The runtime model deliberately separates physical-card lineage from rules
object identity:

- A physical-card ID follows one piece of cardboard and its owner through the
  game. A printing can eventually be attached here without changing the card's
  canonical definition.
- A `GameObjectId` identifies one rules object in its current zone. Moving a
  card from hand to the stack, from the stack to the battlefield, or from the
  battlefield to exile creates a new game object. Effects and targets refer to
  this current incarnation, so an object that leaves and returns is not the
  object that was previously targeted.

Transforming a permanent and phasing it out do not change zones, so both retain
the same `GameObjectId` and permanent state. Shuffling or changing position
within a library likewise does not create a new object. These rules let the
engine distinguish, for example, a Goblin Balloon Brigade card in hand, its
creature spell on the stack, and the creature permanent it becomes without
pretending that they are the same rules object.

An object's characteristics are independent of its physical backing. The
backing is represented conceptually as zero, one, or several physical-card
IDs: a spell copy or token has no physical card, an ordinary card object has
one, and a future melded permanent can have two. A physical card may back at
most one live game object at a time. Physical lineage should normally stay out
of player observations because exposing it would allow a client to track a
known card through a shuffle.

The core runtime now uses this separation. Physical cards live in a private
game registry, live objects carry zero-or-more physical backing IDs separately
from their characteristic source, and actions, targets, observations, and
events use `GameObjectId`. The former `CardInstanceId` and `StackObjectId`
names remain deprecated source-compatibility aliases; they no longer identify
physical lineage or a separate stack-ID namespace. Printing IDs are catalog
metadata today and are not yet an art-selection feature in the UI.

Historical events that outlive a zone object carry its immutable card
definition as well as the former object ID. Activated-ability events carry the
ability object's stack ID separately from the source permanent's ID, so a
sacrificed source does not make the resolution record ambiguous.

## Card parts and contextual characteristics

A `CardPart` is an independently addressable bundle of printed
characteristics. A part is intentionally broader than a physical face: Turn
and Burn are two parts printed on one face, while Huntmaster of the Fells and
Ravager of the Fells occupy opposite faces. `CardStructure` records how parts
are related for single, split, flip, double-faced, alternate-spell, and future
meld-part cards.

Which characteristics apply is a question about an object and its zone, not
about the physical card alone. The contextual resolver owns rules such as
these:

- a transforming double-faced card normally uses its front part outside the
  stack and battlefield;
- a spell on the stack uses the part or ordered combination selected by its
  play option;
- a permanent uses the part currently presented on the battlefield; and
- structure-specific rules, such as the combined characteristics of a split
  card outside the stack, are applied centrally rather than repeated by card
  effects and clients.

Visibility is separate from applicability. A player may be allowed to inspect
another face without that face contributing characteristics in the current
zone.

The catalog now stores parts, structures, and play options. Ordinary cards
receive a synthesized single primary part, while Garruk Relentless, Huntmaster
of the Fells, Izzet Charm, and Turn // Burn exercise the structured metadata.
The contextual part resolver implements the zone/form/presentation selection
above. A permanent observation exposes its presented part, and baseline type,
power/toughness, flying, trample, mana production, and land-type queries read
that part. `CardDefinition.rules` remains a primary/front compatibility view
for older behavior code, and permanents do not yet execute general transform
actions or triggers. Cataloging both faces therefore does not by itself make
Huntmaster or Garruk transform during a game.

Spell resolution determines whether the result is a permanent from the locked
spell form rather than from the canonical front face. Structured target-slot
predicates are checked again at resolution; an object that remains on the
battlefield but ceases to satisfy “target creature,” for example, is no longer
a legal target.

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

Spell choices bind targets to stable target slots. Fireball's legacy behavior
uses one variable-cardinality slot: it enumerates affordable, distinct target
combinations, charges one additional generic mana for each target beyond the
first, and divides X evenly on resolution. Different slots are independent,
so two instructions may choose the same object. After Fork resolves, its
controller chooses legal replacement values for the existing slots or keeps
the original targets. Spell actions also carry explicit payment objects for
costs such as Goblin Grenade's sacrifice.

## Play options, modes, and cast signatures

The casting model keeps several choices separate because they obey different
rules:

- A play option selects what is being played: a split-card half, both halves
  with fuse, an Adventure spell, or one side of a modal double-faced card. It
  also says whether the action casts a spell or plays a land.
- A rules-text mode selects an effect branch, such as one of Izzet Charm's
  three instructions. A card can have one ordinary play option and several
  modes.
- Alternative and additional cost choices describe how the selected form is
  paid for. They do not become extra faces or modes.
- X, targets, and any required divisions are further choices made for the
  particular spell.

After validation, those choices form an immutable cast signature on the stack:
the selected play option and spell form, chosen modes, cost choices, X, and
target-slot assignments. Target slots are associated with their part or mode
so a fused spell and a modal spell retain the target schema that was actually
chosen rather than regenerating one from the canonical card definition.

Copying a spell creates a new game object with no physical backing and copies
the cast signature. It therefore retains the selected split/MDFC/alternate
form, rules modes, X, cost decisions, and targets. The copy is not cast and
does not pay those costs. A copy effect such as Fork may explicitly replace
legal target assignments, but it cannot choose different modes or a different
spell form.

The catalog types and runtime now share this model. `Action::CastSpell` carries
one authoritative `CastChoices` value rather than parallel mode/form/target/X
fields, and a validated cast stores a `CastSignature` on its stack object.
Existing single-faced cards receive a default play option and stable legacy
target slot, so their behavior is unchanged while structured cards use their
declared options, modes, and target slots. Fork copies that signature and can
replace only the target values in its existing slots. Sacrificed, discarded,
or tapped objects remain payment records outside the signature because a copy
does not pay those costs again.

Izzet Charm and Turn // Burn currently exercise this structured catalog and
validation path, including ordered modes, fused forms, and independent target
slots. Their printed effects are still marked metadata-only, so the engine
does not offer casts that would resolve as silent no-ops. Implementing those
effects can enable the existing options without changing the casting model.

Catalog construction rejects ambiguous structured metadata: duplicate local
IDs, missing or out-of-structure parts, invalid mode and target bounds, and a
fused option that does not name every split part in printed order.

## Future composite objects and meld

No supported format currently needs meld, and the engine does not execute it.
The identity model nevertheless avoids assuming that every game object is
backed by exactly one card. A future meld action can consume two zone objects
with one physical backing each and create one battlefield object whose backing
contains both cards. If that permanent later changes zones, the result can be
two new card objects rather than forcing a false one-object/one-card mapping.

Finding the objects named by a meld ability and successfully melding their
physical cards are deliberately different operations. Name conditions inspect
the objects' effective characteristics. A Clone or token copy named Graf Rats
can therefore satisfy Midnight Scavengers' condition. Resolution first
performs the instructed exile zone changes, then the meld attempt validates
that the resulting objects are backed by the two complementary physical meld
cards. With a real Midnight Scavengers and only a copy of Graf Rats, that
validation fails; it does not undo the exile. A physical copy card remains in
exile, while a token follows the normal rule that makes it cease to exist.

`MeldRecipeDef` makes that boundary explicit in catalog data: each component
has a `required_name` for the object-level condition and a separate
`required_card` for physical-backing validation, while `MeldResultDef` owns the
combined object's name and rules instead of pretending it is either component.

This is the same general boundary used elsewhere: characteristic predicates
look at what an object currently is, while structural actions inspect what can
physically represent the requested result.

## Determinism and replay

All random choices use the engine-owned, versioned PRNG. A dependency upgrade
therefore cannot change the meaning of an existing seed. A replay can be
reconstructed from the format/card version, decks, seed, and submitted action
sequence. Events provide a convenient derived trace for debugging and UI use.

## Card behavior

Each built-in canonical card is declared once in the `CARDS` registry of its
representative or debut set module, under the set's release-year module. Its
`CardRecord` keeps identity and its primary `CardRules` together: name, cost,
type, rules text, creature stats, and traits can all be understood at the
card's declaration. Structured cards attach a `CardComposition` containing
their parts, topology, and play options; an ordinary record receives an
equivalent one-part composition automatically. An immediately adjacent
`Implementation status` comment records which rules are executable and which
remain metadata.

A set module's `ADDITIONAL_PRINTINGS` registry points back to those canonical
records for reprints or additional variants in that set. The resulting
`CardPrintingId` combines the canonical definition, set, and variant, so
alternate art can be distinguished while sharing one runtime `CardDefinition`
and its rules. Format legality considers all known printings: a nonbasic card
is legal when at least one printing belongs to the format's allowed sets,
regardless of which printing might eventually be selected for presentation.

Executable game effects are still selected by the closed `CardBehavior` enum,
which is safe to serialize and also provides a compatibility lookup for copied
or temporary card behavior. Unsupported cards can exist in other catalogs and
hidden zones but do not generate cast actions. This makes partial coverage
explicit and keeps arbitrary card code out of serialized game state.

As the corpus grows, behavior should be factored into reusable primitives
(damage, draw, destroy, continuous restrictions, triggers) rather than one
large bespoke function per printed card.

## Rules boundary

Every `Game` owns a `Format` profile. Eternal Central 93/94 uses current Magic
rules plus its explicit exceptions, notably phase-boundary mana burn. The final
pre-Theros ISD–RTR Standard profile uses its eight-set legality snapshot,
empties mana after each step and phase, and has no mana burn. Both currently
use London mulligans. The POC implements priority-bearing turn steps, cleanup,
combat, and fixed built-in decks for both profiles.

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
