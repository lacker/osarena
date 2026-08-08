//! The canonical wire format for bots, and a facade for driving games with it.
//!
//! Everything a bot ever sees crosses this boundary as JSON produced here:
//! the Python bindings, the C FFI, and any future tournament server all call
//! these functions, so a bot trained locally and a bot competing remotely
//! read byte-identical observations. Treat any change to the shapes below as
//! a protocol version bump.
//!
//! Seats are named `"p1"` ([`PlayerId::One`]) and `"p2"` ([`PlayerId::Two`]).
//! A bot acts by sending back the `index` of one of its observation's
//! `legalActions`; the engine validates every index against the legal list,
//! so no illegal move can be expressed at all.

use serde_json::{Value, json};

use crate::card::{
    CardDefinition, CardEffectStatus, CardRules, CardSet, CardStructure, ManaCost, ModeDef,
    PlayActionKind, PlayOptionDef, PlayRestriction, SpellForm, TargetSlotDef,
};
use crate::casting::{CastChoices, CastSignature};
use crate::game::{DecisionKind, DecisionObservation, DecisionOrderSemantics, StackObservation};
use crate::policy::Policy;
use crate::{
    Action, CardCatalog, Deck, Format, Game, GameObjectId, GameResult, HandcraftedPolicy,
    ManaColor, PlayerId, PlayerObservation, RandomPolicy, StackObjectKind, Target, WinReason,
    decks, poc,
};

/// The wire contract: the JSON shapes here and the action space they
/// describe. Bumped whenever a bot written against the old number could
/// misread the new output — a renamed field, or a change to what appears in
/// `legalActions`. Version 1 dropped conceding from the bot's actions. Version
/// 2 added formats, game-object identity, and structured casting choices.
/// Version 3 identifies trigger procedures and triggered stack objects.
pub const PROTOCOL_VERSION: u32 = 3;

/// The engine crate version. Rules behavior is part of the contract too: a
/// fix can change what a trained policy sees even when the shapes hold
/// still, so pin this alongside any trained weights.
pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// One shy of the engine's own replay guard, so a runaway game fails here
/// with a protocol error instead of an engine panic.
const ACTION_LIMIT: usize = 50_000;

const OLD_SCHOOL_DECK_NAMES: &[&str] = &[
    "Goblins",
    "Sligh",
    "Artifacts",
    "Robots",
    "The Deck",
    "Mono Black",
    "White Weenie",
    "Erhnamgeddon",
    "Counterburn",
    "Lions DIB",
    "Lion Dib Bolt",
    "BWR Aggro",
    "GR Aggro",
    "Troll Disk",
    "Jeskai Aggro",
];

const ISD_RTR_STANDARD_DECK_NAMES: &[&str] = &[
    "Briksza Naya Midrange",
    "Greer G/R Aggro",
    "Fyrberg B/G Midrange",
    "Smith Naya Midrange",
    "McDuffie U/W/R Flash",
    "Lorren U/W Flash",
    "Arch U/W Flash",
    "Kuenzinger Junk Reanimator",
];

/// Parses a public protocol format slug.
///
/// # Errors
///
/// Returns a stable message when the slug does not name a supported format.
pub fn parse_format_slug(slug: &str) -> Result<Format, String> {
    match slug.trim().to_ascii_lowercase().as_str() {
        "old-school-93-94" | "old_school_93_94" => Ok(Format::OldSchool9394),
        "isd-rtr-standard" | "isd_rtr_standard" => Ok(Format::IsdRtrStandard),
        _ => Err(format!("unknown format: {slug}")),
    }
}

/// The deck names accepted for `format`, in menu order.
#[must_use]
pub fn deck_names_for_format(format: Format) -> Vec<&'static str> {
    match format {
        Format::OldSchool9394 => OLD_SCHOOL_DECK_NAMES.to_vec(),
        Format::IsdRtrStandard => ISD_RTR_STANDARD_DECK_NAMES.to_vec(),
    }
}

/// Looks up one built-in deck within `format`, case-insensitively.
#[must_use]
pub fn deck_by_name_for_format(format: Format, name: &str) -> Option<Deck> {
    let name = name.trim().to_ascii_lowercase();
    match format {
        Format::OldSchool9394 => match name.as_str() {
            "goblins" => Some(decks::old_school_93_94::goblins()),
            "sligh" => Some(decks::old_school_93_94::sligh()),
            "artifacts" | "mono red atog" | "mono-red atog" => {
                Some(decks::old_school_93_94::artifacts())
            }
            "robots" => Some(decks::old_school_93_94::robots()),
            "the deck" => Some(decks::old_school_93_94::the_deck()),
            "mono black" => Some(decks::old_school_93_94::mono_black()),
            "white weenie" => Some(decks::old_school_93_94::white_weenie()),
            "erhnamgeddon" => Some(decks::old_school_93_94::erhnamgeddon()),
            "counterburn" => Some(decks::old_school_93_94::counterburn()),
            "lions/dib" | "lions dib" => Some(decks::old_school_93_94::lions_dib()),
            "bwr aggro" => Some(decks::old_school_93_94::bwr_aggro()),
            "gr aggro" | "g/r aggro" => Some(decks::old_school_93_94::gr_aggro()),
            "troll disk" => Some(decks::old_school_93_94::troll_disk()),
            "jeskai aggro" => Some(decks::old_school_93_94::jeskai_aggro()),
            "lion dib bolt" | "lions/dib bolt" | "lions dib bolt" => {
                Some(decks::old_school_93_94::lions_dib_bolt())
            }
            _ => None,
        },
        Format::IsdRtrStandard => match name.as_str() {
            "briksza naya midrange"
            | "rudy briksza naya midrange"
            | "naya midrange rudy briksza"
            | "naya_midrange_rudy_briksza" => {
                Some(decks::isd_rtr_standard::naya_midrange_rudy_briksza())
            }
            "greer g/r aggro"
            | "joseph greer g/r aggro"
            | "g/r aggro joseph greer"
            | "gr_aggro_joseph_greer" => Some(decks::isd_rtr_standard::gr_aggro_joseph_greer()),
            "fyrberg b/g midrange"
            | "mike fyrberg b/g midrange"
            | "b/g midrange mike fyrberg"
            | "bg_midrange_mike_fyrberg" => {
                Some(decks::isd_rtr_standard::bg_midrange_mike_fyrberg())
            }
            "smith naya midrange"
            | "jimmie smith naya midrange"
            | "naya midrange jimmie smith"
            | "naya_midrange_jimmie_smith" => {
                Some(decks::isd_rtr_standard::naya_midrange_jimmie_smith())
            }
            "mcduffie u/w/r flash"
            | "korey mcduffie u/w/r flash"
            | "u/w/r flash korey mcduffie"
            | "uwr_flash_korey_mcduffie" => {
                Some(decks::isd_rtr_standard::uwr_flash_korey_mcduffie())
            }
            "lorren u/w flash"
            | "phillip lorren u/w flash"
            | "u/w flash phillip lorren"
            | "uw_flash_phillip_lorren" => Some(decks::isd_rtr_standard::uw_flash_phillip_lorren()),
            "arch u/w flash"
            | "clayton arch u/w flash"
            | "u/w flash clayton arch"
            | "uw_flash_clayton_arch" => Some(decks::isd_rtr_standard::uw_flash_clayton_arch()),
            "kuenzinger junk reanimator"
            | "drew kuenzinger junk reanimator"
            | "junk reanimator drew kuenzinger"
            | "junk_reanimator_drew_kuenzinger" => {
                Some(decks::isd_rtr_standard::junk_reanimator_drew_kuenzinger())
            }
            _ => None,
        },
    }
}

/// The original Old School deck registry, retained for compatibility.
#[must_use]
pub fn deck_names() -> Vec<&'static str> {
    deck_names_for_format(Format::OldSchool9394)
}

/// Looks up an Old School deck by display name, case-insensitively.
#[must_use]
pub fn deck_by_name(name: &str) -> Option<Deck> {
    deck_by_name_for_format(Format::OldSchool9394, name)
}

fn seat_name(player: PlayerId) -> &'static str {
    match player {
        PlayerId::One => "p1",
        PlayerId::Two => "p2",
    }
}

fn seat_by_name(name: &str) -> Option<PlayerId> {
    match name {
        "p1" => Some(PlayerId::One),
        "p2" => Some(PlayerId::Two),
        _ => None,
    }
}

fn target_json(target: Target) -> Value {
    match target {
        Target::Player(player) => json!({ "type": "player", "seat": seat_name(player) }),
        Target::Permanent(id) => json!({
            "type": "permanent",
            "objectId": id.0,
            "instance": id.0,
        }),
        Target::Spell(id) => json!({
            "type": "spell",
            "objectId": id.0,
            "stackId": id.0,
        }),
    }
}

fn instances_json(cards: &[GameObjectId]) -> Value {
    Value::from(cards.iter().map(|card| card.0).collect::<Vec<_>>())
}

fn spell_form_json(form: &SpellForm) -> Value {
    match form {
        SpellForm::Part(part) => json!({
            "kind": "part",
            "partId": part.0,
        }),
        SpellForm::Combined(parts) => json!({
            "kind": "combined",
            "partIds": parts.iter().map(|part| part.0).collect::<Vec<_>>(),
        }),
    }
}

