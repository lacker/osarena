use super::{CardRecord, PrintingRecord};
use crate::card::{CardBehavior, CardKind, CardRules, CardSet, ManaCost, cards};

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static BALL_LIGHTNING: CardRecord = CardRecord::new(
    cards::BALL_LIGHTNING,
    "Ball Lightning",
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
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static BLOOD_MOON: CardRecord = CardRecord::new(
    cards::BLOOD_MOON,
    "Blood Moon",
    CardSet::TheDark,
    false,
    CardBehavior::BloodMoon,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::new(2, 1),
        "Nonbasic lands are Mountains.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static GOBLIN_DIGGING_TEAM: CardRecord = CardRecord::new(
    cards::GOBLIN_DIGGING_TEAM,
    "Goblin Digging Team",
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
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static GOBLINS_OF_THE_FLARG: CardRecord = CardRecord::new(
    cards::GOBLINS_OF_THE_FLARG,
    "Goblins of the Flarg",
    CardSet::TheDark,
    false,
    CardBehavior::GoblinsOfTheFlarg,
    CardRules::new(CardKind::Creature, ManaCost::new(0, 1), "Mountainwalk.")
        .creature(1, 1)
        .goblin()
        .mountainwalk(),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static FELLWAR_STONE: CardRecord = CardRecord::new(
    cards::FELLWAR_STONE,
    "Fellwar Stone",
    CardSet::TheDark,
    false,
    CardBehavior::FellwarStone,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(2, 0),
        "Tap: Add one mana of any color an opponent's land could produce.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static MAZE_OF_ITH: CardRecord = CardRecord::new(
    cards::MAZE_OF_ITH,
    "Maze of Ith",
    CardSet::TheDark,
    false,
    CardBehavior::MazeOfIth,
    CardRules::new(
        CardKind::Land,
        ManaCost::new(0, 0),
        "Tap: Untap target attacking creature and prevent all combat damage it would deal and receive this turn.",
    )
    .activated("Untap {} and take it out of combat", "Take an attacker out of combat"),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static DUST_TO_DUST: CardRecord = CardRecord::new(
    cards::DUST_TO_DUST,
    "Dust to Dust",
    CardSet::TheDark,
    false,
    CardBehavior::DustToDust,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(2, 2, 0, 0, 0, 0),
        "Exile two target artifacts.",
    ),
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
