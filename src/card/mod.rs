//! Card definitions, rules metadata, and the built-in multi-format card corpus.
//!
//! The corpus is grouped by release year and set in [`sets`]. Canonical card
//! records own rules and implementation status, while reprint and alternate-art
//! records provide distinct physical-printing identities without duplicating
//! gameplay definitions.

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
    ActivatedAbilityText, AdditionalCostDef, AlternateManaCost, AlternateSpellKind,
    AlternativeCostDef, CardBehavior, CardComposition, CardDefinition, CardEffectStatus, CardKind,
    CardPart, CardPrinting, CardPrintingId, CardRules, CardSet, CardStructure, CreatureStats,
    DoubleFacedKind, LandEntry, ManaCost, ManaProduction, MeldComponentDef, MeldRecipeDef,
    MeldResultDef, ModeDef, ModeSetDef, PlayActionKind, PlayOptionDef, PlayRestriction, SpellForm,
    TargetPredicate, TargetSlotDef,
};

/// Builds the complete card catalog required by the built-in decks.
///
/// # Errors
///
/// Returns [`CatalogError`] if a built-in definition, name, or printing is
/// accidentally duplicated or references an unknown card.
pub fn catalog() -> Result<CardCatalog, CatalogError> {
    CardCatalog::with_additional_printings(sets::definitions(), sets::additional_printings())
}
