use crate::{CardDefinitionId, GameObjectId, PlayerId, Target};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Step {
    Upkeep,
    Draw,
    PrecombatMain,
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    EndOfCombat,
    PostcombatMain,
    End,
    Cleanup,
}

impl Step {
    pub(super) const fn is_main(self) -> bool {
        matches!(self, Self::PrecombatMain | Self::PostcombatMain)
    }

    pub(super) const fn ends_phase(self) -> bool {
        matches!(
            self,
            Self::Draw
                | Self::PrecombatMain
                | Self::EndOfCombat
                | Self::PostcombatMain
                | Self::Cleanup
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameResult {
    Winner { winner: PlayerId, reason: WinReason },
    Draw,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WinReason {
    OpponentConceded,
    OpponentLostAllLife,
    OpponentTriedToDrawFromEmptyLibrary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StackObjectKind {
    Spell,
    ActivatedAbility,
    TriggeredAbility,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameEvent {
    GameStarted {
        seed: u64,
    },
    CardDrawn {
        player: PlayerId,
        card: GameObjectId,
    },
    CardsDiscarded {
        player: PlayerId,
        cards: Vec<(GameObjectId, CardDefinitionId)>,
    },
    LandPlayed {
        player: PlayerId,
        card: GameObjectId,
        /// Immutable identity for logs after the permanent leaves play.
        definition: CardDefinitionId,
    },
    ManaAdded {
        player: PlayerId,
        source: GameObjectId,
    },
    SpellCast {
        player: PlayerId,
        /// The spell's game object on the stack.
        card: GameObjectId,
        /// Immutable identity for logs after the spell leaves the stack.
        definition: CardDefinitionId,
        targets: Vec<Target>,
    },
    SpellResolved {
        /// The spell's former game object on the stack.
        card: GameObjectId,
        definition: CardDefinitionId,
    },
    /// A targeted spell resolved with every target gone, so it did nothing.
    SpellFizzled {
        /// The spell's former game object on the stack.
        card: GameObjectId,
        definition: CardDefinitionId,
    },
    AbilityActivated {
        player: PlayerId,
        /// The activated ability's own game object on the stack.
        object: GameObjectId,
        /// The permanent object that created the ability.
        source: GameObjectId,
        /// Immutable source definition for logs after the source leaves play.
        definition: CardDefinitionId,
        chosen_permanents: Vec<GameObjectId>,
    },
    AbilityResolved {
        /// The activated ability's former game object on the stack.
        object: GameObjectId,
        /// The permanent object that created the ability.
        source: GameObjectId,
        definition: CardDefinitionId,
    },
    AbilityTriggered {
        player: PlayerId,
        trigger: u32,
        source: GameObjectId,
        definition: CardDefinitionId,
    },
    TriggeredAbilityPutOnStack {
        player: PlayerId,
        trigger: u32,
        object: GameObjectId,
        source: GameObjectId,
        definition: CardDefinitionId,
    },
    TriggeredAbilityResolved {
        object: GameObjectId,
        source: GameObjectId,
        definition: CardDefinitionId,
    },
    AttackDeclared {
        player: PlayerId,
        attackers: Vec<GameObjectId>,
    },
    BlockDeclared {
        player: PlayerId,
        assignments: Vec<(GameObjectId, GameObjectId)>,
    },
    ErhnamForestwalkGranted {
        player: PlayerId,
        source: GameObjectId,
        target: GameObjectId,
    },
    DamageDealt {
        player: PlayerId,
        amount: u16,
    },
    ManaBurn {
        player: PlayerId,
        amount: u16,
    },
    StepChanged {
        turn: u32,
        active_player: PlayerId,
        step: Step,
    },
    /// A permanent left the battlefield. Emitted from the three functions that
    /// can remove one, so nothing leaves play without the log seeing it. The
    /// definition travels with the event because the card is by then in a zone
    /// the observing player may not be able to read.
    PermanentLeftBattlefield {
        controller: PlayerId,
        card: GameObjectId,
        definition: CardDefinitionId,
        destination: BattlefieldExit,
    },
    GameEnded {
        result: GameResult,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BattlefieldExit {
    Graveyard,
    Exile,
    Hand,
}
