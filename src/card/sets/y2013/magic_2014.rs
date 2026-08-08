//! Magic 2014 card records used by the built-in ISD–RTR Standard decks.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, CardArt, CardKind, CardRules, CardSet, EffectDef,
    EvergreenAbility, LandEntry, ManaCost, ManaKindDef, cards,
};
use crate::ids::AbilityId;

pub(in crate::card::sets) static ARCHANGEL_OF_THUNE: CardRecord = CardRecord::new(
    cards::ARCHANGEL_OF_THUNE,
    "Archangel of Thune",
    CardArt::new("531cba81-afd7-4be4-adec-87edb77ba2a9", "James Ryman"),
    CardSet::Magic2014,
    CardRules::new(CardKind::Creature, ManaCost::colored(3, 2, 0, 0, 0, 0), "")
        .with_subtypes(&["Angel"])
        .creature(3, 4)
        .with_abilities(&[
            AbilityDef::evergreen(AbilityId::PRIMARY, "Flying", EvergreenAbility::Flying),
            AbilityDef::evergreen(
                AbilityId(1),
                "Lifelink (Damage dealt by this creature also causes you to gain that much life.)",
                EvergreenAbility::Lifelink,
            ),
            AbilityDef::not_implemented(
                AbilityId(2),
                "Whenever you gain life, put a +1/+1 counter on each creature you control.",
                "The life-gain trigger is not executed.",
            ),
        ]),
);

pub(in crate::card::sets) static BURNING_EARTH: CardRecord = CardRecord::new(
    cards::BURNING_EARTH,
    "Burning Earth",
    CardArt::new("1df3a7c9-5c8d-438c-a5ad-3c9754c6ea5d", "rk post"),
    CardSet::Magic2014,
        CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(3, 0, 0, 0, 1, 0),
        "Whenever a player taps a nonbasic land for mana, this enchantment deals 1 damage to that player.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static CELESTIAL_FLARE: CardRecord = CardRecord::new(
    cards::CELESTIAL_FLARE,
    "Celestial Flare",
    CardArt::new("6c8d1320-0f1a-4c66-86c9-9f8da0f1d9ef", "Clint Cearley"),
    CardSet::Magic2014,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 2, 0, 0, 0, 0),
        "Target player sacrifices an attacking or blocking creature of their choice.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static DOOM_BLADE: CardRecord = CardRecord::new(
    cards::DOOM_BLADE,
    "Doom Blade",
    CardArt::new("75d96a37-bdbe-46ae-926f-8742699a0b20", "Chippy"),
    CardSet::Magic2014,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 0, 0, 1, 0, 0),
        "Destroy target nonblack creature.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static ELVISH_MYSTIC: CardRecord = CardRecord::new(
    cards::ELVISH_MYSTIC,
    "Elvish Mystic",
    CardArt::new("60d0e6a6-629a-45a7-bfcb-25ba7156788b", "Wesley Burt"),
    CardSet::Magic2014,
    CardRules::new(CardKind::Creature, ManaCost::colored(0, 0, 0, 0, 0, 1), "")
        .with_subtypes(&["Elf", "Druid"])
        .creature(1, 1)
        .with_abilities(&[AbilityDef::activated_mana(
            AbilityId::PRIMARY,
            "{T}: Add {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Green)),
        )]),
);

pub(in crate::card::sets) static ENCROACHING_WASTES: CardRecord = CardRecord::new(
    cards::ENCROACHING_WASTES,
    "Encroaching Wastes",
    CardArt::new("1ad5a84b-ae9b-4ed1-a4de-b91bbf8ed0a5", "Noah Bradley"),
    CardSet::Magic2014,
    CardRules::new(CardKind::Land, ManaCost::colored(0, 0, 0, 0, 0, 0), "")
        .land_entry(LandEntry::Untapped)
        .with_abilities(&[
            AbilityDef::activated_mana(
                AbilityId::PRIMARY,
                "{T}: Add {C}.",
                &[AbilityCostDef::TapSource],
                EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Colorless)),
            ),
            AbilityDef::not_implemented(
                AbilityId(1),
                "{4}, {T}, Sacrifice this land: Destroy target nonbasic land.",
                "The land-destruction activated ability is not executed.",
            ),
        ]),
);

