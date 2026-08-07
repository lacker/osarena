use crate::{Action, CardDefinitionId, CardInstanceId, PlayerId, StackObjectId, Target};

use super::{DecisionObservation, GameResult, ManaPool, StackObjectKind, Step};

pub(super) type PublicCard = (CardInstanceId, CardDefinitionId);
pub(super) type LastSeenHand = Option<(PlayerId, Vec<PublicCard>)>;

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct PermanentObservation {
    pub id: CardInstanceId,
    pub definition: CardDefinitionId,
    pub controller: PlayerId,
    pub tapped: bool,
    pub power: Option<i16>,
    pub toughness: Option<i16>,
    pub damage: u16,
    pub attacking: bool,
    pub blocking: Option<CardInstanceId>,
    pub flying: bool,
    pub can_attack: bool,
    pub entered_this_turn: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StackObservation {
    pub id: StackObjectId,
    pub kind: StackObjectKind,
    pub card: CardInstanceId,
    pub definition: CardDefinitionId,
    pub controller: PlayerId,
    pub targets: Vec<Target>,
    pub chosen_permanents: Vec<CardInstanceId>,
    pub x: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayerObservation {
    pub viewer: PlayerId,
    pub turn: u32,
    /// The number of turns the active player has started, including extras.
    pub active_turn: u32,
    pub active_player: PlayerId,
    pub priority: PlayerId,
    pub step: Step,
    pub life_totals: [i16; 2],
    pub mana_pools: [ManaPool; 2],
    pub hand: Vec<(CardInstanceId, CardDefinitionId)>,
    pub opponent_hand_size: usize,
    pub last_seen_hand: Option<(PlayerId, Vec<(CardInstanceId, CardDefinitionId)>)>,
    pub library_sizes: [usize; 2],
    pub graveyards: [Vec<(CardInstanceId, CardDefinitionId)>; 2],
    pub exiles: [Vec<(CardInstanceId, CardDefinitionId)>; 2],
    pub battlefield: Vec<PermanentObservation>,
    pub stack: Vec<StackObservation>,
    pub decision: Option<DecisionObservation>,
    pub result: Option<GameResult>,
    pub legal_actions: Vec<Action>,
}
