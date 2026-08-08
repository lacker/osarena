//! Avacyn Restored card records used by the built-in ISD–RTR Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, AppliedEffectDef, CardArt, CardBehavior,
    CardKind, CardRules, CardSet, EffectDef, ImplementationStatus, LandEntry, ManaCost,
    ManaKindDef, ManaRestrictionDef, ManaSpendEffectDef, cards,
};
use crate::ids::AbilityId;

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static BONFIRE_OF_THE_DAMNED: CardRecord = CardRecord::new(
    cards::BONFIRE_OF_THE_DAMNED,
    "Bonfire of the Damned",
    CardArt::new("e60610fe-891d-46de-b556-d03b637dccec", "James Paick"),
    CardSet::AvacynRestored,
    false,
    CardBehavior::BonfireOfTheDamned,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::variable(0, 0, 0, 0, 1, 0, 2),
        "Bonfire of the Damned deals X damage to target player or planeswalker and each creature that player or that planeswalker's controller controls.\nMiracle {X}{R} (You may cast this card for its miracle cost when you draw it if it's the first card you drew this turn.)",
    )
    .type_line("Sorcery")
    .metadata_only(),
);

static CAVERN_COLORED_MANA_RESTRICTIONS: [ManaRestrictionDef; 1] =
    [ManaRestrictionDef::CastCreatureSpellOfChosenType];

static CAVERN_COLORED_MANA_SPEND_EFFECTS: [ManaSpendEffectDef; 1] =
    [ManaSpendEffectDef::ApplyToPaidSpell(
        AppliedEffectDef::CannotBeCountered,
    )];

static CAVERN_OF_SOULS_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::activated_mana(
        AbilityId::PRIMARY,
        "{T}: Add {C}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Colorless)),
    ),
    AbilityDef::activated_mana(
        AbilityId(1),
        "{T}: Add one mana of any color. Spend it only to cast a creature spell of the chosen type; that spell can't be countered.",
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
    ),
];

pub(in crate::card::sets) static CAVERN_OF_SOULS: CardRecord = CardRecord::new(
    cards::CAVERN_OF_SOULS,
    "Cavern of Souls",
    CardArt::new("1381c8f1-a292-4bdf-b20c-a5c2a169ee84", "Cliff Childs"),
    CardSet::AvacynRestored,
    false,
    CardBehavior::CavernOfSouls,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "As this land enters, choose a creature type.\n{T}: Add {C}.\n{T}: Add one mana of any color. Spend this mana only to cast a creature spell of the chosen type, and that spell can't be countered.",
    )
    .type_line("Land")
    .produces([false, false, false, false, false, true])
    .land_entry(LandEntry::Untapped)
    .with_abilities(&CAVERN_OF_SOULS_ABILITIES)
    .metadata_only(),
)
.with_implementation_status(ImplementationStatus::MetadataOnly {
    explanation: "Untapped entry and colorless mana work; the creature-type choice, restricted colored mana, and cannot-be-countered rider are represented but not executed.",
});

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static DEMONIC_RISING: CardRecord = CardRecord::new(
    cards::DEMONIC_RISING,
    "Demonic Rising",
    CardArt::new("a2136a82-b535-47f6-9eee-5b7585ac5cf1", "Trevor Claxton"),
    CardSet::AvacynRestored,
    false,
    CardBehavior::DemonicRising,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(3, 0, 0, 2, 0, 0),
        "At the beginning of your end step, if you control exactly one creature, create a 5/5 black Demon creature token with flying.",
    )
    .type_line("Enchantment")
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static PILLAR_OF_FLAME: CardRecord = CardRecord::new(
    cards::PILLAR_OF_FLAME,
    "Pillar of Flame",
    CardArt::new("c983e879-d9d2-47cc-9958-506711ca80cd", "Karl Kopinski"),
    CardSet::AvacynRestored,
    false,
    CardBehavior::PillarOfFlame,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(0, 0, 0, 0, 1, 0),
        "Pillar of Flame deals 2 damage to any target. If a creature dealt damage this way would die this turn, exile it instead.",
    )
    .type_line("Sorcery")
    .metadata_only(),
);

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static RESTORATION_ANGEL: CardRecord = CardRecord::new(
    cards::RESTORATION_ANGEL,
    "Restoration Angel",
    CardArt::new("c2ad8639-e586-47f4-baca-2a1af5aa281b", "Johannes Voss"),
    CardSet::AvacynRestored,
    false,
    CardBehavior::RestorationAngel,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(3, 1, 0, 0, 0, 0),
        "Flash\nFlying\nWhen this creature enters, you may exile target non-Angel creature you control, then return that card to the battlefield under your control.",
    )
    .type_line("Creature — Angel")
    .creature(3, 4)
    .flying()
    .flash()
    .metadata_only(),
);

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static SIGARDA_HOST_OF_HERONS: CardRecord = CardRecord::new(
    cards::SIGARDA_HOST_OF_HERONS,
    "Sigarda, Host of Herons",
    CardArt::new("feccd0e2-fae6-4ced-acdf-4252ed5c56e7", "Chris Rahn"),
    CardSet::AvacynRestored,
    false,
    CardBehavior::SigardaHostOfHerons,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(2, 2, 0, 0, 0, 1),
        "Flying, hexproof\nSpells and abilities your opponents control can't cause you to sacrifice permanents.",
    )
    .type_line("Legendary Creature — Angel")
    .creature(5, 5)
    .legendary()
    .flying()
    .hexproof()
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static TERMINUS: CardRecord = CardRecord::new(
    cards::TERMINUS,
    "Terminus",
    CardArt::new("0982ea7e-05a4-4e40-98ab-ea9aa6c7342e", "James Paick"),
    CardSet::AvacynRestored,
    false,
    CardBehavior::Terminus,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(4, 2, 0, 0, 0, 0),
        "Put all creatures on the bottom of their owners' libraries.\nMiracle {W} (You may cast this card for its miracle cost when you draw it if it's the first card you drew this turn.)",
    )
    .type_line("Sorcery")
    .metadata_only(),
);

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static ZEALOUS_CONSCRIPTS: CardRecord = CardRecord::new(
    cards::ZEALOUS_CONSCRIPTS,
    "Zealous Conscripts",
    CardArt::new("fc027b11-1ecc-430d-a862-586a14bb23c3", "Steve Prescott"),
    CardSet::AvacynRestored,
    false,
    CardBehavior::ZealousConscripts,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(4, 0, 0, 0, 1, 0),
        "Haste\nWhen this creature enters, gain control of target permanent until end of turn. Untap that permanent. It gains haste until end of turn.",
    )
    .type_line("Creature — Human Warrior")
    .creature(3, 3)
    .haste()
    .metadata_only(),
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
