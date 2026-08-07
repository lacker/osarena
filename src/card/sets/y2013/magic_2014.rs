//! Magic 2014 card records used by the built-in ISD–RTR Standard decks.

use super::{CardRecord, PrintingRecord};
use crate::card::{CardBehavior, CardKind, CardRules, CardSet, LandEntry, ManaCost, cards};

// Implementation status: Baseline creature casting/combat and declaratively modeled traits are active; remaining printed abilities are pending.
pub(in crate::card::sets) static ARCHANGEL_OF_THUNE: CardRecord = CardRecord::new_with_art(
    cards::ARCHANGEL_OF_THUNE,
    "Archangel of Thune",
    "531cba81-afd7-4be4-adec-87edb77ba2a9",
    "James Ryman",
    CardSet::Magic2014,
    false,
    CardBehavior::ArchangelOfThune,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(3, 2, 0, 0, 0, 0),
        "Flying\nLifelink (Damage dealt by this creature also causes you to gain that much life.)\nWhenever you gain life, put a +1/+1 counter on each creature you control.",
    )
    .type_line("Creature — Angel")
    .creature(3, 4)
    .flying()
    .lifelink()
    .metadata_only(),
);

// Implementation status: Metadata only; this spell is withheld from legal actions.
pub(in crate::card::sets) static BURNING_EARTH: CardRecord = CardRecord::new_with_art(
    cards::BURNING_EARTH,
    "Burning Earth",
    "1df3a7c9-5c8d-438c-a5ad-3c9754c6ea5d",
    "rk post",
    CardSet::Magic2014,
    false,
    CardBehavior::BurningEarth,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(3, 0, 0, 0, 1, 0),
        "Whenever a player taps a nonbasic land for mana, this enchantment deals 1 damage to that player.",
    )
    .type_line("Enchantment")
    .metadata_only(),
);

// Implementation status: Metadata only; this spell is withheld from legal actions.
pub(in crate::card::sets) static CELESTIAL_FLARE: CardRecord = CardRecord::new_with_art(
    cards::CELESTIAL_FLARE,
    "Celestial Flare",
    "6c8d1320-0f1a-4c66-86c9-9f8da0f1d9ef",
    "Clint Cearley",
    CardSet::Magic2014,
    false,
    CardBehavior::CelestialFlare,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 2, 0, 0, 0, 0),
        "Target player sacrifices an attacking or blocking creature of their choice.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Metadata only; this spell is withheld from legal actions.
pub(in crate::card::sets) static DOOM_BLADE: CardRecord = CardRecord::new_with_art(
    cards::DOOM_BLADE,
    "Doom Blade",
    "75d96a37-bdbe-46ae-926f-8742699a0b20",
    "Chippy",
    CardSet::Magic2014,
    false,
    CardBehavior::DoomBlade,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 0, 0, 1, 0, 0),
        "Destroy target nonblack creature.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Baseline creature casting/combat and declaratively modeled traits are active; remaining printed abilities are pending.
pub(in crate::card::sets) static ELVISH_MYSTIC: CardRecord = CardRecord::new_with_art(
    cards::ELVISH_MYSTIC,
    "Elvish Mystic",
    "60d0e6a6-629a-45a7-bfcb-25ba7156788b",
    "Wesley Burt",
    CardSet::Magic2014,
    false,
    CardBehavior::ElvishMystic,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 0, 0, 0, 0, 1),
        "{T}: Add {G}.",
    )
    .type_line("Creature — Elf Druid")
    .creature(1, 1)
    .produces([false, false, false, false, true, false])
    .metadata_only(),
);

// Implementation status: Land entry and mana production are active; other printed abilities are pending.
pub(in crate::card::sets) static ENCROACHING_WASTES: CardRecord = CardRecord::new_with_art(
    cards::ENCROACHING_WASTES,
    "Encroaching Wastes",
    "1ad5a84b-ae9b-4ed1-a4de-b91bbf8ed0a5",
    "Noah Bradley",
    CardSet::Magic2014,
    false,
    CardBehavior::EncroachingWastes,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "{T}: Add {C}.\n{4}, {T}, Sacrifice this land: Destroy target nonbasic land.",
    )
    .type_line("Land")
    .produces([false, false, false, false, false, true])
    .land_entry(LandEntry::Untapped)
    .metadata_only(),
);

// Implementation status: Baseline creature casting/combat and declaratively modeled traits are active; remaining printed abilities are pending.
pub(in crate::card::sets) static LIFEBANE_ZOMBIE: CardRecord = CardRecord::new_with_art(
    cards::LIFEBANE_ZOMBIE,
    "Lifebane Zombie",
    "98370735-5303-40d4-9e80-cdb40dee18e2",
    "Min Yum",
    CardSet::Magic2014,
    false,
    CardBehavior::LifebaneZombie,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 0, 0, 2, 0, 0),
        "Intimidate (This creature can't be blocked except by artifact creatures and/or creatures that share a color with it.)\nWhen this creature enters, target opponent reveals their hand. You choose a green or white creature card from it and exile that card.",
    )
    .type_line("Creature — Zombie Warrior")
    .creature(3, 1)
    .intimidate()
    .metadata_only(),
);

