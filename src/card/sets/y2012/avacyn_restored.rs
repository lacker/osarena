//! Avacyn Restored card records used by the built-in ISD–RTR Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityImplementationDef, AddManaEffectDef, AppliedEffectDef,
    CardArt, CardKind, CardRules, CardSet, CardSupertype, EffectDef, EvergreenAbility, LandEntry,
    ManaCost, ManaKindDef, ManaRestrictionDef, ManaSpendEffectDef, cards,
};
use crate::ids::AbilityId;

pub(in crate::card::sets) static BONFIRE_OF_THE_DAMNED: CardRecord = CardRecord::new(
    cards::BONFIRE_OF_THE_DAMNED,
    "Bonfire of the Damned",
    CardArt::new("e60610fe-891d-46de-b556-d03b637dccec", "James Paick"),
    CardSet::AvacynRestored,
        CardRules::new(
        CardKind::Sorcery,
        ManaCost::variable(0, 0, 0, 0, 1, 0, 2),
        "Bonfire of the Damned deals X damage to target player or planeswalker and each creature that player or that planeswalker's controller controls.\nMiracle {X}{R} (You may cast this card for its miracle cost when you draw it if it's the first card you drew this turn.)",
    )
    .metadata_only(),
);

static CAVERN_COLORED_MANA_RESTRICTIONS: [ManaRestrictionDef; 1] =
    [ManaRestrictionDef::CastCreatureSpellOfChosenType];

static CAVERN_COLORED_MANA_SPEND_EFFECTS: [ManaSpendEffectDef; 1] =
    [ManaSpendEffectDef::ApplyToPaidSpell(
        AppliedEffectDef::CannotBeCountered,
    )];

pub(in crate::card::sets) static CAVERN_OF_SOULS: CardRecord = CardRecord::new(
    cards::CAVERN_OF_SOULS,
    "Cavern of Souls",
    CardArt::new("1381c8f1-a292-4bdf-b20c-a5c2a169ee84", "Cliff Childs"),
    CardSet::AvacynRestored,
        CardRules::new(CardKind::Land, ManaCost::colored(0, 0, 0, 0, 0, 0), "")
    .land_entry(LandEntry::Untapped)
    .with_abilities(&[
        AbilityDef::replacement(
            AbilityId(2),
            "As this land enters, choose a creature type.",
            EffectDef::Special("Choose and store a creature type for this permanent"),
        )
        .with_implementation(AbilityImplementationDef::NotImplemented {
            explanation: "The creature-type choice is represented but is not executed.",
        }),
        AbilityDef::activated_mana(
            AbilityId::PRIMARY,
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Colorless)),
        ),
        AbilityDef::activated_mana(
            AbilityId(1),
            "{T}: Add one mana of any color. Spend this mana only to cast a creature spell of the chosen type, and that spell can't be countered.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(
                AddManaEffectDef::choice(&[
                    ManaKindDef::White,
                    ManaKindDef::Blue,
                    ManaKindDef::Black,
                    ManaKindDef::Red,
                    ManaKindDef::Green,
                ])
                .with_restrictions(&CAVERN_COLORED_MANA_RESTRICTIONS)
                .with_spend_effects(&CAVERN_COLORED_MANA_SPEND_EFFECTS),
            ),
        )
        .with_implementation(AbilityImplementationDef::NotImplemented {
            explanation: "The restricted colored mana and cannot-be-countered rider are represented but not executed.",
        }),
    ]),
);

