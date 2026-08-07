use super::{CardRecord, PrintingRecord};
use crate::card::{CardBehavior, CardKind, CardRules, CardSet, ManaCost, cards};

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static GOBLIN_GRENADE: CardRecord = CardRecord::new(
    cards::GOBLIN_GRENADE,
    "Goblin Grenade",
    CardSet::FallenEmpires,
    false,
    CardBehavior::GoblinGrenade,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::new(0, 1),
        "As an additional cost, sacrifice a Goblin. Deal 5 damage to any target.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static HYMN_TO_TOURACH: CardRecord = CardRecord::new(
    cards::HYMN_TO_TOURACH,
    "Hymn to Tourach",
    CardSet::FallenEmpires,
    false,
    CardBehavior::HymnToTourach,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(0, 0, 0, 2, 0, 0),
        "Target player discards two cards at random.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static ICATIAN_JAVELINEERS: CardRecord = CardRecord::new(
    cards::ICATIAN_JAVELINEERS,
    "Icatian Javelineers",
    CardSet::FallenEmpires,
    false,
    CardBehavior::IcatianJavelineers,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 1, 0, 0, 0, 0),
        "Enters with a javelin counter. Tap, remove it: Deal 1 damage to any target.",
    )
    .creature(1, 1)
    .activated(
        "Deal 1 damage to {} with Icatian Javelineers",
        "Deal 1 damage",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static ORDER_OF_LEITBUR: CardRecord = CardRecord::new(
    cards::ORDER_OF_LEITBUR,
    "Order of Leitbur",
    CardSet::FallenEmpires,
    false,
    CardBehavior::OrderOfLeitbur,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 2, 0, 0, 0, 0),
        "Protection from black. WW: Gets +1/+0 until end of turn. W: Gains first strike until end of turn.",
    )
    .creature(2, 2),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static ORDER_OF_THE_EBON_HAND: CardRecord = CardRecord::new(
    cards::ORDER_OF_THE_EBON_HAND,
    "Order of the Ebon Hand",
    CardSet::FallenEmpires,
    false,
    CardBehavior::OrderOfTheEbonHand,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 0, 0, 2, 0, 0),
        "Protection from white. BB: Gets +1/+0 until end of turn. B: Gains first strike until end of turn.",
    )
    .creature(2, 1),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &GOBLIN_GRENADE,
    &HYMN_TO_TOURACH,
    &ICATIAN_JAVELINEERS,
    &ORDER_OF_LEITBUR,
    &ORDER_OF_THE_EBON_HAND,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
