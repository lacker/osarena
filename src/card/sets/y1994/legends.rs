use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityImplementationDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, CardArt, CardBehavior, CardKind, CardRules, CardSet, CardSupertype, ColorDef,
    EffectDef, EffectRecipientDef, EvergreenAbility, ManaCost, ManaKindDef, ObjectPredicateDef,
    ValueDef, ZoneKind, cards,
};
use crate::ids::{AbilityId, TargetSlotId};

pub(in crate::card::sets) static CHAIN_LIGHTNING: CardRecord = CardRecord::new(
    cards::CHAIN_LIGHTNING,
    "Chain Lightning",
    CardArt::new("b5883762-ca0a-4932-8d2a-41a45796a5f8", "Sandra Everingham"),
    CardSet::Legends,
    CardRules::new(CardKind::Sorcery, ManaCost::new(0, 1), "").with_abilities(&[
        AbilityDef::spell(
            AbilityId::PRIMARY,
            "Chain Lightning deals 3 damage to any target.",
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetSlotId(0)),
                amount: ValueDef::Constant(3),
            },
        )
        .with_targets(&[AbilityTargetDef::exactly_one(
            TargetSlotId(0),
            "any target",
            AbilityTargetPredicate::AnyTarget,
        )]),
        AbilityDef::custom_full(
            AbilityId(1),
            "Then that player or that permanent's controller may pay {R}{R}. If the player does, they may copy this spell and may choose a new target for that copy.",
            CardBehavior::ChainLightning,
            "The optional payment and spell-copy procedure are implemented by the card-local follow-up resolver.",
        ),
    ]),
);

pub(in crate::card::sets) static DIVINE_OFFERING: CardRecord = CardRecord::new(
    cards::DIVINE_OFFERING,
    "Divine Offering",
    CardArt::new("9c78c2f3-2f40-48ad-9dc4-55d1fa399a56", "Jeff A. Menges"),
    CardSet::Legends,
    CardRules::new(CardKind::Instant, ManaCost::colored(1, 1, 0, 0, 0, 0), "").with_abilities(&[
        AbilityDef::custom_full(
            AbilityId::PRIMARY,
            "Destroy target artifact. You gain life equal to its mana value.",
            CardBehavior::DivineOffering,
            "Artifact destruction and life gain are implemented by the legacy spell resolver.",
        ),
    ]),
);

pub(in crate::card::sets) static MANA_DRAIN: CardRecord = CardRecord::new(
    cards::MANA_DRAIN,
    "Mana Drain",
    CardArt::new("e691adef-3027-4e6a-889f-9f4e2df36a7c", "Mark Tedin"),
    CardSet::Legends,
    CardRules::new(CardKind::Instant, ManaCost::colored(0, 0, 2, 0, 0, 0), "")
        .with_abilities(&[AbilityDef::custom_partial(
            AbilityId::PRIMARY,
            "Counter target spell. At the beginning of your next main phase, add an amount of {C} equal to that spell's mana value.",
            CardBehavior::ManaDrain,
            "The delayed mana trigger is stored as a scalar and never becomes a stack object.",
        )]),
);

pub(in crate::card::sets) static RECALL: CardRecord = CardRecord::new(
    cards::RECALL,
    "Recall",
    CardArt::new("33296718-0625-4422-a65c-b21cf99c52ec", "Brian Snõddy"),
    CardSet::Legends,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::variable(0, 0, 1, 0, 0, 0, 2),
        "",
    )
    .with_abilities(&[AbilityDef::custom_partial(
        AbilityId::PRIMARY,
        "Discard X cards, then return a card from your graveyard to your hand for each card discarded this way. Exile Recall.",
        CardBehavior::Recall,
        "The engine incorrectly requires and discards X cards as an additional casting cost instead of discarding during resolution.",
    )]),
);

pub(in crate::card::sets) static SYLVAN_LIBRARY: CardRecord = CardRecord::new(
    cards::SYLVAN_LIBRARY,
    "Sylvan Library",
    CardArt::new("f486df00-7c4a-4ff0-bb0b-c8b5432ac742", "Harold McNeill"),
    CardSet::Legends,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(1, 0, 0, 0, 0, 1),
        "",
    )
    .with_abilities(&[AbilityDef::custom_partial(
        AbilityId::PRIMARY,
        "At the beginning of your draw step, you may draw two additional cards. If you do, choose two cards in your hand drawn this turn. For each of those cards, pay 4 life or put the card on top of your library.",
        CardBehavior::SylvanLibrary,
        "The draw-step trigger and its choices currently bypass the stack.",
    )]),
);

pub(in crate::card::sets) static THUNDER_SPIRIT: CardRecord = CardRecord::new(
    cards::THUNDER_SPIRIT,
    "Thunder Spirit",
    CardArt::new(
        "61a59775-b1cd-4ed0-8abf-c2b37f7be0d5",
        "Randy Asplund-Faith",
    ),
    CardSet::Legends,
    CardRules::new(CardKind::Creature, ManaCost::colored(1, 2, 0, 0, 0, 0), "")
        .creature(2, 2)
        .with_subtypes(&["Elemental", "Spirit"])
        .with_abilities(&[
            AbilityDef::evergreen(AbilityId::PRIMARY, "Flying", EvergreenAbility::Flying),
            AbilityDef::evergreen(AbilityId(1), "First strike", EvergreenAbility::FirstStrike)
                .with_implementation(AbilityImplementationDef::NotImplemented {
                    explanation: "First strike combat damage is not implemented.",
                }),
        ]),
);

