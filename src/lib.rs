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

pub use action::{AbilityOrigin, Action, ActionError, CombatDamageAssignment, ManaColor, Target};
pub use card::{
    AbilityCostDef, AbilityDef, AbilityImplementationDef, AbilityTargetDef, AbilityTargetPredicate,
    ActivatedAbilityDef, ActivatedAbilityText, AddManaEffectDef, AdditionalCostDef,
    AlternateManaCost, AlternateSpellKind, AlternativeCostDef, AppliedEffectDef, BasicLandType,
    CardAbilityList, CardArt, CardBehavior, CardCatalog, CardComposition, CardDefinition,
    CardEffectStatus, CardKind, CardPart, CardPrinting, CardPrintingId, CardRules, CardSet,
    CardStructure, CardSupertype, CatalogError, CharacteristicContext, CharacteristicError,
    ColorDef, CreatureStats, DeclarativeAbilityDef, DoubleFacedKind, EffectDef, EffectDurationDef,
    EffectRecipientDef, EvergreenAbility, ImplementationStatus, LandEntry, ManaCost, ManaKindDef,
    ManaRestrictionDef, ManaSelectionDef, ManaSpendEffectDef, MeldComponentDef, MeldRecipeDef,
    MeldResultDef, ModeDef, ModeSetDef, ObjectPredicateDef, PlayActionKind, PlayOptionDef,
    PlayRestriction, PlayerRelation, PrintedManaCost, ReplacementAbilityDef, SpecialActionDef,
    SpellAbilityDef, SpellForm, StaticAbilityDef, TargetPredicate, TargetSlotDef, TriggerEventDef,
    TriggeredAbilityDef, TurnStepDef, ValueDef, ZoneKind, applicable_part_ids,
    intrinsic_basic_land_mana_ability,
};
pub use casting::{
    CastChoices, CastSignature, CostConfiguration, TargetReplacementError, TargetSelection,
};
pub use deck::{Deck, DeckError, ValidatedDeck};
pub use format::{Format, FormatRules};
pub use game::{
    BattlefieldExit, DecisionObservation, DecisionOption, DecisionPreference, DecisionVisibility,
    DecisionZone, Game, GameError, GameEvent, GameResult, Mana, ManaPool, ManaSource,
    PlayerObservation, StackObjectKind, Step, WinReason, ZoneChangeOutcome,
};
pub use ids::{
    AbilityId, AdditionalCostId, AlternativeCostId, CardDefinitionId, CardInstanceId, CardPartId,
    GameObjectId, MeldRecipeId, ModeId, PhysicalCardId, PlayOptionId, PlayerId, StackObjectId,
    TargetSlotId,
};
pub use policy::{HandcraftedPolicy, PlayError, Policy, RandomPolicy, play_game};
