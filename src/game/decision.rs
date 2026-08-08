use crate::{CardDefinitionId, GameObjectId, PlayerId};

/// The rules procedure represented by a mandatory player decision.
///
/// Ordinary choices are unordered selections. Trigger ordering and trigger
/// placement are separate procedures that happen while no player has
/// priority, even though they use the same indexed-choice protocol command.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DecisionKind {
    Choice,
    TriggerOrder,
    TriggerPlacement,
}

/// How an ordered decision's submitted option list should be interpreted.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DecisionOrderSemantics {
    Resolution,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DecisionVisibility {
    Public,
    Private,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DecisionPreference {
    HigherCardValue,
    LowerCardValue,
    Neutral,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DecisionZone {
    Hand,
    Graveyard,
    Battlefield,
    Stack,
    Library,
    Exile,
    Command,
    DrawnThisStep,
    None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecisionOption {
    pub id: u32,
    pub label: String,
    pub card: Option<(GameObjectId, CardDefinitionId)>,
    /// Frozen creating-ability text when this option represents a pending
    /// trigger. This distinguishes multiple abilities from the same source.
    pub ability_text: Option<String>,
    pub zone: DecisionZone,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecisionObservation {
    pub id: u32,
    pub player: PlayerId,
    pub kind: DecisionKind,
    pub order_semantics: Option<DecisionOrderSemantics>,
    pub prompt: String,
    pub visibility: DecisionVisibility,
    pub preference: DecisionPreference,
    pub minimum: usize,
    pub maximum: usize,
    pub cancellable: bool,
    pub options: Vec<DecisionOption>,
}
