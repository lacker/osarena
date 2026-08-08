//! Return to Ravnica card records used by the built-in ISD–RTR Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityImplementationDef, AddManaEffectDef, CardArt,
    CardComposition, CardEffectStatus, CardKind, CardPart, CardRules, CardSet, CardStructure,
    CardSupertype, EffectDef, EvergreenAbility, LandEntry, ManaCost, ManaKindDef, ModeDef,
    ModeSetDef, PlayOptionDef, SpellForm, TargetPredicate, TargetSlotDef, cards,
};
use crate::ids::{AbilityId, CardPartId, ModeId, PlayOptionId, TargetSlotId};

pub(in crate::card::sets) static ABRUPT_DECAY: CardRecord = CardRecord::new(
    cards::ABRUPT_DECAY,
    "Abrupt Decay",
    CardArt::new("3b1e92b4-6e53-4dba-a572-c67e01965ac5", "Svetlin Velinov"),
    CardSet::ReturnToRavnica,
        CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 0, 1, 0, 1),
        "This spell can't be countered.\nDestroy target nonland permanent with mana value 3 or less.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static ANGEL_OF_SERENITY: CardRecord = CardRecord::new(
    cards::ANGEL_OF_SERENITY,
    "Angel of Serenity",
    CardArt::new("f10d82f7-7759-457e-a9bb-f9a5bd968f82", "Aleksi Briclot"),
    CardSet::ReturnToRavnica,
        CardRules::new(CardKind::Creature, ManaCost::colored(4, 3, 0, 0, 0, 0), "")
    .with_subtypes(&["Angel"])
    .creature(5, 6)
    .with_abilities(&[
        AbilityDef::evergreen(AbilityId::PRIMARY, "Flying", EvergreenAbility::Flying),
        AbilityDef::not_implemented(
            AbilityId(1),
            "When this creature enters, you may exile up to three other target creatures from the battlefield and/or creature cards from graveyards.",
            "The enters-the-battlefield exile ability is not executed.",
        ),
        AbilityDef::not_implemented(
            AbilityId(2),
            "When this creature leaves the battlefield, return the exiled cards to their owners' hands.",
            "The leaves-the-battlefield return ability is not executed.",
        ),
    ]),
);

pub(in crate::card::sets) static AZORIUS_CHARM: CardRecord = CardRecord::new(
    cards::AZORIUS_CHARM,
    "Azorius Charm",
    CardArt::new("26adc211-d089-4102-91e5-225bbeb5f382", "Zoltan Boros"),
    CardSet::ReturnToRavnica,
        CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 1, 1, 0, 0, 0),
        "Choose one —\n• Creatures you control gain lifelink until end of turn.\n• Draw a card.\n• Put target attacking or blocking creature on top of its owner's library.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static COUNTERFLUX: CardRecord = CardRecord::new(
    cards::COUNTERFLUX,
    "Counterflux",
    CardArt::new("94e4b773-40a4-4272-85dd-f728ada22748", "Scott M. Fischer"),
    CardSet::ReturnToRavnica,
        CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 2, 0, 1, 0),
        "This spell can't be countered.\nCounter target spell you don't control.\nOverload {1}{U}{U}{R} (You may cast this spell for its overload cost. If you do, change \"target\" in its text to \"each.\")",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static DESECRATION_DEMON: CardRecord = CardRecord::new(
    cards::DESECRATION_DEMON,
    "Desecration Demon",
    CardArt::new("8242fade-754c-4404-b3fb-f3cccf84b3b6", "Jason Chan"),
    CardSet::ReturnToRavnica,
        CardRules::new(CardKind::Creature, ManaCost::colored(2, 0, 0, 2, 0, 0), "")
    .with_subtypes(&["Demon"])
    .creature(6, 6)
    .with_abilities(&[
        AbilityDef::evergreen(AbilityId::PRIMARY, "Flying", EvergreenAbility::Flying),
        AbilityDef::not_implemented(
            AbilityId(1),
            "At the beginning of each combat, any opponent may sacrifice a creature of their choice. If a player does, tap this creature and put a +1/+1 counter on it.",
            "The beginning-of-combat trigger and opponent choice are not executed.",
        ),
    ]),
);