fn target_selections_json(selections: &[crate::TargetSelection]) -> Vec<Value> {
    selections
        .iter()
        .map(|selection| {
            json!({
                "slotId": selection.slot().0,
                "targets": selection
                    .targets()
                    .iter()
                    .copied()
                    .map(target_json)
                    .collect::<Vec<_>>(),
            })
        })
        .collect()
}

fn cast_choices_json(choices: &CastChoices) -> Value {
    json!({
        "playOptionId": choices.play_option().0,
        "modeIds": choices.modes().iter().map(|mode| mode.0).collect::<Vec<_>>(),
        "alternativeCostId": choices.costs().alternative().map(|cost| cost.0),
        "additionalCostIds": choices
            .costs()
            .additional()
            .iter()
            .map(|cost| cost.0)
            .collect::<Vec<_>>(),
        "x": choices.x(),
        "targetSelections": target_selections_json(choices.targets()),
    })
}

fn cast_signature_json(signature: &CastSignature) -> Value {
    json!({
        "playOptionId": signature.play_option().0,
        "form": spell_form_json(signature.form()),
        "modeIds": signature.modes().iter().map(|mode| mode.0).collect::<Vec<_>>(),
        "alternativeCostId": signature.costs().alternative().map(|cost| cost.0),
        "additionalCostIds": signature
            .costs()
            .additional()
            .iter()
            .map(|cost| cost.0)
            .collect::<Vec<_>>(),
        "x": signature.x(),
        "targetSelections": target_selections_json(signature.targets()),
    })
}

const fn mana_color_name(color: ManaColor) -> &'static str {
    match color {
        ManaColor::White => "white",
        ManaColor::Blue => "blue",
        ManaColor::Black => "black",
        ManaColor::Red => "red",
        ManaColor::Green => "green",
        ManaColor::Colorless => "colorless",
    }
}

/// Serializes one legal action. The `type` tag names the engine's action
/// variant; the remaining fields identify what it operates on.
#[must_use]
pub fn action_json(action: &Action) -> Value {
    match action {
        Action::KeepHand => json!({ "type": "KeepHand" }),
        Action::TakeMulligan => json!({ "type": "TakeMulligan" }),
        Action::BottomCards { cards } => {
            json!({ "type": "BottomCards", "cards": instances_json(cards) })
        }
        Action::DiscardCards { cards } => {
            json!({ "type": "DiscardCards", "cards": instances_json(cards) })
        }
        Action::ChooseDecision { decision, options } => {
            json!({ "type": "ChooseDecision", "decision": decision, "options": options })
        }
        Action::CancelDecision { decision } => {
            json!({ "type": "CancelDecision", "decision": decision })
        }
        Action::ChooseUntap { permanents } => {
            json!({ "type": "ChooseUntap", "permanents": instances_json(permanents) })
        }
        Action::PassPriority => json!({ "type": "PassPriority" }),
        Action::PlayLand { card, option } => json!({
            "type": "PlayLand",
            "card": card.0,
            "playOptionId": option.0,
        }),
        Action::ActivateManaAbility { source, color } => json!({
            "type": "ActivateManaAbility",
            "source": source.0,
            "color": mana_color_name(*color),
        }),
        Action::PayLifeForMana => json!({ "type": "PayLifeForMana" }),
        Action::CastSpell {
            card,
            choices,
            sacrifices,
        } => json!({
            "type": "CastSpell",
            "card": card.0,
            "choices": cast_choices_json(choices),
            "playOptionId": choices.play_option().0,
            "modeIds": choices.modes().iter().map(|mode| mode.0).collect::<Vec<_>>(),
            "targets": choices.iter_targets().copied().map(target_json).collect::<Vec<_>>(),
            "sacrifices": instances_json(sacrifices),
            "x": choices.x(),
        }),
        Action::ActivateAbility {
            source,
            target,
            sacrifice,
        } => json!({
            "type": "ActivateAbility",
            "source": source.0,
            "target": target.map(target_json),
            "sacrifice": sacrifice.map(|card| card.0),
        }),
        Action::DeclareAttacker { attacker } => {
            json!({ "type": "DeclareAttacker", "attacker": attacker.0 })
        }
        Action::FinishDeclaringAttackers => json!({ "type": "FinishDeclaringAttackers" }),
        Action::DeclareBlocker { blocker, attacker } => {
            json!({ "type": "DeclareBlocker", "blocker": blocker.0, "attacker": attacker.0 })
        }
        Action::FinishDeclaringBlockers => json!({ "type": "FinishDeclaringBlockers" }),
        Action::AssignCombatDamage {
            attacker,
            assignments,
        } => json!({
            "type": "AssignCombatDamage",
            "attacker": attacker.0,
            "assignments": assignments.iter().map(|assignment| json!({
                "recipient": target_json(assignment.recipient),
                "amount": assignment.amount,
            })).collect::<Vec<_>>(),
        }),
        Action::Concede => json!({ "type": "Concede" }),
    }
}

fn card_name(catalog: &CardCatalog, definition: crate::CardDefinitionId) -> Value {
    catalog
        .get(definition)
        .map_or(Value::Null, |card| Value::from(card.name.clone()))
}

fn card_part_name(
    catalog: &CardCatalog,
    definition: crate::CardDefinitionId,
    part: crate::CardPartId,
) -> Value {
    catalog.get(definition).map_or(Value::Null, |card| {
        Value::from(
            card.part(part)
                .map_or_else(|| card.name.clone(), |part| part.name.clone()),
        )
    })
}

fn stack_card_name(
    catalog: &CardCatalog,
    definition: crate::CardDefinitionId,
    signature: Option<&CastSignature>,
) -> Value {
    let Some(card) = catalog.get(definition) else {
        return Value::Null;
    };
    let Some(signature) = signature else {
        return Value::from(card.name.clone());
    };

    let resolved = match signature.form() {
        SpellForm::Part(part) => card.part(*part).map(|part| part.name.clone()),
        SpellForm::Combined(parts) if !parts.is_empty() => parts
            .iter()
            .map(|part| card.part(*part).map(|part| part.name.as_str()))
            .collect::<Option<Vec<_>>>()
            .map(|names| names.join(" // ")),
        SpellForm::Combined(_) => None,
    };
    Value::from(resolved.unwrap_or_else(|| card.name.clone()))
}

fn card_list_json(
    catalog: &CardCatalog,
    cards: &[(GameObjectId, crate::CardDefinitionId)],
) -> Value {
    Value::from(
        cards
            .iter()
            .map(|(instance, definition)| {
                json!({
                    "objectId": instance.0,
                    "instance": instance.0,
                    "definition": definition.0,
                    "name": card_name(catalog, *definition),
                })
            })
            .collect::<Vec<_>>(),
    )
}

fn mana_pool_json(pool: &crate::ManaPool) -> Value {
    json!({
        "white": pool.white,
        "blue": pool.blue,
        "black": pool.black,
        "red": pool.red,
        "green": pool.green,
        "colorless": pool.colorless,
    })
}

fn decision_json(catalog: &CardCatalog, decision: &DecisionObservation) -> Value {
    let mut value = json!({
        "id": decision.id,
        "seat": seat_name(decision.player),
        "kind": match decision.kind {
            DecisionKind::Choice => "Choice",
            DecisionKind::TriggerOrder => "TriggerOrder",
            DecisionKind::TriggerPlacement => "TriggerPlacement",
        },
        "prompt": decision.prompt,
        "visibility": format!("{:?}", decision.visibility),
        "minimum": decision.minimum,
        "maximum": decision.maximum,
        "cancellable": decision.cancellable,
        "options": decision.options.iter().map(|option| json!({
            "id": option.id,
            "triggerId": matches!(decision.kind, DecisionKind::TriggerOrder).then_some(option.id),
            "label": option.label,
            "card": option.card.map(|(instance, definition)| json!({
                "objectId": instance.0,
                "instance": instance.0,
                "definition": definition.0,
                "name": card_name(catalog, definition),
            })),
            "abilityText": option.ability_text,
            "zone": format!("{:?}", option.zone),
        })).collect::<Vec<_>>(),
    });
    if let Some(order_semantics) = decision.order_semantics {
        value["orderSemantics"] = Value::from(match order_semantics {
            DecisionOrderSemantics::Resolution => "resolution",
        });
    }
    value
}

fn result_json(result: GameResult) -> Value {
    match result {
        GameResult::Draw => json!({ "winner": Value::Null, "reason": "Draw" }),
        GameResult::Winner { winner, reason } => json!({
            "winner": seat_name(winner),
            "reason": match reason {
                WinReason::OpponentConceded => "OpponentConceded",
                WinReason::OpponentLostAllLife => "OpponentLostAllLife",
                WinReason::OpponentTriedToDrawFromEmptyLibrary =>
                    "OpponentTriedToDrawFromEmptyLibrary",
            },
        }),
    }
}

