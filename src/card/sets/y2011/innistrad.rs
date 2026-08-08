//! Innistrad card records used by the built-in ISD–RTR Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityImplementationDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AppliedEffectDef, CardArt, CardComposition, CardEffectStatus, CardKind,
    CardPart, CardRules, CardSet, CardStructure, CardSupertype, DoubleFacedKind, EffectDef,
    EffectDurationDef, EffectRecipientDef, EvergreenAbility, LandEntry, ManaCost, ManaKindDef,
    ObjectPredicateDef, PlayOptionDef, PlayerRelation, SpellForm, TriggerEventDef, ZoneKind, cards,
};
use crate::ids::{AbilityId, CardPartId, PlayOptionId, TargetSlotId};

pub(in crate::card::sets) static AVACYNS_PILGRIM: CardRecord = CardRecord::new(
    cards::AVACYNS_PILGRIM,
    "Avacyn's Pilgrim",
    CardArt::new(
        "7eb39e97-53c2-4df0-9fb3-a3d6a24ec41f",
        "Jana Schirmer & Johannes Voss",
    ),
    CardSet::Innistrad,
    CardRules::new(CardKind::Creature, ManaCost::colored(0, 0, 0, 0, 0, 1), "")
        .with_subtypes(&["Human", "Monk"])
        .creature(1, 1)
        .with_abilities(&[AbilityDef::activated_mana(
            AbilityId::PRIMARY,
            "{T}: Add {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::White)),
        )]),
);

pub(in crate::card::sets) static BLASPHEMOUS_ACT: CardRecord = CardRecord::new(
    cards::BLASPHEMOUS_ACT,
    "Blasphemous Act",
    CardArt::new("509ce648-fb76-486d-8b39-183e368b7cb7", "Daarken"),
    CardSet::Innistrad,
        CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(8, 0, 0, 0, 1, 0),
        "This spell costs {1} less to cast for each creature on the battlefield.\nBlasphemous Act deals 13 damage to each creature.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static CLIFFTOP_RETREAT: CardRecord = CardRecord::new(
    cards::CLIFFTOP_RETREAT,
    "Clifftop Retreat",
    CardArt::new("fd7e1bf9-bd6a-48e3-9331-178e5142c06a", "John Avon"),
    CardSet::Innistrad,
        CardRules::new(CardKind::Land, ManaCost::colored(0, 0, 0, 0, 0, 0), "")
    .land_entry(LandEntry::TappedUnlessControlsLandType([
        true, false, false, true, false,
    ]))
    .with_abilities(&[
        AbilityDef::replacement(
            AbilityId::PRIMARY,
            "This land enters tapped unless you control a Mountain or a Plains.",
            EffectDef::Special("Apply the declared conditional land-entry procedure"),
        )
        .with_implementation(AbilityImplementationDef::CustomFull {
            behavior: None,
            explanation: "The conditional land-entry procedure is implemented by shared land-entry rules.",
        }),
        AbilityDef::activated_mana(
            AbilityId(1),
            "{T}: Add {R} or {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaKindDef::Red,
                ManaKindDef::White,
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static DISSIPATE: CardRecord = CardRecord::new(
    cards::DISSIPATE,
    "Dissipate",
    CardArt::new("5d778082-bcdb-423a-b16f-57ac0d4dace7", "Tomasz Jedruszek"),
    CardSet::Innistrad,
        CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 0, 2, 0, 0, 0),
        "Counter target spell. If that spell is countered this way, exile it instead of putting it into its owner's graveyard.",
    )
    .metadata_only(),
);

const fn garruk_front_rules() -> CardRules {
    CardRules::new(
        CardKind::Planeswalker,
        ManaCost::colored(3, 0, 0, 0, 0, 1),
        "When Garruk has two or fewer loyalty counters on him, transform him.\n0: Garruk deals 3 damage to target creature. That creature deals damage equal to its power to him.\n0: Create a 2/2 green Wolf creature token.",
    )
    .with_supertype(CardSupertype::Legendary)
    .with_subtypes(&["Garruk"])
    .planeswalker(3)
    .metadata_only()
}

fn garruk_composition() -> CardComposition {
    let front = garruk_front_rules();
    let back = CardRules::new(
        CardKind::Planeswalker,
        ManaCost::default(),
        "+1: Create a 1/1 black Wolf creature token with deathtouch.\n−1: Sacrifice a creature. If you do, search your library for a creature card, reveal it, put it into your hand, then shuffle.\n−3: Creatures you control gain trample and get +X/+X until end of turn, where X is the number of creature cards in your graveyard.",
    )
    .with_supertype(CardSupertype::Legendary)
    .with_subtypes(&["Garruk"])
    .printed_colors([false, false, true, false, true])
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

pub(in crate::card::sets) static GARRUK_RELENTLESS: CardRecord = CardRecord::new(
    cards::GARRUK_RELENTLESS,
    "Garruk Relentless",
    CardArt::new("b4160322-ff40-41a4-887a-73cd6b85ae45", "Eric Deschamps"),
    CardSet::Innistrad,
    garruk_front_rules(),
)
.with_composition(garruk_composition);

pub(in crate::card::sets) static GAVONY_TOWNSHIP: CardRecord = CardRecord::new(
    cards::GAVONY_TOWNSHIP,
    "Gavony Township",
    CardArt::new("b5f73443-2fe8-424f-8e71-fc7ce1f3a3eb", "Peter Mohrbacher"),
    CardSet::Innistrad,
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
                "{2}{G}{W}, {T}: Put a +1/+1 counter on each creature you control.",
                "The counter-placing activated ability is not executed.",
            ),
        ]),
);

