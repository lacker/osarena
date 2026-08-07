use crate::{CardDefinitionId, CardInstanceId, PlayerId, Target};

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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameEvent {
    GameStarted {
        seed: u64,
    },
    CardDrawn {
        player: PlayerId,
        card: CardInstanceId,
    },
    CardsDiscarded {
        player: PlayerId,
        cards: Vec<(CardInstanceId, CardDefinitionId)>,
    },
    LandPlayed {
        player: PlayerId,
        card: CardInstanceId,
    },
    ManaAdded {
        player: PlayerId,
        source: CardInstanceId,
    },
    SpellCast {
        player: PlayerId,
        card: CardInstanceId,
        targets: Vec<Target>,
    },
    SpellResolved {
        card: CardInstanceId,
    },
    /// A targeted spell resolved with every target gone, so it did nothing.
    SpellFizzled {
        card: CardInstanceId,
    },
    AbilityActivated {
        player: PlayerId,
        source: CardInstanceId,
        chosen_permanents: Vec<CardInstanceId>,
    },
    AbilityResolved {
        source: CardInstanceId,
    },
    AttackDeclared {
        player: PlayerId,
        attackers: Vec<CardInstanceId>,
    },
    BlockDeclared {
        player: PlayerId,
        assignments: Vec<(CardInstanceId, CardInstanceId)>,
    },
    ErhnamForestwalkGranted {
        player: PlayerId,
        source: CardInstanceId,
        target: CardInstanceId,
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
        card: CardInstanceId,
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
