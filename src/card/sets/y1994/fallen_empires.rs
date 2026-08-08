use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityImplementationDef, AbilityTargetDef, AbilityTargetPredicate,
    CardArt, CardBehavior, CardKind, CardRules, CardSet, EffectDef, EffectRecipientDef, ManaCost,
    ValueDef, cards,
};
use crate::ids::{AbilityId, TargetSlotId};

pub(in crate::card::sets) static GOBLIN_GRENADE: CardRecord = CardRecord::new(
    cards::GOBLIN_GRENADE,
    "Goblin Grenade",
    CardArt::new("8837eaba-9602-4f63-9897-85583fcdcf51", "Ron Spencer"),
    CardSet::FallenEmpires,
    false,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::new(0, 1),
        "As an additional cost, sacrifice a Goblin. Deal 5 damage to any target.",
    )
    .with_special_behavior(CardBehavior::GoblinGrenade),
);

pub(in crate::card::sets) static HYMN_TO_TOURACH: CardRecord = CardRecord::new(
    cards::HYMN_TO_TOURACH,
    "Hymn to Tourach",
    CardArt::new("eb9273ea-9a41-42e3-8c9c-0d50b127a818", "Susan Van Camp"),
    CardSet::FallenEmpires,
    false,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(0, 0, 0, 2, 0, 0),
        "Target player discards two cards at random.",
    )
    .partial("The spell always affects the opponent instead of selecting its target player.")
    .with_special_behavior(CardBehavior::HymnToTourach),
);

pub(in crate::card::sets) static ICATIAN_JAVELINEERS: CardRecord = CardRecord::new(
    cards::ICATIAN_JAVELINEERS,
    "Icatian Javelineers",
    CardArt::new("f04b8356-2384-4743-80dd-f15ca7ec65f7", "Melissa A. Benson"),
    CardSet::FallenEmpires,
    false,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 1, 0, 0, 0, 0),
        "",
    )
    .creature(1, 1)
    .with_abilities(&[
        AbilityDef::replacement(
            AbilityId::PRIMARY,
            "Enters with a javelin counter.",
            EffectDef::Special("Enter with one javelin counter"),
        )
        .with_implementation(AbilityImplementationDef::CustomFull {
            explanation: "The entry counter is applied by the legacy permanent-entry resolver.",
        }),
        AbilityDef::activated(
            AbilityId(1),
            "Tap, remove it: Deal 1 damage to any target.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::Special("Remove a javelin counter from this source"),
            ],
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
        .with_activation_text(
            "Deal 1 damage to {} with Icatian Javelineers",
            "Deal 1 damage",
        )
        .with_implementation(AbilityImplementationDef::CustomPartial {
            explanation: "Target selection and damage resolution do not account for protection from white.",
        }),
    ])
    .with_special_behavior(CardBehavior::IcatianJavelineers),
);

pub(in crate::card::sets) static ORDER_OF_LEITBUR: CardRecord = CardRecord::new(
    cards::ORDER_OF_LEITBUR,
    "Order of Leitbur",
    CardArt::new("ebd6e51e-f042-4673-a898-291607105829", "Bryon Wackwitz"),
    CardSet::FallenEmpires,
    false,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 2, 0, 0, 0, 0),
        "Protection from black. WW: Gets +1/+0 until end of turn. W: Gains first strike until end of turn.",
    )
    .creature(2, 2)
    .protection([false, false, true, false, false])
    .partial("The pump and first-strike activated abilities are not implemented.")
    .with_special_behavior(CardBehavior::OrderOfLeitbur),
);

pub(in crate::card::sets) static ORDER_OF_THE_EBON_HAND: CardRecord = CardRecord::new(
    cards::ORDER_OF_THE_EBON_HAND,
    "Order of the Ebon Hand",
    CardArt::new("9e51f5d8-a7cc-4720-8af5-e002bcfd78a0", "Melissa A. Benson"),
    CardSet::FallenEmpires,
    false,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 0, 0, 2, 0, 0),
        "Protection from white. BB: Gets +1/+0 until end of turn. B: Gains first strike until end of turn.",
    )
    .creature(2, 1)
    .protection([true, false, false, false, false])
    .partial("The pump and first-strike activated abilities are not implemented.")
    .with_special_behavior(CardBehavior::OrderOfTheEbonHand),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &GOBLIN_GRENADE,
    &HYMN_TO_TOURACH,
    &ICATIAN_JAVELINEERS,
    &ORDER_OF_LEITBUR,
    &ORDER_OF_THE_EBON_HAND,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
