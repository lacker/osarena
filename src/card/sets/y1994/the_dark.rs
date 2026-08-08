use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityImplementationDef, AbilityTargetDef, AbilityTargetPredicate,
    AppliedEffectDef, CardArt, CardBehavior, CardKind, CardRules, CardSet, EffectDef,
    EffectDurationDef, EffectRecipientDef, EvergreenAbility, ManaCost, ObjectPredicateDef,
    PlayerRelation, TriggerEventDef, TurnStepDef, ZoneKind, cards,
};
use crate::ids::{AbilityId, TargetSlotId};

pub(in crate::card::sets) static BALL_LIGHTNING: CardRecord = CardRecord::new(
    cards::BALL_LIGHTNING,
    "Ball Lightning",
    CardArt::new("c1ba83ab-83f5-421d-bba1-0f925870b5c8", "Quinton Hoover"),
    CardSet::TheDark,
    CardRules::new(CardKind::Creature, ManaCost::new(0, 3), "")
    .creature(6, 1)
    .with_subtypes(&["Elemental"])
    .with_abilities(&[
        AbilityDef::evergreen(
            AbilityId::PRIMARY,
            "Trample (This creature can deal excess combat damage to the player or planeswalker it's attacking.)",
            EvergreenAbility::Trample,
        ),
        AbilityDef::evergreen(
            AbilityId(1),
            "Haste (This creature can attack and {T} as soon as it comes under your control.)",
            EvergreenAbility::Haste,
        ),
        AbilityDef::triggered(
            AbilityId(2),
            "At the beginning of the end step, sacrifice this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

pub(in crate::card::sets) static BLOOD_MOON: CardRecord = CardRecord::new(
    cards::BLOOD_MOON,
    "Blood Moon",
    CardArt::new("78373616-e2d6-4ccf-998f-09f02bea45b4", "Tom Wänerstrand"),
    CardSet::TheDark,
    CardRules::new(CardKind::Enchantment, ManaCost::new(2, 1), "").with_abilities(&[
        AbilityDef::static_ability(
            AbilityId::PRIMARY,
            "Nonbasic lands are Mountains.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::Special("nonbasic land"),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                effect: AppliedEffectDef::Special(
                    "Set land subtypes to Mountain and apply the intrinsic Mountain mana ability",
                ),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        )
        .with_implementation(AbilityImplementationDef::CustomPartial {
            behavior: Some(CardBehavior::BloodMoon),
            explanation: "The hard-coded transformation does not yet use the full land-type, ability-loss, and layer system.",
        }),
    ]),
);

pub(in crate::card::sets) static GOBLIN_DIGGING_TEAM: CardRecord = CardRecord::new(
    cards::GOBLIN_DIGGING_TEAM,
    "Goblin Digging Team",
    CardArt::new("8a538b9d-351e-40bb-be11-9ba08c16352b", "Ron Spencer"),
    CardSet::TheDark,
    CardRules::new(CardKind::Creature, ManaCost::new(0, 1), "")
        .creature(1, 1)
        .with_subtypes(&["Goblin"])
        .with_abilities(&[AbilityDef::not_implemented(
            AbilityId::PRIMARY,
            "{T}, Sacrifice this creature: Destroy target Wall.",
            "The activated sacrifice ability and Wall targeting are not implemented.",
        )]),
);

pub(in crate::card::sets) static GOBLINS_OF_THE_FLARG: CardRecord = CardRecord::new(
    cards::GOBLINS_OF_THE_FLARG,
    "Goblins of the Flarg",
    CardArt::new("fd333b18-b896-4ab8-9c46-eed4efdd94f2", "Tom Wänerstrand"),
    CardSet::TheDark,
    CardRules::new(CardKind::Creature, ManaCost::new(0, 1), "")
        .creature(1, 1)
        .with_subtypes(&["Goblin", "Warrior"])
        .with_abilities(&[
            AbilityDef::evergreen(
                AbilityId::PRIMARY,
                "Mountainwalk (This creature can't be blocked as long as defending player controls a Mountain.)",
                EvergreenAbility::Mountainwalk,
            ),
            AbilityDef::not_implemented(
                AbilityId(1),
                "When you control a Dwarf, sacrifice this creature.",
                "The state-triggered sacrifice condition is not implemented.",
            ),
        ]),
);

pub(in crate::card::sets) static FELLWAR_STONE: CardRecord = CardRecord::new(
    cards::FELLWAR_STONE,
    "Fellwar Stone",
    CardArt::new("dc47e322-f8b8-4685-b035-fda0cc433e6b", "Quinton Hoover"),
    CardSet::TheDark,
    CardRules::new(CardKind::Artifact, ManaCost::new(2, 0), "").with_abilities(&[
        AbilityDef::activated_mana(
            AbilityId::PRIMARY,
            "{T}: Add one mana of any color that a land an opponent controls could produce.",
            &[AbilityCostDef::TapSource],
            EffectDef::Special("Add one mana of a color an opponent's land could produce"),
        )
        .with_implementation(AbilityImplementationDef::CustomFull {
            behavior: Some(CardBehavior::FellwarStone),
            explanation: "The available colors are computed dynamically from an opponent's lands.",
        }),
    ]),
);

pub(in crate::card::sets) static MAZE_OF_ITH: CardRecord = CardRecord::new(
    cards::MAZE_OF_ITH,
    "Maze of Ith",
    CardArt::new("42dcceee-2a47-4eaa-a6a3-2931b3d50244", "Anson Maddocks"),
    CardSet::TheDark,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "")
        .with_abilities(&[
            AbilityDef::activated(
                AbilityId::PRIMARY,
                "{T}: Untap target attacking creature. Prevent all combat damage that would be dealt to and dealt by that creature this turn.",
                &[AbilityCostDef::TapSource],
                EffectDef::Special(
                    "Untap the target attacker and prevent its combat damage for the turn",
                ),
            )
            .with_targets(&[AbilityTargetDef::exactly_one(
                TargetSlotId(0),
                "attacking creature",
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Special("attacking creature"),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )])
            .with_activation_text(
                "Untap {} and take it out of combat",
                "Take an attacker out of combat",
            )
            .with_implementation(AbilityImplementationDef::CustomPartial {
                behavior: Some(CardBehavior::MazeOfIth),
                explanation: "The implementation removes the attacker from combat instead of creating combat-damage prevention.",
            }),
        ]),
);

pub(in crate::card::sets) static DUST_TO_DUST: CardRecord = CardRecord::new(
    cards::DUST_TO_DUST,
    "Dust to Dust",
    CardArt::new("ade075fd-73ee-4d12-a2da-48e5938043af", "Drew Tucker"),
    CardSet::TheDark,
    CardRules::new(CardKind::Sorcery, ManaCost::colored(1, 2, 0, 0, 0, 0), "").with_abilities(&[
        AbilityDef::custom_full(
            AbilityId::PRIMARY,
            "Exile two target artifacts.",
            CardBehavior::DustToDust,
            "Artifact targeting and exile are implemented by the legacy spell resolver.",
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BALL_LIGHTNING,
    &BLOOD_MOON,
    &GOBLIN_DIGGING_TEAM,
    &GOBLINS_OF_THE_FLARG,
    &FELLWAR_STONE,
    &MAZE_OF_ITH,
    &DUST_TO_DUST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