/// Translates the engine's action list into the one bots see.
///
/// Two differences from [`PlayerObservation::legal_actions`]:
///
/// Conceding is dropped. It is legal in every state, and for a bot it is
/// strictly dominated — resigning can only lose a game that playing on might
/// win — so both built-in policies already refuse it and no rational bot
/// would pick it. Leaving it in made uniform-random exploration resign on
/// turn one, which is a poor action space for the audience this protocol is
/// for. Humans still concede through the browser, which reads the engine's
/// list directly.
///
/// Pending decisions are expanded. The engine lists one template action with
/// empty `options`, expecting the caller to fill in ids from the decision
/// schema. Bots act by index, so a pick-exactly-one decision becomes one
/// concrete action per option, and a multi-pick keeps a default choice of the
/// first `minimum` options so an index-only bot always has a legal move. Bots
/// that want a different multi-pick send it through
/// [`BotGame::choose_decision`].
#[must_use]
pub fn protocol_actions(observation: &PlayerObservation) -> Vec<Action> {
    let mut actions = Vec::with_capacity(observation.legal_actions.len());
    for action in &observation.legal_actions {
        if matches!(action, Action::Concede) {
            continue;
        }
        match (action, observation.decision.as_ref()) {
            (Action::ChooseDecision { decision, options }, Some(pending))
                if options.is_empty() && *decision == pending.id =>
            {
                if pending.minimum == 1 && pending.maximum == 1 {
                    for option in &pending.options {
                        actions.push(Action::ChooseDecision {
                            decision: *decision,
                            options: vec![option.id],
                        });
                    }
                } else {
                    actions.push(Action::ChooseDecision {
                        decision: *decision,
                        // The neutral default: the first `minimum` options,
                        // which for a may-choose decision means declining.
                        options: pending
                            .options
                            .iter()
                            .take(pending.minimum)
                            .map(|option| option.id)
                            .collect(),
                    });
                }
            }
            _ => actions.push(action.clone()),
        }
    }
    actions
}

/// Serializes one seat's redacted view of the game.
///
/// This is the observation a bot decides from: public zones in full, the
/// opponent's hand as a count, and `legalActions` carrying the indices the
/// bot answers with. `pregame` is true while mulligans are being settled.
/// `actions` is the protocol action list the indices refer to — normally
/// [`protocol_actions`] of the same observation.
#[must_use]
pub fn observation_json(
    catalog: &CardCatalog,
    observation: &PlayerObservation,
    pregame: bool,
    actions: &[Action],
) -> Value {
    observation_json_for_format(
        catalog,
        Format::OldSchool9394,
        observation,
        pregame,
        actions,
    )
}

/// Serializes one seat's redacted view together with its governing format.
#[must_use]
pub fn observation_json_for_format(
    catalog: &CardCatalog,
    format: Format,
    observation: &PlayerObservation,
    pregame: bool,
    actions: &[Action],
) -> Value {
    json!({
        "protocolVersion": PROTOCOL_VERSION,
        "engineVersion": ENGINE_VERSION,
        "format": format.slug(),
        "seat": seat_name(observation.viewer),
        "pregame": pregame,
        "turn": observation.turn,
        "activeTurn": observation.active_turn,
        "activeSeat": seat_name(observation.active_player),
        "prioritySeat": seat_name(observation.priority),
        "step": format!("{:?}", observation.step),
        "life": observation.life_totals,
        "manaPools": [
            mana_pool_json(&observation.mana_pools[0]),
            mana_pool_json(&observation.mana_pools[1]),
        ],
        "hand": card_list_json(catalog, &observation.hand),
        "opponentHandSize": observation.opponent_hand_size,
        "lastSeenHand": observation.last_seen_hand.as_ref().map(|(player, cards)| json!({
            "seat": seat_name(*player),
            "cards": card_list_json(catalog, cards),
        })),
        "librarySizes": observation.library_sizes,
        "graveyards": [
            card_list_json(catalog, &observation.graveyards[0]),
            card_list_json(catalog, &observation.graveyards[1]),
        ],
        "exiles": [
            card_list_json(catalog, &observation.exiles[0]),
            card_list_json(catalog, &observation.exiles[1]),
        ],
        "battlefield": observation.battlefield.iter().map(|permanent| json!({
            "objectId": permanent.id.0,
            "instance": permanent.id.0,
            "definition": permanent.definition.0,
            "presentedPartId": permanent.presented.0,
            "name": card_part_name(catalog, permanent.definition, permanent.presented),
            "controller": seat_name(permanent.controller),
            "tapped": permanent.tapped,
            "power": permanent.power,
            "toughness": permanent.toughness,
            "damage": permanent.damage,
            "attacking": permanent.attacking,
            "blocking": permanent.blocking.map(|id| id.0),
            "flying": permanent.flying,
            "canAttack": permanent.can_attack,
            "enteredThisTurn": permanent.entered_this_turn,
        })).collect::<Vec<_>>(),
        "stack": observation
            .stack
            .iter()
            .map(|object| stack_object_json(catalog, object))
            .collect::<Vec<_>>(),
        "decision": observation.decision.as_ref().map(|decision| decision_json(catalog, decision)),
        "result": observation.result.map(result_json),
        "legalActions": actions.iter().enumerate().map(|(index, action)| {
            let mut value = action_json(action);
            if let Value::Object(map) = &mut value {
                map.insert("index".into(), Value::from(index));
            }
            value
        }).collect::<Vec<_>>(),
    })
}

fn stack_object_json(catalog: &CardCatalog, object: &StackObservation) -> Value {
    json!({
        "objectId": object.id.0,
        "stackId": object.id.0,
        // Compatibility alias: this is a game object, not physical lineage.
        "instance": object.id.0,
        "sourceObjectId": object.source.map(|source| source.0),
        "source": object.source.map(|source| source.0),
        "abilityId": object.ability.map(|ability| ability.0),
        "abilityText": object.ability_text,
        "kind": match object.kind {
            StackObjectKind::Spell => "Spell",
            StackObjectKind::ActivatedAbility => "ActivatedAbility",
            StackObjectKind::TriggeredAbility => "TriggeredAbility",
        },
        "definition": object.definition.0,
        "name": stack_card_name(catalog, object.definition, object.signature.as_ref()),
        "controller": seat_name(object.controller),
        "signature": object.signature.as_ref().map(cast_signature_json),
        "targets": object
            .targets
            .iter()
            .copied()
            .map(target_json)
            .collect::<Vec<_>>(),
        "chosenPermanents": object
            .chosen_permanents
            .iter()
            .map(|permanent| permanent.0)
            .collect::<Vec<_>>(),
        "x": object.x,
    })
}

fn mana_cost_json(cost: &ManaCost) -> Value {
    json!({
        "generic": cost.generic,
        "white": cost.white,
        "blue": cost.blue,
        "black": cost.black,
        "red": cost.red,
        "green": cost.green,
        "whiteRedHybrid": cost.white_red_hybrid,
        "variableX": cost.variable_x,
        "xMultiplier": cost.x_multiplier,
    })
}

const fn effect_status_name(status: CardEffectStatus) -> &'static str {
    match status {
        CardEffectStatus::Implemented => "implemented",
        CardEffectStatus::MetadataOnly => "metadataOnly",
    }
}

const fn card_set_slug(set: CardSet) -> &'static str {
    match set {
        CardSet::Alpha => "alpha",
        CardSet::Beta => "beta",
        CardSet::Unlimited => "unlimited",
        CardSet::CollectorsEdition => "collectors-edition",
        CardSet::InternationalCollectorsEdition => "international-collectors-edition",
        CardSet::ArabianNights => "arabian-nights",
        CardSet::Antiquities => "antiquities",
        CardSet::Revised => "revised",
        CardSet::Legends => "legends",
        CardSet::TheDark => "the-dark",
        CardSet::FallenEmpires => "fallen-empires",
        CardSet::Promo1994 => "promo-1994",
        CardSet::Innistrad => "innistrad",
        CardSet::DarkAscension => "dark-ascension",
        CardSet::AvacynRestored => "avacyn-restored",
        CardSet::Magic2013 => "magic-2013",
        CardSet::ReturnToRavnica => "return-to-ravnica",
        CardSet::Gatecrash => "gatecrash",
        CardSet::DragonsMaze => "dragons-maze",
        CardSet::Magic2014 => "magic-2014",
    }
}

fn rules_json(rules: &CardRules, mana_cost: Option<&ManaCost>) -> Value {
    let stats = rules.creature_stats;
    json!({
        "kind": format!("{:?}", rules.kind),
        "typeLine": rules.type_line,
        "manaCost": mana_cost.map(mana_cost_json),
        "power": stats.map(|stats| stats.power),
        "toughness": stats.map(|stats| stats.toughness),
        "rulesText": rules.text,
        "effectStatus": effect_status_name(rules.effect_status),
        "colors": rules.colors,
    })
}

fn structure_json(structure: &CardStructure) -> Value {
    match structure {
        CardStructure::Single { main } => json!({
            "kind": "single",
            "mainPartId": main.0,
        }),
        CardStructure::Split { parts, fused } => json!({
            "kind": "split",
            "partIds": parts.iter().map(|part| part.0).collect::<Vec<_>>(),
            "fusedPlayOptionId": fused.map(|option| option.0),
        }),
        CardStructure::Flip { normal, flipped } => json!({
            "kind": "flip",
            "normalPartId": normal.0,
            "flippedPartId": flipped.0,
        }),
        CardStructure::DoubleFaced { front, back, kind } => json!({
            "kind": "doubleFaced",
            "frontPartId": front.0,
            "backPartId": back.0,
            "doubleFacedKind": format!("{kind:?}"),
        }),
        CardStructure::AlternateSpell {
            main,
            alternate,
            kind,
        } => json!({
            "kind": "alternateSpell",
            "mainPartId": main.0,
            "alternatePartId": alternate.0,
            "alternateSpellKind": format!("{kind:?}"),
        }),
        CardStructure::MeldPart { front, recipe } => json!({
            "kind": "meldPart",
            "frontPartId": front.0,
            "meldRecipeId": recipe.0,
        }),
    }
}

