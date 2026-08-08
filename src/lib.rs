//! Deterministic engine primitives for supported two-player Magic formats.

pub mod action;
pub mod card;
pub mod casting;
pub mod deck;
pub mod decks;
pub mod format;
pub mod game;
pub mod ids;
pub mod poc;
pub mod policy;
pub mod protocol;
mod rng;
pub mod rules;

pub use action::{Action, ActionError, CombatDamageAssignment, ManaColor, Target};
pub use card::{
    ActivatedAbilityText, AdditionalCostDef, AlternateManaCost, AlternateSpellKind,
    AlternativeCostDef, CardArt, CardBehavior, CardCatalog, CardComposition, CardDefinition,
    CardEffectStatus, CardKind, CardPart, CardPrinting, CardPrintingId, CardRules, CardSet,
    CardStructure, CatalogError, CharacteristicContext, CharacteristicError, CreatureStats,
    DoubleFacedKind, LandEntry, ManaCost, ManaProduction, MeldComponentDef, MeldRecipeDef,
    MeldResultDef, ModeDef, ModeSetDef, PlayActionKind, PlayOptionDef, PlayRestriction, SpellForm,
    TargetPredicate, TargetSlotDef, applicable_part_ids,
};
pub use casting::{
    CastChoices, CastSignature, CostConfiguration, TargetReplacementError, TargetSelection,
};
pub use deck::{Deck, DeckError, ValidatedDeck};
pub use format::{Format, FormatRules};
pub use game::{
    BattlefieldExit, DecisionObservation, DecisionOption, DecisionPreference, DecisionVisibility,
    DecisionZone, Game, GameError, GameEvent, GameResult, ManaPool, PlayerObservation,
    StackObjectKind, Step, WinReason, ZoneChangeOutcome,
};
pub use ids::{
    AdditionalCostId, AlternativeCostId, CardDefinitionId, CardInstanceId, CardPartId,
    GameObjectId, MeldRecipeId, ModeId, PhysicalCardId, PlayOptionId, PlayerId, StackObjectId,
    TargetSlotId,
};
pub use policy::{HandcraftedPolicy, PlayError, Policy, RandomPolicy, play_game};
