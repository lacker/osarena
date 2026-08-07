//! Innistrad card records used by the built-in ISD–RTR Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    CardBehavior, CardComposition, CardEffectStatus, CardKind, CardPart, CardRules, CardSet,
    CardStructure, DoubleFacedKind, LandEntry, ManaCost, PlayOptionDef, SpellForm, cards,
};
use crate::ids::{CardPartId, PlayOptionId};

// Implementation status: Baseline creature and printed mana ability are active.
pub(in crate::card::sets) static AVACYNS_PILGRIM: CardRecord = CardRecord::new(
    cards::AVACYNS_PILGRIM,
    "Avacyn's Pilgrim",
    CardSet::Innistrad,
    false,
    CardBehavior::AvacynsPilgrim,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 0, 0, 0, 0, 1),
        "{T}: Add {W}.",
    )
    .type_line("Creature — Human Monk")
    .creature(1, 1)
    .produces([true, false, false, false, false, false])
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static BLASPHEMOUS_ACT: CardRecord = CardRecord::new(
    cards::BLASPHEMOUS_ACT,
    "Blasphemous Act",
    CardSet::Innistrad,
    false,
    CardBehavior::BlasphemousAct,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(8, 0, 0, 0, 1, 0),
        "This spell costs {1} less to cast for each creature on the battlefield.\nBlasphemous Act deals 13 damage to each creature.",
    )
    .type_line("Sorcery")
    .metadata_only(),
);

// Implementation status: Land entry and mana production are active.
pub(in crate::card::sets) static CLIFFTOP_RETREAT: CardRecord = CardRecord::new(
    cards::CLIFFTOP_RETREAT,
    "Clifftop Retreat",
    CardSet::Innistrad,
    false,
    CardBehavior::ClifftopRetreat,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "This land enters tapped unless you control a Mountain or a Plains.\n{T}: Add {R} or {W}.",
    )
    .type_line("Land")
    .produces([true, false, false, true, false, false])
    .land_entry(LandEntry::TappedUnlessControlsLandType([
        true, false, false, true, false,
    ]))
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static DISSIPATE: CardRecord = CardRecord::new(
    cards::DISSIPATE,
    "Dissipate",
    CardSet::Innistrad,
    false,
    CardBehavior::Dissipate,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 0, 2, 0, 0, 0),
        "Counter target spell. If that spell is countered this way, exile it instead of putting it into its owner's graveyard.",
    )
    .type_line("Instant")
    .metadata_only(),
);

const fn garruk_front_rules() -> CardRules {
    CardRules::new(
        CardKind::Planeswalker,
        ManaCost::colored(3, 0, 0, 0, 0, 1),
        "When Garruk Relentless has two or fewer loyalty counters on him, transform him.\n0: Garruk Relentless deals 3 damage to target creature. That creature deals damage equal to its power to him.\n0: Create a 2/2 green Wolf creature token.",
    )
    .type_line("Legendary Planeswalker — Garruk")
    .planeswalker(3)
    .legendary()
    .metadata_only()
}

fn garruk_composition() -> CardComposition {
    let front = garruk_front_rules();
    let back = CardRules::new(
        CardKind::Planeswalker,
        ManaCost::default(),
        "+1: Create a 1/1 black Wolf creature token with deathtouch.\n−1: Sacrifice a creature. If you do, search your library for a creature card, reveal it, put it into your hand, then shuffle.\n−3: Creatures you control gain trample and get +X/+X until end of turn, where X is the number of creature cards in your graveyard.",
    )
    .type_line("Legendary Planeswalker — Garruk")
    .printed_colors([false, false, true, false, true])
    .legendary()
    .metadata_only();
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Garruk Relentless", front),
            CardPart::new(CardPartId(1), "Garruk, the Veil-Cursed", back).without_mana_cost(),
        ],
        structure: CardStructure::DoubleFaced {
            front: CardPartId::PRIMARY,
            back: CardPartId(1),
            kind: DoubleFacedKind::Transforming,
        },
        play_options: vec![PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Garruk Relentless",
            SpellForm::Part(CardPartId::PRIMARY),
            front.mana_cost,
            CardEffectStatus::MetadataOnly,
        )],
    }
}

