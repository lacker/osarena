use crate::{
    AbilityId, Action, CardDefinitionId, CardPartId, CastSignature, GameObjectId, PlayerId, Target,
};

use super::{DecisionObservation, GameResult, ManaPool, StackObjectKind, Step};

pub(super) type PublicCard = (GameObjectId, CardDefinitionId);
pub(super) type LastSeenHand = Option<(PlayerId, Vec<PublicCard>)>;

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct PermanentObservation {
    pub id: GameObjectId,
    pub definition: CardDefinitionId,
    /// The logical card part currently supplying this permanent's printed
    /// characteristics. Changing faces does not change `id`.
    pub presented: CardPartId,
    pub controller: PlayerId,
    pub tapped: bool,
    pub power: Option<i16>,
    pub toughness: Option<i16>,
    pub damage: u16,
    pub attacking: bool,
    pub blocking: Option<GameObjectId>,
    pub flying: bool,
    pub can_attack: bool,
    pub entered_this_turn: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StackObservation {
    pub id: GameObjectId,
    pub kind: StackObjectKind,
    pub source: Option<GameObjectId>,
    /// The printed or granted ability that created this stack object. Spells
    /// and legacy activated abilities that have not yet migrated omit it.
    pub ability: Option<AbilityId>,
    /// Frozen rules text for the creating ability. This remains inspectable
    /// even when its source changes zones or characteristics.
    pub ability_text: Option<String>,
    pub definition: CardDefinitionId,
    pub controller: PlayerId,
    /// Locked spell form, modes, costs, X, and target slots. Activated
    /// abilities have no cast signature.
    pub signature: Option<CastSignature>,
    pub targets: Vec<Target>,
    pub chosen_permanents: Vec<GameObjectId>,
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
    pub hand: Vec<(GameObjectId, CardDefinitionId)>,
    pub opponent_hand_size: usize,
    pub last_seen_hand: Option<(PlayerId, Vec<(GameObjectId, CardDefinitionId)>)>,
    pub library_sizes: [usize; 2],
    pub graveyards: [Vec<(GameObjectId, CardDefinitionId)>; 2],
    pub exiles: [Vec<(GameObjectId, CardDefinitionId)>; 2],
    pub battlefield: Vec<PermanentObservation>,
    pub stack: Vec<StackObservation>,
    pub decision: Option<DecisionObservation>,
    pub result: Option<GameResult>,
    pub legal_actions: Vec<Action>,
}