pub(in crate::card::sets) static GHOST_QUARTER: CardRecord = CardRecord::new(
    cards::GHOST_QUARTER,
    "Ghost Quarter",
    CardArt::new("1c6456ed-0ffb-4d22-b252-5775076030ce", "Peter Mohrbacher"),
    CardSet::Innistrad,
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
            "{T}, Sacrifice this land: Destroy target land. Its controller may search their library for a basic land card, put it onto the battlefield, then shuffle.",
            "The land-destruction activated ability and optional search are not executed.",
        ),
    ]),
);

pub(in crate::card::sets) static ISOLATED_CHAPEL: CardRecord = CardRecord::new(
    cards::ISOLATED_CHAPEL,
    "Isolated Chapel",
    CardArt::new("b3c1a371-5ded-4a3a-bf96-503c4f1a665d", "Cliff Childs"),
    CardSet::Innistrad,
        CardRules::new(CardKind::Land, ManaCost::colored(0, 0, 0, 0, 0, 0), "")
    .land_entry(LandEntry::TappedUnlessControlsLandType([
        true, false, true, false, false,
    ]))
    .with_abilities(&[
        AbilityDef::replacement(
            AbilityId::PRIMARY,
            "This land enters tapped unless you control a Plains or a Swamp.",
            EffectDef::Special("Apply the declared conditional land-entry procedure"),
        )
        .with_implementation(AbilityImplementationDef::CustomFull {
            behavior: None,
            explanation: "The conditional land-entry procedure is implemented by shared land-entry rules.",
        }),
        AbilityDef::activated_mana(
            AbilityId(1),
            "{T}: Add {W} or {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaKindDef::White,
                ManaKindDef::Black,
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static KESSIG_WOLF_RUN: CardRecord = CardRecord::new(
    cards::KESSIG_WOLF_RUN,
    "Kessig Wolf Run",
    CardArt::new("4a8447fe-7368-470a-911a-1083ec6cc831", "Eytan Zana"),
    CardSet::Innistrad,
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
                "{X}{R}{G}, {T}: Target creature gets +X/+0 and gains trample until end of turn.",
                "The targeted power and trample activated ability is not executed.",
            ),
        ]),
);

pub(in crate::card::sets) static LILIANA_OF_THE_VEIL: CardRecord = CardRecord::new(
    cards::LILIANA_OF_THE_VEIL,
    "Liliana of the Veil",
    CardArt::new("ac506c17-adc8-49c6-9d8d-43db7cb1ec9d", "Steve Argyle"),
    CardSet::Innistrad,
        CardRules::new(
        CardKind::Planeswalker,
        ManaCost::colored(1, 0, 0, 2, 0, 0),
        "+1: Each player discards a card.\n−2: Target player sacrifices a creature.\n−6: Separate all permanents target player controls into two piles. That player sacrifices all permanents in the pile of their choice.",
    )
    .with_supertype(CardSupertype::Legendary)
    .with_subtypes(&["Liliana"])
    .planeswalker(3)
    .metadata_only(),
);

pub(in crate::card::sets) static MOORLAND_HAUNT: CardRecord = CardRecord::new(
    cards::MOORLAND_HAUNT,
    "Moorland Haunt",
    CardArt::new("1d5569e3-278c-4cf3-860e-712010333fe6", "James Paick"),
    CardSet::Innistrad,
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
            "{W}{U}, {T}, Exile a creature card from your graveyard: Create a 1/1 white Spirit creature token with flying.",
            "The graveyard cost and token-creating activated ability are not executed.",
        ),
    ]),
);

pub(in crate::card::sets) static MULCH: CardRecord = CardRecord::new(
    cards::MULCH,
    "Mulch",
    CardArt::new("52a1dabd-82df-4814-9d64-bf7bf9c1018d", "Christopher Moeller"),
    CardSet::Innistrad,
        CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(1, 0, 0, 0, 0, 1),
        "Reveal the top four cards of your library. Put all land cards revealed this way into your hand and the rest into your graveyard.",
    )
    .metadata_only(),
);

static INSTANT_OR_SORCERY_CARD: [ObjectPredicateDef; 2] = [
    ObjectPredicateDef::CardKind(CardKind::Instant),
    ObjectPredicateDef::CardKind(CardKind::Sorcery),
];

static SNAPCASTER_MAGE_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    TargetSlotId(0),
    "instant or sorcery card in your graveyard",
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&INSTANT_OR_SORCERY_CARD),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    },
)];

