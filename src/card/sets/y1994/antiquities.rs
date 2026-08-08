use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityImplementationDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AppliedEffectDef, CardArt, CardBehavior, CardKind, CardRules, CardSet,
    EffectDef, EffectDurationDef, EffectRecipientDef, EvergreenAbility, ManaCost, ManaKindDef,
    ObjectPredicateDef, PlayerRelation, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, cards,
};
use crate::ids::{AbilityId, TargetSlotId};

pub(in crate::card::sets) static ATOG: CardRecord = CardRecord::new(
    cards::ATOG,
    "Atog",
    CardArt::new("2249fc40-4412-48fd-800a-7ea3678aee3f", "Jesper Myrfors"),
    CardSet::Antiquities,
    CardRules::new(CardKind::Creature, ManaCost::new(1, 1), "")
        .creature(1, 2)
        .with_subtypes(&["Atog"])
        .with_abilities(&[AbilityDef::custom_partial(
            AbilityId::PRIMARY,
            "Sacrifice an artifact: This creature gets +2/+2 until end of turn.",
            CardBehavior::Atog,
            "The activated ability currently resolves immediately instead of using the stack.",
        )]),
);

pub(in crate::card::sets) static DETONATE: CardRecord = CardRecord::new(
    cards::DETONATE,
    "Detonate",
    CardArt::new(
        "ffd7eb90-ae95-49df-898a-9510187bce1c",
        "Randy Asplund-Faith",
    ),
    CardSet::Antiquities,
    CardRules::new(CardKind::Sorcery, ManaCost::with_x(1), "").with_abilities(&[
        AbilityDef::custom_partial(
            AbilityId::PRIMARY,
            "Destroy target artifact with mana value X. It can't be regenerated. Detonate deals X damage to that artifact's controller.",
            CardBehavior::Detonate,
            "Artifact destruction and damage are implemented by the legacy resolver, but the no-regeneration clause is not enforced.",
        ),
    ]),
);

pub(in crate::card::sets) static SU_CHI: CardRecord = CardRecord::new(
    cards::SU_CHI,
    "Su-Chi",
    CardArt::new("a64d4f93-0c04-4078-aec0-7e9de92f260f", "Christopher Rush"),
    CardSet::Antiquities,
    CardRules::new(CardKind::ArtifactCreature, ManaCost::new(4, 0), "")
        .creature(4, 4)
        .with_subtypes(&["Construct"])
        .with_abilities(&[AbilityDef::triggered(
            AbilityId::PRIMARY,
            "When this creature dies, add {C}{C}{C}{C}.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Colorless).with_amount(4)),
        )]),
);

pub(in crate::card::sets) static MISHRA_S_FACTORY: CardRecord = CardRecord::new(
    cards::MISHRA_S_FACTORY,
    "Mishra's Factory",
    CardArt::new("a696c5b6-f216-454d-8029-74e84bbd1428", "Kaja Foglio & Phil Foglio"),
    CardSet::Antiquities,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "")
        .with_abilities(&[
            AbilityDef::activated_mana(
                AbilityId::PRIMARY,
                "{T}: Add {C}.",
                &[AbilityCostDef::TapSource],
                EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Colorless)),
            ),
            AbilityDef::activated(
                AbilityId(1),
                "{1}: This land becomes a 2/2 Assembly-Worker artifact creature until end of turn. It's still a land.",
                &[AbilityCostDef::Mana(ManaCost::new(1, 0))],
                EffectDef::Special("Animate this land as a 2/2 Assembly-Worker artifact creature"),
            )
            .with_implementation(AbilityImplementationDef::CustomPartial {
                behavior: Some(CardBehavior::MishrasFactory),
                explanation: "The animation ability is implemented, but currently resolves immediately instead of using the stack.",
            }),
            AbilityDef::activated(
                AbilityId(2),
                "{T}: Target Assembly-Worker creature gets +1/+1 until end of turn.",
                &[AbilityCostDef::TapSource],
                EffectDef::Special("Give the target Assembly-Worker +1/+1 until end of turn"),
            )
            .with_targets(&[AbilityTargetDef::exactly_one(
                TargetSlotId(0),
                "Assembly-Worker",
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Special("Assembly-Worker permanent"),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )])
            .with_implementation(AbilityImplementationDef::CustomPartial {
                behavior: Some(CardBehavior::MishrasFactory),
                explanation: "The pump ability is implemented, but currently resolves immediately instead of using the stack.",
            })
            .with_activation_text(
                "Give {} +1/+1 with Mishra's Factory",
                "Give an Assembly-Worker +1/+1",
            ),
        ]),
);

