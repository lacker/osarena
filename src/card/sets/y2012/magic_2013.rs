//! Magic 2013 card records used by the built-in ISD–RTR Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{CardBehavior, CardKind, CardRules, CardSet, LandEntry, ManaCost, cards};

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static ARBOR_ELF: CardRecord = CardRecord::new(
    cards::ARBOR_ELF,
    "Arbor Elf",
    CardSet::Magic2013,
    false,
    CardBehavior::ArborElf,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 0, 0, 0, 0, 1),
        "{T}: Untap target Forest.",
    )
    .type_line("Creature — Elf Druid")
    .creature(1, 1)
    .metadata_only(),
);

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static AUGUR_OF_BOLAS: CardRecord = CardRecord::new(
    cards::AUGUR_OF_BOLAS,
    "Augur of Bolas",
    CardSet::Magic2013,
    false,
    CardBehavior::AugurOfBolas,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 0, 1, 0, 0, 0),
        "When this creature enters, look at the top three cards of your library. You may reveal an instant or sorcery card from among them and put it into your hand. Put the rest on the bottom of your library in any order.",
    )
    .type_line("Creature — Merfolk Wizard")
    .creature(1, 3)
    .metadata_only(),
);

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static DISCIPLE_OF_BOLAS: CardRecord = CardRecord::new(
    cards::DISCIPLE_OF_BOLAS,
    "Disciple of Bolas",
    CardSet::Magic2013,
    false,
    CardBehavior::DiscipleOfBolas,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(3, 0, 0, 1, 0, 0),
        "When this creature enters, sacrifice another creature. You gain X life and draw X cards, where X is that creature's power.",
    )
    .type_line("Creature — Human Wizard")
    .creature(2, 1)
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static DURESS: CardRecord = CardRecord::new(
    cards::DURESS,
    "Duress",
    CardSet::Magic2013,
    false,
    CardBehavior::Duress,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(0, 0, 0, 1, 0, 0),
        "Target opponent reveals their hand. You choose a noncreature, nonland card from it. That player discards that card.",
    )
    .type_line("Sorcery")
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static ESSENCE_SCATTER: CardRecord = CardRecord::new(
    cards::ESSENCE_SCATTER,
    "Essence Scatter",
    CardSet::Magic2013,
    false,
    CardBehavior::EssenceScatter,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 0, 1, 0, 0, 0),
        "Counter target creature spell.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static FLAMES_OF_THE_FIREBRAND: CardRecord = CardRecord::new(
    cards::FLAMES_OF_THE_FIREBRAND,
    "Flames of the Firebrand",
    CardSet::Magic2013,
    false,
    CardBehavior::FlamesOfTheFirebrand,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(2, 0, 0, 0, 1, 0),
        "Flames of the Firebrand deals 3 damage divided as you choose among one, two, or three targets.",
    )
    .type_line("Sorcery")
    .metadata_only(),
);

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static FLINTHOOF_BOAR: CardRecord = CardRecord::new(
    cards::FLINTHOOF_BOAR,
    "Flinthoof Boar",
    CardSet::Magic2013,
    false,
    CardBehavior::FlinthoofBoar,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 0, 0, 0, 0, 1),
        "This creature gets +1/+1 as long as you control a Mountain.\n{R}: This creature gains haste until end of turn. (It can attack and {T} this turn.)",
    )
    .type_line("Creature — Boar")
    .creature(2, 2)
    .metadata_only(),
);