fn target_slot_json(slot: &TargetSlotDef) -> Value {
    json!({
        "id": slot.id.0,
        "label": slot.label,
        "predicate": format!("{:?}", slot.predicate),
        "minimum": slot.minimum,
        "maximum": slot.maximum,
    })
}

fn mode_json(mode: &ModeDef) -> Value {
    json!({
        "id": mode.id.0,
        "label": mode.label,
        "targets": mode.targets.iter().map(target_slot_json).collect::<Vec<_>>(),
        "effectStatus": effect_status_name(mode.effect_status),
    })
}

fn play_option_json(option: &PlayOptionDef) -> Value {
    json!({
        "id": option.id.0,
        "label": option.label,
        "action": match option.action {
            PlayActionKind::CastSpell => "CastSpell",
            PlayActionKind::PlayLand => "PlayLand",
        },
        "form": spell_form_json(&option.form),
        "manaCost": option.mana_cost.as_ref().map(mana_cost_json),
        "restriction": match option.restriction {
            PlayRestriction::Normal => "normal",
            PlayRestriction::FromHandOnly => "fromHandOnly",
        },
        "modes": option.modes.as_ref().map(|modes| json!({
            "minimum": modes.minimum,
            "maximum": modes.maximum,
            "mayRepeat": modes.may_repeat,
            "choices": modes.modes.iter().map(mode_json).collect::<Vec<_>>(),
        })),
        "targets": option.targets.iter().map(target_slot_json).collect::<Vec<_>>(),
        "alternativeCosts": option.alternative_costs.iter().map(|cost| json!({
            "id": cost.id.0,
            "label": cost.label,
            "manaCost": mana_cost_json(&cost.mana_cost),
        })).collect::<Vec<_>>(),
        "additionalCosts": option.additional_costs.iter().map(|cost| json!({
            "id": cost.id.0,
            "label": cost.label,
            "manaCost": cost.mana_cost.as_ref().map(mana_cost_json),
        })).collect::<Vec<_>>(),
        "effectStatus": effect_status_name(option.effect_status),
    })
}

fn definition_json(catalog: &CardCatalog, format: Format, card: &CardDefinition) -> Value {
    let rules = &card.rules;
    let stats = rules.creature_stats;
    let allowed = catalog.is_allowed_in(card.id, format);
    let banned = catalog.is_banned_in(card.id, format);
    let restricted = catalog.is_restricted_in(card.id, format);
    json!({
        // Compatibility fields retained from protocol v1.
        "definition": card.id.0,
        "name": card.name,
        "kind": format!("{:?}", rules.kind),
        "isBasicLand": card.is_basic_land,
        "manaCost": mana_cost_json(&rules.mana_cost),
        "power": stats.map(|stats| stats.power),
        "toughness": stats.map(|stats| stats.toughness),
        "rulesText": rules.text,
        "banned": banned,
        "restricted": restricted,
        // Protocol v2 structured and format-aware metadata.
        "allowed": allowed,
        "legal": allowed && !banned,
        "debutSet": card_set_slug(card.set),
        "effectStatus": effect_status_name(rules.effect_status),
        "structure": structure_json(&card.structure),
        "parts": card.parts.iter().map(|part| {
            let mut value = rules_json(&part.rules, part.mana_cost.as_ref());
            let Value::Object(fields) = &mut value else {
                unreachable!("rules JSON is always an object");
            };
            fields.insert("id".into(), Value::from(part.id.0));
            fields.insert("name".into(), Value::from(part.name.clone()));
            value
        }).collect::<Vec<_>>(),
        "playOptions": card.play_options.iter().map(play_option_json).collect::<Vec<_>>(),
        "printings": card.printings.iter().map(|printing| json!({
            "set": card_set_slug(printing.id.set),
            "variant": printing.id.variant,
        })).collect::<Vec<_>>(),
    })
}

/// Serializes every card definition for the default Old School format.
#[must_use]
pub fn catalog_json(catalog: &CardCatalog) -> Value {
    catalog_json_for_format(catalog, Format::OldSchool9394)
}

/// Serializes every canonical definition, its structured play metadata, and
/// legality in `format`. Printings remain metadata and never duplicate a card
/// definition in the returned list.
#[must_use]
pub fn catalog_json_for_format(catalog: &CardCatalog, format: Format) -> Value {
    json!({
        "protocolVersion": PROTOCOL_VERSION,
        "engineVersion": ENGINE_VERSION,
        "format": format.slug(),
        "formatName": format.display_name(),
        "cards": catalog
            .definitions()
            .into_iter()
            .map(|card| definition_json(catalog, format, card))
            .collect::<Vec<_>>(),
    })
}

/// Which policy, if any, plays the seat a bot is not driving.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Opponent {
    /// No built-in opponent: the caller drives both seats (self-play).
    External,
    /// The seeded uniform-random baseline.
    Random,
    /// The strongest built-in policy.
    Handcrafted,
}

// Concrete policy types rather than a boxed trait object, so the enum stays
// Clone — forking a game for search clones the opponent's state with it —
// and Send + Sync, which the Python bindings need to move games across
// threads and parallel rollout collection wants anyway.
#[derive(Clone)]
enum OpponentPolicy {
    External,
    Random(RandomPolicy),
    Handcrafted(HandcraftedPolicy),
}

/// A game driven through the bot protocol.
///
/// With a scripted opponent, [`BotGame::act`] plays your action and then lets
/// the opponent play until you have a real choice again, exactly like a
/// hotseat against the built-in bot. With [`Opponent::External`] it stops at
/// every decision, whichever seat owns it, so one loop can drive both sides
/// for self-play.
///
/// Cloning a `BotGame` snapshots everything — the game and any scripted
/// opponent's state — so a clone fed the same indices replays the identical
/// game, and a clone fed different ones never disturbs the original. That
/// fork-and-try is the primitive rollout- and tree-search bots are built on.
#[derive(Clone)]
pub struct BotGame {
    game: Game,
    catalog: CardCatalog,
    format: Format,
    opponent_seat: PlayerId,
    opponent: OpponentPolicy,
}

impl BotGame {
    /// Starts a game. `p1_deck`/`p2_deck` name built-in decks; `opponent`
    /// plays `opponent_seat` unless it is [`Opponent::External`].
    ///
    /// # Errors
    ///
    /// Returns a message when a deck name is unknown, the game cannot be
    /// built, or the scripted opponent cannot reach the first decision.
    pub fn new(
        p1_deck: &str,
        p2_deck: &str,
        opponent: Opponent,
        opponent_seat: PlayerId,
        seed: u64,
    ) -> Result<Self, String> {
        Self::new_with_format(
            Format::OldSchool9394,
            p1_deck,
            p2_deck,
            opponent,
            opponent_seat,
            seed,
        )
    }

    /// Starts a game using decks and rules from `format`.
    ///
    /// # Errors
    ///
    /// Returns a message when a deck does not belong to `format`, the game
    /// cannot be built, or the scripted opponent cannot reach a decision.
    pub fn new_with_format(
        format: Format,
        p1_deck: &str,
        p2_deck: &str,
        opponent: Opponent,
        opponent_seat: PlayerId,
        seed: u64,
    ) -> Result<Self, String> {
        let catalog = poc::catalog().map_err(|error| error.to_string())?;
        let deck_one = deck_by_name_for_format(format, p1_deck)
            .ok_or_else(|| format!("unknown deck for {}: {p1_deck}", format.slug()))?;
        let deck_two = deck_by_name_for_format(format, p2_deck)
            .ok_or_else(|| format!("unknown deck for {}: {p2_deck}", format.slug()))?;
        let game = Game::new_with_format(format, catalog.clone(), [deck_one, deck_two], seed)
            .map_err(|error| error.to_string())?;
        let opponent = match opponent {
            Opponent::External => OpponentPolicy::External,
            Opponent::Random => OpponentPolicy::Random(RandomPolicy::new(seed ^ 0x00b0_7b07)),
            Opponent::Handcrafted => {
                OpponentPolicy::Handcrafted(HandcraftedPolicy::new(catalog.clone()))
            }
        };
        let mut bot_game = Self {
            game,
            catalog,
            format,
            opponent_seat,
            opponent,
        };
        bot_game.advance()?;
        Ok(bot_game)
    }

