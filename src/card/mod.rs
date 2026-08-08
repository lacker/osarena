//! Card definitions, rules metadata, and the built-in multi-format card corpus.
//!
//! The corpus is grouped by release year and set in [`sets`]. Canonical card
//! records own rules and implementation status, while reprint and alternate-art
//! records provide distinct physical-printing identities without duplicating
//! gameplay definitions.

use std::sync::LazyLock;

mod behavior;
mod catalog;
mod characteristics;
mod model;
mod record;
mod sets;

pub mod cards;

pub use catalog::{CardCatalog, CatalogError};
pub use characteristics::{CharacteristicContext, CharacteristicError, applicable_part_ids};
pub use model::{
    AbilityCostDef, AbilityDef, AbilityImplementationDef, AbilityTargetDef, AbilityTargetPredicate,
    ActivatedAbilityDef, ActivatedAbilityText, AddManaEffectDef, AdditionalCostDef,
    AlternateManaCost, AlternateSpellKind, AlternativeCostDef, AppliedEffectDef, BasicLandType,
    CardAbilityList, CardArt, CardBehavior, CardComposition, CardDefinition, CardEffectStatus,
    CardKind, CardPart, CardPrinting, CardPrintingId, CardRules, CardSet, CardStructure,
    CardSupertype, ColorDef, CreatureStats, DeclarativeAbilityDef, DoubleFacedKind, EffectDef,
    EffectDurationDef, EffectRecipientDef, EvergreenAbility, ImplementationStatus, LandEntry,
    ManaCost, ManaKindDef, ManaRestrictionDef, ManaSelectionDef, ManaSpendEffectDef,
    MeldComponentDef, MeldRecipeDef, MeldResultDef, ModeDef, ModeSetDef, ObjectPredicateDef,
    PlayActionKind, PlayOptionDef, PlayRestriction, PlayerRelation, PrintedManaCost,
    ReplacementAbilityDef, SpecialActionDef, SpellAbilityDef, SpellForm, StaticAbilityDef,
    TargetPredicate, TargetSlotDef, TriggerEventDef, TriggeredAbilityDef, TurnStepDef, ValueDef,
    ZoneKind, intrinsic_basic_land_mana_ability,
};

/// The built-in catalog, validated once per process. Construction walks every
/// definition and printing, and callers used to pay for it on every game — a
/// training loop opens thousands.
static BUILT_IN: LazyLock<Result<CardCatalog, CatalogError>> = LazyLock::new(|| {
    CardCatalog::with_additional_printings(sets::definitions(), sets::additional_printings())
});

/// Builds the complete card catalog required by the built-in decks.
///
/// The returned catalog shares its definitions with every other caller, so
/// this is cheap enough to call per game.
///
/// # Errors
///
/// Returns [`CatalogError`] if a built-in definition, name, or printing is
/// accidentally duplicated or references an unknown card.
pub fn catalog() -> Result<CardCatalog, CatalogError> {
    BUILT_IN.clone()
}
