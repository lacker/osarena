use super::{CardRecord, PrintingRecord};
use crate::card::{CardBehavior, CardKind, CardRules, CardSet, ManaCost, cards};

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static ANKH_OF_MISHRA: CardRecord = CardRecord::new(
    cards::ANKH_OF_MISHRA,
    "Ankh of Mishra",
    CardSet::Alpha,
    false,
    CardBehavior::AnkhOfMishra,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(2, 0),
        "Whenever a land enters, Ankh of Mishra deals 2 damage to its controller.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static BLACK_VISE: CardRecord = CardRecord::new(
    cards::BLACK_VISE,
    "Black Vise",
    CardSet::Alpha,
    false,
    CardBehavior::BlackVise,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(1, 0),
        "As Black Vise enters, choose an opponent. At their upkeep, it deals 1 damage for each card in their hand beyond four.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static COPPER_TABLET: CardRecord = CardRecord::new(
    cards::COPPER_TABLET,
    "Copper Tablet",
    CardSet::Alpha,
    false,
    CardBehavior::CopperTablet,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(2, 0),
        "At the beginning of each player's upkeep, Copper Tablet deals 1 damage to that player.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static FIREBALL: CardRecord = CardRecord::new(
    cards::FIREBALL,
    "Fireball",
    CardSet::Alpha,
    false,
    CardBehavior::Fireball,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::with_x(1),
        "Deal X damage divided evenly among the chosen targets. Each target beyond the first costs 1 more.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static FORK: CardRecord = CardRecord::new(
    cards::FORK,
    "Fork",
    CardSet::Alpha,
    false,
    CardBehavior::Fork,
    CardRules::new(
        CardKind::Instant,
        ManaCost::new(0, 2),
        "Copy target instant or sorcery. You may choose new targets for the copy.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static GLASSES_OF_URZA: CardRecord = CardRecord::new(
    cards::GLASSES_OF_URZA,
    "Glasses of Urza",
    CardSet::Alpha,
    false,
    CardBehavior::GlassesOfUrza,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(1, 0),
        "Tap: Look at target player's hand.",
    )
    .activated(
        "Look at {}'s hand with Glasses of Urza",
        "Look at a player's hand",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static IRON_STAR: CardRecord = CardRecord::new(
    cards::IRON_STAR,
    "Iron Star",
    CardSet::Alpha,
    false,
    CardBehavior::IronStar,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(1, 0),
        "Whenever a red spell is cast, you may pay 1. If you do, gain 1 life.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static LIGHTNING_BOLT: CardRecord = CardRecord::new(
    cards::LIGHTNING_BOLT,
    "Lightning Bolt",
    CardSet::Alpha,
    false,
    CardBehavior::LightningBolt,
    CardRules::new(
        CardKind::Instant,
        ManaCost::new(0, 1),
        "Deal 3 damage to any target.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static MOUNTAIN: CardRecord = CardRecord::new(
    cards::MOUNTAIN,
    "Mountain",
    CardSet::Alpha,
    true,
    CardBehavior::Mountain,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add R."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static RED_ELEMENTAL_BLAST: CardRecord = CardRecord::new(
    cards::RED_ELEMENTAL_BLAST,
    "Red Elemental Blast",
    CardSet::Alpha,
    false,
    CardBehavior::RedElementalBlast,
    CardRules::new(
        CardKind::Instant,
        ManaCost::new(0, 1),
        "Counter target blue spell or destroy target blue permanent.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static SHATTER: CardRecord = CardRecord::new(
    cards::SHATTER,
    "Shatter",
    CardSet::Alpha,
    false,
    CardBehavior::Shatter,
    CardRules::new(
        CardKind::Instant,
        ManaCost::new(1, 1),
        "Destroy target artifact.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static SMOKE: CardRecord = CardRecord::new(
    cards::SMOKE,
    "Smoke",
    CardSet::Alpha,
    false,
    CardBehavior::Smoke,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::new(0, 2),
        "Players can't untap more than one creature during their untap steps.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static STONE_GIANT: CardRecord = CardRecord::new(
    cards::STONE_GIANT,
    "Stone Giant",
    CardSet::Alpha,
    false,
    CardBehavior::StoneGiant,
    CardRules::new(
        CardKind::Creature,
        ManaCost::new(2, 2),
        "Tap: A smaller creature you control gains flying until end of turn. Destroy it at the end step.",
    )
    .creature(3, 4)
    .activated("Give {} flying with Stone Giant", "Give a smaller creature flying"),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static WINTER_ORB: CardRecord = CardRecord::new(
    cards::WINTER_ORB,
    "Winter Orb",
    CardSet::Alpha,
    false,
    CardBehavior::WinterOrb,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(2, 0),
        "While untapped, players can't untap more than one land during their untap steps.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static BLACK_LOTUS: CardRecord = CardRecord::new(
    cards::BLACK_LOTUS,
    "Black Lotus",
    CardSet::Alpha,
    false,
    CardBehavior::BlackLotus,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(0, 0),
        "Tap, sacrifice Black Lotus: Add RRR.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static CHAOS_ORB: CardRecord = CardRecord::new(
    cards::CHAOS_ORB,
    "Chaos Orb",
    CardSet::Alpha,
    false,
    CardBehavior::ChaosOrb,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(2, 0),
        "1, Tap: Choose a permanent. On resolution, destroy it and Chaos Orb if Chaos Orb is still on the battlefield.",
    )
    .activated("Flip Chaos Orb onto {}", "Flip Chaos Orb onto a permanent"),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static DRAGON_WHELP: CardRecord = CardRecord::new(
    cards::DRAGON_WHELP,
    "Dragon Whelp",
    CardSet::Alpha,
    false,
    CardBehavior::DragonWhelp,
    CardRules::new(
        CardKind::Creature,
        ManaCost::new(2, 2),
        "Flying. R: +1/+0 until end of turn. If activated four or more times this turn, destroy it at the end step.",
    )
    .creature(2, 3)
    .flying(),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static GOBLIN_BALLOON_BRIGADE: CardRecord = CardRecord::new(
    cards::GOBLIN_BALLOON_BRIGADE,
    "Goblin Balloon Brigade",
    CardSet::Alpha,
    false,
    CardBehavior::GoblinBalloonBrigade,
    CardRules::new(
        CardKind::Creature,
        ManaCost::new(0, 1),
        "R: Gains flying until end of turn.",
    )
    .creature(1, 1)
    .goblin(),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static GOBLIN_KING: CardRecord = CardRecord::new(
    cards::GOBLIN_KING,
    "Goblin King",
    CardSet::Alpha,
    false,
    CardBehavior::GoblinKing,
    CardRules::new(
        CardKind::Creature,
        ManaCost::new(1, 2),
        "Other Goblins get +1/+1 and have mountainwalk.",
    )
    .creature(2, 2)
    .goblin(),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static GRANITE_GARGOYLE: CardRecord = CardRecord::new(
    cards::GRANITE_GARGOYLE,
    "Granite Gargoyle",
    CardSet::Alpha,
    false,
    CardBehavior::GraniteGargoyle,
    CardRules::new(
        CardKind::Creature,
        ManaCost::new(2, 1),
        "Flying. R: Gets +0/+1 until end of turn.",
    )
    .creature(2, 2)
    .flying(),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static IRONCLAW_ORCS: CardRecord = CardRecord::new(
    cards::IRONCLAW_ORCS,
    "Ironclaw Orcs",
    CardSet::Alpha,
    false,
    CardBehavior::IronclawOrcs,
    CardRules::new(
        CardKind::Creature,
        ManaCost::new(1, 1),
        "Can't block creatures with power 2 or greater.",
    )
    .creature(2, 2),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static MOX_EMERALD: CardRecord = CardRecord::new(
    cards::MOX_EMERALD,
    "Mox Emerald",
    CardSet::Alpha,
    false,
    CardBehavior::MoxEmerald,
    CardRules::new(CardKind::Artifact, ManaCost::new(0, 0), "Tap: Add 1."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static MOX_JET: CardRecord = CardRecord::new(
    cards::MOX_JET,
    "Mox Jet",
    CardSet::Alpha,
    false,
    CardBehavior::MoxJet,
    CardRules::new(CardKind::Artifact, ManaCost::new(0, 0), "Tap: Add 1."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static MOX_PEARL: CardRecord = CardRecord::new(
    cards::MOX_PEARL,
    "Mox Pearl",
    CardSet::Alpha,
    false,
    CardBehavior::MoxPearl,
    CardRules::new(CardKind::Artifact, ManaCost::new(0, 0), "Tap: Add 1."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static MOX_RUBY: CardRecord = CardRecord::new(
    cards::MOX_RUBY,
    "Mox Ruby",
    CardSet::Alpha,
    false,
    CardBehavior::MoxRuby,
    CardRules::new(CardKind::Artifact, ManaCost::new(0, 0), "Tap: Add R."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static MOX_SAPPHIRE: CardRecord = CardRecord::new(
    cards::MOX_SAPPHIRE,
    "Mox Sapphire",
    CardSet::Alpha,
    false,
    CardBehavior::MoxSapphire,
    CardRules::new(CardKind::Artifact, ManaCost::new(0, 0), "Tap: Add 1."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static SOL_RING: CardRecord = CardRecord::new(
    cards::SOL_RING,
    "Sol Ring",
    CardSet::Alpha,
    false,
    CardBehavior::SolRing,
    CardRules::new(CardKind::Artifact, ManaCost::new(1, 0), "Tap: Add 2."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static WHEEL_OF_FORTUNE: CardRecord = CardRecord::new(
    cards::WHEEL_OF_FORTUNE,
    "Wheel of Fortune",
    CardSet::Alpha,
    false,
    CardBehavior::WheelOfFortune,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::new(2, 1),
        "Each player discards their hand, then draws seven cards.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static JUGGERNAUT: CardRecord = CardRecord::new(
    cards::JUGGERNAUT,
    "Juggernaut",
    CardSet::Alpha,
    false,
    CardBehavior::Juggernaut,
    CardRules::new(
        CardKind::ArtifactCreature,
        ManaCost::new(4, 0),
        "Attacks each combat if able. Juggernaut can't be blocked by Walls.",
    )
    .creature(5, 3),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static MANA_VAULT: CardRecord = CardRecord::new(
    cards::MANA_VAULT,
    "Mana Vault",
    CardSet::Alpha,
    false,
    CardBehavior::ManaVault,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(1, 0),
        "Mana Vault doesn't untap during your untap step. At your upkeep, you may pay 4 to untap it. At your draw step, if tapped, it deals 1 damage to you. Tap: Add 3.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static ANCESTRAL_RECALL: CardRecord = CardRecord::new(
    cards::ANCESTRAL_RECALL,
    "Ancestral Recall",
    CardSet::Alpha,
    false,
    CardBehavior::AncestralRecall,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 1, 0, 0, 0),
        "Target player draws three cards.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static BRAINGEYSER: CardRecord = CardRecord::new(
    cards::BRAINGEYSER,
    "Braingeyser",
    CardSet::Alpha,
    false,
    CardBehavior::Braingeyser,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored_x(0, 2, 0, 0, 0),
        "Target player draws X cards.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static COUNTERSPELL: CardRecord = CardRecord::new(
    cards::COUNTERSPELL,
    "Counterspell",
    CardSet::Alpha,
    false,
    CardBehavior::Counterspell,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 2, 0, 0, 0),
        "Counter target spell.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static DISENCHANT: CardRecord = CardRecord::new(
    cards::DISENCHANT,
    "Disenchant",
    CardSet::Alpha,
    false,
    CardBehavior::Disenchant,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 1, 0, 0, 0, 0),
        "Destroy target artifact or enchantment.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static ISLAND: CardRecord = CardRecord::new(
    cards::ISLAND,
    "Island",
    CardSet::Alpha,
    true,
    CardBehavior::Island,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add U."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static JAYEMDAE_TOME: CardRecord = CardRecord::new(
    cards::JAYEMDAE_TOME,
    "Jayemdae Tome",
    CardSet::Alpha,
    false,
    CardBehavior::JayemdaeTome,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(4, 0),
        "4, Tap: Draw a card.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static PLAINS: CardRecord = CardRecord::new(
    cards::PLAINS,
    "Plains",
    CardSet::Alpha,
    true,
    CardBehavior::Plains,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add W."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static SERRA_ANGEL: CardRecord = CardRecord::new(
    cards::SERRA_ANGEL,
    "Serra Angel",
    CardSet::Alpha,
    false,
    CardBehavior::SerraAngel,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(3, 2, 0, 0, 0, 0),
        "Flying, vigilance.",
    )
    .creature(4, 4)
    .flying()
    .vigilance(),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static SWORDS_TO_PLOWSHARES: CardRecord = CardRecord::new(
    cards::SWORDS_TO_PLOWSHARES,
    "Swords to Plowshares",
    CardSet::Alpha,
    false,
    CardBehavior::SwordsToPlowshares,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 1, 0, 0, 0, 0),
        "Exile target creature. Its controller gains life equal to its power.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static TIME_WALK: CardRecord = CardRecord::new(
    cards::TIME_WALK,
    "Time Walk",
    CardSet::Alpha,
    false,
    CardBehavior::TimeWalk,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(1, 0, 1, 0, 0, 0),
        "Take an extra turn after this one.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static TUNDRA: CardRecord = CardRecord::new(
    cards::TUNDRA,
    "Tundra",
    CardSet::Alpha,
    false,
    CardBehavior::Tundra,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add W or U."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static ARMAGEDDON: CardRecord = CardRecord::new(
    cards::ARMAGEDDON,
    "Armageddon",
    CardSet::Alpha,
    false,
    CardBehavior::Armageddon,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(3, 1, 0, 0, 0, 0),
        "Destroy all lands.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static BADLANDS: CardRecord = CardRecord::new(
    cards::BADLANDS,
    "Badlands",
    CardSet::Alpha,
    false,
    CardBehavior::Badlands,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add B or R."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static BALANCE: CardRecord = CardRecord::new(
    cards::BALANCE,
    "Balance",
    CardSet::Alpha,
    false,
    CardBehavior::Balance,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(1, 1, 0, 0, 0, 0),
        "Each player discards and sacrifices creatures and lands until tied for the fewest of each.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static BAYOU: CardRecord = CardRecord::new(
    cards::BAYOU,
    "Bayou",
    CardSet::Alpha,
    false,
    CardBehavior::Bayou,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add B or G."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static BLACK_KNIGHT: CardRecord = CardRecord::new(
    cards::BLACK_KNIGHT,
    "Black Knight",
    CardSet::Alpha,
    false,
    CardBehavior::BlackKnight,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 0, 0, 2, 0, 0),
        "First strike, protection from white.",
    )
    .creature(2, 2),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static BIRDS_OF_PARADISE: CardRecord = CardRecord::new(
    cards::BIRDS_OF_PARADISE,
    "Birds of Paradise",
    CardSet::Alpha,
    false,
    CardBehavior::BirdsOfParadise,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 0, 0, 0, 0, 1),
        "Flying. Tap: Add one mana of any color.",
    )
    .creature(0, 1)
    .flying(),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static BLUE_ELEMENTAL_BLAST: CardRecord = CardRecord::new(
    cards::BLUE_ELEMENTAL_BLAST,
    "Blue Elemental Blast",
    CardSet::Alpha,
    false,
    CardBehavior::BlueElementalBlast,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 1, 0, 0, 0),
        "Counter target red spell or destroy target red permanent.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static CHANNEL: CardRecord = CardRecord::new(
    cards::CHANNEL,
    "Channel",
    CardSet::Alpha,
    false,
    CardBehavior::Channel,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(0, 0, 0, 0, 0, 2),
        "Until end of turn, you may pay 1 life to add one colorless mana.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static CRUSADE: CardRecord = CardRecord::new(
    cards::CRUSADE,
    "Crusade",
    CardSet::Alpha,
    false,
    CardBehavior::Crusade,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(0, 2, 0, 0, 0, 0),
        "White creatures get +1/+1.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static DARK_RITUAL: CardRecord = CardRecord::new(
    cards::DARK_RITUAL,
    "Dark Ritual",
    CardSet::Alpha,
    false,
    CardBehavior::DarkRitual,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 0, 1, 0, 0),
        "Add BBB.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static DEMONIC_TUTOR: CardRecord = CardRecord::new(
    cards::DEMONIC_TUTOR,
    "Demonic Tutor",
    CardSet::Alpha,
    false,
    CardBehavior::DemonicTutor,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(1, 0, 0, 1, 0, 0),
        "Search your library for a card, put it into your hand, then shuffle.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static DRAIN_LIFE: CardRecord = CardRecord::new(
    cards::DRAIN_LIFE,
    "Drain Life",
    CardSet::Alpha,
    false,
    CardBehavior::DrainLife,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::variable(1, 0, 0, 1, 0, 0, 1),
        "Drain Life deals X damage to any target and you gain that much life.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static EARTHQUAKE: CardRecord = CardRecord::new(
    cards::EARTHQUAKE,
    "Earthquake",
    CardSet::Alpha,
    false,
    CardBehavior::Earthquake,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::with_x(1),
        "Earthquake deals X damage to each player and each creature without flying.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static FOREST: CardRecord = CardRecord::new(
    cards::FOREST,
    "Forest",
    CardSet::Alpha,
    true,
    CardBehavior::Forest,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add G."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static HYPNOTIC_SPECTER: CardRecord = CardRecord::new(
    cards::HYPNOTIC_SPECTER,
    "Hypnotic Specter",
    CardSet::Alpha,
    false,
    CardBehavior::HypnoticSpecter,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 0, 0, 2, 0, 0),
        "Flying. Whenever Hypnotic Specter damages an opponent, they discard a card at random.",
    )
    .creature(2, 2)
    .flying(),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static MIND_TWIST: CardRecord = CardRecord::new(
    cards::MIND_TWIST,
    "Mind Twist",
    CardSet::Alpha,
    false,
    CardBehavior::MindTwist,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored_x(0, 0, 1, 0, 0),
        "Target player discards X cards at random.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static NEVINYRRALS_DISK: CardRecord = CardRecord::new(
    cards::NEVINYRRALS_DISK,
    "Nevinyrral's Disk",
    CardSet::Alpha,
    false,
    CardBehavior::NevinyrralsDisk,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(4, 0),
        "Enters tapped. 1, Tap: Destroy all artifacts, creatures, and enchantments.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static PLATEAU: CardRecord = CardRecord::new(
    cards::PLATEAU,
    "Plateau",
    CardSet::Alpha,
    false,
    CardBehavior::Plateau,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add R or W."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static PSIONIC_BLAST: CardRecord = CardRecord::new(
    cards::PSIONIC_BLAST,
    "Psionic Blast",
    CardSet::Alpha,
    false,
    CardBehavior::PsionicBlast,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(2, 0, 1, 0, 0, 0),
        "Deal 4 damage to any target and 2 damage to you.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static REGROWTH: CardRecord = CardRecord::new(
    cards::REGROWTH,
    "Regrowth",
    CardSet::Alpha,
    false,
    CardBehavior::Regrowth,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(1, 0, 0, 0, 0, 1),
        "Return target card from your graveyard to your hand.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static SAVANNAH: CardRecord = CardRecord::new(
    cards::SAVANNAH,
    "Savannah",
    CardSet::Alpha,
    false,
    CardBehavior::Savannah,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add G or W."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static SAVANNAH_LIONS: CardRecord = CardRecord::new(
    cards::SAVANNAH_LIONS,
    "Savannah Lions",
    CardSet::Alpha,
    false,
    CardBehavior::SavannahLions,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 1, 0, 0, 0, 0),
        "A swift 2/1 creature.",
    )
    .creature(2, 1),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static SCRUBLAND: CardRecord = CardRecord::new(
    cards::SCRUBLAND,
    "Scrubland",
    CardSet::Alpha,
    false,
    CardBehavior::Scrubland,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add W or B."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static SENGIR_VAMPIRE: CardRecord = CardRecord::new(
    cards::SENGIR_VAMPIRE,
    "Sengir Vampire",
    CardSet::Alpha,
    false,
    CardBehavior::SengirVampire,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(3, 0, 0, 2, 0, 0),
        "Flying. Whenever a creature damaged by Sengir Vampire dies, put a +1/+1 counter on it.",
    )
    .creature(4, 4)
    .flying(),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static SINKHOLE: CardRecord = CardRecord::new(
    cards::SINKHOLE,
    "Sinkhole",
    CardSet::Alpha,
    false,
    CardBehavior::Sinkhole,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(0, 0, 0, 2, 0, 0),
        "Destroy target land.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static SWAMP: CardRecord = CardRecord::new(
    cards::SWAMP,
    "Swamp",
    CardSet::Alpha,
    true,
    CardBehavior::Swamp,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add B."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static TAIGA: CardRecord = CardRecord::new(
    cards::TAIGA,
    "Taiga",
    CardSet::Alpha,
    false,
    CardBehavior::Taiga,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add R or G."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static TERROR: CardRecord = CardRecord::new(
    cards::TERROR,
    "Terror",
    CardSet::Alpha,
    false,
    CardBehavior::Terror,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 0, 0, 1, 0, 0),
        "Destroy target nonartifact, nonblack creature. It can't be regenerated.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static TIME_VAULT: CardRecord = CardRecord::new(
    cards::TIME_VAULT,
    "Time Vault",
    CardSet::Alpha,
    false,
    CardBehavior::TimeVault,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(2, 0),
        "Enters tapped and doesn't untap normally. Skip a turn to untap it. Tap: Take an extra turn.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static TIMETWISTER: CardRecord = CardRecord::new(
    cards::TIMETWISTER,
    "Timetwister",
    CardSet::Alpha,
    false,
    CardBehavior::Timetwister,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(2, 0, 1, 0, 0, 0),
        "Each player shuffles their hand and graveyard into their library, then draws seven cards.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static TROPICAL_ISLAND: CardRecord = CardRecord::new(
    cards::TROPICAL_ISLAND,
    "Tropical Island",
    CardSet::Alpha,
    false,
    CardBehavior::TropicalIsland,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add U or G."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static UNDERGROUND_SEA: CardRecord = CardRecord::new(
    cards::UNDERGROUND_SEA,
    "Underground Sea",
    CardSet::Alpha,
    false,
    CardBehavior::UndergroundSea,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add U or B."),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static WHITE_KNIGHT: CardRecord = CardRecord::new(
    cards::WHITE_KNIGHT,
    "White Knight",
    CardSet::Alpha,
    false,
    CardBehavior::WhiteKnight,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 2, 0, 0, 0, 0),
        "First strike, protection from black.",
    )
    .creature(2, 2),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static BERSERK: CardRecord = CardRecord::new(
    cards::BERSERK,
    "Berserk",
    CardSet::Alpha,
    false,
    CardBehavior::Berserk,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 0, 0, 0, 1),
        "Target creature gains trample and gets +X/+0 until end of turn, where X is its power. Destroy it at end of turn if it attacked this turn.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static COPY_ARTIFACT: CardRecord = CardRecord::new(
    cards::COPY_ARTIFACT,
    "Copy Artifact",
    CardSet::Alpha,
    false,
    CardBehavior::CopyArtifact,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(1, 0, 1, 0, 0, 0),
        "You may have Copy Artifact enter as a copy of any artifact on the battlefield.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static GIANT_GROWTH: CardRecord = CardRecord::new(
    cards::GIANT_GROWTH,
    "Giant Growth",
    CardSet::Alpha,
    false,
    CardBehavior::GiantGrowth,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 0, 0, 0, 1),
        "Target creature gets +3/+3 until end of turn.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static ICY_MANIPULATOR: CardRecord = CardRecord::new(
    cards::ICY_MANIPULATOR,
    "Icy Manipulator",
    CardSet::Alpha,
    false,
    CardBehavior::IcyManipulator,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(4, 0),
        "1, Tap: Tap target artifact, creature, or land.",
    )
    .activated(
        "Tap {} with Icy Manipulator",
        "Tap an artifact, creature, or land",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static LLANOWAR_ELVES: CardRecord = CardRecord::new(
    cards::LLANOWAR_ELVES,
    "Llanowar Elves",
    CardSet::Alpha,
    false,
    CardBehavior::LlanowarElves,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 0, 0, 0, 0, 1),
        "Tap: Add G.",
    )
    .creature(1, 1),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static SCRYB_SPRITES: CardRecord = CardRecord::new(
    cards::SCRYB_SPRITES,
    "Scryb Sprites",
    CardSet::Alpha,
    false,
    CardBehavior::ScrybSprites,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 0, 0, 0, 0, 1),
        "Flying.",
    )
    .creature(1, 1)
    .flying(),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static STONE_RAIN: CardRecord = CardRecord::new(
    cards::STONE_RAIN,
    "Stone Rain",
    CardSet::Alpha,
    false,
    CardBehavior::StoneRain,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::new(2, 1),
        "Destroy target land.",
    ),
);

// Implementation status: complete — card rules are executed by the engine.
pub(in crate::card::sets) static WRATH_OF_GOD: CardRecord = CardRecord::new(
    cards::WRATH_OF_GOD,
    "Wrath of God",
    CardSet::Alpha,
    false,
    CardBehavior::WrathOfGod,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(2, 2, 0, 0, 0, 0),
        "Destroy all creatures. They can't be regenerated.",
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANKH_OF_MISHRA,
    &BLACK_VISE,
    &COPPER_TABLET,
    &FIREBALL,
    &FORK,
    &GLASSES_OF_URZA,
    &IRON_STAR,
    &LIGHTNING_BOLT,
    &MOUNTAIN,
    &RED_ELEMENTAL_BLAST,
    &SHATTER,
    &SMOKE,
    &STONE_GIANT,
    &WINTER_ORB,
    &BLACK_LOTUS,
    &CHAOS_ORB,
    &DRAGON_WHELP,
    &GOBLIN_BALLOON_BRIGADE,
    &GOBLIN_KING,
    &GRANITE_GARGOYLE,
    &IRONCLAW_ORCS,
    &MOX_EMERALD,
    &MOX_JET,
    &MOX_PEARL,
    &MOX_RUBY,
    &MOX_SAPPHIRE,
    &SOL_RING,
    &WHEEL_OF_FORTUNE,
    &JUGGERNAUT,
    &MANA_VAULT,
    &ANCESTRAL_RECALL,
    &BRAINGEYSER,
    &COUNTERSPELL,
    &DISENCHANT,
    &ISLAND,
    &JAYEMDAE_TOME,
    &PLAINS,
    &SERRA_ANGEL,
    &SWORDS_TO_PLOWSHARES,
    &TIME_WALK,
    &TUNDRA,
    &ARMAGEDDON,
    &BADLANDS,
    &BALANCE,
    &BAYOU,
    &BLACK_KNIGHT,
    &BIRDS_OF_PARADISE,
    &BLUE_ELEMENTAL_BLAST,
    &CHANNEL,
    &CRUSADE,
    &DARK_RITUAL,
    &DEMONIC_TUTOR,
    &DRAIN_LIFE,
    &EARTHQUAKE,
    &FOREST,
    &HYPNOTIC_SPECTER,
    &MIND_TWIST,
    &NEVINYRRALS_DISK,
    &PLATEAU,
    &PSIONIC_BLAST,
    &REGROWTH,
    &SAVANNAH,
    &SAVANNAH_LIONS,
    &SCRUBLAND,
    &SENGIR_VAMPIRE,
    &SINKHOLE,
    &SWAMP,
    &TAIGA,
    &TERROR,
    &TIME_VAULT,
    &TIMETWISTER,
    &TROPICAL_ISLAND,
    &UNDERGROUND_SEA,
    &WHITE_KNIGHT,
    &BERSERK,
    &COPY_ARTIFACT,
    &GIANT_GROWTH,
    &ICY_MANIPULATOR,
    &LLANOWAR_ELVES,
    &SCRYB_SPRITES,
    &STONE_RAIN,
    &WRATH_OF_GOD,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&PLAINS, 1),
    PrintingRecord::alternate(&ISLAND, 1),
    PrintingRecord::alternate(&SWAMP, 1),
    PrintingRecord::alternate(&MOUNTAIN, 1),
    PrintingRecord::alternate(&FOREST, 1),
];