pub(in crate::card::sets) static DEMONIC_RISING: CardRecord = CardRecord::new(
    cards::DEMONIC_RISING,
    "Demonic Rising",
    CardArt::new("a2136a82-b535-47f6-9eee-5b7585ac5cf1", "Trevor Claxton"),
    CardSet::AvacynRestored,
        CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(3, 0, 0, 2, 0, 0),
        "At the beginning of your end step, if you control exactly one creature, create a 5/5 black Demon creature token with flying.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static PILLAR_OF_FLAME: CardRecord = CardRecord::new(
    cards::PILLAR_OF_FLAME,
    "Pillar of Flame",
    CardArt::new("c983e879-d9d2-47cc-9958-506711ca80cd", "Karl Kopinski"),
    CardSet::AvacynRestored,
        CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(0, 0, 0, 0, 1, 0),
        "Pillar of Flame deals 2 damage to any target. If a creature dealt damage this way would die this turn, exile it instead.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static RESTORATION_ANGEL: CardRecord = CardRecord::new(
    cards::RESTORATION_ANGEL,
    "Restoration Angel",
    CardArt::new("c2ad8639-e586-47f4-baca-2a1af5aa281b", "Johannes Voss"),
    CardSet::AvacynRestored,
        CardRules::new(CardKind::Creature, ManaCost::colored(3, 1, 0, 0, 0, 0), "")
    .with_subtypes(&["Angel"])
    .creature(3, 4)
    .with_abilities(&[
        AbilityDef::evergreen(AbilityId::PRIMARY, "Flash", EvergreenAbility::Flash),
        AbilityDef::evergreen(AbilityId(1), "Flying", EvergreenAbility::Flying),
        AbilityDef::not_implemented(
            AbilityId(2),
            "When this creature enters, you may exile target non-Angel creature you control, then return that card to the battlefield under your control.",
            "The enters-the-battlefield blink ability is not executed.",
        ),
    ]),
);

pub(in crate::card::sets) static SIGARDA_HOST_OF_HERONS: CardRecord = CardRecord::new(
    cards::SIGARDA_HOST_OF_HERONS,
    "Sigarda, Host of Herons",
    CardArt::new("feccd0e2-fae6-4ced-acdf-4252ed5c56e7", "Chris Rahn"),
    CardSet::AvacynRestored,
        CardRules::new(CardKind::Creature, ManaCost::colored(2, 2, 0, 0, 0, 1), "")
    .with_supertype(CardSupertype::Legendary)
    .with_subtypes(&["Angel"])
    .creature(5, 5)
    .with_abilities(&[
        AbilityDef::evergreen(AbilityId::PRIMARY, "Flying", EvergreenAbility::Flying),
        AbilityDef::evergreen(AbilityId(1), "Hexproof", EvergreenAbility::Hexproof),
        AbilityDef::not_implemented(
            AbilityId(2),
            "Spells and abilities your opponents control can't cause you to sacrifice permanents.",
            "The sacrifice-prevention static ability is not executed.",
        ),
    ]),
);

pub(in crate::card::sets) static TERMINUS: CardRecord = CardRecord::new(
    cards::TERMINUS,
    "Terminus",
    CardArt::new("0982ea7e-05a4-4e40-98ab-ea9aa6c7342e", "James Paick"),
    CardSet::AvacynRestored,
        CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(4, 2, 0, 0, 0, 0),
        "Put all creatures on the bottom of their owners' libraries.\nMiracle {W} (You may cast this card for its miracle cost when you draw it if it's the first card you drew this turn.)",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static ZEALOUS_CONSCRIPTS: CardRecord = CardRecord::new(
    cards::ZEALOUS_CONSCRIPTS,
    "Zealous Conscripts",
    CardArt::new("fc027b11-1ecc-430d-a862-586a14bb23c3", "Steve Prescott"),
    CardSet::AvacynRestored,
        CardRules::new(CardKind::Creature, ManaCost::colored(4, 0, 0, 0, 1, 0), "")
    .with_subtypes(&["Human", "Warrior"])
    .creature(3, 3)
    .with_abilities(&[
        AbilityDef::evergreen(AbilityId::PRIMARY, "Haste", EvergreenAbility::Haste),
        AbilityDef::not_implemented(
            AbilityId(1),
            "When this creature enters, gain control of target permanent until end of turn. Untap that permanent. It gains haste until end of turn.",
            "The enters-the-battlefield control-changing ability is not executed.",
        ),
    ]),
);
pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BONFIRE_OF_THE_DAMNED,
    &CAVERN_OF_SOULS,
    &DEMONIC_RISING,
    &PILLAR_OF_FLAME,
    &RESTORATION_ANGEL,
    &SIGARDA_HOST_OF_HERONS,
    &TERMINUS,
    &ZEALOUS_CONSCRIPTS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
