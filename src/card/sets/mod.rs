//! Built-in card records, grouped by release year and set.
//!
//! Each canonical card is defined in one set module. Records default to a
//! complete implementation and explicitly carry a reason when they are partial
//! or metadata-only. Reprints and alternate-art variants point back to that
//! canonical record from their own set module.

mod y1993;
mod y1994;
mod y2011;
mod y2012;
mod y2013;

use super::record::{CardRecord, PrintingRecord};
use crate::card::{CardBehavior, CardDefinition, CardPrinting, CardRules, CardSet};

static UNSUPPORTED_RULES: CardRules = CardRules::unsupported();

struct SetModule {
    set: CardSet,
    cards: &'static [&'static CardRecord],
    additional_printings: &'static [PrintingRecord],
}

impl SetModule {
    const fn new(
        set: CardSet,
        cards: &'static [&'static CardRecord],
        additional_printings: &'static [PrintingRecord],
    ) -> Self {
        Self {
            set,
            cards,
            additional_printings,
        }
    }
}

/// Every supported set has one source module. `cards` contains definitions
/// introduced by that module; `additional_printings` contains reprints and
/// further variants of definitions introduced elsewhere.
const SET_MODULES: &[SetModule] = &[
    SetModule::new(
        CardSet::Alpha,
        y1993::alpha::CARDS,
        y1993::alpha::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Beta,
        y1993::beta::CARDS,
        y1993::beta::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Unlimited,
        y1993::unlimited::CARDS,
        y1993::unlimited::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::CollectorsEdition,
        y1993::collectors_edition::CARDS,
        y1993::collectors_edition::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::InternationalCollectorsEdition,
        y1993::international_collectors_edition::CARDS,
        y1993::international_collectors_edition::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ArabianNights,
        y1993::arabian_nights::CARDS,
        y1993::arabian_nights::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Antiquities,
        y1994::antiquities::CARDS,
        y1994::antiquities::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Revised,
        y1994::revised::CARDS,
        y1994::revised::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Legends,
        y1994::legends::CARDS,
        y1994::legends::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::TheDark,
        y1994::the_dark::CARDS,
        y1994::the_dark::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::FallenEmpires,
        y1994::fallen_empires::CARDS,
        y1994::fallen_empires::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Promo1994,
        y1994::promo_1994::CARDS,
        y1994::promo_1994::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Innistrad,
        y2011::innistrad::CARDS,
        y2011::innistrad::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::DarkAscension,
        y2012::dark_ascension::CARDS,
        y2012::dark_ascension::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::AvacynRestored,
        y2012::avacyn_restored::CARDS,
        y2012::avacyn_restored::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Magic2013,
        y2012::magic_2013::CARDS,
        y2012::magic_2013::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ReturnToRavnica,
        y2012::return_to_ravnica::CARDS,
        y2012::return_to_ravnica::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Gatecrash,
        y2013::gatecrash::CARDS,
        y2013::gatecrash::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::DragonsMaze,
        y2013::dragons_maze::CARDS,
        y2013::dragons_maze::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Magic2014,
        y2013::magic_2014::CARDS,
        y2013::magic_2014::ADDITIONAL_PRINTINGS,
    ),
];

pub(super) fn definitions() -> Vec<CardDefinition> {
    let mut definitions = Vec::with_capacity(244);
    for module in SET_MODULES {
        definitions.extend(module.cards.iter().map(|record| record.definition()));
    }
    definitions
}

pub(super) fn additional_printings() -> Vec<CardPrinting> {
    SET_MODULES
        .iter()
        .flat_map(|module| {
            module
                .additional_printings
                .iter()
                .map(|record| record.printing(module.set))
        })
        .collect()
}

