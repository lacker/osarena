use crate::{CardDefinitionId, GameObjectId, PlayerId};

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
    Library,
    DrawnThisStep,
    None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecisionOption {
    pub id: u32,
    pub label: String,
    pub card: Option<(GameObjectId, CardDefinitionId)>,
    pub zone: DecisionZone,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecisionObservation {
    pub id: u32,
    pub player: PlayerId,
    pub prompt: String,
    pub visibility: DecisionVisibility,
    pub preference: DecisionPreference,
    pub minimum: usize,
    pub maximum: usize,
    pub cancellable: bool,
    pub options: Vec<DecisionOption>,
}