pub(in crate::card::sets) static ORCISH_MECHANICS: CardRecord = CardRecord::new(
    cards::ORCISH_MECHANICS,
    "Orcish Mechanics",
    CardArt::new("5e34fc6b-5f00-4a22-9ee2-afc1caf99961", "Pete Venters"),
    CardSet::Antiquities,
    CardRules::new(CardKind::Creature, ManaCost::new(2, 1), "")
        .creature(1, 1)
        .with_subtypes(&["Orc"])
        .with_abilities(&[
            AbilityDef::activated(
                AbilityId::PRIMARY,
                "{T}, Sacrifice an artifact: This creature deals 2 damage to any target.",
                &[
                    AbilityCostDef::TapSource,
                    AbilityCostDef::SacrificePermanent {
                        object: ObjectPredicateDef::Artifact,
                        controller: PlayerRelation::You,
                    },
                ],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetSlotId(0)),
                    amount: ValueDef::Constant(2),
                },
            )
            .with_targets(&[AbilityTargetDef::exactly_one(
                TargetSlotId(0),
                "any target",
                AbilityTargetPredicate::AnyTarget,
            )])
            .with_activation_text(
                "Deal 2 damage to {} with Orcish Mechanics",
                "Deal 2 damage",
            )
            .with_implementation(AbilityImplementationDef::CustomFull {
                behavior: Some(CardBehavior::OrcishMechanics),
                explanation: "The artifact sacrifice, target selection, and damage are implemented by the legacy activated-ability resolver.",
            }),
        ]),
);

pub(in crate::card::sets) static STRIP_MINE: CardRecord = CardRecord::new(
    cards::STRIP_MINE,
    "Strip Mine",
    CardArt::new("e7880157-7f27-4f1b-9cdc-ab36a6252376", "Daniel Gelon"),
    CardSet::Antiquities,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "").with_abilities(&[
        AbilityDef::activated_mana(
            AbilityId::PRIMARY,
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Colorless)),
        ),
        AbilityDef::activated(
            AbilityId(1),
            "{T}, Sacrifice this land: Destroy target land.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetSlotId(0)),
                can_regenerate: true,
            },
        )
        .with_targets(&[AbilityTargetDef::exactly_one(
            TargetSlotId(0),
            "land",
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Land,
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )])
        .with_activation_text("Destroy {} with Strip Mine", "Destroy a land"),
    ]),
);

pub(in crate::card::sets) static TRISKELION: CardRecord = CardRecord::new(
    cards::TRISKELION,
    "Triskelion",
    CardArt::new("a79c99e1-722a-44b6-8fa3-2be3f0c193d8", "Douglas Shuler"),
    CardSet::Antiquities,
    CardRules::new(CardKind::ArtifactCreature, ManaCost::new(6, 0), "")
        .creature(1, 1)
        .with_subtypes(&["Construct"])
        .with_abilities(&[
            AbilityDef::replacement(
                AbilityId::PRIMARY,
                "This creature enters with three +1/+1 counters on it.",
                EffectDef::Special("Enter with three +1/+1 counters"),
            )
            .with_implementation(AbilityImplementationDef::CustomPartial {
                behavior: Some(CardBehavior::Triskelion),
                explanation: "The entry counters are applied when Triskelion resolves normally, but copied Triskelion rules do not apply them.",
            }),
            AbilityDef::activated(
                AbilityId(1),
                "Remove a +1/+1 counter from this creature: It deals 1 damage to any target.",
                &[AbilityCostDef::Special(
                    "Remove a +1/+1 counter from this source",
                )],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetSlotId(0)),
                    amount: ValueDef::Constant(1),
                },
            )
            .with_targets(&[AbilityTargetDef::exactly_one(
                TargetSlotId(0),
                "any target",
                AbilityTargetPredicate::AnyTarget,
            )])
            .with_activation_text("Deal 1 damage to {} with Triskelion", "Deal 1 damage")
            .with_implementation(AbilityImplementationDef::CustomFull {
                behavior: Some(CardBehavior::Triskelion),
                explanation: "Counter removal, target selection, and damage are implemented by the legacy activated-ability resolver.",
            }),
        ]),
);