// This is deliberately only an index: special hooks and the legacy
// CardDefinition::new compatibility keys resolve to card-local rules.
#[allow(clippy::too_many_lines)]
pub(super) const fn rules(behavior: CardBehavior) -> &'static CardRules {
    match behavior {
        CardBehavior::Atog => &y1994::antiquities::ATOG.rules,
        CardBehavior::BlackVise => &y1993::alpha::BLACK_VISE.rules,
        CardBehavior::BloodMoon => &y1994::the_dark::BLOOD_MOON.rules,
        CardBehavior::ChainLightning => &y1994::legends::CHAIN_LIGHTNING.rules,
        CardBehavior::Detonate => &y1994::antiquities::DETONATE.rules,
        CardBehavior::Fireball => &y1993::alpha::FIREBALL.rules,
        CardBehavior::Fork => &y1993::alpha::FORK.rules,
        CardBehavior::GlassesOfUrza => &y1993::alpha::GLASSES_OF_URZA.rules,
        CardBehavior::LightningBolt => &y1993::alpha::LIGHTNING_BOLT.rules,
        CardBehavior::RedElementalBlast => &y1993::alpha::RED_ELEMENTAL_BLAST.rules,
        CardBehavior::Smoke => &y1993::alpha::SMOKE.rules,
        CardBehavior::StoneGiant => &y1993::alpha::STONE_GIANT.rules,
        CardBehavior::WinterOrb => &y1993::alpha::WINTER_ORB.rules,
        CardBehavior::ChaosOrb => &y1993::alpha::CHAOS_ORB.rules,
        CardBehavior::DragonWhelp => &y1993::alpha::DRAGON_WHELP.rules,
        CardBehavior::GoblinGrenade => &y1994::fallen_empires::GOBLIN_GRENADE.rules,
        CardBehavior::IronclawOrcs => &y1993::alpha::IRONCLAW_ORCS.rules,
        CardBehavior::MishrasFactory => &y1994::antiquities::MISHRA_S_FACTORY.rules,
        CardBehavior::OrcishMechanics => &y1994::antiquities::ORCISH_MECHANICS.rules,
        CardBehavior::WheelOfFortune => &y1993::alpha::WHEEL_OF_FORTUNE.rules,
        CardBehavior::Juggernaut => &y1993::alpha::JUGGERNAUT.rules,
        CardBehavior::ManaVault => &y1993::alpha::MANA_VAULT.rules,
        CardBehavior::Triskelion => &y1994::antiquities::TRISKELION.rules,
        CardBehavior::FellwarStone => &y1994::the_dark::FELLWAR_STONE.rules,
        CardBehavior::SwordsToPlowshares => &y1993::alpha::SWORDS_TO_PLOWSHARES.rules,
        CardBehavior::TimeWalk => &y1993::alpha::TIME_WALK.rules,
        CardBehavior::Balance => &y1993::alpha::BALANCE.rules,
        CardBehavior::BlueElementalBlast => &y1993::alpha::BLUE_ELEMENTAL_BLAST.rules,
        CardBehavior::Channel => &y1993::alpha::CHANNEL.rules,
        CardBehavior::Crusade => &y1993::alpha::CRUSADE.rules,
        CardBehavior::DemonicTutor => &y1993::alpha::DEMONIC_TUTOR.rules,
        CardBehavior::DivineOffering => &y1994::legends::DIVINE_OFFERING.rules,
        CardBehavior::DrainLife => &y1993::alpha::DRAIN_LIFE.rules,
        CardBehavior::Earthquake => &y1993::alpha::EARTHQUAKE.rules,
        CardBehavior::ErhnamDjinn => &y1993::arabian_nights::ERHNAM_DJINN.rules,
        CardBehavior::HymnToTourach => &y1994::fallen_empires::HYMN_TO_TOURACH.rules,
        CardBehavior::HypnoticSpecter => &y1993::alpha::HYPNOTIC_SPECTER.rules,
        CardBehavior::IcatianJavelineers => &y1994::fallen_empires::ICATIAN_JAVELINEERS.rules,
        CardBehavior::LibraryOfAlexandria => &y1993::arabian_nights::LIBRARY_OF_ALEXANDRIA.rules,
        CardBehavior::ManaDrain => &y1994::legends::MANA_DRAIN.rules,
        CardBehavior::MazeOfIth => &y1994::the_dark::MAZE_OF_ITH.rules,
        CardBehavior::MindTwist => &y1993::alpha::MIND_TWIST.rules,
        CardBehavior::NevinyrralsDisk => &y1993::alpha::NEVINYRRALS_DISK.rules,
        CardBehavior::Recall => &y1994::legends::RECALL.rules,
        CardBehavior::Regrowth => &y1993::alpha::REGROWTH.rules,
        CardBehavior::SylvanLibrary => &y1994::legends::SYLVAN_LIBRARY.rules,
        CardBehavior::Terror => &y1993::alpha::TERROR.rules,
        CardBehavior::TimeVault => &y1993::alpha::TIME_VAULT.rules,
        CardBehavior::Timetwister => &y1993::alpha::TIMETWISTER.rules,
        CardBehavior::WhirlingDervish => &y1994::legends::WHIRLING_DERVISH.rules,
        CardBehavior::ArgothianPixies => &y1994::antiquities::ARGOTHIAN_PIXIES.rules,
        CardBehavior::Berserk => &y1993::alpha::BERSERK.rules,
        CardBehavior::CityInABottle => &y1993::arabian_nights::CITY_IN_A_BOTTLE.rules,
        CardBehavior::CopyArtifact => &y1993::alpha::COPY_ARTIFACT.rules,
        CardBehavior::DustToDust => &y1994::the_dark::DUST_TO_DUST.rules,
        CardBehavior::GiantGrowth => &y1993::alpha::GIANT_GROWTH.rules,
        CardBehavior::HurkylsRecall => &y1994::antiquities::HURKYLS_RECALL.rules,
        CardBehavior::IcyManipulator => &y1993::alpha::ICY_MANIPULATOR.rules,
        CardBehavior::KirdApe => &y1993::arabian_nights::KIRD_APE.rules,
        CardBehavior::Moat => &y1994::legends::MOAT.rules,
        CardBehavior::Pendelhaven => &y1994::legends::PENDELHAVEN.rules,
        CardBehavior::SedgeTroll => &y1993::alpha::SEDGE_TROLL.rules,
        CardBehavior::Tetravus => &y1994::antiquities::TETRAVUS.rules,
        CardBehavior::TheAbyss => &y1994::legends::THE_ABYSS.rules,
        CardBehavior::Mountain => &y1993::alpha::MOUNTAIN.rules,
        CardBehavior::Plains => &y1993::alpha::PLAINS.rules,
        CardBehavior::Unsupported => &UNSUPPORTED_RULES,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{CardRecord, SET_MODULES, y1993, y1994, y2011, y2012, y2013};
    use crate::card::{
        AbilityCostDef, AbilityDef, AbilityImplementationDef, AppliedEffectDef, BasicLandType,
        CardPrinting, CardPrintingId, CardStructure, CardSupertype, DeclarativeAbilityDef,
        DoubleFacedKind, EffectDef, EffectDurationDef, EffectRecipientDef, EvergreenAbility,
        ImplementationStatus, ManaKindDef, ManaRestrictionDef, ManaSelectionDef,
        ManaSpendEffectDef, ObjectPredicateDef, PlayActionKind, PlayRestriction, SpellForm,
        TargetPredicate, TriggerEventDef, ZoneKind, cards,
    };
    use crate::{AbilityId, CardDefinitionId, CardPartId, CardSet, Format, ModeId, PlayOptionId};

    fn standard_records() -> Vec<&'static CardRecord> {
        let mut records = SET_MODULES
            .iter()
            .filter(|module| Format::IsdRtrStandard.allows_set(module.set))
            .flat_map(|module| module.cards.iter().copied())
            .collect::<Vec<_>>();
        records.sort_unstable_by_key(|record| record.id);
        records
    }

    fn is_uuid(value: &str) -> bool {
        value.len() == 36
            && value.bytes().enumerate().all(|(index, byte)| match index {
                8 | 13 | 18 | 23 => byte == b'-',
                _ => byte.is_ascii_hexdigit(),
            })
    }

    fn printings_for_set(set: CardSet) -> Vec<CardPrinting> {
        let module = SET_MODULES.iter().find(|module| module.set == set).unwrap();
        module
            .cards
            .iter()
            .map(|record| CardPrinting::new(record.id, set))
            .chain(
                module
                    .additional_printings
                    .iter()
                    .map(|record| record.printing(set)),
            )
            .collect()
    }

    fn shared_object_predicate(predicate: ObjectPredicateDef) -> bool {
        match predicate {
            ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => {
                predicates.iter().copied().all(shared_object_predicate)
            }
            ObjectPredicateDef::Not(predicate) => shared_object_predicate(*predicate),
            ObjectPredicateDef::Special(_) => false,
            ObjectPredicateDef::Any
            | ObjectPredicateDef::Source
            | ObjectPredicateDef::Land
            | ObjectPredicateDef::Creature
            | ObjectPredicateDef::Artifact
            | ObjectPredicateDef::Spell
            | ObjectPredicateDef::NoncreatureSpell
            | ObjectPredicateDef::CardKind(_)
            | ObjectPredicateDef::Color(_)
            | ObjectPredicateDef::Subtype(_) => true,
        }
    }

    fn shared_effect_recipient(recipient: EffectRecipientDef) -> bool {
        match recipient {
            EffectRecipientDef::MatchingObjects { object, zones, .. } => {
                !zones.is_empty()
                    && zones
                        .iter()
                        .all(|zone| matches!(zone, ZoneKind::Battlefield | ZoneKind::Stack))
                    && shared_object_predicate(object)
            }
            EffectRecipientDef::Source
            | EffectRecipientDef::Controller
            | EffectRecipientDef::Opponent
            | EffectRecipientDef::Target(_)
            | EffectRecipientDef::TriggeringObject
            | EffectRecipientDef::ControllerOfTriggeringObject
            | EffectRecipientDef::EventPlayer => true,
        }
    }

    fn shared_evergreen(keyword: EvergreenAbility) -> bool {
        matches!(
            keyword,
            EvergreenAbility::Flying
                | EvergreenAbility::Trample
                | EvergreenAbility::Haste
                | EvergreenAbility::Vigilance
                | EvergreenAbility::Deathtouch
                | EvergreenAbility::Lifelink
                | EvergreenAbility::Reach
                | EvergreenAbility::Flash
                | EvergreenAbility::Hexproof
                | EvergreenAbility::Intimidate
                | EvergreenAbility::Mountainwalk
                | EvergreenAbility::ProtectionFrom(_)
        )
    }

    fn shared_mana_effect(effect: EffectDef, choices_are_supported: bool) -> bool {
        let EffectDef::AddMana(mana) = effect else {
            return false;
        };
        let selection_is_supported = match mana.mana {
            ManaSelectionDef::One(_) => true,
            ManaSelectionDef::Choice(colors) => choices_are_supported && !colors.is_empty(),
        };
        selection_is_supported
            && mana.amount > 0
            // Mana stores these annotations, but payment does not enforce
            // restrictions yet and only paid-spell riders have a partial path.
            && mana.restrictions.is_empty()
            && mana.spend_effects.is_empty()
    }

    fn shared_resolving_apply(
        recipient: EffectRecipientDef,
        effect: AppliedEffectDef,
        duration: EffectDurationDef,
    ) -> bool {
        if duration != EffectDurationDef::UntilEndOfTurn || !shared_effect_recipient(recipient) {
            return false;
        }
        match effect {
            AppliedEffectDef::ModifyPowerToughness { .. } => true,
            AppliedEffectDef::GrantAbility(ability) => {
                ability.implementation == AbilityImplementationDef::Definition
                    && matches!(ability.definition, DeclarativeAbilityDef::Evergreen(keyword) if shared_evergreen(keyword))
            }
            AppliedEffectDef::CannotBeCountered | AppliedEffectDef::Special(_) => false,
        }
    }

    fn shared_stack_effect(effect: EffectDef) -> bool {
        match effect {
            EffectDef::Sequence(effects) => {
                !effects.is_empty() && effects.iter().copied().all(shared_stack_effect)
            }
            EffectDef::AddMana(_) => shared_mana_effect(effect, false),
            EffectDef::DealDamage { recipient, .. }
            | EffectDef::GainLife { recipient, .. }
            | EffectDef::DrawCards { recipient, .. }
            | EffectDef::LoseLife { recipient, .. } => shared_effect_recipient(recipient),
            EffectDef::Tap { object }
            | EffectDef::Destroy { object, .. }
            | EffectDef::Sacrifice { object }
            | EffectDef::Counter { object }
            | EffectDef::AddPlusOneCounters { object, .. } => shared_effect_recipient(object),
            EffectDef::OptionalManaPayment { effect, .. } => shared_stack_effect(*effect),
            EffectDef::Apply {
                recipient,
                effect,
                duration,
            } => shared_resolving_apply(recipient, effect, duration),
            EffectDef::None
            | EffectDef::EntersTapped
            | EffectDef::MoveToZone { .. }
            | EffectDef::Special(_) => false,
        }
    }

    fn shared_trigger_event(event: TriggerEventDef) -> bool {
        match event {
            TriggerEventDef::ZoneChanged { object, from, to } => {
                const COMMITTED_TRANSITIONS: [(ZoneKind, ZoneKind); 5] = [
                    (ZoneKind::Hand, ZoneKind::Battlefield),
                    (ZoneKind::Stack, ZoneKind::Battlefield),
                    (ZoneKind::Battlefield, ZoneKind::Graveyard),
                    (ZoneKind::Battlefield, ZoneKind::Exile),
                    (ZoneKind::Battlefield, ZoneKind::Hand),
                ];
                shared_object_predicate(object)
                    && COMMITTED_TRANSITIONS
                        .iter()
                        .any(|(actual_from, actual_to)| {
                            from.is_none_or(|expected| expected == *actual_from)
                                && to.is_none_or(|expected| expected == *actual_to)
                        })
            }
            TriggerEventDef::BecomesTapped(object) | TriggerEventDef::SpellCast(object) => {
                shared_object_predicate(object)
            }
            TriggerEventDef::StepBegins { .. } | TriggerEventDef::DamagedCreatureDied => true,
            TriggerEventDef::AbilityActivated(_)
            | TriggerEventDef::DamageDealt { .. }
            | TriggerEventDef::ManaAdded(_)
            | TriggerEventDef::Special(_) => false,
        }
    }

    fn shared_activated_costs(costs: &[AbilityCostDef]) -> bool {
        let sacrifice_choices = costs
            .iter()
            .filter(|cost| matches!(cost, AbilityCostDef::SacrificePermanent { .. }))
            .count();
        sacrifice_choices <= 1
            && costs.iter().all(|cost| match cost {
                AbilityCostDef::Mana(cost) => !cost.variable_x && cost.x_multiplier == 0,
                AbilityCostDef::SacrificePermanent { object, .. } => {
                    shared_object_predicate(*object)
                }
                AbilityCostDef::TapSource
                | AbilityCostDef::SacrificeSource
                | AbilityCostDef::PayLife(_) => true,
                AbilityCostDef::UntapSource
                | AbilityCostDef::DiscardCards(_)
                | AbilityCostDef::ExileSource
                | AbilityCostDef::Special(_) => false,
            })
    }

    fn shared_static_effect(effect: EffectDef) -> bool {
        match effect {
            EffectDef::Sequence(effects) => {
                !effects.is_empty() && effects.iter().copied().all(shared_static_effect)
            }
            EffectDef::Apply {
                recipient,
                effect,
                duration,
            } => {
                let recipient_is_supported = match recipient {
                    EffectRecipientDef::Source => true,
                    EffectRecipientDef::MatchingObjects { object, zones, .. } => {
                        zones == [ZoneKind::Battlefield] && shared_object_predicate(object)
                    }
                    EffectRecipientDef::Controller
                    | EffectRecipientDef::Opponent
                    | EffectRecipientDef::Target(_)
                    | EffectRecipientDef::TriggeringObject
                    | EffectRecipientDef::ControllerOfTriggeringObject
                    | EffectRecipientDef::EventPlayer => false,
                };
                let applied_effect_is_supported = match effect {
                    AppliedEffectDef::ModifyPowerToughness { power, toughness } => {
                        matches!(power, crate::card::ValueDef::Constant(_))
                            && matches!(toughness, crate::card::ValueDef::Constant(_))
                    }
                    AppliedEffectDef::GrantAbility(ability) => shared_definition_ability(ability),
                    AppliedEffectDef::CannotBeCountered | AppliedEffectDef::Special(_) => false,
                };
                recipient_is_supported
                    && applied_effect_is_supported
                    && matches!(
                        duration,
                        EffectDurationDef::WhileSourceRemainsInZone
                            | EffectDurationDef::UntilSourceLeavesZone
                    )
            }
            EffectDef::None
            | EffectDef::AddMana(_)
            | EffectDef::DealDamage { .. }
            | EffectDef::GainLife { .. }
            | EffectDef::DrawCards { .. }
            | EffectDef::LoseLife { .. }
            | EffectDef::Tap { .. }
            | EffectDef::Destroy { .. }
            | EffectDef::Sacrifice { .. }
            | EffectDef::Counter { .. }
            | EffectDef::AddPlusOneCounters { .. }
            | EffectDef::OptionalManaPayment { .. }
            | EffectDef::EntersTapped
            | EffectDef::MoveToZone { .. }
            | EffectDef::Special(_) => false,
        }
    }

    fn battlefield_only(zones: &[ZoneKind]) -> bool {
        zones == [ZoneKind::Battlefield]
    }

    fn shared_definition_ability(ability: &AbilityDef) -> bool {
        if ability.implementation != AbilityImplementationDef::Definition {
            return false;
        }
        match ability.definition {
            DeclarativeAbilityDef::Spell(_) => shared_stack_effect(ability.effect),
            DeclarativeAbilityDef::ActivatedMana(definition) => {
                battlefield_only(definition.source_zones)
                    && definition.costs.iter().any(|cost| {
                        matches!(
                            cost,
                            AbilityCostDef::TapSource | AbilityCostDef::SacrificeSource
                        )
                    })
                    && definition.costs.iter().all(|cost| {
                        matches!(
                            cost,
                            AbilityCostDef::TapSource
                                | AbilityCostDef::SacrificeSource
                                | AbilityCostDef::PayLife(_)
                        )
                    })
                    && shared_mana_effect(ability.effect, true)
            }
            DeclarativeAbilityDef::TriggeredMana(definition) => {
                fn immediate_mana_effect(effect: EffectDef) -> bool {
                    match effect {
                        EffectDef::Sequence(effects) => {
                            !effects.is_empty()
                                && effects.iter().copied().all(immediate_mana_effect)
                        }
                        EffectDef::AddMana(_) => shared_mana_effect(effect, false),
                        EffectDef::None
                        | EffectDef::DealDamage { .. }
                        | EffectDef::GainLife { .. }
                        | EffectDef::DrawCards { .. }
                        | EffectDef::LoseLife { .. }
                        | EffectDef::Tap { .. }
                        | EffectDef::Destroy { .. }
                        | EffectDef::Sacrifice { .. }
                        | EffectDef::Counter { .. }
                        | EffectDef::AddPlusOneCounters { .. }
                        | EffectDef::OptionalManaPayment { .. }
                        | EffectDef::EntersTapped
                        | EffectDef::MoveToZone { .. }
                        | EffectDef::Apply { .. }
                        | EffectDef::Special(_) => false,
                    }
                }
                battlefield_only(definition.source_zones)
                    && shared_trigger_event(definition.event)
                    && immediate_mana_effect(ability.effect)
            }
            DeclarativeAbilityDef::Activated(definition) => {
                battlefield_only(definition.source_zones)
                    && shared_activated_costs(definition.costs)
                    && shared_stack_effect(ability.effect)
            }
            DeclarativeAbilityDef::Triggered(definition) => {
                battlefield_only(definition.source_zones)
                    && shared_trigger_event(definition.event)
                    && shared_stack_effect(ability.effect)
            }
            DeclarativeAbilityDef::Static(definition) => {
                battlefield_only(definition.source_zones) && shared_static_effect(ability.effect)
            }
            DeclarativeAbilityDef::Replacement(definition) => {
                battlefield_only(definition.source_zones)
                    && ability.effect == EffectDef::EntersTapped
            }
            DeclarativeAbilityDef::Evergreen(keyword) => shared_evergreen(keyword),
            DeclarativeAbilityDef::SpecialAction(_) | DeclarativeAbilityDef::Legacy => false,
        }
    }

    fn assert_nested_definition_abilities(card_name: &str, effect: EffectDef) {
        match effect {
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    assert_nested_definition_abilities(card_name, *effect);
                }
            }
            EffectDef::OptionalManaPayment { effect, .. } => {
                assert_nested_definition_abilities(card_name, *effect);
            }
            EffectDef::Apply {
                effect: AppliedEffectDef::GrantAbility(ability),
                ..
            } => {
                if ability.implementation == AbilityImplementationDef::Definition {
                    assert!(
                        shared_definition_ability(ability),
                        "{card_name} contains a nested Definition ability outside the shared runtime boundary: {ability:?}",
                    );
                }
                assert_nested_definition_abilities(card_name, ability.effect);
            }
            EffectDef::None
            | EffectDef::AddMana(_)
            | EffectDef::DealDamage { .. }
            | EffectDef::GainLife { .. }
            | EffectDef::DrawCards { .. }
            | EffectDef::LoseLife { .. }
            | EffectDef::Tap { .. }
            | EffectDef::Destroy { .. }
            | EffectDef::Sacrifice { .. }
            | EffectDef::Counter { .. }
            | EffectDef::AddPlusOneCounters { .. }
            | EffectDef::EntersTapped
            | EffectDef::MoveToZone { .. }
            | EffectDef::Apply { .. }
            | EffectDef::Special(_) => {}
        }
    }

    #[test]
    fn every_supported_set_has_one_matching_module() {
        let expected_sets = Format::OldSchool9394
            .rules()
            .allowed_sets
            .iter()
            .chain(Format::IsdRtrStandard.rules().allowed_sets)
            .copied()
            .collect::<Vec<_>>();
        let registered_sets = SET_MODULES
            .iter()
            .map(|module| module.set)
            .collect::<Vec<_>>();

        assert_eq!(registered_sets, expected_sets);
        assert_eq!(registered_sets.len(), 20);
        assert_eq!(
            registered_sets
                .iter()
                .copied()
                .collect::<HashSet<_>>()
                .len(),
            20
        );

        for module in SET_MODULES {
            for record in module.cards {
                assert_eq!(
                    record.set, module.set,
                    "{} is registered in the wrong set",
                    record.name
                );
            }
        }
    }

    #[test]
    fn built_in_records_keep_stable_dense_ids_and_unique_identity() {
        let records = SET_MODULES
            .iter()
            .flat_map(|module| module.cards.iter().copied())
            .collect::<Vec<_>>();
        assert_eq!(records.len(), 244);

        let mut ids = records.iter().map(|record| record.id).collect::<Vec<_>>();
        ids.sort_unstable();
        assert_eq!(
            ids.iter().map(|id| id.0).collect::<Vec<_>>(),
            (1..=244).collect::<Vec<_>>()
        );
        assert_eq!(
            records
                .iter()
                .map(|record| record.name)
                .collect::<HashSet<_>>()
                .len(),
            records.len()
        );
    }

    #[test]
    fn built_in_catalog_indexes_definitions_and_printings_separately() {
        let catalog = crate::card::catalog().unwrap();
        let printing_count = (1..=244)
            .map(|id| catalog.printings_for(CardDefinitionId(id)).len())
            .sum::<usize>();

        assert_eq!(printing_count, 624);
        for variant in 0..3 {
            assert!(
                catalog
                    .get_printing(CardPrintingId::with_variant(
                        cards::PLAINS,
                        CardSet::Beta,
                        variant,
                    ))
                    .is_some()
            );
        }
        assert_eq!(catalog.find_by_name("Plains"), Some(cards::PLAINS));
    }

    #[test]
    fn every_non_declarative_clause_explains_its_implementation() {
        let records = SET_MODULES
            .iter()
            .flat_map(|module| module.cards.iter().copied())
            .collect::<Vec<_>>();
        assert_eq!(records.len(), 244);

        for record in records {
            let definition = record.definition();
            for part in &definition.parts {
                for ability in part.rules.ability_clauses() {
                    if !matches!(ability.implementation, AbilityImplementationDef::Definition) {
                        assert!(
                            ability
                                .implementation
                                .explanation()
                                .is_some_and(|explanation| !explanation.trim().is_empty()),
                            "{} has a non-declarative clause without an explanation: {}",
                            record.name,
                            ability.text
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn standard_records_cover_the_top_eight_pool_with_stable_unique_ids() {
        let records = standard_records();
        assert_eq!(records.len(), 116);

        let mut names = HashSet::new();
        for (offset, record) in records.iter().enumerate() {
            assert_eq!(usize::from(record.id.0), 129 + offset);
            assert!(names.insert(record.name));
            assert!(!record.rules.has_supertype(CardSupertype::Basic));
            assert!(Format::IsdRtrStandard.allows_set(record.set));
            if let Some(behavior) = record.rules.special_behavior() {
                assert_eq!(behavior.rules(), &record.rules);
            }
        }

        assert!(!names.contains("Celestial Purge"));
        assert!(names.contains("Celestial Flare"));
    }

    #[test]
    fn standard_records_have_complete_unique_scryfall_metadata() {
        let records = standard_records();
        let mut scryfall_ids = HashSet::new();

        for record in records {
            let scryfall_id = record.art.scryfall_id;
            assert!(
                is_uuid(scryfall_id),
                "{} has an invalid Scryfall ID: {scryfall_id}",
                record.name
            );
            assert!(
                scryfall_ids.insert(scryfall_id),
                "{} repeats Scryfall ID {scryfall_id}",
                record.name
            );
            assert!(
                !record.art.artist.trim().is_empty(),
                "{} is missing its artist",
                record.name
            );
        }

        assert_eq!(scryfall_ids.len(), 116);
    }

    #[test]
    fn structured_records_expose_parts_and_play_options_without_losing_primary_rules() {
        let garruk = y2011::innistrad::GARRUK_RELENTLESS.definition();
        assert_eq!(garruk.name, "Garruk Relentless");
        assert_eq!(garruk.rules, garruk.primary_part().unwrap().rules);
        assert_eq!(garruk.parts.len(), 2);
        assert_eq!(garruk.parts[1].name, "Garruk, the Veil-Cursed");
        assert_eq!(garruk.parts[1].mana_cost, None);
        assert_eq!(
            garruk.parts[1].rules.colors,
            [false, false, true, false, true]
        );
        assert!(matches!(
            garruk.structure,
            CardStructure::DoubleFaced {
                front: CardPartId(0),
                back: CardPartId(1),
                kind: DoubleFacedKind::Transforming,
            }
        ));

        let huntmaster = y2012::dark_ascension::HUNTMASTER_OF_THE_FELLS.definition();
        assert_eq!(huntmaster.rules, huntmaster.primary_part().unwrap().rules);
        assert_eq!(huntmaster.parts.len(), 2);
        assert_eq!(huntmaster.parts[1].name, "Ravager of the Fells");
        assert_eq!(huntmaster.parts[1].mana_cost, None);
        assert_eq!(huntmaster.parts[1].rules.creature_stats.unwrap().power, 4);
        assert!(
            huntmaster.parts[1]
                .rules
                .has_evergreen(EvergreenAbility::Trample)
        );

        let turn_burn = y2013::dragons_maze::TURN_BURN.definition();
        assert_eq!(turn_burn.name, "Turn // Burn");
        assert_eq!(turn_burn.rules, turn_burn.parts[0].rules);
        assert!(turn_burn.rules.alternate_mana_costs.is_empty());
        assert_eq!(turn_burn.parts.len(), 2);
        assert_eq!(turn_burn.parts[0].name, "Turn");
        assert_eq!(turn_burn.parts[1].name, "Burn");
        assert_eq!(
            turn_burn.parts[1].rules.colors,
            [false, false, false, true, false]
        );
        assert!(matches!(
            turn_burn.structure,
            CardStructure::Split {
                ref parts,
                fused: Some(PlayOptionId(2)),
            } if parts == &[CardPartId(0), CardPartId(1)]
        ));
        assert_eq!(turn_burn.play_options.len(), 3);
        assert!(matches!(
            turn_burn.play_options[2].form,
            SpellForm::Combined(ref parts) if parts == &[CardPartId(0), CardPartId(1)]
        ));
        assert_eq!(
            turn_burn.play_options[2].restriction,
            PlayRestriction::FromHandOnly
        );
        assert_eq!(turn_burn.play_options[2].targets.len(), 2);

        let charm = y2012::return_to_ravnica::IZZET_CHARM.definition();
        assert_eq!(charm.parts.len(), 1);
        assert_eq!(charm.play_options.len(), 1);
        let modes = charm.play_options[0].modes.as_ref().unwrap();
        assert_eq!(
            (modes.minimum, modes.maximum, modes.may_repeat),
            (1, 1, false)
        );
        assert_eq!(modes.modes.len(), 3);
        assert_eq!(modes.modes[0].id, ModeId(0));
        assert_eq!(
            modes.modes[0].targets[0].predicate,
            TargetPredicate::NoncreatureSpell
        );
        assert_eq!(
            modes.modes[1].targets[0].predicate,
            TargetPredicate::CreaturePermanent
        );
        assert!(modes.modes[2].targets.is_empty());
        assert_eq!(charm.play_options[0].action, PlayActionKind::CastSpell);
    }

    #[test]
    fn ordinary_records_synthesize_one_primary_part_and_play_option() {
        let bolt = y1993::alpha::LIGHTNING_BOLT.definition();
        assert_eq!(bolt.parts.len(), 1);
        assert_eq!(bolt.primary_part_id(), CardPartId::PRIMARY);
        assert_eq!(bolt.primary_part().unwrap().rules, bolt.rules);
        assert!(matches!(
            bolt.structure,
            CardStructure::Single {
                main: CardPartId::PRIMARY,
            }
        ));
        assert_eq!(bolt.play_options.len(), 1);
        assert_eq!(bolt.play_options[0].id, PlayOptionId::DEFAULT);
        assert_eq!(
            bolt.play_options[0].form,
            SpellForm::Part(CardPartId::PRIMARY)
        );

        let mountain = y1993::alpha::MOUNTAIN.definition();
        assert_eq!(mountain.parts[0].mana_cost, None);
        assert_eq!(mountain.play_options[0].action, PlayActionKind::PlayLand);
        assert_eq!(mountain.play_options[0].mana_cost, None);
    }

    #[test]
    fn cavern_records_both_mana_abilities_and_the_colored_mana_riders() {
        let abilities = y2012::avacyn_restored::CAVERN_OF_SOULS
            .rules
            .ability_clauses();
        assert_eq!(abilities.len(), 3);
        assert!(matches!(
            abilities[1].definition,
            DeclarativeAbilityDef::ActivatedMana(_)
        ));
        assert!(matches!(
            abilities[1].effect,
            EffectDef::AddMana(mana)
                if mana.mana == ManaSelectionDef::One(ManaKindDef::Colorless)
                    && mana.amount == 1
                    && mana.restrictions.is_empty()
                    && mana.spend_effects.is_empty()
        ));
        assert!(matches!(
            abilities[2].definition,
            DeclarativeAbilityDef::ActivatedMana(_)
        ));
        assert!(matches!(
            abilities[2].effect,
            EffectDef::AddMana(mana)
                if mana.mana == ManaSelectionDef::Choice(&[
                    ManaKindDef::White,
                    ManaKindDef::Blue,
                    ManaKindDef::Black,
                    ManaKindDef::Red,
                    ManaKindDef::Green,
                ])
                    && mana.amount == 1
                    && mana.restrictions
                        == [ManaRestrictionDef::CastCreatureSpellOfChosenType]
                    && mana.spend_effects
                        == [ManaSpendEffectDef::ApplyToPaidSpell(
                            AppliedEffectDef::CannotBeCountered,
                        )]
        ));
    }

    #[test]
    fn every_builtin_land_prints_its_mana_ability_clauses() {
        let lands = SET_MODULES
            .iter()
            .flat_map(|module| module.cards.iter().copied())
            .filter(|record| record.rules.kind == crate::card::CardKind::Land)
            .collect::<Vec<_>>();
        assert_eq!(lands.len(), 45);

        let lands_without_mana = lands
            .iter()
            .filter(|record| {
                !record.rules.ability_clauses().iter().any(|ability| {
                    matches!(ability.definition, DeclarativeAbilityDef::ActivatedMana(_))
                })
            })
            .map(|record| record.name)
            .collect::<Vec<_>>();
        assert_eq!(lands_without_mana, ["Maze of Ith"]);

        for land in lands {
            let basic_subtypes = BasicLandType::ALL
                .into_iter()
                .filter(|land_type| land.rules.has_subtype(land_type.subtype()))
                .count();
            let printed_mana_abilities = land
                .rules
                .ability_clauses()
                .iter()
                .filter(|ability| {
                    matches!(ability.definition, DeclarativeAbilityDef::ActivatedMana(_))
                })
                .count();
            assert!(
                printed_mana_abilities >= basic_subtypes,
                "{} has {basic_subtypes} basic land subtypes but only {printed_mana_abilities} printed mana abilities",
                land.name
            );
        }
    }

    #[test]
    fn every_nonland_mana_permanent_has_an_activated_mana_clause() {
        let records = [
            &y1993::alpha::BLACK_LOTUS,
            &y1993::alpha::MOX_EMERALD,
            &y1993::alpha::MOX_JET,
            &y1993::alpha::MOX_PEARL,
            &y1993::alpha::MOX_RUBY,
            &y1993::alpha::MOX_SAPPHIRE,
            &y1993::alpha::SOL_RING,
            &y1993::alpha::MANA_VAULT,
            &y1993::alpha::BIRDS_OF_PARADISE,
            &y1993::alpha::LLANOWAR_ELVES,
            &y1994::the_dark::FELLWAR_STONE,
            &y2011::innistrad::AVACYNS_PILGRIM,
            &y2013::magic_2014::ELVISH_MYSTIC,
        ];
        assert_eq!(records.len(), 13);
        for record in records {
            assert!(
                record.rules.ability_clauses().iter().any(|ability| {
                    matches!(ability.definition, DeclarativeAbilityDef::ActivatedMana(_))
                }),
                "{} is missing its activated mana clause",
                record.name
            );
        }
    }

    #[test]
    fn activation_presentation_lives_on_the_exact_activated_clause() {
        let cases = [
            (
                &y1993::alpha::GLASSES_OF_URZA,
                AbilityId::PRIMARY,
                "Look at {}'s hand with Glasses of Urza",
                "Look at a player's hand",
            ),
            (
                &y1993::alpha::STONE_GIANT,
                AbilityId::PRIMARY,
                "Give {} flying with Stone Giant",
                "Give a smaller creature flying",
            ),
            (
                &y1993::alpha::CHAOS_ORB,
                AbilityId::PRIMARY,
                "Flip Chaos Orb onto {}",
                "Flip Chaos Orb onto a permanent",
            ),
            (
                &y1993::alpha::ICY_MANIPULATOR,
                AbilityId::PRIMARY,
                "Tap {} with Icy Manipulator",
                "Tap an artifact, creature, or land",
            ),
            (
                &y1994::antiquities::MISHRA_S_FACTORY,
                AbilityId(2),
                "Give {} +1/+1 with Mishra's Factory",
                "Give an Assembly-Worker +1/+1",
            ),
            (
                &y1994::antiquities::ORCISH_MECHANICS,
                AbilityId::PRIMARY,
                "Deal 2 damage to {} with Orcish Mechanics",
                "Deal 2 damage",
            ),
            (
                &y1994::antiquities::STRIP_MINE,
                AbilityId(1),
                "Destroy {} with Strip Mine",
                "Destroy a land",
            ),
            (
                &y1994::antiquities::TRISKELION,
                AbilityId(1),
                "Deal 1 damage to {} with Triskelion",
                "Deal 1 damage",
            ),
            (
                &y1994::fallen_empires::ICATIAN_JAVELINEERS,
                AbilityId(1),
                "Deal 1 damage to {} with Icatian Javelineers",
                "Deal 1 damage",
            ),
            (
                &y1994::legends::PENDELHAVEN,
                AbilityId(1),
                "Give {} +1/+2 with Pendelhaven",
                "Give a 1/1 creature +1/+2",
            ),
            (
                &y1994::legends::RELIC_BARRIER,
                AbilityId::PRIMARY,
                "Tap {} with Relic Barrier",
                "Tap an artifact",
            ),
            (
                &y1994::the_dark::MAZE_OF_ITH,
                AbilityId::PRIMARY,
                "Untap {} and take it out of combat",
                "Take an attacker out of combat",
            ),
        ];

        for (record, ability_id, targeted, summary) in cases {
            let ability = record
                .rules
                .ability_clauses()
                .iter()
                .find(|ability| ability.id == ability_id)
                .unwrap_or_else(|| panic!("{} is missing ability {ability_id:?}", record.name));
            assert!(matches!(
                ability.definition,
                DeclarativeAbilityDef::Activated(_)
            ));
            let presentation = ability.activation_text.unwrap();
            assert_eq!(presentation.targeted, targeted);
            assert_eq!(presentation.summary, summary);
        }

        let presentation_count = SET_MODULES
            .iter()
            .flat_map(|module| module.cards.iter())
            .flat_map(|record| record.rules.ability_clauses())
            .filter(|ability| ability.activation_text.is_some())
            .count();
        assert_eq!(presentation_count, cases.len());
    }

    #[test]
    fn migrated_activated_cards_preserve_their_derived_implementation_status() {
        let partial = [
            &y1993::alpha::GLASSES_OF_URZA,
            &y1993::alpha::STONE_GIANT,
            &y1993::alpha::CHAOS_ORB,
            &y1994::antiquities::MISHRA_S_FACTORY,
            &y1994::antiquities::TRISKELION,
            &y1994::fallen_empires::ICATIAN_JAVELINEERS,
            &y1994::legends::PENDELHAVEN,
            &y1994::the_dark::MAZE_OF_ITH,
        ];
        let complete = [
            &y1993::alpha::ICY_MANIPULATOR,
            &y1994::antiquities::ORCISH_MECHANICS,
            &y1994::antiquities::STRIP_MINE,
            &y1994::legends::RELIC_BARRIER,
        ];

        for record in partial {
            assert_eq!(
                record.rules.implementation_status(),
                ImplementationStatus::Partial,
                "{} should remain partially implemented",
                record.name
            );
        }
        for record in complete {
            assert_eq!(
                record.rules.implementation_status(),
                ImplementationStatus::Complete,
                "{} should remain completely implemented",
                record.name
            );
        }
    }

    #[test]
    fn early_core_sets_reuse_definitions_without_duplicating_identity() {
        let all_definition_ids = SET_MODULES
            .iter()
            .flat_map(|module| module.cards.iter().map(|record| record.id))
            .collect::<HashSet<_>>();
        let basics = [
            cards::PLAINS,
            cards::ISLAND,
            cards::SWAMP,
            cards::MOUNTAIN,
            cards::FOREST,
        ];

        let early_sets = [
            (CardSet::Alpha, 83, 88, 2_u16),
            (CardSet::Beta, 84, 94, 3_u16),
            (CardSet::Unlimited, 84, 94, 3_u16),
            (CardSet::CollectorsEdition, 84, 94, 3_u16),
            (CardSet::InternationalCollectorsEdition, 84, 94, 3_u16),
        ];

        let mut printing_ids = HashSet::new();
        for (set, expected_cards, expected_printings, expected_basic_variants) in early_sets {
            let printings = printings_for_set(set);
            assert_eq!(printings.len(), expected_printings);
            assert_eq!(
                printings
                    .iter()
                    .map(|printing| printing.id.definition)
                    .collect::<HashSet<_>>()
                    .len(),
                expected_cards
            );

            for printing in &printings {
                assert!(all_definition_ids.contains(&printing.id.definition));
                assert_eq!(printing.id.set, set);
                assert!(printing_ids.insert(printing.id));
            }
            for basic in basics {
                let variants = printings
                    .iter()
                    .filter(|printing| printing.id.definition == basic)
                    .map(|printing| printing.id.variant)
                    .collect::<HashSet<_>>();
                assert_eq!(variants.len(), usize::from(expected_basic_variants));
                assert_eq!(variants, (0..expected_basic_variants).collect());
            }
        }

        assert_eq!(y1993::beta::VOLCANIC_ISLAND.id, cards::VOLCANIC_ISLAND);
        assert_eq!(y1993::beta::VOLCANIC_ISLAND.set, CardSet::Beta);
    }

    #[test]
    fn fully_declarative_clauses_stay_within_the_shared_runtime_boundary() {
        for record in SET_MODULES
            .iter()
            .flat_map(|module| module.cards.iter().copied())
        {
            let definition = record.definition();
            for part in &definition.parts {
                for ability in part.rules.ability_clauses() {
                    assert!(
                        !matches!(
                            (ability.definition, ability.implementation),
                            (
                                DeclarativeAbilityDef::Legacy,
                                AbilityImplementationDef::CustomFull { behavior: None, .. }
                            )
                        ),
                        "{} {:?} ability {:?} is legacy text claiming full implementation without an executable behavior: {ability:?}",
                        definition.name,
                        part.id,
                        ability.id,
                    );
                    if ability.implementation == AbilityImplementationDef::Definition {
                        assert!(
                            shared_definition_ability(ability),
                            "{} {:?} ability {:?} claims Definition outside the shared runtime boundary: {ability:?}",
                            definition.name,
                            part.id,
                            ability.id,
                        );
                    }
                    assert_nested_definition_abilities(&definition.name, ability.effect);
                }
            }
        }
    }
}