pub(in crate::card::sets) static LIFEBANE_ZOMBIE: CardRecord = CardRecord::new(
    cards::LIFEBANE_ZOMBIE,
    "Lifebane Zombie",
    CardArt::new("98370735-5303-40d4-9e80-cdb40dee18e2", "Min Yum"),
    CardSet::Magic2014,
        CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 0, 0, 2, 0, 0),
        "",
    )
    .with_subtypes(&["Zombie", "Warrior"])
    .creature(3, 1)
    .with_abilities(&[
        AbilityDef::evergreen(
            AbilityId::PRIMARY,
            "Intimidate (This creature can't be blocked except by artifact creatures and/or creatures that share a color with it.)",
            EvergreenAbility::Intimidate,
        ),
        AbilityDef::not_implemented(
            AbilityId(1),
            "When this creature enters, target opponent reveals their hand. You choose a green or white creature card from it and exile that card.",
            "The enters-the-battlefield hand-reveal and exile trigger is not executed.",
        ),
    ]),
);

pub(in crate::card::sets) static MUTAVAULT: CardRecord = CardRecord::new(
    cards::MUTAVAULT,
    "Mutavault",
    CardArt::new("927ed667-c228-4b96-a9f6-7cbadade8134", "Fred Fields"),
    CardSet::Magic2014,
        CardRules::new(CardKind::Land, ManaCost::colored(0, 0, 0, 0, 0, 0), "")
    .land_entry(LandEntry::Untapped)
    .with_abilities(&[
        AbilityDef::activated_mana(
            AbilityId::PRIMARY,
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Colorless)),
        ),
        AbilityDef::not_implemented(
            AbilityId(1),
            "{1}: This land becomes a 2/2 creature with all creature types until end of turn. It's still a land.",
            "The animation activated ability is not executed.",
        ),
    ]),
);

pub(in crate::card::sets) static PRIMEVAL_BOUNTY: CardRecord = CardRecord::new(
    cards::PRIMEVAL_BOUNTY,
    "Primeval Bounty",
    CardArt::new("e750d55d-d5e8-4abe-99cf-f6b8ba86cf16", "Christine Choi"),
    CardSet::Magic2014,
        CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(5, 0, 0, 0, 0, 1),
        "Whenever you cast a creature spell, create a 3/3 green Beast creature token.\nWhenever you cast a noncreature spell, put three +1/+1 counters on target creature you control.\nLandfall — Whenever a land you control enters, you gain 3 life.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static QUICKEN: CardRecord = CardRecord::new(
    cards::QUICKEN,
    "Quicken",
    CardArt::new("066bef3d-c785-4b25-9b91-8f676aa9906f", "Aleksi Briclot"),
    CardSet::Magic2014,
        CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 1, 0, 0, 0),
        "The next sorcery spell you cast this turn can be cast as though it had flash. (It can be cast any time you could cast an instant.)\nDraw a card.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static RATCHET_BOMB: CardRecord = CardRecord::new(
    cards::RATCHET_BOMB,
    "Ratchet Bomb",
    CardArt::new("3e9045df-3eff-4236-9bbb-77537b302e27", "Austin Hsu"),
    CardSet::Magic2014,
        CardRules::new(
        CardKind::Artifact,
        ManaCost::colored(2, 0, 0, 0, 0, 0),
        "{T}: Put a charge counter on this artifact.\n{T}, Sacrifice this artifact: Destroy each nonland permanent with mana value equal to the number of charge counters on this artifact.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static SCAVENGING_OOZE: CardRecord = CardRecord::new(
    cards::SCAVENGING_OOZE,
    "Scavenging Ooze",
    CardArt::new("ec30153a-36b5-42f8-beed-9efab09f1051", "Austin Hsu"),
    CardSet::Magic2014,
        CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 0, 0, 0, 0, 1),
        "{G}: Exile target card from a graveyard. If it was a creature card, put a +1/+1 counter on this creature and you gain 1 life.",
    )
    .with_subtypes(&["Ooze"])
    .creature(2, 2)
    .metadata_only(),
);

pub(in crate::card::sets) static SHADOWBORN_DEMON: CardRecord = CardRecord::new(
    cards::SHADOWBORN_DEMON,
    "Shadowborn Demon",
    CardArt::new("3884c05b-c10e-4f1d-a8bd-8b5118657972", "Lucas Graciano"),
    CardSet::Magic2014,
        CardRules::new(
        CardKind::Creature,
        ManaCost::colored(3, 0, 0, 2, 0, 0),
        "",
    )
    .with_subtypes(&["Demon"])
    .creature(5, 6)
    .with_abilities(&[
        AbilityDef::evergreen(AbilityId::PRIMARY, "Flying", EvergreenAbility::Flying),
        AbilityDef::not_implemented(
            AbilityId(1),
            "When this creature enters, destroy target non-Demon creature.",
            "The enters-the-battlefield destruction trigger is not executed.",
        ),
        AbilityDef::not_implemented(
            AbilityId(2),
            "At the beginning of your upkeep, if there are fewer than six creature cards in your graveyard, sacrifice a creature.",
            "The conditional upkeep sacrifice trigger is not executed.",
        ),
    ]),
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