// Implementation status: Land entry and mana production are active.
pub(in crate::card::sets) static GLACIAL_FORTRESS: CardRecord = CardRecord::new(
    cards::GLACIAL_FORTRESS,
    "Glacial Fortress",
    CardSet::Magic2013,
    false,
    CardBehavior::GlacialFortress,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "This land enters tapped unless you control a Plains or an Island.\n{T}: Add {W} or {U}.",
    )
    .type_line("Land")
    .produces([true, true, false, false, false, false])
    .land_entry(LandEntry::TappedUnlessControlsLandType([
        true, true, false, false, false,
    ]))
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static JACE_MEMORY_ADEPT: CardRecord = CardRecord::new(
    cards::JACE_MEMORY_ADEPT,
    "Jace, Memory Adept",
    CardSet::Magic2013,
    false,
    CardBehavior::JaceMemoryAdept,
    CardRules::new(
        CardKind::Planeswalker,
        ManaCost::colored(3, 0, 2, 0, 0, 0),
        "+1: Draw a card. Target player mills a card.\n0: Target player mills ten cards.\n−7: Any number of target players each draw twenty cards.",
    )
    .type_line("Legendary Planeswalker — Jace")
    .planeswalker(4)
    .legendary()
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static MUTILATE: CardRecord = CardRecord::new(
    cards::MUTILATE,
    "Mutilate",
    CardSet::Magic2013,
    false,
    CardBehavior::Mutilate,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(2, 0, 0, 2, 0, 0),
        "All creatures get -1/-1 until end of turn for each Swamp you control.",
    )
    .type_line("Sorcery")
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static NEGATE: CardRecord = CardRecord::new(
    cards::NEGATE,
    "Negate",
    CardSet::Magic2013,
    false,
    CardBehavior::Negate,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 0, 1, 0, 0, 0),
        "Counter target noncreature spell.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static OBLIVION_RING: CardRecord = CardRecord::new(
    cards::OBLIVION_RING,
    "Oblivion Ring",
    CardSet::Magic2013,
    false,
    CardBehavior::OblivionRing,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(2, 1, 0, 0, 0, 0),
        "When this enchantment enters, exile another target nonland permanent.\nWhen this enchantment leaves the battlefield, return the exiled card to the battlefield under its owner's control.",
    )
    .type_line("Enchantment")
    .metadata_only(),
);

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static RHOX_FAITHMENDER: CardRecord = CardRecord::new(
    cards::RHOX_FAITHMENDER,
    "Rhox Faithmender",
    CardSet::Magic2013,
    false,
    CardBehavior::RhoxFaithmender,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(3, 1, 0, 0, 0, 0),
        "Lifelink (Damage dealt by this creature also causes you to gain that much life.)\nIf you would gain life, you gain twice that much life instead.",
    )
    .type_line("Creature — Rhino Monk")
    .creature(1, 5)
    .lifelink()
    .metadata_only(),
);

// Implementation status: Land entry and mana production are active.
pub(in crate::card::sets) static ROOTBOUND_CRAG: CardRecord = CardRecord::new(
    cards::ROOTBOUND_CRAG,
    "Rootbound Crag",
    CardSet::Magic2013,
    false,
    CardBehavior::RootboundCrag,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "This land enters tapped unless you control a Mountain or a Forest.\n{T}: Add {R} or {G}.",
    )
    .type_line("Land")
    .produces([false, false, false, true, true, false])
    .land_entry(LandEntry::TappedUnlessControlsLandType([
        false, false, false, true, true,
    ]))
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static SIGN_IN_BLOOD: CardRecord = CardRecord::new(
    cards::SIGN_IN_BLOOD,
    "Sign in Blood",
    CardSet::Magic2013,
    false,
    CardBehavior::SignInBlood,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(0, 0, 0, 2, 0, 0),
        "Target player draws two cards and loses 2 life.",
    )
    .type_line("Sorcery")
    .metadata_only(),
);

