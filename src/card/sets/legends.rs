use super::CardRecord;
use crate::card::{CardBehavior, CardKind, CardRules, CardSet, ManaCost, cards};

pub(super) static CHAIN_LIGHTNING: CardRecord = CardRecord::new(
    cards::CHAIN_LIGHTNING,
    "Chain Lightning",
    CardSet::Legends,
    false,
    CardBehavior::ChainLightning,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::new(0, 1),
        "Deal 3 damage to any target. That target's controller may pay RR to copy it and choose a new target.",
    ),
);

pub(super) static DIVINE_OFFERING: CardRecord = CardRecord::new(
    cards::DIVINE_OFFERING,
    "Divine Offering",
    CardSet::Legends,
    false,
    CardBehavior::DivineOffering,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 1, 0, 0, 0, 0),
        "Destroy target artifact. You gain life equal to its mana value.",
    ),
);

pub(super) static MANA_DRAIN: CardRecord = CardRecord::new(
    cards::MANA_DRAIN,
    "Mana Drain",
    CardSet::Legends,
    false,
    CardBehavior::ManaDrain,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 2, 0, 0, 0),
        "Counter target spell. At your next main phase, add colorless mana equal to its mana value.",
    ),
);

pub(super) static RECALL: CardRecord = CardRecord::new(
    cards::RECALL,
    "Recall",
    CardSet::Legends,
    false,
    CardBehavior::Recall,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::variable(0, 0, 1, 0, 0, 0, 2),
        "Discard X cards, then return X cards from your graveyard to your hand. Exile Recall.",
    ),
);

pub(super) static SYLVAN_LIBRARY: CardRecord = CardRecord::new(
    cards::SYLVAN_LIBRARY,
    "Sylvan Library",
    CardSet::Legends,
    false,
    CardBehavior::SylvanLibrary,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(1, 0, 0, 0, 0, 1),
        "At your draw step, draw two additional cards, then put two cards drawn this turn back unless you pay 4 life for each.",
    ),
);

pub(super) static THUNDER_SPIRIT: CardRecord = CardRecord::new(
    cards::THUNDER_SPIRIT,
    "Thunder Spirit",
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
);

pub(super) static WHIRLING_DERVISH: CardRecord = CardRecord::new(
    cards::WHIRLING_DERVISH,
    "Whirling Dervish",
    CardSet::Legends,
    false,
    CardBehavior::WhirlingDervish,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 0, 0, 0, 0, 2),
        "Protection from black. At each end step, if it damaged an opponent this turn, put a +1/+1 counter on it.",
    )
    .creature(2, 2),
);

pub(super) static ENERGY_FLUX: CardRecord = CardRecord::new(
    cards::ENERGY_FLUX,
    "Energy Flux",
    CardSet::Legends,
    false,
    CardBehavior::EnergyFlux,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(2, 0, 1, 0, 0, 0),
        "At the beginning of each player's upkeep, sacrifice each artifact unless you pay 2 for it.",
    ),
);

pub(super) static MOAT: CardRecord = CardRecord::new(
    cards::MOAT,
    "Moat",
    CardSet::Legends,
    false,
    CardBehavior::Moat,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(2, 2, 0, 0, 0, 0),
        "Creatures without flying can't attack.",
    ),
);

pub(super) static PENDELHAVEN: CardRecord = CardRecord::new(
    cards::PENDELHAVEN,
    "Pendelhaven",
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
);

pub(super) static RELIC_BARRIER: CardRecord = CardRecord::new(
    cards::RELIC_BARRIER,
    "Relic Barrier",
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

pub(super) static SEDGE_TROLL: CardRecord = CardRecord::new(
    cards::SEDGE_TROLL,
    "Sedge Troll",
    CardSet::Legends,
    false,
    CardBehavior::SedgeTroll,
    CardRules::new(
        CardKind::Creature,
        ManaCost::new(2, 1),
        "Sedge Troll gets +1/+1 as long as you control a Swamp. R: Regenerate Sedge Troll.",
    )
    .creature(2, 2),
);

pub(super) static THE_ABYSS: CardRecord = CardRecord::new(
    cards::THE_ABYSS,
    "The Abyss",
    CardSet::Legends,
    false,
    CardBehavior::TheAbyss,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(3, 0, 0, 1, 0, 0),
        "At the beginning of each upkeep, destroy target nonartifact creature.",
    ),
);

pub(super) static CARDS: &[&CardRecord] = &[
    &CHAIN_LIGHTNING,
    &DIVINE_OFFERING,
    &MANA_DRAIN,
    &RECALL,
    &SYLVAN_LIBRARY,
    &THUNDER_SPIRIT,
    &WHIRLING_DERVISH,
    &ENERGY_FLUX,
    &MOAT,
    &PENDELHAVEN,
    &RELIC_BARRIER,
    &SEDGE_TROLL,
    &THE_ABYSS,
];