pub(in crate::card::sets) static SNAPCASTER_MAGE: CardRecord = CardRecord::new(
    cards::SNAPCASTER_MAGE,
    "Snapcaster Mage",
    CardArt::new("9e5b279e-4670-4a1e-87d0-3cab7e4f9e58", "Volkan Baǵa"),
    CardSet::Innistrad,
        CardRules::new(CardKind::Creature, ManaCost::colored(1, 0, 1, 0, 0, 0), "")
    .with_subtypes(&["Human", "Wizard"])
    .creature(2, 1)
    .with_abilities(&[
        AbilityDef::evergreen(AbilityId(1), "Flash", EvergreenAbility::Flash),
        AbilityDef::triggered(
            AbilityId::PRIMARY,
            "When this creature enters, target instant or sorcery card in your graveyard gains flashback until end of turn. The flashback cost is equal to its mana cost. (You may cast that card from your graveyard for its flashback cost. Then exile it.)",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetSlotId(0)),
                effect: AppliedEffectDef::Special(
                    "Grant flashback with a flashback cost equal to this card's mana cost",
                ),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_targets(&SNAPCASTER_MAGE_TARGETS)
        .with_implementation(AbilityImplementationDef::NotImplemented {
            explanation: "The declared ETB target, flashback grant, graveyard casting, flashback cost, and exile-after-casting behavior are not executed.",
        }),
    ]),
);

pub(in crate::card::sets) static SULFUR_FALLS: CardRecord = CardRecord::new(
    cards::SULFUR_FALLS,
    "Sulfur Falls",
    CardArt::new("4968b65d-50e5-4d7e-b78b-cdada1cbf7a7", "Cliff Childs"),
    CardSet::Innistrad,
        CardRules::new(CardKind::Land, ManaCost::colored(0, 0, 0, 0, 0, 0), "")
    .land_entry(LandEntry::TappedUnlessControlsLandType([
        false, true, false, true, false,
    ]))
    .with_abilities(&[
        AbilityDef::replacement(
            AbilityId::PRIMARY,
            "This land enters tapped unless you control an Island or a Mountain.",
            EffectDef::Special("Apply the declared conditional land-entry procedure"),
        )
        .with_implementation(AbilityImplementationDef::CustomFull {
            behavior: None,
            explanation: "The conditional land-entry procedure is implemented by shared land-entry rules.",
        }),
        AbilityDef::activated_mana(
            AbilityId(1),
            "{T}: Add {U} or {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaKindDef::Blue,
                ManaKindDef::Red,
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static THINK_TWICE: CardRecord = CardRecord::new(
    cards::THINK_TWICE,
    "Think Twice",
    CardArt::new("53e44060-a9a2-4095-9f5b-f60297525315", "Anthony Francisco"),
    CardSet::Innistrad,
        CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 0, 1, 0, 0, 0),
        "Draw a card.\nFlashback {2}{U} (You may cast this card from your graveyard for its flashback cost. Then exile it.)",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static UNBURIAL_RITES: CardRecord = CardRecord::new(
    cards::UNBURIAL_RITES,
    "Unburial Rites",
    CardArt::new("2794c82b-e5ce-4369-894e-bf56c6402ae1", "Ryan Pancoast"),
    CardSet::Innistrad,
        CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(4, 0, 0, 1, 0, 0),
        "Return target creature card from your graveyard to the battlefield.\nFlashback {3}{W} (You may cast this card from your graveyard for its flashback cost. Then exile it.)",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static URGENT_EXORCISM: CardRecord = CardRecord::new(
    cards::URGENT_EXORCISM,
    "Urgent Exorcism",
    CardArt::new("516a437c-a2ee-43c6-876c-1a63a455c97c", "Svetlin Velinov"),
    CardSet::Innistrad,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 1, 0, 0, 0, 0),
        "Destroy target Spirit or enchantment.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static WOODLAND_CEMETERY: CardRecord = CardRecord::new(
    cards::WOODLAND_CEMETERY,
    "Woodland Cemetery",
    CardArt::new("67139101-ec5e-434b-be3a-21338cc33840", "Lars Grant-West"),
    CardSet::Innistrad,
        CardRules::new(CardKind::Land, ManaCost::colored(0, 0, 0, 0, 0, 0), "")
    .land_entry(LandEntry::TappedUnlessControlsLandType([
        false, false, true, false, true,
    ]))
    .with_abilities(&[
        AbilityDef::replacement(
            AbilityId::PRIMARY,
            "This land enters tapped unless you control a Swamp or a Forest.",
            EffectDef::Special("Apply the declared conditional land-entry procedure"),
        )
        .with_implementation(AbilityImplementationDef::CustomFull {
            behavior: None,
            explanation: "The conditional land-entry procedure is implemented by shared land-entry rules.",
        }),
        AbilityDef::activated_mana(
            AbilityId(1),
            "{T}: Add {B} or {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaKindDef::Black,
                ManaKindDef::Green,
            ])),
        ),
    ]),
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
