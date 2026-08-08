use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, CardArt, CardBehavior, CardKind, CardRules, CardSet, EffectDef,
    EffectDurationDef, EffectRecipientDef, ImplementationStatus, ManaCost, ObjectPredicateDef,
    PlayerRelation, ZoneKind, cards,
};
use crate::ids::AbilityId;

pub(in crate::card::sets) static BALL_LIGHTNING: CardRecord = CardRecord::new(
    cards::BALL_LIGHTNING,
    "Ball Lightning",
    CardArt::new("c1ba83ab-83f5-421d-bba1-0f925870b5c8", "Quinton Hoover"),
    CardSet::TheDark,
    false,
    CardBehavior::BallLightning,
    CardRules::new(
        CardKind::Creature,
        ManaCost::new(0, 3),
        "Trample, haste. Sacrifice Ball Lightning at the beginning of the end step.",
    )
    .creature(6, 1)
    .haste()
    .trample(),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The end-step sacrifice trigger currently resolves outside the stack.",
});

static BLOOD_MOON_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
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
)];

pub(in crate::card::sets) static BLOOD_MOON: CardRecord = CardRecord::new(
    cards::BLOOD_MOON,
    "Blood Moon",
    CardArt::new("78373616-e2d6-4ccf-998f-09f02bea45b4", "Tom Wänerstrand"),
    CardSet::TheDark,
    false,
    CardBehavior::BloodMoon,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::new(2, 1),
        "Nonbasic lands are Mountains.",
    )
    .with_abilities(&BLOOD_MOON_ABILITIES),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The hard-coded transformation does not yet use the full land-type, ability-loss, and layer system.",
    });

pub(in crate::card::sets) static GOBLIN_DIGGING_TEAM: CardRecord = CardRecord::new(
    cards::GOBLIN_DIGGING_TEAM,
    "Goblin Digging Team",
    CardArt::new("8a538b9d-351e-40bb-be11-9ba08c16352b", "Ron Spencer"),
    CardSet::TheDark,
    false,
    CardBehavior::GoblinDiggingTeam,
    CardRules::new(
        CardKind::Creature,
        ManaCost::new(0, 1),
        "Sacrifice Goblin Digging Team: Destroy target Wall.",
    )
    .creature(1, 1)
    .goblin(),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The sacrifice ability that destroys a Wall is not implemented.",
});

pub(in crate::card::sets) static GOBLINS_OF_THE_FLARG: CardRecord = CardRecord::new(
    cards::GOBLINS_OF_THE_FLARG,
    "Goblins of the Flarg",
    CardArt::new("fd333b18-b896-4ab8-9c46-eed4efdd94f2", "Tom Wänerstrand"),
    CardSet::TheDark,
    false,
    CardBehavior::GoblinsOfTheFlarg,
    CardRules::new(CardKind::Creature, ManaCost::new(0, 1), "Mountainwalk.")
        .creature(1, 1)
        .goblin()
        .mountainwalk(),
)
.with_implementation_status(ImplementationStatus::Partial {
            explanation: "Mountainwalk does not currently recognize Mountains supplied by dual-land subtypes.",
        });

pub(in crate::card::sets) static FELLWAR_STONE: CardRecord = CardRecord::new(
    cards::FELLWAR_STONE,
    "Fellwar Stone",
    CardArt::new("dc47e322-f8b8-4685-b035-fda0cc433e6b", "Quinton Hoover"),
    CardSet::TheDark,
    false,
    CardBehavior::FellwarStone,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(2, 0),
        "Tap: Add one mana of any color an opponent's land could produce.",
    ),
);

pub(in crate::card::sets) static MAZE_OF_ITH: CardRecord = CardRecord::new(
    cards::MAZE_OF_ITH,
    "Maze of Ith",
    CardArt::new("42dcceee-2a47-4eaa-a6a3-2931b3d50244", "Anson Maddocks"),
    CardSet::TheDark,
    false,
    CardBehavior::MazeOfIth,
    CardRules::new(
        CardKind::Land,
        ManaCost::new(0, 0),
        "Tap: Untap target attacking creature and prevent all combat damage it would deal and receive this turn.",
    )
    .activated("Untap {} and take it out of combat", "Take an attacker out of combat"),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The implementation removes the attacker from combat instead of creating combat-damage prevention.",
    });

pub(in crate::card::sets) static DUST_TO_DUST: CardRecord = CardRecord::new(
    cards::DUST_TO_DUST,
    "Dust to Dust",
    CardArt::new("ade075fd-73ee-4d12-a2da-48e5938043af", "Drew Tucker"),
    CardSet::TheDark,
    false,
    CardBehavior::DustToDust,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(2, 2, 0, 0, 0, 0),
        "Exile two target artifacts.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "Its implemented mana cost is {2}{W}{W} instead of the printed {1}{W}{W}.",
});

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
