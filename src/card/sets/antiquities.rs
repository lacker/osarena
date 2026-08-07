use super::CardRecord;
use crate::card::{CardBehavior, CardKind, CardRules, CardSet, ManaCost, cards};

pub(super) static ATOG: CardRecord = CardRecord::new(
    cards::ATOG,
    "Atog",
    CardSet::Antiquities,
    false,
    CardBehavior::Atog,
    CardRules::new(
        CardKind::Creature,
        ManaCost::new(1, 1),
        "Sacrifice an artifact: Atog gets +2/+2 until end of turn.",
    )
    .creature(1, 2),
);

pub(super) static DETONATE: CardRecord = CardRecord::new(
    cards::DETONATE,
    "Detonate",
    CardSet::Antiquities,
    false,
    CardBehavior::Detonate,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::with_x(1),
        "Destroy target artifact with mana value X. Its controller takes X damage.",
    ),
);

pub(super) static SU_CHI: CardRecord = CardRecord::new(
    cards::SU_CHI,
    "Su-Chi",
    CardSet::Antiquities,
    false,
    CardBehavior::SuChi,
    CardRules::new(
        CardKind::ArtifactCreature,
        ManaCost::new(4, 0),
        "When Su-Chi dies, add 4.",
    )
    .creature(4, 4),
);

pub(super) static MISHRA_S_FACTORY: CardRecord = CardRecord::new(
    cards::MISHRA_S_FACTORY,
    "Mishra's Factory",
    CardSet::Antiquities,
    false,
    CardBehavior::MishrasFactory,
    CardRules::new(
        CardKind::Land,
        ManaCost::new(0, 0),
        "Tap: Add 1. 1: Becomes a 2/2 Assembly-Worker artifact creature until end of turn. Tap: Target Assembly-Worker gets +1/+1 until end of turn.",
    )
    .activated("Give {} +1/+1 with Mishra's Factory", "Give an Assembly-Worker +1/+1"),
);

pub(super) static ORCISH_MECHANICS: CardRecord = CardRecord::new(
    cards::ORCISH_MECHANICS,
    "Orcish Mechanics",
    CardSet::Antiquities,
    false,
    CardBehavior::OrcishMechanics,
    CardRules::new(
        CardKind::Creature,
        ManaCost::new(1, 1),
        "Tap, sacrifice an artifact: Deal 2 damage to any target.",
    )
    .creature(1, 1)
    .activated("Deal 2 damage to {} with Orcish Mechanics", "Deal 2 damage"),
);

pub(super) static STRIP_MINE: CardRecord = CardRecord::new(
    cards::STRIP_MINE,
    "Strip Mine",
    CardSet::Antiquities,
    false,
    CardBehavior::StripMine,
    CardRules::new(
        CardKind::Land,
        ManaCost::new(0, 0),
        "Tap, sacrifice Strip Mine: Destroy target land.",
    )
    .activated("Destroy {} with Strip Mine", "Destroy a land"),
);

pub(super) static TRISKELION: CardRecord = CardRecord::new(
    cards::TRISKELION,
    "Triskelion",
    CardSet::Antiquities,
    false,
    CardBehavior::Triskelion,
    CardRules::new(
        CardKind::ArtifactCreature,
        ManaCost::new(6, 0),
        "Enters with three +1/+1 counters. Remove a +1/+1 counter: Deal 1 damage to any target.",
    )
    .creature(1, 1)
    .activated("Deal 1 damage to {} with Triskelion", "Deal 1 damage"),
);

pub(super) static IVORY_TOWER: CardRecord = CardRecord::new(
    cards::IVORY_TOWER,
    "Ivory Tower",
    CardSet::Antiquities,
    false,
    CardBehavior::IvoryTower,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(1, 0),
        "At the beginning of your upkeep, gain 1 life for each card in your hand beyond four.",
    ),
);

pub(super) static MISHRA_S_WORKSHOP: CardRecord = CardRecord::new(
    cards::MISHRA_S_WORKSHOP,
    "Mishra's Workshop",
    CardSet::Antiquities,
    false,
    CardBehavior::MishrasWorkshop,
    CardRules::new(
        CardKind::Land,
        ManaCost::new(0, 0),
        "Tap: Add 3. Spend this mana only to cast artifact spells.",
    ),
);

pub(super) static ARGOTHIAN_PIXIES: CardRecord = CardRecord::new(
    cards::ARGOTHIAN_PIXIES,
    "Argothian Pixies",
    CardSet::Antiquities,
    false,
    CardBehavior::ArgothianPixies,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 0, 0, 0, 0, 1),
        "Argothian Pixies can't be blocked by artifact creatures.",
    )
    .creature(2, 1),
);

pub(super) static HURKYLS_RECALL: CardRecord = CardRecord::new(
    cards::HURKYLS_RECALL,
    "Hurkyl's Recall",
    CardSet::Antiquities,
    false,
    CardBehavior::HurkylsRecall,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 0, 1, 0, 0, 0),
        "Return all artifacts target player controls to their owner's hand.",
    ),
);

pub(super) static SAGE_OF_LAT_NAM: CardRecord = CardRecord::new(
    cards::SAGE_OF_LAT_NAM,
    "Sage of Lat-Nam",
    CardSet::Antiquities,
    false,
    CardBehavior::SageOfLatNam,
    CardRules::new(
        CardKind::ArtifactCreature,
        ManaCost::colored(1, 0, 1, 0, 0, 0),
        "Tap, sacrifice an artifact: Draw a card.",
    )
    .creature(1, 1),
);

pub(super) static TETRAVUS: CardRecord = CardRecord::new(
    cards::TETRAVUS,
    "Tetravus",
    CardSet::Antiquities,
    false,
    CardBehavior::Tetravus,
    CardRules::new(
        CardKind::ArtifactCreature,
        ManaCost::new(6, 0),
        "Flying. Tetravus enters with three +1/+1 counters on it.",
    )
    .creature(1, 1)
    .flying(),
);

pub(super) static CARDS: &[&CardRecord] = &[
    &ATOG,
    &DETONATE,
    &SU_CHI,
    &MISHRA_S_FACTORY,
    &ORCISH_MECHANICS,
    &STRIP_MINE,
    &TRISKELION,
    &IVORY_TOWER,
    &MISHRA_S_WORKSHOP,
    &ARGOTHIAN_PIXIES,
    &HURKYLS_RECALL,
    &SAGE_OF_LAT_NAM,
    &TETRAVUS,
];
