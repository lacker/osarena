use std::error::Error;
use std::fmt;

use crate::casting::CastChoices;
use crate::{GameObjectId, PlayOptionId, PlayerId};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ManaColor {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Target {
    Player(PlayerId),
    Permanent(GameObjectId),
    Spell(GameObjectId),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CombatDamageAssignment {
    pub recipient: Target,
    pub amount: u16,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Action {
    KeepHand,
    TakeMulligan,
    BottomCards {
        cards: Vec<GameObjectId>,
    },
    DiscardCards {
        cards: Vec<GameObjectId>,
    },
    ChooseDecision {
        decision: u32,
        options: Vec<u32>,
    },
    CancelDecision {
        decision: u32,
    },
    ChooseUntap {
        permanents: Vec<GameObjectId>,
    },
    PassPriority,
    PlayLand {
        card: GameObjectId,
        option: PlayOptionId,
    },
    ActivateManaAbility {
        source: GameObjectId,
        color: ManaColor,
    },
    PayLifeForMana,
    CastSpell {
        card: GameObjectId,
        choices: CastChoices,
        sacrifices: Vec<GameObjectId>,
    },
    ActivateAbility {
        source: GameObjectId,
        target: Option<Target>,
        sacrifice: Option<GameObjectId>,
    },
    DeclareAttacker {
        attacker: GameObjectId,
    },
    FinishDeclaringAttackers,
    DeclareBlocker {
        blocker: GameObjectId,
        attacker: GameObjectId,
    },
    FinishDeclaringBlockers,
    AssignCombatDamage {
        attacker: GameObjectId,
        assignments: Vec<CombatDamageAssignment>,
    },
    Concede,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActionError {
    GameAlreadyFinished,
    NotLegal { player: PlayerId, action: Action },
}

impl fmt::Display for ActionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GameAlreadyFinished => formatter.write_str("the game is already finished"),
            Self::NotLegal { player, action } => {
                write!(formatter, "{action:?} is not legal for {player}")
            }
        }
    }
}

impl Error for ActionError {}