pub(in crate::card::sets) static IVORY_TOWER: CardRecord = CardRecord::new(
    cards::IVORY_TOWER,
    "Ivory Tower",
    CardArt::new(
        "a5f23039-45ca-4c15-af50-bfd40ea26453",
        "Margaret Organ-Kean",
    ),
    CardSet::Antiquities,
    CardRules::new(CardKind::Artifact, ManaCost::new(1, 0), "").with_abilities(&[
        AbilityDef::triggered(
            AbilityId::PRIMARY,
            "At the beginning of your upkeep, you gain X life, where X is the number of cards in your hand minus 4.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::CardsInHandAbove {
                    player: PlayerRelation::You,
                    threshold: 4,
                },
            },
        ),
    ]),
);

pub(in crate::card::sets) static MISHRA_S_WORKSHOP: CardRecord = CardRecord::new(
    cards::MISHRA_S_WORKSHOP,
    "Mishra's Workshop",
    CardArt::new("135de5c7-6ac9-4b68-8f1a-97f120a4b125", "Kaja Foglio"),
    CardSet::Antiquities,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "").with_abilities(&[
        AbilityDef::activated_mana(
            AbilityId::PRIMARY,
            "{T}: Add {C}{C}{C}. Spend this mana only to cast artifact spells.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Colorless).with_amount(3)),
        )
        .with_implementation(AbilityImplementationDef::CustomPartial {
            behavior: None,
            explanation: "The artifact-spell spending restriction is cataloged in the clause text but is not yet represented on the produced mana.",
        }),
    ]),
);

pub(in crate::card::sets) static ARGOTHIAN_PIXIES: CardRecord = CardRecord::new(
    cards::ARGOTHIAN_PIXIES,
    "Argothian Pixies",
    CardArt::new("5712e87a-2381-4f5b-a853-6973841f9bf1", "Amy Weber"),
    CardSet::Antiquities,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 0, 0, 0, 0, 1),
        "",
    )
    .creature(2, 1)
    .with_subtypes(&["Faerie"])
    .with_abilities(&[AbilityDef::custom_partial(
        AbilityId::PRIMARY,
        "This creature can't be blocked by artifact creatures.\nPrevent all damage that would be dealt to this creature by artifact creatures.",
        CardBehavior::ArgothianPixies,
        "The artifact-creature blocking restriction works, but damage from artifact creatures is not prevented.",
    )]),
);

pub(in crate::card::sets) static HURKYLS_RECALL: CardRecord = CardRecord::new(
    cards::HURKYLS_RECALL,
    "Hurkyl's Recall",
    CardArt::new("f32373dd-06d8-45d1-8777-3b1411bcb30a", "NéNé Thomas"),
    CardSet::Antiquities,
    CardRules::new(CardKind::Instant, ManaCost::colored(1, 0, 1, 0, 0, 0), "")
        .with_abilities(&[AbilityDef::custom_partial(
            AbilityId::PRIMARY,
            "Return all artifacts target player owns to their hand.",
            CardBehavior::HurkylsRecall,
            "The resolver currently returns artifacts the targeted player controls instead of artifacts they own.",
        )]),
);

pub(in crate::card::sets) static SAGE_OF_LAT_NAM: CardRecord = CardRecord::new(
    cards::SAGE_OF_LAT_NAM,
    "Sage of Lat-Nam",
    CardArt::new("b4ff60ce-073c-46b8-807c-8b40467b960c", "Pete Venters"),
    CardSet::Antiquities,
    CardRules::new(CardKind::Creature, ManaCost::colored(1, 0, 1, 0, 0, 0), "")
        .creature(1, 2)
        .with_subtypes(&["Human", "Artificer"])
        .with_abilities(&[AbilityDef::activated(
            AbilityId::PRIMARY,
            "{T}, Sacrifice an artifact: Draw a card.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::Artifact,
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )]),
);

