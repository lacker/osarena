//! Card definitions, rules metadata, and the built-in Old School card corpus.
//!
//! The corpus is grouped by first printing in [`sets`], while the reusable
//! card model and catalog implementation remain independent of that data.

mod behavior;
mod catalog;
mod model;
mod record;
mod sets;

pub mod cards;

pub use catalog::{CardCatalog, CatalogError};
pub use model::{
    ActivatedAbilityText, CardBehavior, CardDefinition, CardKind, CardRules, CardSet,
    CreatureStats, ManaCost,
};

/// Builds the complete card catalog required by the built-in decks.
///
/// # Errors
///
/// Returns [`CatalogError`] if a built-in ID or name is accidentally duplicated.
pub fn catalog() -> Result<CardCatalog, CatalogError> {
    CardCatalog::new(sets::definitions())
}
