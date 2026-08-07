use super::{CardRecord, PrintingRecord};
use crate::card::{CardBehavior, CardKind, CardRules, CardSet, ManaCost, cards};

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static CITY_OF_BRASS: CardRecord = CardRecord::new_with_art(
    cards::CITY_OF_BRASS,
    "City of Brass",
    "f4e32327-380d-471e-813b-4c27477787ce",
    "Mark Tedin",
    CardSet::ArabianNights,
    false,
    CardBehavior::CityOfBrass,
    CardRules::new(
        CardKind::Land,
        ManaCost::new(0, 0),
        "Whenever City of Brass becomes tapped, it deals 1 damage to you. Tap: Add one mana of any color.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static ERHNAM_DJINN: CardRecord = CardRecord::new_with_art(
    cards::ERHNAM_DJINN,
    "Erhnam Djinn",
    "42bc0c3f-0a52-4bdc-83da-6484bf3102f3",
    "Ken Meyer, Jr.",
    CardSet::ArabianNights,
    false,
    CardBehavior::ErhnamDjinn,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(3, 0, 0, 0, 0, 1),
        "At your upkeep, target opponent's creature gains forestwalk until your next upkeep.",
    )
    .creature(4, 5),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static JUZAM_DJINN: CardRecord = CardRecord::new_with_art(
    cards::JUZAM_DJINN,
    "Juzam Djinn",
    "31bf3f14-b5df-498b-a1bb-965885c82401",
    "Mark Tedin",
    CardSet::ArabianNights,
    false,
    CardBehavior::JuzamDjinn,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(2, 0, 0, 2, 0, 0),
        "At your upkeep, Juzam Djinn deals 1 damage to you.",
    )
    .creature(5, 5),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static LIBRARY_OF_ALEXANDRIA: CardRecord = CardRecord::new_with_art(
    cards::LIBRARY_OF_ALEXANDRIA,
    "Library of Alexandria",
    "ee266113-34ce-4189-84e7-ee2c86a2722c",
    "Mark Poole",
    CardSet::ArabianNights,
    false,
    CardBehavior::LibraryOfAlexandria,
    CardRules::new(
        CardKind::Land,
        ManaCost::new(0, 0),
        "Tap: Add 1. Tap: Draw a card. Activate only with exactly seven cards in hand.",
    )
    .legendary(),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static SERENDIB_EFREET: CardRecord = CardRecord::new_with_art(
    cards::SERENDIB_EFREET,
    "Serendib Efreet",
    "cf56e862-3169-4f63-acd0-731080fa32f2",
    "Anson Maddocks",
    CardSet::ArabianNights,
    false,
    CardBehavior::SerendibEfreet,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(2, 0, 1, 0, 0, 0),
        "Flying. At your upkeep, Serendib Efreet deals 1 damage to you.",
    )
    .creature(3, 4)
    .flying(),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static CITY_IN_A_BOTTLE: CardRecord = CardRecord::new_with_art(
    cards::CITY_IN_A_BOTTLE,
    "City in a Bottle",
    "9598b346-a47d-4c4c-9571-156824e86b9c",
    "Drew Tucker",
    CardSet::ArabianNights,
    false,
    CardBehavior::CityInABottle,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(2, 0),
        "At the beginning of each upkeep, destroy each other permanent from Arabian Nights.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static KIRD_APE: CardRecord = CardRecord::new_with_art(
    cards::KIRD_APE,
    "Kird Ape",
    "ebe8845e-df1c-481c-949c-aab84af99a05",
    "Ken Meyer, Jr.",
    CardSet::ArabianNights,
    false,
    CardBehavior::KirdApe,
    CardRules::new(
        CardKind::Creature,
        ManaCost::new(0, 1),
        "Kird Ape gets +1/+2 as long as you control a Forest.",
    )
    .creature(1, 1),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CITY_OF_BRASS,
    &ERHNAM_DJINN,
    &JUZAM_DJINN,
    &LIBRARY_OF_ALEXANDRIA,
    &SERENDIB_EFREET,
    &CITY_IN_A_BOTTLE,
    &KIRD_APE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