// Implementation status: Land entry and mana production are active; other printed abilities are pending.
pub(in crate::card::sets) static MUTAVAULT: CardRecord = CardRecord::new_with_art(
    cards::MUTAVAULT,
    "Mutavault",
    "927ed667-c228-4b96-a9f6-7cbadade8134",
    "Fred Fields",
    CardSet::Magic2014,
    false,
    CardBehavior::Mutavault,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "{T}: Add {C}.\n{1}: This land becomes a 2/2 creature with all creature types until end of turn. It's still a land.",
    )
    .type_line("Land")
    .produces([false, false, false, false, false, true])
    .land_entry(LandEntry::Untapped)
    .metadata_only(),
);

// Implementation status: Metadata only; this spell is withheld from legal actions.
pub(in crate::card::sets) static PRIMEVAL_BOUNTY: CardRecord = CardRecord::new_with_art(
    cards::PRIMEVAL_BOUNTY,
    "Primeval Bounty",
    "e750d55d-d5e8-4abe-99cf-f6b8ba86cf16",
    "Christine Choi",
    CardSet::Magic2014,
    false,
    CardBehavior::PrimevalBounty,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(5, 0, 0, 0, 0, 1),
        "Whenever you cast a creature spell, create a 3/3 green Beast creature token.\nWhenever you cast a noncreature spell, put three +1/+1 counters on target creature you control.\nLandfall — Whenever a land you control enters, you gain 3 life.",
    )
    .type_line("Enchantment")
    .metadata_only(),
);

// Implementation status: Metadata only; this spell is withheld from legal actions.
pub(in crate::card::sets) static QUICKEN: CardRecord = CardRecord::new_with_art(
    cards::QUICKEN,
    "Quicken",
    "066bef3d-c785-4b25-9b91-8f676aa9906f",
    "Aleksi Briclot",
    CardSet::Magic2014,
    false,
    CardBehavior::Quicken,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 1, 0, 0, 0),
        "The next sorcery spell you cast this turn can be cast as though it had flash. (It can be cast any time you could cast an instant.)\nDraw a card.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Metadata only; this spell is withheld from legal actions.
pub(in crate::card::sets) static RATCHET_BOMB: CardRecord = CardRecord::new_with_art(
    cards::RATCHET_BOMB,
    "Ratchet Bomb",
    "3e9045df-3eff-4236-9bbb-77537b302e27",
    "Austin Hsu",
    CardSet::Magic2014,
    false,
    CardBehavior::RatchetBomb,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::colored(2, 0, 0, 0, 0, 0),
        "{T}: Put a charge counter on this artifact.\n{T}, Sacrifice this artifact: Destroy each nonland permanent with mana value equal to the number of charge counters on this artifact.",
    )
    .type_line("Artifact")
    .metadata_only(),
);

// Implementation status: Baseline creature casting/combat and declaratively modeled traits are active; remaining printed abilities are pending.
pub(in crate::card::sets) static SCAVENGING_OOZE: CardRecord = CardRecord::new_with_art(
    cards::SCAVENGING_OOZE,
    "Scavenging Ooze",
    "ec30153a-36b5-42f8-beed-9efab09f1051",
    "Austin Hsu",
    CardSet::Magic2014,
    false,
    CardBehavior::ScavengingOoze,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 0, 0, 0, 0, 1),
        "{G}: Exile target card from a graveyard. If it was a creature card, put a +1/+1 counter on this creature and you gain 1 life.",
    )
    .type_line("Creature — Ooze")
    .creature(2, 2)
    .metadata_only(),
);

// Implementation status: Baseline creature casting/combat and declaratively modeled traits are active; remaining printed abilities are pending.
pub(in crate::card::sets) static SHADOWBORN_DEMON: CardRecord = CardRecord::new_with_art(
    cards::SHADOWBORN_DEMON,
    "Shadowborn Demon",
    "3884c05b-c10e-4f1d-a8bd-8b5118657972",
    "Lucas Graciano",
    CardSet::Magic2014,
    false,
    CardBehavior::ShadowbornDemon,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(3, 0, 0, 2, 0, 0),
        "Flying\nWhen this creature enters, destroy target non-Demon creature.\nAt the beginning of your upkeep, if there are fewer than six creature cards in your graveyard, sacrifice a creature.",
    )
    .type_line("Creature — Demon")
    .creature(5, 6)
    .flying()
    .metadata_only(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARCHANGEL_OF_THUNE,
    &BURNING_EARTH,
    &CELESTIAL_FLARE,
    &DOOM_BLADE,
    &ELVISH_MYSTIC,
    &ENCROACHING_WASTES,
    &LIFEBANE_ZOMBIE,
    &MUTAVAULT,
    &PRIMEVAL_BOUNTY,
    &QUICKEN,
    &RATCHET_BOMB,
    &SCAVENGING_OOZE,
    &SHADOWBORN_DEMON,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
