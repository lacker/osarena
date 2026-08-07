use super::CardRecord;
use crate::card::{CardBehavior, CardKind, CardRules, CardSet, ManaCost, cards};

pub(super) static CITY_OF_BRASS: CardRecord = CardRecord::new(
    cards::CITY_OF_BRASS,
    "City of Brass",
    CardSet::ArabianNights,
    false,
    CardBehavior::CityOfBrass,
    CardRules::new(
        CardKind::Land,
        ManaCost::new(0, 0),
        "Whenever City of Brass becomes tapped, it deals 1 damage to you. Tap: Add one mana of any color.",
    ),
);

pub(super) static ERHNAM_DJINN: CardRecord = CardRecord::new(
    cards::ERHNAM_DJINN,
    "Erhnam Djinn",
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

pub(super) static JUZAM_DJINN: CardRecord = CardRecord::new(
    cards::JUZAM_DJINN,
    "Juzam Djinn",
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

pub(super) static LIBRARY_OF_ALEXANDRIA: CardRecord = CardRecord::new(
    cards::LIBRARY_OF_ALEXANDRIA,
    "Library of Alexandria",
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

pub(super) static SERENDIB_EFREET: CardRecord = CardRecord::new(
    cards::SERENDIB_EFREET,
    "Serendib Efreet",
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

pub(super) static CITY_IN_A_BOTTLE: CardRecord = CardRecord::new(
    cards::CITY_IN_A_BOTTLE,
    "City in a Bottle",
    CardSet::ArabianNights,
    false,
    CardBehavior::CityInABottle,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(2, 0),
        "At the beginning of each upkeep, destroy each other permanent from Arabian Nights.",
    ),
);

pub(super) static KIRD_APE: CardRecord = CardRecord::new(
    cards::KIRD_APE,
    "Kird Ape",
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

pub(super) static CARDS: &[&CardRecord] = &[
    &CITY_OF_BRASS,
    &ERHNAM_DJINN,
    &JUZAM_DJINN,
    &LIBRARY_OF_ALEXANDRIA,
    &SERENDIB_EFREET,
    &CITY_IN_A_BOTTLE,
    &KIRD_APE,
];