pub(in crate::card::sets) static DETENTION_SPHERE: CardRecord = CardRecord::new(
    cards::DETENTION_SPHERE,
    "Detention Sphere",
    CardArt::new("afee5464-83b7-4d7a-b407-9ee7de21535b", "Kev Walker"),
    CardSet::ReturnToRavnica,
        CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(1, 1, 1, 0, 0, 0),
        "When this enchantment enters, you may exile target nonland permanent not named Detention Sphere and all other permanents with the same name as that permanent.\nWhen this enchantment leaves the battlefield, return the exiled cards to the battlefield under their owner's control.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static DISPEL: CardRecord = CardRecord::new(
    cards::DISPEL,
    "Dispel",
    CardArt::new("08d4a8d7-c136-472f-8146-a1100701ca4f", "Chase Stone"),
    CardSet::ReturnToRavnica,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 1, 0, 0, 0),
        "Counter target instant spell.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static GOLGARI_GUILDGATE: CardRecord = CardRecord::new(
    cards::GOLGARI_GUILDGATE,
    "Golgari Guildgate",
    CardArt::new("8fe2fd1a-f7d3-48b4-bad8-be5ee45d6121", "Eytan Zana"),
    CardSet::ReturnToRavnica,
        CardRules::new(CardKind::Land, ManaCost::colored(0, 0, 0, 0, 0, 0), "")
    .with_subtypes(&["Gate"])
    .land_entry(LandEntry::Tapped)
    .with_abilities(&[
        AbilityDef::replacement(
            AbilityId::PRIMARY,
            "This land enters tapped.",
            EffectDef::Special("Have this land enter tapped"),
        )
        .with_implementation(AbilityImplementationDef::CustomFull {
            behavior: None,
            explanation: "The tapped land-entry procedure is implemented by shared land-entry rules.",
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

pub(in crate::card::sets) static GRISLY_SALVAGE: CardRecord = CardRecord::new(
    cards::GRISLY_SALVAGE,
    "Grisly Salvage",
    CardArt::new("dcb5eb2a-ae7a-4416-970c-6e9306689c88", "Dave Kendall"),
    CardSet::ReturnToRavnica,
        CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 0, 1, 0, 1),
        "Reveal the top five cards of your library. You may put a creature or land card from among them into your hand. Put the rest into your graveyard.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static HALLOWED_FOUNTAIN: CardRecord = CardRecord::new(
    cards::HALLOWED_FOUNTAIN,
    "Hallowed Fountain",
    CardArt::new("af7091c9-5f98-4078-a42b-c9e057346d9b", "Jung Park"),
    CardSet::ReturnToRavnica,
        CardRules::new(CardKind::Land, ManaCost::colored(0, 0, 0, 0, 0, 0), "")
    .with_subtypes(&["Plains", "Island"])
    .land_entry(LandEntry::PayLifeOrTapped(2))
    .with_abilities(&[
        AbilityDef::replacement(
            AbilityId::PRIMARY,
            "As this land enters, you may pay 2 life. If you don't, it enters tapped.",
            EffectDef::Special("Choose whether to pay 2 life or have this land enter tapped"),
        )
        .with_implementation(AbilityImplementationDef::NotImplemented {
            explanation: "Choosing to pay 2 life instead of having this land enter tapped is not executed.",
        }),
        AbilityDef::activated_mana(
            AbilityId(1),
            "{T}: Add {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::White)),
        ),
        AbilityDef::activated_mana(
            AbilityId(2),
            "{T}: Add {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Blue)),
        ),
    ]),
);

const fn izzet_charm_rules() -> CardRules {
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 1, 0, 1, 0),
        "Choose one —\n• Counter target noncreature spell unless its controller pays {2}.\n• Izzet Charm deals 2 damage to target creature.\n• Draw two cards, then discard two cards.",
    )
    .metadata_only()
}

fn izzet_charm_composition() -> CardComposition {
    let rules = izzet_charm_rules();
    let modes = ModeSetDef::choose_one(vec![
        ModeDef {
            id: ModeId(0),
            label: "Counter a noncreature spell unless its controller pays {2}".into(),
            targets: vec![TargetSlotDef::exactly_one(
                TargetSlotId(0),
                "noncreature spell",
                TargetPredicate::NoncreatureSpell,
            )],
            effect_status: CardEffectStatus::MetadataOnly,
        },
        ModeDef {
            id: ModeId(1),
            label: "Deal 2 damage to a creature".into(),
            targets: vec![TargetSlotDef::exactly_one(
                TargetSlotId(1),
                "creature",
                TargetPredicate::CreaturePermanent,
            )],
            effect_status: CardEffectStatus::MetadataOnly,
        },
        ModeDef {
            id: ModeId(2),
            label: "Draw two cards, then discard two cards".into(),
            targets: Vec::new(),
            effect_status: CardEffectStatus::MetadataOnly,
        },
    ]);
    CardComposition {
        parts: vec![CardPart::new(CardPartId::PRIMARY, "Izzet Charm", rules)],
        structure: CardStructure::Single {
            main: CardPartId::PRIMARY,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Izzet Charm",
                SpellForm::Part(CardPartId::PRIMARY),
                rules.mana_cost,
                CardEffectStatus::MetadataOnly,
            )
            .with_modes(modes),
        ],
    }
}