// Implementation status: Spell is withheld from play; both faces and transformation topology are cataloged, while printed effects are pending.
pub(in crate::card::sets) static GARRUK_RELENTLESS: CardRecord = CardRecord::new(
    cards::GARRUK_RELENTLESS,
    "Garruk Relentless",
    CardSet::Innistrad,
    false,
    CardBehavior::GarrukRelentless,
    garruk_front_rules(),
)
.with_composition(garruk_composition);

// Implementation status: Land entry and modeled mana production are active; other printed abilities are pending.
pub(in crate::card::sets) static GAVONY_TOWNSHIP: CardRecord = CardRecord::new(
    cards::GAVONY_TOWNSHIP,
    "Gavony Township",
    CardSet::Innistrad,
    false,
    CardBehavior::GavonyTownship,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "{T}: Add {C}.\n{2}{G}{W}, {T}: Put a +1/+1 counter on each creature you control.",
    )
    .type_line("Land")
    .produces([false, false, false, false, false, true])
    .land_entry(LandEntry::Untapped)
    .metadata_only(),
);

// Implementation status: Land entry and modeled mana production are active; other printed abilities are pending.
pub(in crate::card::sets) static GHOST_QUARTER: CardRecord = CardRecord::new(
    cards::GHOST_QUARTER,
    "Ghost Quarter",
    CardSet::Innistrad,
    false,
    CardBehavior::GhostQuarter,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "{T}: Add {C}.\n{T}, Sacrifice this land: Destroy target land. Its controller may search their library for a basic land card, put it onto the battlefield, then shuffle.",
    )
    .type_line("Land")
    .produces([false, false, false, false, false, true])
    .land_entry(LandEntry::Untapped)
    .metadata_only(),
);

// Implementation status: Land entry and mana production are active.
pub(in crate::card::sets) static ISOLATED_CHAPEL: CardRecord = CardRecord::new(
    cards::ISOLATED_CHAPEL,
    "Isolated Chapel",
    CardSet::Innistrad,
    false,
    CardBehavior::IsolatedChapel,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "This land enters tapped unless you control a Plains or a Swamp.\n{T}: Add {W} or {B}.",
    )
    .type_line("Land")
    .produces([true, false, true, false, false, false])
    .land_entry(LandEntry::TappedUnlessControlsLandType([
        true, false, true, false, false,
    ]))
    .metadata_only(),
);

// Implementation status: Land entry and modeled mana production are active; other printed abilities are pending.
pub(in crate::card::sets) static KESSIG_WOLF_RUN: CardRecord = CardRecord::new(
    cards::KESSIG_WOLF_RUN,
    "Kessig Wolf Run",
    CardSet::Innistrad,
    false,
    CardBehavior::KessigWolfRun,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "{T}: Add {C}.\n{X}{R}{G}, {T}: Target creature gets +X/+0 and gains trample until end of turn.",
    )
    .type_line("Land")
    .produces([false, false, false, false, false, true])
    .land_entry(LandEntry::Untapped)
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static LILIANA_OF_THE_VEIL: CardRecord = CardRecord::new(
    cards::LILIANA_OF_THE_VEIL,
    "Liliana of the Veil",
    CardSet::Innistrad,
    false,
    CardBehavior::LilianaOfTheVeil,
    CardRules::new(
        CardKind::Planeswalker,
        ManaCost::colored(1, 0, 0, 2, 0, 0),
        "+1: Each player discards a card.\n−2: Target player sacrifices a creature.\n−6: Separate all permanents target player controls into two piles. That player sacrifices all permanents in the pile of their choice.",
    )
    .type_line("Legendary Planeswalker — Liliana")
    .planeswalker(3)
    .legendary()
    .metadata_only(),
);

