//! Deterministic engine primitives for Eternal Central Old School 93/94.

pub mod action;
pub mod card;
pub mod deck;
pub mod decks;
pub mod game;
pub mod ids;
pub mod poc;
pub mod policy;
pub mod protocol;
mod rng;
pub mod rules;

pub use action::{Action, ActionError, CombatDamageAssignment, ManaColor, Target};
pub use card::{
    ActivatedAbilityText, CardBehavior, CardCatalog, CardDefinition, CardKind, CardRules, CardSet,
    CatalogError, CreatureStats, ManaCost,
};
pub use deck::{Deck, DeckError, ValidatedDeck};
pub use game::{
    BattlefieldExit, DecisionObservation, DecisionOption, DecisionPreference, DecisionVisibility,
    DecisionZone, Game, GameError, GameEvent, GameResult, ManaPool, PlayerObservation,
    StackObjectKind, Step, WinReason,
};
pub use ids::{CardDefinitionId, CardInstanceId, PlayerId, StackObjectId};
pub use policy::{HandcraftedPolicy, PlayError, Policy, RandomPolicy, play_game};