pub(in crate::card::sets) static IZZET_CHARM: CardRecord = CardRecord::new(
    cards::IZZET_CHARM,
    "Izzet Charm",
    CardArt::new("1e3a5af6-5423-442b-a207-364e97a871d8", "Zoltan Boros"),
    CardSet::ReturnToRavnica,
    izzet_charm_rules(),
)
.with_composition(izzet_charm_composition);

pub(in crate::card::sets) static IZZET_STATICASTER: CardRecord = CardRecord::new(
    cards::IZZET_STATICASTER,
    "Izzet Staticaster",
    CardArt::new("190ac2fe-532d-4d7e-9d74-07ae6850aac8", "Scott M. Fischer"),
    CardSet::ReturnToRavnica,
        CardRules::new(CardKind::Creature, ManaCost::colored(1, 0, 1, 0, 1, 0), "")
    .with_subtypes(&["Human", "Wizard"])
    .creature(0, 3)
    .with_abilities(&[
        AbilityDef::evergreen(
            AbilityId::PRIMARY,
            "Flash (You may cast this spell any time you could cast an instant.)",
            EvergreenAbility::Flash,
        ),
        AbilityDef::evergreen(AbilityId(1), "Haste", EvergreenAbility::Haste),
        AbilityDef::not_implemented(
            AbilityId(2),
            "{T}: This creature deals 1 damage to target creature and each other creature with the same name as that creature.",
            "The targeted activated damage ability is not executed.",
        ),
    ]),
);

pub(in crate::card::sets) static JACE_ARCHITECT_OF_THOUGHT: CardRecord = CardRecord::new(
    cards::JACE_ARCHITECT_OF_THOUGHT,
    "Jace, Architect of Thought",
    CardArt::new("d4df3a38-678e-42dc-a3fd-d1d399368f07", "Jaime Jones"),
    CardSet::ReturnToRavnica,
        CardRules::new(
        CardKind::Planeswalker,
        ManaCost::colored(2, 0, 2, 0, 0, 0),
        "+1: Until your next turn, whenever a creature an opponent controls attacks, it gets -1/-0 until end of turn.\n−2: Reveal the top three cards of your library. An opponent separates those cards into two piles. Put one pile into your hand and the other on the bottom of your library in any order.\n−8: For each player, search that player's library for a nonland card and exile it, then that player shuffles. You may cast those cards without paying their mana costs.",
    )
    .with_supertype(CardSupertype::Legendary)
    .with_subtypes(&["Jace"])
    .planeswalker(4)
    .metadata_only(),
);

pub(in crate::card::sets) static LOXODON_SMITER: CardRecord = CardRecord::new(
    cards::LOXODON_SMITER,
    "Loxodon Smiter",
    CardArt::new("69247168-2bfb-4cce-a2a6-61459a0fbce4", "Ryan Barger"),
    CardSet::ReturnToRavnica,
        CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 1, 0, 0, 0, 1),
        "This spell can't be countered.\nIf a spell or ability an opponent controls causes you to discard this card, put it onto the battlefield instead of putting it into your graveyard.",
    )
    .with_subtypes(&["Elephant", "Soldier"])
    .creature(4, 4)
    .metadata_only(),
);