// Implementation status: Land entry and mana production are active.
pub(in crate::card::sets) static SUNPETAL_GROVE: CardRecord = CardRecord::new(
    cards::SUNPETAL_GROVE,
    "Sunpetal Grove",
    CardSet::Magic2013,
    false,
    CardBehavior::SunpetalGrove,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "This land enters tapped unless you control a Forest or a Plains.\n{T}: Add {G} or {W}.",
    )
    .type_line("Land")
    .produces([true, false, false, false, true, false])
    .land_entry(LandEntry::TappedUnlessControlsLandType([
        true, false, false, false, true,
    ]))
    .metadata_only(),
);

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static THRAGTUSK: CardRecord = CardRecord::new(
    cards::THRAGTUSK,
    "Thragtusk",
    CardSet::Magic2013,
    false,
    CardBehavior::Thragtusk,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(4, 0, 0, 0, 0, 1),
        "When this creature enters, you gain 5 life.\nWhen this creature leaves the battlefield, create a 3/3 green Beast creature token.",
    )
    .type_line("Creature — Beast")
    .creature(5, 3)
    .metadata_only(),
);

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static THUNDERMAW_HELLKITE: CardRecord = CardRecord::new(
    cards::THUNDERMAW_HELLKITE,
    "Thundermaw Hellkite",
    CardSet::Magic2013,
    false,
    CardBehavior::ThundermawHellkite,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(3, 0, 0, 0, 2, 0),
        "Flying\nHaste (This creature can attack and {T} as soon as it comes under your control.)\nWhen this creature enters, it deals 1 damage to each creature with flying your opponents control. Tap those creatures.",
    )
    .type_line("Creature — Dragon")
    .creature(5, 5)
    .flying()
    .haste()
    .metadata_only(),
);

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static VAMPIRE_NIGHTHAWK: CardRecord = CardRecord::new(
    cards::VAMPIRE_NIGHTHAWK,
    "Vampire Nighthawk",
    CardSet::Magic2013,
    false,
    CardBehavior::VampireNighthawk,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 0, 0, 2, 0, 0),
        "Flying\nDeathtouch (Any amount of damage this deals to a creature is enough to destroy it.)\nLifelink (Damage dealt by this creature also causes you to gain that much life.)",
    )
    .type_line("Creature — Vampire Shaman")
    .creature(2, 3)
    .flying()
    .deathtouch()
    .lifelink()
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static VOLCANIC_STRENGTH: CardRecord = CardRecord::new(
    cards::VOLCANIC_STRENGTH,
    "Volcanic Strength",
    CardSet::Magic2013,
    false,
    CardBehavior::VolcanicStrength,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(1, 0, 0, 0, 1, 0),
        "Enchant creature\nEnchanted creature gets +2/+2 and has mountainwalk. (It can't be blocked as long as defending player controls a Mountain.)",
    )
    .type_line("Enchantment — Aura")
    .metadata_only(),
);

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static WAR_PRIEST_OF_THUNE: CardRecord = CardRecord::new(
    cards::WAR_PRIEST_OF_THUNE,
    "War Priest of Thune",
    CardSet::Magic2013,
    false,
    CardBehavior::WarPriestOfThune,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 1, 0, 0, 0, 0),
        "When this creature enters, you may destroy target enchantment.",
    )
    .type_line("Creature — Human Cleric")
    .creature(2, 2)
    .metadata_only(),
);
pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARBOR_ELF,
    &AUGUR_OF_BOLAS,
    &DISCIPLE_OF_BOLAS,
    &DURESS,
    &ESSENCE_SCATTER,
    &FLAMES_OF_THE_FIREBRAND,
    &FLINTHOOF_BOAR,
    &GLACIAL_FORTRESS,
    &JACE_MEMORY_ADEPT,
    &MUTILATE,
    &NEGATE,
    &OBLIVION_RING,
    &RHOX_FAITHMENDER,
    &ROOTBOUND_CRAG,
    &SIGN_IN_BLOOD,
    &SUNPETAL_GROVE,
    &THRAGTUSK,
    &THUNDERMAW_HELLKITE,
    &VAMPIRE_NIGHTHAWK,
    &VOLCANIC_STRENGTH,
    &WAR_PRIEST_OF_THUNE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
