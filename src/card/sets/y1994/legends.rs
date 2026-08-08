use super::{CardRecord, PrintingRecord};
use crate::card::{
    CardArt, CardBehavior, CardKind, CardRules, CardSet, ImplementationStatus, ManaCost, cards,
};

pub(in crate::card::sets) static CHAIN_LIGHTNING: CardRecord = CardRecord::new(
    cards::CHAIN_LIGHTNING,
    "Chain Lightning",
    CardArt::new("b5883762-ca0a-4932-8d2a-41a45796a5f8", "Sandra Everingham"),
    CardSet::Legends,
    false,
    CardBehavior::ChainLightning,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::new(0, 1),
        "Deal 3 damage to any target. That target's controller may pay RR to copy it and choose a new target.",
    ),
);

pub(in crate::card::sets) static DIVINE_OFFERING: CardRecord = CardRecord::new(
    cards::DIVINE_OFFERING,
    "Divine Offering",
    CardArt::new("9c78c2f3-2f40-48ad-9dc4-55d1fa399a56", "Jeff A. Menges"),
    CardSet::Legends,
    false,
    CardBehavior::DivineOffering,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 1, 0, 0, 0, 0),
        "Destroy target artifact. You gain life equal to its mana value.",
    ),
);

pub(in crate::card::sets) static MANA_DRAIN: CardRecord = CardRecord::new(
    cards::MANA_DRAIN,
    "Mana Drain",
    CardArt::new("e691adef-3027-4e6a-889f-9f4e2df36a7c", "Mark Tedin"),
    CardSet::Legends,
    false,
    CardBehavior::ManaDrain,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 2, 0, 0, 0),
        "Counter target spell. At your next main phase, add colorless mana equal to its mana value.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The delayed mana trigger is stored as a scalar and never becomes a stack object.",
    });

pub(in crate::card::sets) static RECALL: CardRecord = CardRecord::new(
    cards::RECALL,
    "Recall",
    CardArt::new("33296718-0625-4422-a65c-b21cf99c52ec", "Brian Snõddy"),
    CardSet::Legends,
    false,
    CardBehavior::Recall,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::variable(0, 0, 1, 0, 0, 0, 2),
        "Discard X cards, then return X cards from your graveyard to your hand. Exile Recall.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The engine incorrectly requires and discards X cards as an additional casting cost instead of discarding during resolution.",
});

pub(in crate::card::sets) static SYLVAN_LIBRARY: CardRecord = CardRecord::new(
    cards::SYLVAN_LIBRARY,
    "Sylvan Library",
    CardArt::new("f486df00-7c4a-4ff0-bb0b-c8b5432ac742", "Harold McNeill"),
    CardSet::Legends,
    false,
    CardBehavior::SylvanLibrary,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(1, 0, 0, 0, 0, 1),
        "At your draw step, draw two additional cards, then put two cards drawn this turn back unless you pay 4 life for each.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The draw-step trigger and its choices currently bypass the stack.",
    });

pub(in crate::card::sets) static THUNDER_SPIRIT: CardRecord = CardRecord::new(
    cards::THUNDER_SPIRIT,
    "Thunder Spirit",
    CardArt::new(
        "61a59775-b1cd-4ed0-8abf-c2b37f7be0d5",
        "Randy Asplund-Faith",
    ),
    CardSet::Legends,
    false,
    CardBehavior::ThunderSpirit,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 2, 0, 0, 0, 0),
        "Flying, first strike.",
    )
    .creature(2, 2)
    .flying(),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "First strike is not implemented.",
});

pub(in crate::card::sets) static WHIRLING_DERVISH: CardRecord = CardRecord::new(
    cards::WHIRLING_DERVISH,
    "Whirling Dervish",
    CardArt::new("eba294e7-7097-4bc3-b396-72e85dd4f441", "Susan Van Camp"),
    CardSet::Legends,
    false,
    CardBehavior::WhirlingDervish,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 0, 0, 0, 0, 2),
        "Protection from black. At each end step, if it damaged an opponent this turn, put a +1/+1 counter on it.",
    )
    .creature(2, 2),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The end-step trigger currently resolves outside the stack.",
    });

pub(in crate::card::sets) static MOAT: CardRecord = CardRecord::new(
    cards::MOAT,
    "Moat",
    CardArt::new("952ba126-0915-47f0-9b6a-a0a6dcd22c6f", "Jeff A. Menges"),
    CardSet::Legends,
    false,
    CardBehavior::Moat,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(2, 2, 0, 0, 0, 0),
        "Creatures without flying can't attack.",
    ),
);

pub(in crate::card::sets) static PENDELHAVEN: CardRecord = CardRecord::new(
    cards::PENDELHAVEN,
    "Pendelhaven",
    CardArt::new("79427109-c1f3-476d-a029-0049217237b5", "Bryon Wackwitz"),
    CardSet::Legends,
    false,
    CardBehavior::Pendelhaven,
    CardRules::new(
        CardKind::Land,
        ManaCost::new(0, 0),
        "Tap: Add G. Tap: Target 1/1 creature gets +1/+2 until end of turn.",
    )
    .legendary()
    .activated(
        "Give {} +1/+2 with Pendelhaven",
        "Give a 1/1 creature +1/+2",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The 1/1 target restriction is checked on activation but is not rechecked when the ability resolves.",
});

pub(in crate::card::sets) static RELIC_BARRIER: CardRecord = CardRecord::new(
    cards::RELIC_BARRIER,
    "Relic Barrier",
    CardArt::new("c062cbae-ce5e-43be-9932-c81a0a3622e8", "Harold McNeill"),
    CardSet::Legends,
    false,
    CardBehavior::RelicBarrier,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(2, 0),
        "Tap: Tap target artifact.",
    )
    .activated("Tap {} with Relic Barrier", "Tap an artifact"),
);

pub(in crate::card::sets) static THE_ABYSS: CardRecord = CardRecord::new(
    cards::THE_ABYSS,
    "The Abyss",
    CardArt::new("86a27d68-3e58-4ade-976d-36381beed451", "Pete Venters"),
    CardSet::Legends,
    false,
    CardBehavior::TheAbyss,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(3, 0, 0, 1, 0, 0),
        "At the beginning of each upkeep, destroy target nonartifact creature.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The target is selected automatically and the upkeep trigger never becomes a stack object.",
    });

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CHAIN_LIGHTNING,
    &DIVINE_OFFERING,
    &MANA_DRAIN,
    &RECALL,
    &SYLVAN_LIBRARY,
    &THUNDER_SPIRIT,
    &WHIRLING_DERVISH,
    &MOAT,
    &PENDELHAVEN,
    &RELIC_BARRIER,
    &THE_ABYSS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