    /// Starts a game from a JSON config, the single entry point the FFI and
    /// Python bindings share:
    ///
    /// ```json
    /// {"format": "old-school-93-94", "p1Deck": "Sligh", "p2Deck": "The Deck",
    ///  "opponent": "handcrafted", "opponentSeat": "p2", "seed": 42}
    /// ```
    ///
    /// `format` defaults to `"old-school-93-94"`; `opponent` is `"random"`,
    /// `"handcrafted"`, or `"external"`; `opponentSeat` defaults to `"p2"`.
    ///
    /// # Errors
    ///
    /// Returns a message for malformed JSON, unknown deck or opponent names,
    /// or a game that cannot start.
    pub fn from_config_json(config: &str) -> Result<Self, String> {
        let value: Value =
            serde_json::from_str(config).map_err(|error| format!("bad config JSON: {error}"))?;
        let field = |name: &str| -> Result<&str, String> {
            value[name]
                .as_str()
                .ok_or_else(|| format!("config field {name} must be a string"))
        };
        let opponent = match field("opponent").unwrap_or("handcrafted") {
            "external" => Opponent::External,
            "random" => Opponent::Random,
            "handcrafted" => Opponent::Handcrafted,
            other => return Err(format!("unknown opponent: {other}")),
        };
        let opponent_seat = seat_by_name(field("opponentSeat").unwrap_or("p2"))
            .ok_or_else(|| "opponentSeat must be \"p1\" or \"p2\"".to_string())?;
        let format = match value.get("format") {
            None => Format::OldSchool9394,
            Some(value) => parse_format_slug(
                value
                    .as_str()
                    .ok_or_else(|| "config field format must be a string".to_string())?,
            )?,
        };
        let seed = value["seed"].as_u64().unwrap_or(0);
        Self::new_with_format(
            format,
            field("p1Deck")?,
            field("p2Deck")?,
            opponent,
            opponent_seat,
            seed,
        )
    }

    /// The format whose rules and deck registry this game uses.
    #[must_use]
    pub const fn format(&self) -> Format {
        self.format
    }

    /// The seat that must act next, or `None` when the game is over.
    #[must_use]
    pub fn decision_seat(&self) -> Option<PlayerId> {
        self.game.decision_player()
    }