pub(in crate::card::sets) static TETRAVUS: CardRecord = CardRecord::new(
    cards::TETRAVUS,
    "Tetravus",
    CardArt::new("23eb19f9-2e8f-4bf0-9bf8-868e6da70e2d", "Mark Tedin"),
    CardSet::Antiquities,
    CardRules::new(CardKind::ArtifactCreature, ManaCost::new(6, 0), "")
    .creature(1, 1)
    .with_subtypes(&["Construct"])
    .with_abilities(&[
        AbilityDef::evergreen(AbilityId(1), "Flying", EvergreenAbility::Flying),
        AbilityDef::replacement(
            AbilityId::PRIMARY,
            "This creature enters with three +1/+1 counters on it.",
            EffectDef::Special("Enter with three +1/+1 counters"),
        )
        .with_implementation(AbilityImplementationDef::CustomFull {
            behavior: Some(CardBehavior::Tetravus),
            explanation: "The entry counters are applied by the legacy permanent-entry resolver.",
        }),
        AbilityDef::not_implemented(
            AbilityId(2),
            "At the beginning of your upkeep, you may remove any number of +1/+1 counters from this creature. If you do, create that many 1/1 colorless Tetravite artifact creature tokens. They each have flying and \"This token can't be enchanted.\"",
            "Creating Tetravite tokens and choosing how many counters to remove are not implemented.",
        ),
        AbilityDef::not_implemented(
            AbilityId(3),
            "At the beginning of your upkeep, you may exile any number of tokens created with this creature. If you do, put that many +1/+1 counters on this creature.",
            "Exiling linked Tetravite tokens and returning their counters are not implemented.",
        ),
    ]),
);

static ENERGY_FLUX_GRANTED_ABILITY: AbilityDef = AbilityDef::triggered(
    AbilityId::PRIMARY,
    "At the beginning of your upkeep, sacrifice this artifact unless you pay {2}.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::Special("Sacrifice this artifact unless its controller pays {2}"),
)
.with_implementation(AbilityImplementationDef::NotImplemented {
    explanation: "The per-artifact upkeep trigger and its unless-payment branch are not executed yet.",
});

pub(in crate::card::sets) static ENERGY_FLUX: CardRecord = CardRecord::new(
    cards::ENERGY_FLUX,
    "Energy Flux",
    CardArt::new("bd1f624b-e8f2-462f-838a-7cb9e8fda988", "Kaja Foglio"),
    CardSet::Antiquities,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(2, 0, 1, 0, 0, 0),
        "",
    )
    .with_abilities(&[AbilityDef::static_ability(
        AbilityId::PRIMARY,
        "All artifacts have \"At the beginning of your upkeep, sacrifice this artifact unless you pay {2}.\"",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::Artifact,
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            effect: AppliedEffectDef::GrantAbility(&ENERGY_FLUX_GRANTED_ABILITY),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )
    .with_implementation(AbilityImplementationDef::NotImplemented {
        explanation: "Static ability granting, per-artifact upkeep triggers, and optional payments are represented but not executed yet.",
    })]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ATOG,
    &DETONATE,
    &SU_CHI,
    &MISHRA_S_FACTORY,
    &ORCISH_MECHANICS,
    &STRIP_MINE,
    &TRISKELION,
    &IVORY_TOWER,
    &MISHRA_S_WORKSHOP,
    &ARGOTHIAN_PIXIES,
    &HURKYLS_RECALL,
    &SAGE_OF_LAT_NAM,
    &TETRAVUS,
    &ENERGY_FLUX,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];

#[cfg(test)]
mod tests {
    use super::{ENERGY_FLUX, ENERGY_FLUX_GRANTED_ABILITY};
    use crate::card::{
        AbilityImplementationDef, AppliedEffectDef, CardEffectStatus, DeclarativeAbilityDef,
        EffectDef, ImplementationStatus,
    };

    #[test]
    fn energy_flux_models_its_granted_trigger_without_claiming_execution() {
        let definition = ENERGY_FLUX.definition();
        let clauses = definition.rules.ability_clauses();
        assert_eq!(clauses.len(), 1);
        assert_eq!(
            clauses[0].text,
            "All artifacts have \"At the beginning of your upkeep, sacrifice this artifact unless you pay {2}.\""
        );
        assert!(matches!(
            clauses[0].definition,
            DeclarativeAbilityDef::Static(_)
        ));
        assert!(matches!(
            clauses[0].implementation,
            AbilityImplementationDef::NotImplemented { .. }
        ));
        assert!(matches!(
            clauses[0].effect,
            EffectDef::Apply {
                effect: AppliedEffectDef::GrantAbility(granted),
                ..
            } if granted == &ENERGY_FLUX_GRANTED_ABILITY
        ));
        assert!(matches!(
            ENERGY_FLUX_GRANTED_ABILITY.definition,
            DeclarativeAbilityDef::Triggered(_)
        ));
        assert!(matches!(
            ENERGY_FLUX_GRANTED_ABILITY.implementation,
            AbilityImplementationDef::NotImplemented { .. }
        ));
        assert_eq!(
            definition.implementation_status(),
            ImplementationStatus::MetadataOnly
        );
        assert_eq!(
            definition.play_options[0].effect_status,
            CardEffectStatus::MetadataOnly
        );
    }
}
