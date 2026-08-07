use super::{CardRecord, PrintingRecord};
use crate::card::{CardBehavior, CardKind, CardRules, CardSet, ManaCost, cards};

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static ATOG: CardRecord = CardRecord::new_with_art(
    cards::ATOG,
    "Atog",
    "2249fc40-4412-48fd-800a-7ea3678aee3f",
    "Jesper Myrfors",
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

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static DETONATE: CardRecord = CardRecord::new_with_art(
    cards::DETONATE,
    "Detonate",
    "ffd7eb90-ae95-49df-898a-9510187bce1c",
    "Randy Asplund-Faith",
    CardSet::Antiquities,
    false,
    CardBehavior::Detonate,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::with_x(1),
        "Destroy target artifact with mana value X. Its controller takes X damage.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static SU_CHI: CardRecord = CardRecord::new_with_art(
    cards::SU_CHI,
    "Su-Chi",
    "a64d4f93-0c04-4078-aec0-7e9de92f260f",
    "Christopher Rush",
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

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static MISHRA_S_FACTORY: CardRecord = CardRecord::new_with_art(
    cards::MISHRA_S_FACTORY,
    "Mishra's Factory",
    "a696c5b6-f216-454d-8029-74e84bbd1428",
    "Kaja Foglio & Phil Foglio",
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

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static ORCISH_MECHANICS: CardRecord = CardRecord::new_with_art(
    cards::ORCISH_MECHANICS,
    "Orcish Mechanics",
    "5e34fc6b-5f00-4a22-9ee2-afc1caf99961",
    "Pete Venters",
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

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static STRIP_MINE: CardRecord = CardRecord::new_with_art(
    cards::STRIP_MINE,
    "Strip Mine",
    "e7880157-7f27-4f1b-9cdc-ab36a6252376",
    "Daniel Gelon",
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

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static TRISKELION: CardRecord = CardRecord::new_with_art(
    cards::TRISKELION,
    "Triskelion",
    "a79c99e1-722a-44b6-8fa3-2be3f0c193d8",
    "Douglas Shuler",
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

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static IVORY_TOWER: CardRecord = CardRecord::new_with_art(
    cards::IVORY_TOWER,
    "Ivory Tower",
    "a5f23039-45ca-4c15-af50-bfd40ea26453",
    "Margaret Organ-Kean",
    CardSet::Antiquities,
    false,
    CardBehavior::IvoryTower,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(1, 0),
        "At the beginning of your upkeep, gain 1 life for each card in your hand beyond four.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static MISHRA_S_WORKSHOP: CardRecord = CardRecord::new_with_art(
    cards::MISHRA_S_WORKSHOP,
    "Mishra's Workshop",
    "135de5c7-6ac9-4b68-8f1a-97f120a4b125",
    "Kaja Foglio",
    CardSet::Antiquities,
    false,
    CardBehavior::MishrasWorkshop,
    CardRules::new(
        CardKind::Land,
        ManaCost::new(0, 0),
        "Tap: Add 3. Spend this mana only to cast artifact spells.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static ARGOTHIAN_PIXIES: CardRecord = CardRecord::new_with_art(
    cards::ARGOTHIAN_PIXIES,
    "Argothian Pixies",
    "5712e87a-2381-4f5b-a853-6973841f9bf1",
    "Amy Weber",
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

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static HURKYLS_RECALL: CardRecord = CardRecord::new_with_art(
    cards::HURKYLS_RECALL,
    "Hurkyl's Recall",
    "f32373dd-06d8-45d1-8777-3b1411bcb30a",
    "NéNé Thomas",
    CardSet::Antiquities,
    false,
    CardBehavior::HurkylsRecall,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 0, 1, 0, 0, 0),
        "Return all artifacts target player controls to their owner's hand.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static SAGE_OF_LAT_NAM: CardRecord = CardRecord::new_with_art(
    cards::SAGE_OF_LAT_NAM,
    "Sage of Lat-Nam",
    "b4ff60ce-073c-46b8-807c-8b40467b960c",
    "Pete Venters",
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

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static TETRAVUS: CardRecord = CardRecord::new_with_art(
    cards::TETRAVUS,
    "Tetravus",
    "23eb19f9-2e8f-4bf0-9bf8-868e6da70e2d",
    "Mark Tedin",
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

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static ENERGY_FLUX: CardRecord = CardRecord::new_with_art(
    cards::ENERGY_FLUX,
    "Energy Flux",
    "bd1f624b-e8f2-462f-838a-7cb9e8fda988",
    "Kaja Foglio",
    CardSet::Antiquities,
    false,
    CardBehavior::EnergyFlux,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(2, 0, 1, 0, 0, 0),
        "At the beginning of each player's upkeep, sacrifice each artifact unless you pay 2 for it.",
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
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
    &ENERGY_FLUX,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