pub(in crate::card::sets) static MIZZIUM_MORTARS: CardRecord = CardRecord::new(
    cards::MIZZIUM_MORTARS,
    "Mizzium Mortars",
    CardArt::new("d4ded88d-2688-4f5e-a8b2-16216cf9c792", "Noah Bradley"),
    CardSet::ReturnToRavnica,
        CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(1, 0, 0, 0, 1, 0),
        "Mizzium Mortars deals 4 damage to target creature you don't control.\nOverload {3}{R}{R}{R} (You may cast this spell for its overload cost. If you do, change \"target\" in its text to \"each.\")",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static OVERGROWN_TOMB: CardRecord = CardRecord::new(
    cards::OVERGROWN_TOMB,
    "Overgrown Tomb",
    CardArt::new("1c7d50d6-b63a-4d8c-88fa-1d78ae693a45", "Steven Belledin"),
    CardSet::ReturnToRavnica,
        CardRules::new(CardKind::Land, ManaCost::colored(0, 0, 0, 0, 0, 0), "")
    .with_subtypes(&["Swamp", "Forest"])
    .land_entry(LandEntry::PayLifeOrTapped(2))
    .with_abilities(&[
        AbilityDef::replacement(
            AbilityId::PRIMARY,
            "As this land enters, you may pay 2 life. If you don't, it enters tapped.",
            EffectDef::Special("Choose whether to pay 2 life or have this land enter tapped"),
        )
        .with_implementation(AbilityImplementationDef::NotImplemented {
            explanation: "Choosing to pay 2 life instead of having this land enter tapped is not executed.",
        }),
        AbilityDef::activated_mana(
            AbilityId(1),
            "{T}: Add {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Black)),
        ),
        AbilityDef::activated_mana(
            AbilityId(2),
            "{T}: Add {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Green)),
        ),
    ]),
);

pub(in crate::card::sets) static PITHING_NEEDLE: CardRecord = CardRecord::new(
    cards::PITHING_NEEDLE,
    "Pithing Needle",
    CardArt::new("786c1e91-9d75-46a3-9e0d-56d29fcb01a7", "Anthony Palumbo"),
    CardSet::ReturnToRavnica,
        CardRules::new(
        CardKind::Artifact,
        ManaCost::colored(1, 0, 0, 0, 0, 0),
        "As this artifact enters, choose a card name.\nActivated abilities of sources with the chosen name can't be activated unless they're mana abilities.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static REST_IN_PEACE: CardRecord = CardRecord::new(
    cards::REST_IN_PEACE,
    "Rest in Peace",
    CardArt::new("37c2b1d1-faa0-40fd-82f4-216604ce7635", "Terese Nielsen"),
    CardSet::ReturnToRavnica,
        CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(1, 1, 0, 0, 0, 0),
        "When this enchantment enters, exile all graveyards.\nIf a card or token would be put into a graveyard from anywhere, exile it instead.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static SELESNYA_CHARM: CardRecord = CardRecord::new(
    cards::SELESNYA_CHARM,
    "Selesnya Charm",
    CardArt::new("a9848eab-1d3a-4ab0-adf6-c20858aa3afb", "Zoltan Boros"),
    CardSet::ReturnToRavnica,
        CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 1, 0, 0, 0, 1),
        "Choose one —\n• Target creature gets +2/+2 and gains trample until end of turn.\n• Exile target creature with power 5 or greater.\n• Create a 2/2 white Knight creature token with vigilance.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static SPHINXS_REVELATION: CardRecord = CardRecord::new(
    cards::SPHINXS_REVELATION,
    "Sphinx's Revelation",
    CardArt::new("404d9413-ef57-4b6e-8584-48a1dc7fe6f1", "Slawomir Maniak"),
    CardSet::ReturnToRavnica,
    CardRules::new(
        CardKind::Instant,
        ManaCost::variable(0, 1, 2, 0, 0, 0, 1),
        "You gain X life and draw X cards.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static STEAM_VENTS: CardRecord = CardRecord::new(
    cards::STEAM_VENTS,
    "Steam Vents",
    CardArt::new("de911c88-f5c8-4955-9fa5-1f28a9b17236", "Yeong-Hao Han"),
    CardSet::ReturnToRavnica,
        CardRules::new(CardKind::Land, ManaCost::colored(0, 0, 0, 0, 0, 0), "")
    .with_subtypes(&["Island", "Mountain"])
    .land_entry(LandEntry::PayLifeOrTapped(2))
    .with_abilities(&[
        AbilityDef::replacement(
            AbilityId::PRIMARY,
            "As this land enters, you may pay 2 life. If you don't, it enters tapped.",
            EffectDef::Special("Choose whether to pay 2 life or have this land enter tapped"),
        )
        .with_implementation(AbilityImplementationDef::NotImplemented {
            explanation: "Choosing to pay 2 life instead of having this land enter tapped is not executed.",
        }),
        AbilityDef::activated_mana(
            AbilityId(1),
            "{T}: Add {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Blue)),
        ),
        AbilityDef::activated_mana(
            AbilityId(2),
            "{T}: Add {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Red)),
        ),
    ]),
);

pub(in crate::card::sets) static SUPREME_VERDICT: CardRecord = CardRecord::new(
    cards::SUPREME_VERDICT,
    "Supreme Verdict",
    CardArt::new("4e9648f9-7a67-4717-bca1-861d1f7fed43", "Sam Burley"),
    CardSet::ReturnToRavnica,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(1, 2, 1, 0, 0, 0),
        "This spell can't be countered.\nDestroy all creatures.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static SYNCOPATE: CardRecord = CardRecord::new(
    cards::SYNCOPATE,
    "Syncopate",
    CardArt::new("ba6f218f-83b0-4b68-a00f-0327cd79f32a", "Clint Cearley"),
    CardSet::ReturnToRavnica,
        CardRules::new(
        CardKind::Instant,
        ManaCost::variable(0, 0, 1, 0, 0, 0, 1),
        "Counter target spell unless its controller pays {X}. If that spell is countered this way, exile it instead of putting it into its owner's graveyard.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static TEMPLE_GARDEN: CardRecord = CardRecord::new(
    cards::TEMPLE_GARDEN,
    "Temple Garden",
    CardArt::new("b821e604-f9fd-47a4-b5ff-bfb5022834c2", "Volkan Baǵa"),
    CardSet::ReturnToRavnica,
        CardRules::new(CardKind::Land, ManaCost::colored(0, 0, 0, 0, 0, 0), "")
    .with_subtypes(&["Forest", "Plains"])
    .land_entry(LandEntry::PayLifeOrTapped(2))
    .with_abilities(&[
        AbilityDef::replacement(
            AbilityId::PRIMARY,
            "As this land enters, you may pay 2 life. If you don't, it enters tapped.",
            EffectDef::Special("Choose whether to pay 2 life or have this land enter tapped"),
        )
        .with_implementation(AbilityImplementationDef::NotImplemented {
            explanation: "Choosing to pay 2 life instead of having this land enter tapped is not executed.",
        }),
        AbilityDef::activated_mana(
            AbilityId(1),
            "{T}: Add {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Green)),
        ),
        AbilityDef::activated_mana(
            AbilityId(2),
            "{T}: Add {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::White)),
        ),
    ]),
);

pub(in crate::card::sets) static ULTIMATE_PRICE: CardRecord = CardRecord::new(
    cards::ULTIMATE_PRICE,
    "Ultimate Price",
    CardArt::new("d2b4912a-83a2-4870-8fac-81fa79da2830", "Karl Kopinski"),
    CardSet::ReturnToRavnica,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 0, 0, 1, 0, 0),
        "Destroy target monocolored creature.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static UNDERWORLD_CONNECTIONS: CardRecord = CardRecord::new(
    cards::UNDERWORLD_CONNECTIONS,
    "Underworld Connections",
    CardArt::new("19c52e3b-b3b8-4243-96fe-fa4c8eea7c59", "Yeong-Hao Han"),
    CardSet::ReturnToRavnica,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(1, 0, 0, 2, 0, 0),
        "Enchant land\nEnchanted land has \"{T}, Pay 1 life: Draw a card.\"",
    )
    .with_subtypes(&["Aura"])
    .metadata_only(),
);

pub(in crate::card::sets) static VRASKA_THE_UNSEEN: CardRecord = CardRecord::new(
    cards::VRASKA_THE_UNSEEN,
    "Vraska the Unseen",
    CardArt::new("8971938c-cd26-4b83-96d7-1408cd0b0de6", "Aleksi Briclot"),
    CardSet::ReturnToRavnica,
        CardRules::new(
        CardKind::Planeswalker,
        ManaCost::colored(3, 0, 0, 1, 0, 1),
        "+1: Until your next turn, whenever a creature deals combat damage to Vraska, destroy that creature.\n−3: Destroy target nonland permanent.\n−7: Create three 1/1 black Assassin creature tokens with \"Whenever this token deals combat damage to a player, that player loses the game.\"",
    )
    .with_supertype(CardSupertype::Legendary)
    .with_subtypes(&["Vraska"])
    .planeswalker(5)
    .metadata_only(),
);
pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ABRUPT_DECAY,
    &ANGEL_OF_SERENITY,
    &AZORIUS_CHARM,
    &COUNTERFLUX,
    &DESECRATION_DEMON,
    &DETENTION_SPHERE,
    &DISPEL,
    &GOLGARI_GUILDGATE,
    &GRISLY_SALVAGE,
    &HALLOWED_FOUNTAIN,
    &IZZET_CHARM,
    &IZZET_STATICASTER,
    &JACE_ARCHITECT_OF_THOUGHT,
    &LOXODON_SMITER,
    &MIZZIUM_MORTARS,
    &OVERGROWN_TOMB,
    &PITHING_NEEDLE,
    &REST_IN_PEACE,
    &SELESNYA_CHARM,
    &SPHINXS_REVELATION,
    &STEAM_VENTS,
    &SUPREME_VERDICT,
    &SYNCOPATE,
    &TEMPLE_GARDEN,
    &ULTIMATE_PRICE,
    &UNDERWORLD_CONNECTIONS,
    &VRASKA_THE_UNSEEN,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
