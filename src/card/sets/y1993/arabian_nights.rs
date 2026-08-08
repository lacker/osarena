use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityImplementationDef, AddManaEffectDef, CardArt, CardBehavior,
    CardKind, CardRules, CardSet, EffectDef, EffectRecipientDef, EvergreenAbility, ManaCost,
    ManaKindDef, ObjectPredicateDef, PlayerRelation, TriggerEventDef, TurnStepDef, ValueDef, cards,
};
use crate::ids::AbilityId;

pub(in crate::card::sets) static CITY_OF_BRASS: CardRecord = CardRecord::new(
    cards::CITY_OF_BRASS,
    "City of Brass",
    CardArt::new("f4e32327-380d-471e-813b-4c27477787ce", "Mark Tedin"),
    CardSet::ArabianNights,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "").with_abilities(&[
        AbilityDef::triggered(
            AbilityId(1),
            "Whenever this land becomes tapped, it deals 1 damage to you.",
            TriggerEventDef::BecomesTapped(ObjectPredicateDef::Source),
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            AbilityId::PRIMARY,
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaKindDef::White,
                ManaKindDef::Blue,
                ManaKindDef::Black,
                ManaKindDef::Red,
                ManaKindDef::Green,
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static ERHNAM_DJINN: CardRecord = CardRecord::new(
    cards::ERHNAM_DJINN,
    "Erhnam Djinn",
    CardArt::new("42bc0c3f-0a52-4bdc-83da-6484bf3102f3", "Ken Meyer, Jr."),
    CardSet::ArabianNights,
    CardRules::new(CardKind::Creature, ManaCost::colored(3, 0, 0, 0, 0, 1), "")
    .with_subtypes(&["Djinn"])
    .creature(4, 5)
    .with_abilities(&[AbilityDef::custom_partial(
        AbilityId::PRIMARY,
        "At the beginning of your upkeep, target non-Wall creature an opponent controls gains forestwalk until your next upkeep. (It can't be blocked as long as defending player controls a Forest.)",
        CardBehavior::ErhnamDjinn,
        "The targeted upkeep trigger is handled outside the stack.",
    )]),
);

pub(in crate::card::sets) static JUZAM_DJINN: CardRecord = CardRecord::new(
    cards::JUZAM_DJINN,
    "Juzám Djinn",
    CardArt::new("31bf3f14-b5df-498b-a1bb-965885c82401", "Mark Tedin"),
    CardSet::ArabianNights,
    CardRules::new(CardKind::Creature, ManaCost::colored(2, 0, 0, 2, 0, 0), "")
        .with_subtypes(&["Djinn"])
        .creature(5, 5)
        .with_abilities(&[AbilityDef::triggered(
            AbilityId::PRIMARY,
            "At the beginning of your upkeep, this creature deals 1 damage to you.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )]),
);

pub(in crate::card::sets) static LIBRARY_OF_ALEXANDRIA: CardRecord = CardRecord::new(
    cards::LIBRARY_OF_ALEXANDRIA,
    "Library of Alexandria",
    CardArt::new("ee266113-34ce-4189-84e7-ee2c86a2722c", "Mark Poole"),
    CardSet::ArabianNights,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "").with_abilities(&[
        AbilityDef::activated_mana(
            AbilityId::PRIMARY,
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Colorless)),
        ),
        AbilityDef::activated(
            AbilityId(1),
            "{T}: Draw a card. Activate only if you have exactly seven cards in hand.",
            &[AbilityCostDef::TapSource],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )
        .with_implementation(AbilityImplementationDef::CustomFull {
            behavior: Some(CardBehavior::LibraryOfAlexandria),
            explanation: "The seven-card activation restriction and card draw are implemented by the card-local activated-action resolver.",
        }),
    ]),
);

pub(in crate::card::sets) static SERENDIB_EFREET: CardRecord = CardRecord::new(
    cards::SERENDIB_EFREET,
    "Serendib Efreet",
    CardArt::new("cf56e862-3169-4f63-acd0-731080fa32f2", "Anson Maddocks"),
    CardSet::ArabianNights,
    CardRules::new(CardKind::Creature, ManaCost::colored(2, 0, 1, 0, 0, 0), "")
        .with_subtypes(&["Efreet"])
        .creature(3, 4)
        .with_abilities(&[
            AbilityDef::evergreen(AbilityId(1), "Flying", EvergreenAbility::Flying),
            AbilityDef::triggered(
                AbilityId::PRIMARY,
                "At the beginning of your upkeep, this creature deals 1 damage to you.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

pub(in crate::card::sets) static CITY_IN_A_BOTTLE: CardRecord = CardRecord::new(
    cards::CITY_IN_A_BOTTLE,
    "City in a Bottle",
    CardArt::new("9598b346-a47d-4c4c-9571-156824e86b9c", "Drew Tucker"),
    CardSet::ArabianNights,
    CardRules::new(CardKind::Artifact, ManaCost::new(2, 0), "").with_abilities(&[
        AbilityDef::custom_partial(
            AbilityId::PRIMARY,
            "Whenever one or more other nontoken permanents with a name originally printed in the Arabian Nights expansion are on the battlefield, their controllers sacrifice them.\nPlayers can't cast spells or play lands with a name originally printed in the Arabian Nights expansion.",
            CardBehavior::CityInABottle,
            "The state trigger currently destroys affected permanents instead of making their controllers sacrifice them, and the casting and land-play prohibition is not implemented.",
        ),
    ]),
);

pub(in crate::card::sets) static KIRD_APE: CardRecord = CardRecord::new(
    cards::KIRD_APE,
    "Kird Ape",
    CardArt::new("ebe8845e-df1c-481c-949c-aab84af99a05", "Ken Meyer, Jr."),
    CardSet::ArabianNights,
    CardRules::new(CardKind::Creature, ManaCost::new(0, 1), "")
    .with_subtypes(&["Ape"])
    .creature(1, 1)
    .with_abilities(&[AbilityDef::custom_full(
        AbilityId::PRIMARY,
        "This creature gets +1/+2 as long as you control a Forest.",
        CardBehavior::KirdApe,
        "The conditional power and toughness bonus is implemented by the legacy characteristic evaluator.",
    )]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CITY_OF_BRASS,
    &ERHNAM_DJINN,
    &JUZAM_DJINN,
    &LIBRARY_OF_ALEXANDRIA,
    &SERENDIB_EFREET,
    &CITY_IN_A_BOTTLE,
    &KIRD_APE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