    /// [`Self::decision_seat`] as a protocol seat name.
    #[must_use]
    pub fn decision_seat_name(&self) -> Option<&'static str> {
        self.decision_seat().map(seat_name)
    }

    /// The observation for one seat as canonical protocol JSON.
    #[must_use]
    pub fn observe_json(&self, seat: PlayerId) -> String {
        let observation = self.game.observe(seat);
        let actions = protocol_actions(&observation);
        observation_json_for_format(
            &self.catalog,
            self.format,
            &observation,
            self.game.in_pregame(),
            &actions,
        )
        .to_string()
    }

    /// The number of legal actions for the seat that must act, so FFI
    /// callers can pick an index without parsing JSON.
    #[must_use]
    pub fn legal_action_count(&self) -> usize {
        self.decision_seat()
            .map_or(0, |seat| protocol_actions(&self.game.observe(seat)).len())
    }

    /// The finished game's result, if any, as protocol JSON.
    #[must_use]
    pub fn result(&self) -> Option<GameResult> {
        self.game.observe(PlayerId::One).result
    }

    /// Plays the given index from the acting seat's `legalActions`, then lets
    /// a scripted opponent play until the driven seat has a real choice.
    ///
    /// # Errors
    ///
    /// Returns a message when the game is over, the index is out of range, or
    /// the engine rejects the action.
    pub fn act(&mut self, action_index: usize) -> Result<(), String> {
        let seat = self.decision_seat().ok_or("the game is over")?;
        let actions = protocol_actions(&self.game.observe(seat));
        let action = actions.get(action_index).cloned().ok_or_else(|| {
            format!(
                "action index {action_index} out of range ({} legal actions)",
                actions.len()
            )
        })?;
        self.game
            .apply(seat, action)
            .map_err(|error| error.to_string())?;
        self.advance()
    }

    /// Answers the pending decision with an explicit set of option ids, for
    /// multi-pick decisions where the default expansion is not what you want.
    /// The observation's `decision` object lists the options and bounds.
    ///
    /// # Errors
    ///
    /// Returns a message when no decision is pending or the engine rejects
    /// the selection.
    pub fn choose_decision(&mut self, option_ids: &[u32]) -> Result<(), String> {
        let seat = self.decision_seat().ok_or("the game is over")?;
        let observation = self.game.observe(seat);
        let decision = observation
            .decision
            .as_ref()
            .ok_or("no decision is pending")?;
        self.game
            .apply(
                seat,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: option_ids.to_vec(),
                },
            )
            .map_err(|error| error.to_string())?;
        self.advance()
    }

    /// Runs the scripted opponent until the driven seat must decide or the
    /// game ends. A no-op with an external opponent.
    fn advance(&mut self) -> Result<(), String> {
        let policy: &mut dyn Policy = match &mut self.opponent {
            OpponentPolicy::External => return Ok(()),
            OpponentPolicy::Random(policy) => policy,
            OpponentPolicy::Handcrafted(policy) => policy,
        };
        for _ in 0..ACTION_LIMIT {
            let Some(player) = self.game.decision_player() else {
                return Ok(());
            };
            if player != self.opponent_seat {
                return Ok(());
            }
            let observation = self.game.observe(player);
            let action = policy
                .choose_action(&observation)
                .ok_or("the scripted opponent returned no action")?;
            self.game
                .apply(player, action)
                .map_err(|error| error.to_string())?;
        }
        Err("the game exceeded its action limit".to_string())
    }

    /// The catalog as protocol JSON.
    #[must_use]
    pub fn catalog_json(&self) -> String {
        catalog_json_for_format(&self.catalog, self.format).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn finish(mut game: BotGame, mut pick: impl FnMut(usize, &Value) -> usize) -> GameResult {
        for turn in 0..ACTION_LIMIT {
            if let Some(result) = game.result() {
                return result;
            }
            let seat = game.decision_seat().expect("no result means a decision");
            let observation: Value =
                serde_json::from_str(&game.observe_json(seat)).expect("valid JSON");
            let count = observation["legalActions"]
                .as_array()
                .expect("legalActions is an array")
                .len();
            assert!(count > 0, "a decision always has options");
            game.act(pick(turn, &observation))
                .expect("chosen index is legal");
        }
        panic!("game did not finish");
    }

    /// A do-nothing bot written the way the docs tell people to write one:
    /// read `legalActions`, prefer the quiet options by their `type` tags.
    /// Note it never has to avoid anything: nothing in the list loses on
    /// the spot.
    fn pass_bot(observation: &Value) -> usize {
        let actions = observation["legalActions"].as_array().expect("array");
        for preferred in [
            "KeepHand",
            "ChooseDecision",
            "PassPriority",
            "FinishDeclaringAttackers",
            "FinishDeclaringBlockers",
            "AssignCombatDamage",
            "DiscardCards",
            "BottomCards",
            "ChooseUntap",
        ] {
            if let Some(action) = actions.iter().find(|action| action["type"] == preferred) {
                return usize::try_from(action["index"].as_u64().expect("index"))
                    .expect("index fits");
            }
        }
        0
    }

    fn assert_no_physical_lineage_keys(value: &Value) {
        fn visit(value: &Value, path: &str) {
            match value {
                Value::Object(fields) => {
                    for (key, child) in fields {
                        let normalized = key.to_ascii_lowercase();
                        assert!(
                            !normalized.contains("physical") && !normalized.contains("backing"),
                            "protocol exposed physical-card lineage at {path}.{key}"
                        );
                        visit(child, &format!("{path}.{key}"));
                    }
                }
                Value::Array(items) => {
                    for (index, child) in items.iter().enumerate() {
                        visit(child, &format!("{path}[{index}]"));
                    }
                }
                Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
            }
        }

        visit(value, "$observed");
    }

    #[test]
    fn a_scripted_game_runs_to_a_result_through_json_alone() {
        let game = BotGame::new("Sligh", "The Deck", Opponent::Handcrafted, PlayerId::Two, 7)
            .expect("game starts");
        let result = finish(game, |_, observation| pass_bot(observation));
        assert!(matches!(
            result,
            GameResult::Winner { .. } | GameResult::Draw
        ));
    }

    #[test]
    fn an_external_game_lets_one_loop_drive_both_seats() {
        let game = BotGame::new("Goblins", "Sligh", Opponent::External, PlayerId::Two, 11)
            .expect("game starts");
        let result = finish(game, |_, observation| pass_bot(observation));
        assert!(matches!(
            result,
            GameResult::Winner { .. } | GameResult::Draw
        ));
    }

    #[test]
    fn the_same_seed_produces_the_same_bytes() {
        let make = || {
            BotGame::new("Sligh", "Goblins", Opponent::Random, PlayerId::Two, 99)
                .expect("game starts")
        };
        let (mut first, mut second) = (make(), make());
        for _ in 0..40 {
            if first.result().is_some() {
                break;
            }
            let seat = first.decision_seat().expect("still running");
            assert_eq!(first.observe_json(seat), second.observe_json(seat));
            first.act(0).expect("index 0 is legal");
            second.act(0).expect("index 0 is legal");
        }
    }

    #[test]
    fn a_clone_replays_identically_and_diverges_independently() {
        let mut game = BotGame::new("Sligh", "The Deck", Opponent::Handcrafted, PlayerId::Two, 7)
            .expect("game starts");
        // Reach a mid-game state with real board state on both sides.
        for _ in 0..30 {
            let seat = game.decision_seat().expect("game is still running");
            let observation: Value =
                serde_json::from_str(&game.observe_json(seat)).expect("valid JSON");
            game.act(pass_bot(&observation)).expect("legal index");
        }
        let seat = game.decision_seat().expect("game is still running");
        let mut replay = game.clone();
        assert_eq!(game.observe_json(seat), replay.observe_json(seat));

        // Determinism: the same indices drive both copies — the scripted
        // opponent's state included — to byte-identical observations.
        for _ in 0..20 {
            let seat = game.decision_seat().expect("game is still running");
            let observation: Value =
                serde_json::from_str(&game.observe_json(seat)).expect("valid JSON");
            let choice = pass_bot(&observation);
            game.act(choice).expect("legal in the original");
            replay.act(choice).expect("legal in the clone");
            assert_eq!(game.observe_json(seat), replay.observe_json(seat));
        }

        // Independence: the fork plays a different legal action than the
        // original, the two games stop matching, and the original never
        // notices. Walk to a decision with at least two options first.
        let (seat, choice, other) = loop {
            let seat = game.decision_seat().expect("game is still running");
            let observation: Value =
                serde_json::from_str(&game.observe_json(seat)).expect("valid JSON");
            let count = observation["legalActions"].as_array().expect("array").len();
            if count >= 2 {
                let choice = pass_bot(&observation);
                break (seat, choice, (choice + 1) % count);
            }
            game.act(0).expect("legal index");
        };
        let before = game.observe_json(seat);
        let mut fork = game.clone();
        fork.act(other).expect("legal in the fork");
        assert_eq!(game.observe_json(seat), before, "the original is untouched");
        game.act(choice).expect("legal in the original");
        assert_ne!(
            game.observe_json(seat),
            fork.observe_json(seat),
            "different actions, different games",
        );

        // A fork is a live game, not a snapshot: it plays on by itself.
        for _ in 0..10 {
            if fork.result().is_some() {
                break;
            }
            let seat = fork.decision_seat().expect("fork is still running");
            let observation: Value =
                serde_json::from_str(&fork.observe_json(seat)).expect("valid JSON");
            fork.act(pass_bot(&observation)).expect("legal in the fork");
        }
    }

    #[test]
    fn observations_carry_indexed_legal_actions_and_no_hidden_cards() {
        let game = BotGame::new("Sligh", "The Deck", Opponent::Handcrafted, PlayerId::Two, 3)
            .expect("game starts");
        let seat = game.decision_seat().expect("mulligan decision");
        let observation: Value =
            serde_json::from_str(&game.observe_json(seat)).expect("valid JSON");
        assert_eq!(observation["seat"], "p1");
        assert_eq!(observation["pregame"], true);
        let actions = observation["legalActions"].as_array().expect("array");
        for (index, action) in actions.iter().enumerate() {
            assert_eq!(action["index"], index, "indices match positions");
            assert!(action["type"].is_string(), "every action is tagged");
        }
        // The opponent's hand is a count, never a list of cards.
        assert!(observation["opponentHandSize"].is_u64());
        assert_eq!(observation["hand"].as_array().expect("hand").len(), 7);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn protocol_reincarnates_public_object_identity_across_cast_zones() {
        let mut game = BotGame::new("Goblins", "Goblins", Opponent::External, PlayerId::Two, 0)
            .expect("game starts");

        let (casting_seat, hand_id, definition_id) = (0..600)
            .find_map(|_| {
                let seat = game.decision_seat().expect("game has not ended");
                let observation: Value =
                    serde_json::from_str(&game.observe_json(seat)).expect("valid observation JSON");
                assert_no_physical_lineage_keys(&observation);
                let actions = observation["legalActions"].as_array().expect("actions");

                let permanent_cast = actions.iter().find_map(|action| {
                    if action["type"] != "CastSpell" {
                        return None;
                    }
                    let hand_raw = action["card"].as_u64()?;
                    let hand_card = observation["hand"]
                        .as_array()
                        .expect("hand")
                        .iter()
                        .find(|card| card["objectId"].as_u64() == Some(hand_raw))?;
                    let definition_raw = hand_card["definition"].as_u64()?;
                    let definition = crate::CardDefinitionId(u16::try_from(definition_raw).ok()?);
                    if !game
                        .catalog
                        .get(definition)
                        .is_some_and(|card| card.rules.kind.is_permanent())
                    {
                        return None;
                    }
                    Some((
                        usize::try_from(action["index"].as_u64()?).ok()?,
                        GameObjectId(u32::try_from(hand_raw).ok()?),
                        definition,
                    ))
                });
                if let Some((index, hand_id, definition)) = permanent_cast {
                    let hand_card = observation["hand"]
                        .as_array()
                        .expect("hand")
                        .iter()
                        .find(|card| card["objectId"].as_u64() == Some(u64::from(hand_id.0)))
                        .expect("cast card was public in hand");
                    assert_eq!(hand_card["instance"], hand_id.0);
                    game.act(index).expect("cast action is legal");
                    return Some((seat, hand_id, definition));
                }

                let find_action = |kind: &str| {
                    actions
                        .iter()
                        .find(|action| action["type"] == kind)
                        .and_then(|action| action["index"].as_u64())
                        .and_then(|index| usize::try_from(index).ok())
                };
                let main_phase = matches!(
                    observation["step"].as_str(),
                    Some("PrecombatMain" | "PostcombatMain")
                );
                let mut selected = find_action("KeepHand");
                if selected.is_none() && main_phase {
                    selected = find_action("PlayLand");
                }
                if selected.is_none() && main_phase {
                    selected = actions
                        .iter()
                        .find(|action| {
                            action["type"] == "ActivateManaAbility" && action["color"] == "red"
                        })
                        .or_else(|| {
                            actions
                                .iter()
                                .find(|action| action["type"] == "ActivateManaAbility")
                        })
                        .and_then(|action| action["index"].as_u64())
                        .and_then(|index| usize::try_from(index).ok());
                }
                for kind in [
                    "BottomCards",
                    "DiscardCards",
                    "ChooseDecision",
                    "ChooseUntap",
                    "FinishDeclaringAttackers",
                    "FinishDeclaringBlockers",
                    "AssignCombatDamage",
                    "PassPriority",
                ] {
                    if selected.is_none() {
                        selected = find_action(kind);
                    }
                }
                game.act(selected.expect("the protocol always offers progress"))
                    .expect("selected protocol action is legal");
                None
            })
            .expect("the seeded game reaches a castable permanent");

        let stack_observation: Value = serde_json::from_str(&game.observe_json(casting_seat))
            .expect("valid stack observation");
        assert_no_physical_lineage_keys(&stack_observation);
        assert!(
            stack_observation["hand"]
                .as_array()
                .expect("hand")
                .iter()
                .all(|card| card["objectId"].as_u64() != Some(u64::from(hand_id.0))),
            "the hand object ceased to exist when the card changed zones"
        );
        let spell = stack_observation["stack"]
            .as_array()
            .expect("stack")
            .iter()
            .find(|object| object["definition"].as_u64() == Some(u64::from(definition_id.0)))
            .expect("cast spell is public on the stack");
        assert_eq!(spell["kind"], "Spell");
        assert!(spell["sourceObjectId"].is_null());
        assert!(spell["source"].is_null());
        assert!(spell["signature"].is_object());
        let spell_id = GameObjectId(
            u32::try_from(spell["objectId"].as_u64().expect("stack object ID")).expect("ID fits"),
        );
        assert_ne!(spell_id, hand_id);

        for _ in 0..2 {
            let seat = game.decision_seat().expect("priority decision");
            let observation: Value =
                serde_json::from_str(&game.observe_json(seat)).expect("valid priority observation");
            assert_no_physical_lineage_keys(&observation);
            let pass = observation["legalActions"]
                .as_array()
                .expect("actions")
                .iter()
                .find(|action| action["type"] == "PassPriority")
                .and_then(|action| action["index"].as_u64())
                .and_then(|index| usize::try_from(index).ok())
                .expect("priority can be passed");
            game.act(pass).expect("priority pass is legal");
        }

        let battlefield_observation: Value = serde_json::from_str(&game.observe_json(casting_seat))
            .expect("valid battlefield observation");
        assert_no_physical_lineage_keys(&battlefield_observation);
        let permanent = battlefield_observation["battlefield"]
            .as_array()
            .expect("battlefield")
            .iter()
            .find(|object| object["definition"].as_u64() == Some(u64::from(definition_id.0)))
            .expect("resolved permanent is public on the battlefield");
        let permanent_id = GameObjectId(
            u32::try_from(permanent["objectId"].as_u64().expect("permanent object ID"))
                .expect("ID fits"),
        );
        assert_ne!(permanent_id, hand_id);
        assert_ne!(permanent_id, spell_id);
        assert_eq!(permanent["instance"], permanent_id.0);
        assert!(
            battlefield_observation["stack"]
                .as_array()
                .expect("stack")
                .iter()
                .all(|object| object["objectId"].as_u64() != Some(u64::from(spell_id.0))),
            "the stack object ceased to exist when the permanent was created"
        );
    }

    #[test]
    #[ignore = "slow simulation sweep"]
    fn bots_are_never_offered_the_chance_to_resign() {
        // Conceding is legal in every state and strictly dominated, so it is
        // not in the bot's list at all. That makes picking blindly — index
        // zero, or uniform random — a weak bot rather than an instant loss,
        // which is what a random baseline has to be worth measuring against.
        let decks = deck_names();
        let mut observations = 0_u32;
        let mut rng = 12_345_u64;
        for index in 0..12 {
            let mut game = BotGame::new(
                decks[index % decks.len()],
                decks[(index * 7 + 3) % decks.len()],
                Opponent::External,
                PlayerId::Two,
                index as u64 * 101,
            )
            .expect("game starts");
            for _ in 0..1_500 {
                if game.result().is_some() {
                    break;
                }
                let seat = game.decision_seat().expect("still running");
                let observation: Value =
                    serde_json::from_str(&game.observe_json(seat)).expect("valid JSON");
                let actions = observation["legalActions"].as_array().expect("array");
                assert!(
                    !actions.is_empty(),
                    "removing Concede never empties the list",
                );
                for action in actions {
                    assert_ne!(action["type"], "Concede", "no way to resign by index");
                }
                // Uniform random over the whole list, the baseline a new bot
                // author measures against.
                rng = rng.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
                let pick = usize::try_from(rng >> 33).unwrap_or(0) % actions.len();
                game.act(pick).expect("legal index");
                observations += 1;
            }
        }
        assert!(
            observations > 2_000,
            "played enough to be meaningful, saw {observations}",
        );
    }

    #[test]
    #[ignore = "slow simulation sweep"]
    fn decisions_reach_bots_as_concrete_indexed_actions() {
        // The engine's decision template has empty options; the protocol
        // must never show that to a bot. Play games until decisions appear
        // and check every ChooseDecision carries a concrete selection.
        // A bot that plays lands and casts spells, so card effects (Chain
        // Lightning copies, discard effects) actually raise decisions.
        let cast_bot = |observation: &Value| {
            let actions = observation["legalActions"].as_array().expect("array");
            for preferred in ["PlayLand", "CastSpell"] {
                if let Some(action) = actions.iter().find(|action| action["type"] == preferred) {
                    return usize::try_from(action["index"].as_u64().expect("index"))
                        .expect("index fits");
                }
            }
            pass_bot(observation)
        };
        let mut saw_decision = false;
        for seed in 0..20 {
            let mut game = BotGame::new(
                "Sligh",
                "The Deck",
                Opponent::Handcrafted,
                PlayerId::Two,
                seed,
            )
            .expect("game starts");
            for _ in 0..2_000 {
                if game.result().is_some() {
                    break;
                }
                let seat = game.decision_seat().expect("still running");
                let observation: Value =
                    serde_json::from_str(&game.observe_json(seat)).expect("valid JSON");
                for action in observation["legalActions"].as_array().expect("array") {
                    if action["type"] == "ChooseDecision" {
                        saw_decision = true;
                        assert!(
                            !observation["decision"].is_null(),
                            "a decision action implies a decision object",
                        );
                        let minimum = observation["decision"]["minimum"].as_u64().expect("min");
                        let chosen = action["options"].as_array().expect("options").len();
                        assert!(
                            u64::try_from(chosen).expect("fits") >= minimum,
                            "every offered selection is submittable as-is",
                        );
                    }
                }
                game.act(cast_bot(&observation)).expect("legal index");
            }
            if saw_decision {
                break;
            }
        }
        assert!(saw_decision, "the seeded games reached a decision");
    }

    #[test]
    fn the_catalog_lists_every_card_with_names_and_costs() {
        let catalog = poc::catalog().expect("catalog builds");
        let value = catalog_json(&catalog);
        let cards = value["cards"].as_array().expect("cards array");
        assert!(cards.len() > 100, "the pool is substantial");
        assert!(cards.iter().all(|card| card["name"].is_string()));
        assert!(
            cards
                .iter()
                .any(|card| card["name"] == "Lightning Bolt" && card["manaCost"]["red"] == 1)
        );
    }

    #[test]
    fn deck_names_all_resolve() {
        for name in deck_names() {
            assert!(deck_by_name(name).is_some(), "{name} resolves");
        }
        assert!(deck_by_name("Not A Deck").is_none());
    }

    #[test]
    fn both_format_deck_registries_resolve_without_cross_format_leakage() {
        assert_eq!(deck_names(), deck_names_for_format(Format::OldSchool9394));
        assert_eq!(deck_names_for_format(Format::OldSchool9394).len(), 15);
        assert_eq!(deck_names_for_format(Format::IsdRtrStandard).len(), 8);

        for format in [Format::OldSchool9394, Format::IsdRtrStandard] {
            for name in deck_names_for_format(format) {
                assert!(
                    deck_by_name_for_format(format, name).is_some(),
                    "{name} resolves in {format}"
                );
            }
        }

        assert!(deck_by_name_for_format(Format::OldSchool9394, "Briksza Naya Midrange").is_none());
        assert!(deck_by_name_for_format(Format::IsdRtrStandard, "Sligh").is_none());
        assert!(
            deck_by_name_for_format(Format::IsdRtrStandard, "naya_midrange_rudy_briksza").is_some()
        );
        assert_eq!(
            parse_format_slug("old_school_93_94"),
            Ok(Format::OldSchool9394)
        );
        assert_eq!(
            parse_format_slug("isd-rtr-standard"),
            Ok(Format::IsdRtrStandard)
        );
        assert!(parse_format_slug("vintage").is_err());
    }

    #[test]
    fn bot_game_stores_and_emits_its_format_and_rejects_wrong_decks() {
        let old_school = BotGame::new("Sligh", "Goblins", Opponent::External, PlayerId::Two, 18)
            .expect("compatibility constructor starts Old School");
        assert_eq!(old_school.format(), Format::OldSchool9394);

        let standard = BotGame::new_with_format(
            Format::IsdRtrStandard,
            "Briksza Naya Midrange",
            "Greer G/R Aggro",
            Opponent::External,
            PlayerId::Two,
            19,
        )
        .expect("Standard game starts");
        assert_eq!(standard.format(), Format::IsdRtrStandard);
        let seat = standard.decision_seat().expect("opening-hand decision");
        let observation: Value =
            serde_json::from_str(&standard.observe_json(seat)).expect("valid observation JSON");
        assert_eq!(observation["protocolVersion"], PROTOCOL_VERSION);
        assert_eq!(observation["format"], "isd-rtr-standard");
        assert!(
            observation["legalActions"]
                .as_array()
                .expect("actions")
                .iter()
                .all(|action| action["type"] != "Concede")
        );

        let configured = BotGame::from_config_json(
            r#"{"format":"isd-rtr-standard","p1Deck":"Lorren U/W Flash","p2Deck":"Arch U/W Flash","opponent":"external","seed":4}"#,
        )
        .expect("format slug selects Standard");
        assert_eq!(configured.format(), Format::IsdRtrStandard);
        assert!(
            BotGame::from_config_json(r#"{"format":2,"p1Deck":"Sligh","p2Deck":"Goblins"}"#)
                .err()
                .expect("non-string format is rejected")
                .contains("format must be a string")
        );

        assert!(
            BotGame::new_with_format(
                Format::OldSchool9394,
                "Briksza Naya Midrange",
                "Sligh",
                Opponent::External,
                PlayerId::Two,
                0,
            )
            .err()
            .expect("cross-format deck is rejected")
            .contains("unknown deck for old-school-93-94")
        );
        assert!(
            BotGame::new_with_format(
                Format::IsdRtrStandard,
                "Sligh",
                "Briksza Naya Midrange",
                Opponent::External,
                PlayerId::Two,
                0,
            )
            .err()
            .expect("cross-format deck is rejected")
            .contains("unknown deck for isd-rtr-standard")
        );
    }

    fn structured_choices() -> CastChoices {
        CastChoices::new(crate::PlayOptionId(2))
            .with_modes(vec![crate::ModeId(3), crate::ModeId(1)])
            .with_costs(crate::CostConfiguration::new(
                Some(crate::AlternativeCostId(4)),
                vec![crate::AdditionalCostId(5)],
            ))
            .with_x(6)
            .with_targets(vec![
                crate::TargetSelection::single(
                    crate::TargetSlotId(7),
                    Target::Permanent(GameObjectId(20)),
                ),
                crate::TargetSelection::single(
                    crate::TargetSlotId(8),
                    Target::Spell(GameObjectId(21)),
                ),
            ])
    }

    #[test]
    fn action_json_locks_play_option_modes_costs_x_and_target_slots() {
        let land = action_json(&Action::PlayLand {
            card: GameObjectId(10),
            option: crate::PlayOptionId(9),
        });
        assert_eq!(land["card"], 10);
        assert_eq!(land["playOptionId"], 9);

        let spell = action_json(&Action::CastSpell {
            card: GameObjectId(11),
            choices: structured_choices(),
            sacrifices: vec![GameObjectId(12)],
        });
        assert_eq!(spell["card"], 11);
        assert_eq!(spell["playOptionId"], 2);
        assert_eq!(spell["choices"]["modeIds"], json!([3, 1]));
        assert_eq!(spell["choices"]["alternativeCostId"], 4);
        assert_eq!(spell["choices"]["additionalCostIds"], json!([5]));
        assert_eq!(spell["choices"]["x"], 6);
        assert_eq!(spell["choices"]["targetSelections"][0]["slotId"], 7);
        assert_eq!(
            spell["choices"]["targetSelections"][0]["targets"][0]["objectId"],
            20
        );
        assert_eq!(
            spell["choices"]["targetSelections"][1]["targets"][0]["objectId"],
            21
        );
        assert_eq!(spell["sacrifices"], json!([12]));
    }

    #[test]
    fn battlefield_name_uses_the_presented_card_part() {
        let catalog = poc::catalog().expect("catalog builds");
        let observation = PlayerObservation {
            viewer: PlayerId::One,
            turn: 1,
            active_turn: 1,
            active_player: PlayerId::One,
            priority: PlayerId::One,
            step: crate::game::Step::PrecombatMain,
            life_totals: [20, 20],
            mana_pools: [crate::ManaPool::default(); 2],
            hand: Vec::new(),
            opponent_hand_size: 0,
            last_seen_hand: None,
            library_sizes: [0, 0],
            graveyards: [Vec::new(), Vec::new()],
            exiles: [Vec::new(), Vec::new()],
            battlefield: vec![crate::game::PermanentObservation {
                id: GameObjectId(30),
                definition: crate::card::cards::HUNTMASTER_OF_THE_FELLS,
                presented: crate::CardPartId(1),
                controller: PlayerId::One,
                tapped: false,
                power: Some(4),
                toughness: Some(4),
                damage: 0,
                attacking: false,
                blocking: None,
                flying: false,
                can_attack: true,
                entered_this_turn: false,
            }],
            stack: Vec::new(),
            decision: None,
            result: None,
            legal_actions: Vec::new(),
        };

        let value =
            observation_json_for_format(&catalog, Format::IsdRtrStandard, &observation, false, &[]);
        assert_eq!(value["battlefield"][0]["objectId"], 30);
        assert_eq!(value["battlefield"][0]["presentedPartId"], 1);
        assert_eq!(value["battlefield"][0]["name"], "Ravager of the Fells");
        assert_eq!(
            card_part_name(
                &catalog,
                crate::card::cards::HUNTMASTER_OF_THE_FELLS,
                crate::CardPartId(99),
            ),
            "Huntmaster of the Fells"
        );
    }

    #[test]
    fn stack_json_uses_game_object_identity_and_preserves_cast_signature() {
        let catalog = poc::catalog().expect("catalog builds");
        let signature = CastSignature::from_validated_choices(
            SpellForm::Combined(vec![crate::CardPartId(0), crate::CardPartId(1)]),
            structured_choices(),
        );
        let object = StackObservation {
            id: GameObjectId(40),
            kind: StackObjectKind::Spell,
            source: None,
            ability: None,
            ability_text: None,
            definition: crate::card::cards::TURN_BURN,
            controller: PlayerId::One,
            targets: signature.iter_targets().copied().collect(),
            chosen_permanents: Vec::new(),
            x: signature.x(),
            signature: Some(signature),
        };
        let value = stack_object_json(&catalog, &object);

        assert_eq!(value["objectId"], 40);
        assert_eq!(value["stackId"], 40);
        assert!(value["sourceObjectId"].is_null());
        assert!(value["source"].is_null());
        assert_eq!(value["signature"]["playOptionId"], 2);
        assert_eq!(value["signature"]["form"]["kind"], "combined");
        assert_eq!(value["signature"]["form"]["partIds"], json!([0, 1]));
        assert_eq!(value["signature"]["modeIds"], json!([3, 1]));
        assert_eq!(value["signature"]["alternativeCostId"], 4);
        assert_eq!(value["signature"]["additionalCostIds"], json!([5]));
        assert_eq!(value["signature"]["x"], 6);
        assert_eq!(value["signature"]["targetSelections"][1]["slotId"], 8);

        let burn_signature = CastSignature::from_validated_choices(
            SpellForm::Part(crate::CardPartId(1)),
            CastChoices::new(crate::PlayOptionId(1)),
        );
        let burn = StackObservation {
            id: GameObjectId(41),
            kind: StackObjectKind::Spell,
            source: None,
            ability: None,
            ability_text: None,
            definition: crate::card::cards::TURN_BURN,
            controller: PlayerId::One,
            targets: Vec::new(),
            chosen_permanents: Vec::new(),
            x: 0,
            signature: Some(burn_signature),
        };
        assert_eq!(stack_object_json(&catalog, &burn)["name"], "Burn");

        let ability = StackObservation {
            id: GameObjectId(42),
            kind: StackObjectKind::ActivatedAbility,
            source: Some(GameObjectId(39)),
            ability: None,
            ability_text: None,
            definition: crate::card::cards::MISHRA_S_FACTORY,
            controller: PlayerId::One,
            targets: Vec::new(),
            chosen_permanents: Vec::new(),
            x: 0,
            signature: None,
        };
        let ability_value = stack_object_json(&catalog, &ability);
        assert_eq!(ability_value["objectId"], 42);
        assert_eq!(ability_value["stackId"], 42);
        assert_eq!(ability_value["sourceObjectId"], 39);
        assert_eq!(ability_value["source"], 39);
        assert_ne!(
            ability_value["objectId"], ability_value["sourceObjectId"],
            "the ability and its source are distinct game objects"
        );
        assert!(ability_value["signature"].is_null());

        let trigger = StackObservation {
            id: GameObjectId(43),
            kind: StackObjectKind::TriggeredAbility,
            source: Some(GameObjectId(38)),
            ability: Some(crate::AbilityId::PRIMARY),
            ability_text: Some(
                "Whenever a land enters, Ankh of Mishra deals 2 damage to its controller.".into(),
            ),
            definition: crate::card::cards::ANKH_OF_MISHRA,
            controller: PlayerId::Two,
            targets: Vec::new(),
            chosen_permanents: Vec::new(),
            x: 0,
            signature: None,
        };
        let trigger_value = stack_object_json(&catalog, &trigger);
        assert_eq!(trigger_value["kind"], "TriggeredAbility");
        assert_eq!(trigger_value["sourceObjectId"], 38);
        assert_eq!(trigger_value["abilityId"], 0);
        assert_eq!(
            trigger_value["abilityText"],
            "Whenever a land enters, Ankh of Mishra deals 2 damage to its controller."
        );
        assert_eq!(trigger_value["controller"], "p2");
    }

    #[test]
    fn decision_json_exposes_trigger_procedure_and_resolution_order_semantics() {
        let catalog = poc::catalog().expect("catalog builds");
        let decision = DecisionObservation {
            id: 7,
            player: PlayerId::One,
            kind: DecisionKind::TriggerOrder,
            order_semantics: Some(DecisionOrderSemantics::Resolution),
            prompt: "Choose the order your triggers resolve".into(),
            visibility: crate::game::DecisionVisibility::Public,
            preference: crate::game::DecisionPreference::Neutral,
            minimum: 2,
            maximum: 2,
            cancellable: false,
            options: vec![
                crate::game::DecisionOption {
                    id: 11,
                    label: "First Ankh trigger".into(),
                    card: Some((GameObjectId(81), crate::card::cards::ANKH_OF_MISHRA)),
                    ability_text: Some("First frozen trigger text".into()),
                    zone: crate::game::DecisionZone::Battlefield,
                },
                crate::game::DecisionOption {
                    id: 12,
                    label: "Second Ankh trigger".into(),
                    card: Some((GameObjectId(82), crate::card::cards::ANKH_OF_MISHRA)),
                    ability_text: Some("Second frozen trigger text".into()),
                    zone: crate::game::DecisionZone::Battlefield,
                },
            ],
        };

        let value = decision_json(&catalog, &decision);
        assert_eq!(value["kind"], "TriggerOrder");
        assert_eq!(value["orderSemantics"], "resolution");
        assert_eq!(value["options"][0]["triggerId"], 11);
        assert_eq!(value["options"][0]["card"]["objectId"], 81);
        assert_eq!(
            value["options"][0]["abilityText"],
            "First frozen trigger text"
        );

        let ordinary = DecisionObservation {
            kind: DecisionKind::Choice,
            order_semantics: None,
            ..decision
        };
        assert!(
            decision_json(&catalog, &ordinary)
                .get("orderSemantics")
                .is_none()
        );
    }

    #[test]
    fn catalog_json_is_structured_and_legality_is_format_specific() {
        let catalog = poc::catalog().expect("catalog builds");
        let old_school = catalog_json(&catalog);
        let standard = catalog_json_for_format(&catalog, Format::IsdRtrStandard);
        assert_eq!(old_school["format"], "old-school-93-94");
        assert_eq!(standard["format"], "isd-rtr-standard");

        let cards = standard["cards"].as_array().expect("cards array");
        assert!(cards.windows(2).all(|pair| {
            pair[0]["definition"].as_u64().expect("id")
                < pair[1]["definition"].as_u64().expect("id")
        }));
        let find = |name: &str| {
            cards
                .iter()
                .find(|card| card["name"] == name)
                .unwrap_or_else(|| panic!("{name} is cataloged"))
        };
        let turn_burn = find("Turn // Burn");
        assert_eq!(turn_burn["legal"], true);
        assert_eq!(turn_burn["structure"]["kind"], "split");
        assert_eq!(turn_burn["parts"].as_array().expect("parts").len(), 2);
        assert_eq!(
            turn_burn["playOptions"]
                .as_array()
                .expect("play options")
                .len(),
            3
        );
        assert!(
            !turn_burn["printings"]
                .as_array()
                .expect("printings")
                .is_empty()
        );

        let charm = find("Izzet Charm");
        assert_eq!(
            charm["playOptions"][0]["modes"]["choices"]
                .as_array()
                .expect("modes")
                .len(),
            3
        );
        assert_eq!(find("Lightning Bolt")["legal"], false);
        let old_bolt = old_school["cards"]
            .as_array()
            .expect("cards")
            .iter()
            .find(|card| card["name"] == "Lightning Bolt")
            .expect("bolt");
        assert_eq!(old_bolt["legal"], true);
    }
}