// Implementation status: Land entry and modeled mana production are active; other printed abilities are pending.
pub(in crate::card::sets) static MOORLAND_HAUNT: CardRecord = CardRecord::new(
    cards::MOORLAND_HAUNT,
    "Moorland Haunt",
    CardSet::Innistrad,
    false,
    CardBehavior::MoorlandHaunt,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "{T}: Add {C}.\n{W}{U}, {T}, Exile a creature card from your graveyard: Create a 1/1 white Spirit creature token with flying.",
    )
    .type_line("Land")
    .produces([false, false, false, false, false, true])
    .land_entry(LandEntry::Untapped)
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static MULCH: CardRecord = CardRecord::new(
    cards::MULCH,
    "Mulch",
    CardSet::Innistrad,
    false,
    CardBehavior::Mulch,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(1, 0, 0, 0, 0, 1),
        "Reveal the top four cards of your library. Put all land cards revealed this way into your hand and the rest into your graveyard.",
    )
    .type_line("Sorcery")
    .metadata_only(),
);

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static SNAPCASTER_MAGE: CardRecord = CardRecord::new(
    cards::SNAPCASTER_MAGE,
    "Snapcaster Mage",
    CardSet::Innistrad,
    false,
    CardBehavior::SnapcasterMage,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 0, 1, 0, 0, 0),
        "Flash\nWhen this creature enters, target instant or sorcery card in your graveyard gains flashback until end of turn. The flashback cost is equal to its mana cost. (You may cast that card from your graveyard for its flashback cost. Then exile it.)",
    )
    .type_line("Creature — Human Wizard")
    .creature(2, 1)
    .flash()
    .metadata_only(),
);

// Implementation status: Land entry and mana production are active.
pub(in crate::card::sets) static SULFUR_FALLS: CardRecord = CardRecord::new(
    cards::SULFUR_FALLS,
    "Sulfur Falls",
    CardSet::Innistrad,
    false,
    CardBehavior::SulfurFalls,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "This land enters tapped unless you control an Island or a Mountain.\n{T}: Add {U} or {R}.",
    )
    .type_line("Land")
    .produces([false, true, false, true, false, false])
    .land_entry(LandEntry::TappedUnlessControlsLandType([
        false, true, false, true, false,
    ]))
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static THINK_TWICE: CardRecord = CardRecord::new(
    cards::THINK_TWICE,
    "Think Twice",
    CardSet::Innistrad,
    false,
    CardBehavior::ThinkTwice,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 0, 1, 0, 0, 0),
        "Draw a card.\nFlashback {2}{U} (You may cast this card from your graveyard for its flashback cost. Then exile it.)",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static UNBURIAL_RITES: CardRecord = CardRecord::new(
    cards::UNBURIAL_RITES,
    "Unburial Rites",
    CardSet::Innistrad,
    false,
    CardBehavior::UnburialRites,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(4, 0, 0, 1, 0, 0),
        "Return target creature card from your graveyard to the battlefield.\nFlashback {3}{W} (You may cast this card from your graveyard for its flashback cost. Then exile it.)",
    )
    .type_line("Sorcery")
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static URGENT_EXORCISM: CardRecord = CardRecord::new(
    cards::URGENT_EXORCISM,
    "Urgent Exorcism",
    CardSet::Innistrad,
    false,
    CardBehavior::UrgentExorcism,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 1, 0, 0, 0, 0),
        "Destroy target Spirit or enchantment.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Land entry and mana production are active.
pub(in crate::card::sets) static WOODLAND_CEMETERY: CardRecord = CardRecord::new(
    cards::WOODLAND_CEMETERY,
    "Woodland Cemetery",
    CardSet::Innistrad,
    false,
    CardBehavior::WoodlandCemetery,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "This land enters tapped unless you control a Swamp or a Forest.\n{T}: Add {B} or {G}.",
    )
    .type_line("Land")
    .produces([false, false, true, false, true, false])
    .land_entry(LandEntry::TappedUnlessControlsLandType([
        false, false, true, false, true,
    ]))
    .metadata_only(),
);
pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AVACYNS_PILGRIM,
    &BLASPHEMOUS_ACT,
    &CLIFFTOP_RETREAT,
    &DISSIPATE,
    &GARRUK_RELENTLESS,
    &GAVONY_TOWNSHIP,
    &GHOST_QUARTER,
    &ISOLATED_CHAPEL,
    &KESSIG_WOLF_RUN,
    &LILIANA_OF_THE_VEIL,
    &MOORLAND_HAUNT,
    &MULCH,
    &SNAPCASTER_MAGE,
    &SULFUR_FALLS,
    &THINK_TWICE,
    &UNBURIAL_RITES,
    &URGENT_EXORCISM,
    &WOODLAND_CEMETERY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