pub(in crate::card::sets) static WHIRLING_DERVISH: CardRecord = CardRecord::new(
    cards::WHIRLING_DERVISH,
    "Whirling Dervish",
    CardArt::new("eba294e7-7097-4bc3-b396-72e85dd4f441", "Susan Van Camp"),
    CardSet::Legends,
    CardRules::new(CardKind::Creature, ManaCost::colored(0, 0, 0, 0, 0, 2), "")
        .creature(1, 1)
        .with_subtypes(&["Human", "Monk"])
        .with_abilities(&[
            AbilityDef::evergreen(
                AbilityId(1),
                "Protection from black",
                EvergreenAbility::ProtectionFrom(ColorDef::Black),
            ),
            AbilityDef::custom_partial(
                AbilityId::PRIMARY,
                "At the beginning of each end step, if this creature dealt damage to an opponent this turn, put a +1/+1 counter on it.",
                CardBehavior::WhirlingDervish,
                "The end-step trigger currently resolves outside the stack.",
            ),
        ]),
);

pub(in crate::card::sets) static MOAT: CardRecord = CardRecord::new(
    cards::MOAT,
    "Moat",
    CardArt::new("952ba126-0915-47f0-9b6a-a0a6dcd22c6f", "Jeff A. Menges"),
    CardSet::Legends,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(2, 2, 0, 0, 0, 0),
        "",
    )
    .with_abilities(&[AbilityDef::custom_full(
        AbilityId::PRIMARY,
        "Creatures without flying can't attack.",
        CardBehavior::Moat,
        "The attack restriction is implemented by the legacy combat legality check.",
    )]),
);

pub(in crate::card::sets) static PENDELHAVEN: CardRecord = CardRecord::new(
    cards::PENDELHAVEN,
    "Pendelhaven",
    CardArt::new("79427109-c1f3-476d-a029-0049217237b5", "Bryon Wackwitz"),
    CardSet::Legends,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "")
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        AbilityDef::activated_mana(
            AbilityId::PRIMARY,
            "{T}: Add {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Green)),
        ),
        AbilityDef::activated(
            AbilityId(1),
            "{T}: Target 1/1 creature gets +1/+2 until end of turn.",
            &[AbilityCostDef::TapSource],
            EffectDef::Special("Give the target 1/1 creature +1/+2 until end of turn"),
        )
        .with_targets(&[AbilityTargetDef::exactly_one(
            TargetSlotId(0),
            "1/1 creature",
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Special("creature with power 1 and toughness 1"),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )])
        .with_implementation(AbilityImplementationDef::CustomPartial {
            behavior: Some(CardBehavior::Pendelhaven),
            explanation: "The 1/1 target restriction is checked on activation but is not rechecked when the ability resolves.",
        })
        .with_activation_text(
            "Give {} +1/+2 with Pendelhaven",
            "Give a 1/1 creature +1/+2",
        ),
    ]),
);

pub(in crate::card::sets) static RELIC_BARRIER: CardRecord = CardRecord::new(
    cards::RELIC_BARRIER,
    "Relic Barrier",
    CardArt::new("c062cbae-ce5e-43be-9932-c81a0a3622e8", "Harold McNeill"),
    CardSet::Legends,
    CardRules::new(CardKind::Artifact, ManaCost::new(2, 0), "").with_abilities(&[
        AbilityDef::activated(
            AbilityId::PRIMARY,
            "{T}: Tap target artifact.",
            &[AbilityCostDef::TapSource],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetSlotId(0)),
            },
        )
        .with_targets(&[AbilityTargetDef::exactly_one(
            TargetSlotId(0),
            "artifact",
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Artifact,
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )])
        .with_activation_text("Tap {} with Relic Barrier", "Tap an artifact"),
    ]),
);

pub(in crate::card::sets) static THE_ABYSS: CardRecord = CardRecord::new(
    cards::THE_ABYSS,
    "The Abyss",
    CardArt::new("86a27d68-3e58-4ade-976d-36381beed451", "Pete Venters"),
    CardSet::Legends,
    CardRules::new(CardKind::Enchantment, ManaCost::colored(3, 0, 0, 1, 0, 0), "")
        .with_supertype(CardSupertype::World)
        .with_abilities(&[AbilityDef::custom_partial(
            AbilityId::PRIMARY,
            "At the beginning of each player's upkeep, destroy target nonartifact creature that player controls of their choice. It can't be regenerated.",
            CardBehavior::TheAbyss,
            "The target is selected automatically and the upkeep trigger never becomes a stack object.",
        )]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CHAIN_LIGHTNING,
    &DIVINE_OFFERING,
    &MANA_DRAIN,
    &RECALL,
    &SYLVAN_LIBRARY,
    &THUNDER_SPIRIT,
    &WHIRLING_DERVISH,
    &MOAT,
    &PENDELHAVEN,
    &RELIC_BARRIER,
    &THE_ABYSS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
