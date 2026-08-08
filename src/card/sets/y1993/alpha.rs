use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, CardArt, CardBehavior, CardKind, CardRules,
    CardSet, EffectDef, EffectRecipientDef, ImplementationStatus, ManaCost, ManaKindDef,
    ObjectPredicateDef, TriggerEventDef, ValueDef, ZoneKind, cards,
};
use crate::ids::AbilityId;

static ANKH_OF_MISHRA_ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered(
    AbilityId::PRIMARY,
    "Whenever a land enters, deal 2 damage to its controller.",
    TriggerEventDef::ZoneChanged {
        object: ObjectPredicateDef::Land,
        from: None,
        to: Some(ZoneKind::Battlefield),
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::TriggeringPlayer,
        amount: ValueDef::Constant(2),
    },
)];

pub(in crate::card::sets) static ANKH_OF_MISHRA: CardRecord = CardRecord::new(
    cards::ANKH_OF_MISHRA,
    "Ankh of Mishra",
    CardArt::new("f594b7aa-d44e-47c4-989b-565f881e25f1", "Amy Weber"),
    CardSet::Alpha,
    false,
    CardBehavior::AnkhOfMishra,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(2, 0),
        "Whenever a land enters, Ankh of Mishra deals 2 damage to its controller.",
    )
    .with_abilities(&ANKH_OF_MISHRA_ABILITIES),
);

pub(in crate::card::sets) static BLACK_VISE: CardRecord = CardRecord::new(
    cards::BLACK_VISE,
    "Black Vise",
    CardArt::new("76ac72f8-5b1e-4d67-a796-ef69cde27424", "Richard Thomas"),
    CardSet::Alpha,
    false,
    CardBehavior::BlackVise,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(1, 0),
        "As Black Vise enters, choose an opponent. At their upkeep, it deals 1 damage for each card in their hand beyond four.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The upkeep trigger currently resolves immediately instead of using the stack.",
    });

pub(in crate::card::sets) static COPPER_TABLET: CardRecord = CardRecord::new(
    cards::COPPER_TABLET,
    "Copper Tablet",
    CardArt::new("30935e4a-013e-4c46-ad05-304df8e5dfa4", "Amy Weber"),
    CardSet::Alpha,
    false,
    CardBehavior::CopperTablet,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(2, 0),
        "At the beginning of each player's upkeep, Copper Tablet deals 1 damage to that player.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The upkeep trigger currently resolves immediately instead of using the stack.",
});

pub(in crate::card::sets) static FIREBALL: CardRecord = CardRecord::new(
    cards::FIREBALL,
    "Fireball",
    CardArt::new("b7623c00-144b-4a8f-9c6c-f5e9e4f65ece", "Mark Tedin"),
    CardSet::Alpha,
    false,
    CardBehavior::Fireball,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::with_x(1),
        "Deal X damage divided evenly among the chosen targets. Each target beyond the first costs 1 more.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The complex multi-target cost and damage-division rules need a full correctness review.",
    });

pub(in crate::card::sets) static FORK: CardRecord = CardRecord::new(
    cards::FORK,
    "Fork",
    CardArt::new("e6b43916-fe2d-417a-a550-d7c795023297", "Amy Weber"),
    CardSet::Alpha,
    false,
    CardBehavior::Fork,
    CardRules::new(
        CardKind::Instant,
        ManaCost::new(0, 2),
        "Copy target instant or sorcery. You may choose new targets for the copy.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The copy does not become red as required by Fork's copy effect.",
});

pub(in crate::card::sets) static GLASSES_OF_URZA: CardRecord = CardRecord::new(
    cards::GLASSES_OF_URZA,
    "Glasses of Urza",
    CardArt::new("cafc2350-5d64-4379-9198-79a114654d45", "Douglas Shuler"),
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
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The activated ability currently resolves immediately instead of using the stack.",
});

pub(in crate::card::sets) static IRON_STAR: CardRecord = CardRecord::new(
    cards::IRON_STAR,
    "Iron Star",
    CardArt::new("5786de12-cade-43c2-a6b0-0c5b294b9d0e", "Dan Frazier"),
    CardSet::Alpha,
    false,
    CardBehavior::IronStar,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(1, 0),
        "Whenever a red spell is cast, you may pay 1. If you do, gain 1 life.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The spell-cast trigger currently bypasses the stack.",
});

pub(in crate::card::sets) static LIGHTNING_BOLT: CardRecord = CardRecord::new(
    cards::LIGHTNING_BOLT,
    "Lightning Bolt",
    CardArt::new("d573ef03-4730-45aa-93dd-e45ac1dbaf4a", "Christopher Rush"),
    CardSet::Alpha,
    false,
    CardBehavior::LightningBolt,
    CardRules::new(
        CardKind::Instant,
        ManaCost::new(0, 1),
        "Deal 3 damage to any target.",
    ),
);

pub(in crate::card::sets) static MOUNTAIN: CardRecord = CardRecord::new(
    cards::MOUNTAIN,
    "Mountain",
    CardArt::new("eace2c85-976c-425e-9800-5a6ccbd91b56", "Douglas Shuler"),
    CardSet::Alpha,
    true,
    CardBehavior::Mountain,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add R."),
);

pub(in crate::card::sets) static RED_ELEMENTAL_BLAST: CardRecord = CardRecord::new(
    cards::RED_ELEMENTAL_BLAST,
    "Red Elemental Blast",
    CardArt::new("776ad9be-3309-4f1d-9f27-6219d9477662", "Richard Thomas"),
    CardSet::Alpha,
    false,
    CardBehavior::RedElementalBlast,
    CardRules::new(
        CardKind::Instant,
        ManaCost::new(0, 1),
        "Counter target blue spell or destroy target blue permanent.",
    ),
);

pub(in crate::card::sets) static SHATTER: CardRecord = CardRecord::new(
    cards::SHATTER,
    "Shatter",
    CardArt::new("50dc7fc1-cb6a-4c68-b993-1a25cf16226e", "Amy Weber"),
    CardSet::Alpha,
    false,
    CardBehavior::Shatter,
    CardRules::new(
        CardKind::Instant,
        ManaCost::new(1, 1),
        "Destroy target artifact.",
    ),
);

pub(in crate::card::sets) static SMOKE: CardRecord = CardRecord::new(
    cards::SMOKE,
    "Smoke",
    CardArt::new("7c67788e-d713-47c3-ab9f-b8a6212ae24f", "Jesper Myrfors"),
    CardSet::Alpha,
    false,
    CardBehavior::Smoke,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::new(0, 2),
        "Players can't untap more than one creature during their untap steps.",
    ),
);

pub(in crate::card::sets) static STONE_GIANT: CardRecord = CardRecord::new(
    cards::STONE_GIANT,
    "Stone Giant",
    CardArt::new("7ffaedb9-25f8-4304-9085-e12505b93312", "Dameon Willich"),
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
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The activated ability and its delayed end-step trigger currently bypass the stack.",
    });

pub(in crate::card::sets) static WINTER_ORB: CardRecord = CardRecord::new(
    cards::WINTER_ORB,
    "Winter Orb",
    CardArt::new("9359f60c-9a27-4e53-b35b-964a121a6fba", "Mark Tedin"),
    CardSet::Alpha,
    false,
    CardBehavior::WinterOrb,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(2, 0),
        "While untapped, players can't untap more than one land during their untap steps.",
    ),
);

pub(in crate::card::sets) static BLACK_LOTUS: CardRecord = CardRecord::new(
    cards::BLACK_LOTUS,
    "Black Lotus",
    CardArt::new("b0faa7f2-b547-42c4-a810-839da50dadfe", "Christopher Rush"),
    CardSet::Alpha,
    false,
    CardBehavior::BlackLotus,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(0, 0),
        "Tap, sacrifice Black Lotus: Add RRR.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The catalog text incorrectly says the mana is red; the chosen color is not represented here.",
    });

pub(in crate::card::sets) static CHAOS_ORB: CardRecord = CardRecord::new(
    cards::CHAOS_ORB,
    "Chaos Orb",
    CardArt::new("92274971-7c4a-4326-b0fe-75e2d124f718", "Mark Tedin"),
    CardSet::Alpha,
    false,
    CardBehavior::ChaosOrb,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(2, 0),
        "1, Tap: Choose a permanent. On resolution, destroy it and Chaos Orb if Chaos Orb is still on the battlefield.",
    )
    .activated("Flip Chaos Orb onto {}", "Flip Chaos Orb onto a permanent"),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The engine uses a deterministic chosen-permanent approximation rather than the physical flip procedure.",
    });

pub(in crate::card::sets) static DRAGON_WHELP: CardRecord = CardRecord::new(
    cards::DRAGON_WHELP,
    "Dragon Whelp",
    CardArt::new("6bbf1eab-bc32-4835-b566-8634b1fe81b0", "Amy Weber"),
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
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The pump ability and its delayed end-step trigger currently bypass the stack.",
    });

pub(in crate::card::sets) static GOBLIN_BALLOON_BRIGADE: CardRecord = CardRecord::new(
    cards::GOBLIN_BALLOON_BRIGADE,
    "Goblin Balloon Brigade",
    CardArt::new("5129b422-7a35-4bc5-b14b-c814012a0d8f", "Andi Rusu"),
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
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The flying-granting activated ability currently resolves immediately instead of using the stack.",
    });

pub(in crate::card::sets) static GOBLIN_KING: CardRecord = CardRecord::new(
    cards::GOBLIN_KING,
    "Goblin King",
    CardArt::new("5873672d-37ea-4c0f-97f3-12b74fde112d", "Jesper Myrfors"),
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
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "Mountainwalk does not currently recognize Mountains supplied by dual-land subtypes.",
    });

pub(in crate::card::sets) static GRANITE_GARGOYLE: CardRecord = CardRecord::new(
    cards::GRANITE_GARGOYLE,
    "Granite Gargoyle",
    CardArt::new("f15bf2b2-6848-4fbd-b89a-8d8da8ae1cdc", "Christopher Rush"),
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
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The toughness-pumping activated ability currently resolves immediately instead of using the stack.",
    });

pub(in crate::card::sets) static IRONCLAW_ORCS: CardRecord = CardRecord::new(
    cards::IRONCLAW_ORCS,
    "Ironclaw Orcs",
    CardArt::new("d56421a8-34ae-4033-943f-c59a7bf2b6f9", "Anson Maddocks"),
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

pub(in crate::card::sets) static MOX_EMERALD: CardRecord = CardRecord::new(
    cards::MOX_EMERALD,
    "Mox Emerald",
    CardArt::new("b0e1427c-05cd-465b-be59-97ed6e39f7ba", "Dan Frazier"),
    CardSet::Alpha,
    false,
    CardBehavior::MoxEmerald,
    CardRules::new(CardKind::Artifact, ManaCost::new(0, 0), "Tap: Add 1."),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The catalog text does not identify the green mana this ability produces.",
});

pub(in crate::card::sets) static MOX_JET: CardRecord = CardRecord::new(
    cards::MOX_JET,
    "Mox Jet",
    CardArt::new("92bcd1ce-19b1-4d78-8b09-95242ca08d76", "Dan Frazier"),
    CardSet::Alpha,
    false,
    CardBehavior::MoxJet,
    CardRules::new(CardKind::Artifact, ManaCost::new(0, 0), "Tap: Add 1."),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The catalog text does not identify the black mana this ability produces.",
});

pub(in crate::card::sets) static MOX_PEARL: CardRecord = CardRecord::new(
    cards::MOX_PEARL,
    "Mox Pearl",
    CardArt::new("8ebe4be7-e12a-4596-a899-fbd5b152e879", "Dan Frazier"),
    CardSet::Alpha,
    false,
    CardBehavior::MoxPearl,
    CardRules::new(CardKind::Artifact, ManaCost::new(0, 0), "Tap: Add 1."),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The catalog text does not identify the white mana this ability produces.",
});

pub(in crate::card::sets) static MOX_RUBY: CardRecord = CardRecord::new(
    cards::MOX_RUBY,
    "Mox Ruby",
    CardArt::new("8945585f-4773-493d-a0fe-d707db910b38", "Dan Frazier"),
    CardSet::Alpha,
    false,
    CardBehavior::MoxRuby,
    CardRules::new(CardKind::Artifact, ManaCost::new(0, 0), "Tap: Add R."),
);

pub(in crate::card::sets) static MOX_SAPPHIRE: CardRecord = CardRecord::new(
    cards::MOX_SAPPHIRE,
    "Mox Sapphire",
    CardArt::new("82da0972-b17b-4600-9efd-e9430a0db04b", "Dan Frazier"),
    CardSet::Alpha,
    false,
    CardBehavior::MoxSapphire,
    CardRules::new(CardKind::Artifact, ManaCost::new(0, 0), "Tap: Add 1."),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The catalog text does not identify the blue mana this ability produces.",
});

pub(in crate::card::sets) static SOL_RING: CardRecord = CardRecord::new(
    cards::SOL_RING,
    "Sol Ring",
    CardArt::new("c4300d24-1cae-4dd5-be7e-38cc677cf5bd", "Mark Tedin"),
    CardSet::Alpha,
    false,
    CardBehavior::SolRing,
    CardRules::new(CardKind::Artifact, ManaCost::new(1, 0), "Tap: Add 2."),
);

pub(in crate::card::sets) static WHEEL_OF_FORTUNE: CardRecord = CardRecord::new(
    cards::WHEEL_OF_FORTUNE,
    "Wheel of Fortune",
    CardArt::new("67b369c4-faa8-45c8-a1b9-98f228b69682", "Daniel Gelon"),
    CardSet::Alpha,
    false,
    CardBehavior::WheelOfFortune,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::new(2, 1),
        "Each player discards their hand, then draws seven cards.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "Simultaneous discard, draw, and loss handling still uses a legacy shortcut and needs review.",
    });

pub(in crate::card::sets) static JUGGERNAUT: CardRecord = CardRecord::new(
    cards::JUGGERNAUT,
    "Juggernaut",
    CardArt::new("dcd6a291-5282-4f49-8203-d9b416083c48", "Dan Frazier"),
    CardSet::Alpha,
    false,
    CardBehavior::Juggernaut,
    CardRules::new(
        CardKind::ArtifactCreature,
        ManaCost::new(4, 0),
        "Attacks each combat if able. Juggernaut can't be blocked by Walls.",
    )
    .creature(5, 3),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The restriction preventing Walls from blocking Juggernaut is not implemented.",
});

pub(in crate::card::sets) static MANA_VAULT: CardRecord = CardRecord::new(
    cards::MANA_VAULT,
    "Mana Vault",
    CardArt::new("19499cb7-eccb-4e69-af32-6002d447a160", "Mark Tedin"),
    CardSet::Alpha,
    false,
    CardBehavior::ManaVault,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(1, 0),
        "Mana Vault doesn't untap during your untap step. At your upkeep, you may pay 4 to untap it. At your draw step, if tapped, it deals 1 damage to you. Tap: Add 3.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "Its upkeep and draw-step triggers currently resolve outside the stack.",
    });

pub(in crate::card::sets) static ANCESTRAL_RECALL: CardRecord = CardRecord::new(
    cards::ANCESTRAL_RECALL,
    "Ancestral Recall",
    CardArt::new("70e7ddf2-5604-41e7-bb9d-ddd03d3e9d0b", "Mark Poole"),
    CardSet::Alpha,
    false,
    CardBehavior::AncestralRecall,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 1, 0, 0, 0),
        "Target player draws three cards.",
    ),
);

pub(in crate::card::sets) static BRAINGEYSER: CardRecord = CardRecord::new(
    cards::BRAINGEYSER,
    "Braingeyser",
    CardArt::new("62b19a12-6914-430e-81ce-dcfca47884df", "Mark Tedin"),
    CardSet::Alpha,
    false,
    CardBehavior::Braingeyser,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored_x(0, 2, 0, 0, 0),
        "Target player draws X cards.",
    ),
);

pub(in crate::card::sets) static COUNTERSPELL: CardRecord = CardRecord::new(
    cards::COUNTERSPELL,
    "Counterspell",
    CardArt::new("0df55e3f-14de-46ef-b6b1-616618724d9e", "Mark Poole"),
    CardSet::Alpha,
    false,
    CardBehavior::Counterspell,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 2, 0, 0, 0),
        "Counter target spell.",
    ),
);

pub(in crate::card::sets) static DISENCHANT: CardRecord = CardRecord::new(
    cards::DISENCHANT,
    "Disenchant",
    CardArt::new("2722d7e2-61c6-4934-9c21-875ee78fd06c", "Amy Weber"),
    CardSet::Alpha,
    false,
    CardBehavior::Disenchant,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 1, 0, 0, 0, 0),
        "Destroy target artifact or enchantment.",
    ),
);

pub(in crate::card::sets) static ISLAND: CardRecord = CardRecord::new(
    cards::ISLAND,
    "Island",
    CardArt::new("90a57c0e-fa61-45ef-955d-d296403967d5", "Mark Poole"),
    CardSet::Alpha,
    true,
    CardBehavior::Island,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add U."),
);

pub(in crate::card::sets) static JAYEMDAE_TOME: CardRecord = CardRecord::new(
    cards::JAYEMDAE_TOME,
    "Jayemdae Tome",
    CardArt::new("cac8c421-5b92-481d-b2de-560c0231ab58", "Mark Tedin"),
    CardSet::Alpha,
    false,
    CardBehavior::JayemdaeTome,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(4, 0),
        "4, Tap: Draw a card.",
    ),
);

pub(in crate::card::sets) static PLAINS: CardRecord = CardRecord::new(
    cards::PLAINS,
    "Plains",
    CardArt::new("b1623d57-4729-4796-b3f7-f1837a05c6ed", "Jesper Myrfors"),
    CardSet::Alpha,
    true,
    CardBehavior::Plains,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add W."),
);

pub(in crate::card::sets) static SERRA_ANGEL: CardRecord = CardRecord::new(
    cards::SERRA_ANGEL,
    "Serra Angel",
    CardArt::new("f8ac5006-91bd-4803-93da-f87cf196dd2f", "Douglas Shuler"),
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

pub(in crate::card::sets) static SWORDS_TO_PLOWSHARES: CardRecord = CardRecord::new(
    cards::SWORDS_TO_PLOWSHARES,
    "Swords to Plowshares",
    CardArt::new("386ea9eb-abc1-4862-aa2d-8fb808d79490", "Jeff A. Menges"),
    CardSet::Alpha,
    false,
    CardBehavior::SwordsToPlowshares,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 1, 0, 0, 0, 0),
        "Exile target creature. Its controller gains life equal to its power.",
    ),
);

pub(in crate::card::sets) static TIME_WALK: CardRecord = CardRecord::new(
    cards::TIME_WALK,
    "Time Walk",
    CardArt::new("e0139f60-d48e-46fb-9f5a-1e3d7558c834", "Amy Weber"),
    CardSet::Alpha,
    false,
    CardBehavior::TimeWalk,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(1, 0, 1, 0, 0, 0),
        "Take an extra turn after this one.",
    ),
);

pub(in crate::card::sets) static TUNDRA: CardRecord = CardRecord::new(
    cards::TUNDRA,
    "Tundra",
    CardArt::new("a03e8c5b-f4ed-4fd7-ba05-db813ccc05eb", "Jesper Myrfors"),
    CardSet::Alpha,
    false,
    CardBehavior::Tundra,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add W or U."),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The Plains and Island basic land subtypes are not represented.",
});

pub(in crate::card::sets) static ARMAGEDDON: CardRecord = CardRecord::new(
    cards::ARMAGEDDON,
    "Armageddon",
    CardArt::new("5b6ddce7-b9c5-431d-a0b0-46d4aa93cbcb", "Jesper Myrfors"),
    CardSet::Alpha,
    false,
    CardBehavior::Armageddon,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(3, 1, 0, 0, 0, 0),
        "Destroy all lands.",
    ),
);

pub(in crate::card::sets) static BADLANDS: CardRecord = CardRecord::new(
    cards::BADLANDS,
    "Badlands",
    CardArt::new("717f6d10-9144-4ade-9ac6-a481cc66b875", "Rob Alexander"),
    CardSet::Alpha,
    false,
    CardBehavior::Badlands,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add B or R."),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The Swamp and Mountain basic land subtypes are not represented.",
});

pub(in crate::card::sets) static BALANCE: CardRecord = CardRecord::new(
    cards::BALANCE,
    "Balance",
    CardArt::new("6f9ea46a-411f-40ce-a873-a905180093f4", "Mark Poole"),
    CardSet::Alpha,
    false,
    CardBehavior::Balance,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(1, 1, 0, 0, 0, 0),
        "Each player discards and sacrifices creatures and lands until tied for the fewest of each.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The legacy decision sequence needs review against the simultaneous-choice procedure.",
    });

pub(in crate::card::sets) static BAYOU: CardRecord = CardRecord::new(
    cards::BAYOU,
    "Bayou",
    CardArt::new("412ceddd-2b9a-4551-a6bf-ae2830a2010a", "Jesper Myrfors"),
    CardSet::Alpha,
    false,
    CardBehavior::Bayou,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add B or G."),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The Swamp and Forest basic land subtypes are not represented.",
});

pub(in crate::card::sets) static BLACK_KNIGHT: CardRecord = CardRecord::new(
    cards::BLACK_KNIGHT,
    "Black Knight",
    CardArt::new("c1662949-0d69-49a3-8c69-daf10717ed4e", "Jeff A. Menges"),
    CardSet::Alpha,
    false,
    CardBehavior::BlackKnight,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 0, 0, 2, 0, 0),
        "First strike, protection from white.",
    )
    .creature(2, 2),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "First strike is not implemented.",
});

pub(in crate::card::sets) static BIRDS_OF_PARADISE: CardRecord = CardRecord::new(
    cards::BIRDS_OF_PARADISE,
    "Birds of Paradise",
    CardArt::new("55fe6449-1f23-43dc-adee-d144cd505b5c", "Mark Poole"),
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

pub(in crate::card::sets) static BLUE_ELEMENTAL_BLAST: CardRecord = CardRecord::new(
    cards::BLUE_ELEMENTAL_BLAST,
    "Blue Elemental Blast",
    CardArt::new("20d666ef-39bf-4fbf-8201-5f1056539da2", "Richard Thomas"),
    CardSet::Alpha,
    false,
    CardBehavior::BlueElementalBlast,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 1, 0, 0, 0),
        "Counter target red spell or destroy target red permanent.",
    ),
);

pub(in crate::card::sets) static CHANNEL: CardRecord = CardRecord::new(
    cards::CHANNEL,
    "Channel",
    CardArt::new("c1862c47-71cc-45a3-8805-a5ddc62e55ea", "Richard Thomas"),
    CardSet::Alpha,
    false,
    CardBehavior::Channel,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(0, 0, 0, 0, 0, 2),
        "Until end of turn, you may pay 1 life to add one colorless mana.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "Paying life for mana is not yet integrated with all mana-payment timing windows.",
});

pub(in crate::card::sets) static CRUSADE: CardRecord = CardRecord::new(
    cards::CRUSADE,
    "Crusade",
    CardArt::new("057986c7-20c0-4157-b4df-beae4ef5c66d", "Mark Poole"),
    CardSet::Alpha,
    false,
    CardBehavior::Crusade,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(0, 2, 0, 0, 0, 0),
        "White creatures get +1/+1.",
    ),
);

pub(in crate::card::sets) static DARK_RITUAL: CardRecord = CardRecord::new(
    cards::DARK_RITUAL,
    "Dark Ritual",
    CardArt::new("ebb6664d-23ca-456e-9916-afcd6f26aa7f", "Sandra Everingham"),
    CardSet::Alpha,
    false,
    CardBehavior::DarkRitual,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 0, 1, 0, 0),
        "Add BBB.",
    ),
);

pub(in crate::card::sets) static DEMONIC_TUTOR: CardRecord = CardRecord::new(
    cards::DEMONIC_TUTOR,
    "Demonic Tutor",
    CardArt::new("711d4d54-5520-4de8-9b93-79902ed8e562", "Douglas Shuler"),
    CardSet::Alpha,
    false,
    CardBehavior::DemonicTutor,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(1, 0, 0, 1, 0, 0),
        "Search your library for a card, put it into your hand, then shuffle.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "Resolving with an empty library currently creates an impossible mandatory choice and stalls the game.",
});

pub(in crate::card::sets) static DRAIN_LIFE: CardRecord = CardRecord::new(
    cards::DRAIN_LIFE,
    "Drain Life",
    CardArt::new("5d077a49-73d4-4958-b42a-31b814e110e8", "Douglas Shuler"),
    CardSet::Alpha,
    false,
    CardBehavior::DrainLife,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::variable(1, 0, 0, 1, 0, 0, 1),
        "Drain Life deals X damage to any target and you gain that much life.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "Black-mana spending restrictions and the life-gain cap are not fully enforced.",
});

pub(in crate::card::sets) static EARTHQUAKE: CardRecord = CardRecord::new(
    cards::EARTHQUAKE,
    "Earthquake",
    CardArt::new("e68ac362-6cdc-48a6-bdd3-4f8ea32add64", "Dan Frazier"),
    CardSet::Alpha,
    false,
    CardBehavior::Earthquake,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::with_x(1),
        "Earthquake deals X damage to each player and each creature without flying.",
    ),
);

static FOREST_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_mana(
    AbilityId::PRIMARY,
    "{T}: Add {G}.",
    &[AbilityCostDef::TapSource],
    EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Green)),
)];

pub(in crate::card::sets) static FOREST: CardRecord = CardRecord::new(
    cards::FOREST,
    "Forest",
    CardArt::new("6f1c8cb0-38eb-408b-94e8-16db83999b3b", "Christopher Rush"),
    CardSet::Alpha,
    true,
    CardBehavior::Forest,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add G.")
        .with_abilities(&FOREST_ABILITIES),
);

pub(in crate::card::sets) static HYPNOTIC_SPECTER: CardRecord = CardRecord::new(
    cards::HYPNOTIC_SPECTER,
    "Hypnotic Specter",
    CardArt::new("b43b900f-2d9b-442b-9699-058483604ec9", "Douglas Shuler"),
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
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The combat-damage trigger currently resolves without becoming a stack object.",
});

pub(in crate::card::sets) static MIND_TWIST: CardRecord = CardRecord::new(
    cards::MIND_TWIST,
    "Mind Twist",
    CardArt::new("eee9e106-a248-49d2-b8c8-6bbcd56ce739", "Julie Baroh"),
    CardSet::Alpha,
    false,
    CardBehavior::MindTwist,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored_x(0, 0, 1, 0, 0),
        "Target player discards X cards at random.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The spell always affects the opponent instead of selecting its target player.",
});

pub(in crate::card::sets) static NEVINYRRALS_DISK: CardRecord = CardRecord::new(
    cards::NEVINYRRALS_DISK,
    "Nevinyrral's Disk",
    CardArt::new("12926dc8-8e6f-4a47-a12b-4d674189615a", "Mark Tedin"),
    CardSet::Alpha,
    false,
    CardBehavior::NevinyrralsDisk,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(4, 0),
        "Enters tapped. 1, Tap: Destroy all artifacts, creatures, and enchantments. They can't be regenerated.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The destruction effect currently allows affected permanents to regenerate.",
});

pub(in crate::card::sets) static PLATEAU: CardRecord = CardRecord::new(
    cards::PLATEAU,
    "Plateau",
    CardArt::new("6eafa00b-c628-40f6-86eb-88e1361fc7a0", "Drew Tucker"),
    CardSet::Alpha,
    false,
    CardBehavior::Plateau,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add R or W."),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The Mountain and Plains basic land subtypes are not represented.",
});

pub(in crate::card::sets) static PSIONIC_BLAST: CardRecord = CardRecord::new(
    cards::PSIONIC_BLAST,
    "Psionic Blast",
    CardArt::new("a6a86e6e-bfff-46af-9d36-c912901fea92", "Douglas Shuler"),
    CardSet::Alpha,
    false,
    CardBehavior::PsionicBlast,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(2, 0, 1, 0, 0, 0),
        "Deal 4 damage to any target and 2 damage to you.",
    ),
);

pub(in crate::card::sets) static REGROWTH: CardRecord = CardRecord::new(
    cards::REGROWTH,
    "Regrowth",
    CardArt::new("badc73ec-3728-4246-90c7-5f4eb7051ed5", "Dameon Willich"),
    CardSet::Alpha,
    false,
    CardBehavior::Regrowth,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(1, 0, 0, 0, 0, 1),
        "Return target card from your graveyard to your hand.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The graveyard card is selected by position rather than as a target.",
});

pub(in crate::card::sets) static SAVANNAH: CardRecord = CardRecord::new(
    cards::SAVANNAH,
    "Savannah",
    CardArt::new("94f7e24c-2546-41b6-81ad-5e920b07e64e", "Rob Alexander"),
    CardSet::Alpha,
    false,
    CardBehavior::Savannah,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add G or W."),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The Forest and Plains basic land subtypes are not represented.",
});

pub(in crate::card::sets) static SAVANNAH_LIONS: CardRecord = CardRecord::new(
    cards::SAVANNAH_LIONS,
    "Savannah Lions",
    CardArt::new("d05b92bd-797e-413f-a8b0-32e0937a1ee0", "Daniel Gelon"),
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

pub(in crate::card::sets) static SCRUBLAND: CardRecord = CardRecord::new(
    cards::SCRUBLAND,
    "Scrubland",
    CardArt::new("bebe39d4-21fb-46a4-a1ec-b97102e46c15", "Jesper Myrfors"),
    CardSet::Alpha,
    false,
    CardBehavior::Scrubland,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add W or B."),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The Plains and Swamp basic land subtypes are not represented.",
});

pub(in crate::card::sets) static SENGIR_VAMPIRE: CardRecord = CardRecord::new(
    cards::SENGIR_VAMPIRE,
    "Sengir Vampire",
    CardArt::new("510840f4-7c0e-4b47-8ebf-23c20cac4bd9", "Anson Maddocks"),
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
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The damage-and-death trigger is not implemented as a stack ability.",
});

pub(in crate::card::sets) static SINKHOLE: CardRecord = CardRecord::new(
    cards::SINKHOLE,
    "Sinkhole",
    CardArt::new("04b31611-9053-4eaf-b392-21bb644fef5f", "Sandra Everingham"),
    CardSet::Alpha,
    false,
    CardBehavior::Sinkhole,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(0, 0, 0, 2, 0, 0),
        "Destroy target land.",
    ),
);

pub(in crate::card::sets) static SWAMP: CardRecord = CardRecord::new(
    cards::SWAMP,
    "Swamp",
    CardArt::new("6176936d-72e2-4205-8871-4c5a4f1cb2d8", "Dan Frazier"),
    CardSet::Alpha,
    true,
    CardBehavior::Swamp,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add B."),
);

static TAIGA_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_mana(
    AbilityId::PRIMARY,
    "{T}: Add {R} or {G}.",
    &[AbilityCostDef::TapSource],
    EffectDef::AddMana(AddManaEffectDef::choice(&[
        ManaKindDef::Red,
        ManaKindDef::Green,
    ])),
)];

pub(in crate::card::sets) static TAIGA: CardRecord = CardRecord::new(
    cards::TAIGA,
    "Taiga",
    CardArt::new("60df6592-0b3b-4b87-aeb2-8fa94b4fb7be", "Rob Alexander"),
    CardSet::Alpha,
    false,
    CardBehavior::Taiga,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add R or G.")
        .with_abilities(&TAIGA_ABILITIES),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The Mountain and Forest basic land subtypes are not represented.",
});

pub(in crate::card::sets) static TERROR: CardRecord = CardRecord::new(
    cards::TERROR,
    "Terror",
    CardArt::new("21004958-2c7e-4a55-bc80-411c4d780106", "Ron Spencer"),
    CardSet::Alpha,
    false,
    CardBehavior::Terror,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 0, 0, 1, 0, 0),
        "Destroy target nonartifact, nonblack creature. It can't be regenerated.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "Target selection and resolution do not account for protection from black.",
});

pub(in crate::card::sets) static TIME_VAULT: CardRecord = CardRecord::new(
    cards::TIME_VAULT,
    "Time Vault",
    CardArt::new("902441dc-c976-4c92-b897-6376eaa0fe38", "Mark Tedin"),
    CardSet::Alpha,
    false,
    CardBehavior::TimeVault,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(2, 0),
        "Enters tapped and doesn't untap normally. Skip a turn to untap it. Tap: Take an extra turn.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The hard-coded turn-skip shortcut needs review against replacement-effect timing.",
    });

pub(in crate::card::sets) static TIMETWISTER: CardRecord = CardRecord::new(
    cards::TIMETWISTER,
    "Timetwister",
    CardArt::new("9a49dc44-616e-4bdd-8220-0bb71eccc512", "Mark Tedin"),
    CardSet::Alpha,
    false,
    CardBehavior::Timetwister,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(2, 0, 1, 0, 0, 0),
        "Each player shuffles their hand and graveyard into their library, then draws seven cards.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "Simultaneous shuffle, draw, and loss handling still uses a legacy shortcut and needs review.",
    });

pub(in crate::card::sets) static TROPICAL_ISLAND: CardRecord = CardRecord::new(
    cards::TROPICAL_ISLAND,
    "Tropical Island",
    CardArt::new("a9c6c759-aabf-44e7-ba8c-33c5df232b56", "Jesper Myrfors"),
    CardSet::Alpha,
    false,
    CardBehavior::TropicalIsland,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add U or G."),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The Island and Forest basic land subtypes are not represented.",
});

pub(in crate::card::sets) static UNDERGROUND_SEA: CardRecord = CardRecord::new(
    cards::UNDERGROUND_SEA,
    "Underground Sea",
    CardArt::new("ff76ac86-8a8a-47fe-9388-8950ca3e26c3", "Rob Alexander"),
    CardSet::Alpha,
    false,
    CardBehavior::UndergroundSea,
    CardRules::new(CardKind::Land, ManaCost::new(0, 0), "Tap: Add U or B."),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The Island and Swamp basic land subtypes are not represented.",
});

pub(in crate::card::sets) static WHITE_KNIGHT: CardRecord = CardRecord::new(
    cards::WHITE_KNIGHT,
    "White Knight",
    CardArt::new("50abfba8-c9f9-4ebf-965a-4b425fe83129", "Daniel Gelon"),
    CardSet::Alpha,
    false,
    CardBehavior::WhiteKnight,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 2, 0, 0, 0, 0),
        "First strike, protection from black.",
    )
    .creature(2, 2),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "First strike is not implemented.",
});

pub(in crate::card::sets) static BERSERK: CardRecord = CardRecord::new(
    cards::BERSERK,
    "Berserk",
    CardArt::new("e173c8ce-2352-405e-ad00-e3bb94ced1ad", "Dan Frazier"),
    CardSet::Alpha,
    false,
    CardBehavior::Berserk,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 0, 0, 0, 1),
        "Target creature gains trample and gets +X/+0 until end of turn, where X is its power. Destroy it at end of turn if it attacked this turn.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "Targeting is restricted to your creatures, and the delayed destruction bypasses the stack.",
    });

pub(in crate::card::sets) static COPY_ARTIFACT: CardRecord = CardRecord::new(
    cards::COPY_ARTIFACT,
    "Copy Artifact",
    CardArt::new("fd5ed955-1193-4e6a-a3e2-f54c1f9bf063", "Amy Weber"),
    CardSet::Alpha,
    false,
    CardBehavior::CopyArtifact,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(1, 0, 1, 0, 0, 0),
        "You may have Copy Artifact enter as a copy of any artifact on the battlefield.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The optional copy choice is incorrectly modeled as a targeted spell choice.",
});

pub(in crate::card::sets) static GIANT_GROWTH: CardRecord = CardRecord::new(
    cards::GIANT_GROWTH,
    "Giant Growth",
    CardArt::new("367dbefe-3366-408e-9fcf-7dc00f8cc201", "Sandra Everingham"),
    CardSet::Alpha,
    false,
    CardBehavior::GiantGrowth,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 0, 0, 0, 1),
        "Target creature gets +3/+3 until end of turn.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "Targeting is incorrectly restricted to creatures you control.",
});

pub(in crate::card::sets) static ICY_MANIPULATOR: CardRecord = CardRecord::new(
    cards::ICY_MANIPULATOR,
    "Icy Manipulator",
    CardArt::new("29dc1596-a2e7-4d60-9f99-89babaef8a06", "Douglas Shuler"),
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
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The current target selector also permits enchantments.",
});

pub(in crate::card::sets) static LLANOWAR_ELVES: CardRecord = CardRecord::new(
    cards::LLANOWAR_ELVES,
    "Llanowar Elves",
    CardArt::new("d4f1cc9e-4f99-4c26-ac1b-8ef069fa8ceb", "Anson Maddocks"),
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

pub(in crate::card::sets) static SCRYB_SPRITES: CardRecord = CardRecord::new(
    cards::SCRYB_SPRITES,
    "Scryb Sprites",
    CardArt::new("6d929c38-91e6-457c-937a-d1884f4bba44", "Amy Weber"),
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

pub(in crate::card::sets) static STONE_RAIN: CardRecord = CardRecord::new(
    cards::STONE_RAIN,
    "Stone Rain",
    CardArt::new("57ff74cb-a2ed-4123-ac42-f72f9820049e", "Daniel Gelon"),
    CardSet::Alpha,
    false,
    CardBehavior::StoneRain,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::new(2, 1),
        "Destroy target land.",
    ),
);

// The chosen presentation art is its Beta printing; the definition debuted in Alpha.
pub(in crate::card::sets) static SEDGE_TROLL: CardRecord = CardRecord::new(
    cards::SEDGE_TROLL,
    "Sedge Troll",
    CardArt::new("02ec317b-52a6-4490-80e5-a56826b06771", "Dan Frazier"),
    CardSet::Alpha,
    false,
    CardBehavior::SedgeTroll,
    CardRules::new(
        CardKind::Creature,
        ManaCost::new(2, 1),
        "Sedge Troll gets +1/+1 as long as you control a Swamp. B: Regenerate Sedge Troll.",
    )
    .creature(2, 2),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "Its regeneration ability is currently offered and charged as red mana instead of black mana.",
});

pub(in crate::card::sets) static WRATH_OF_GOD: CardRecord = CardRecord::new(
    cards::WRATH_OF_GOD,
    "Wrath of God",
    CardArt::new("a2788d69-6a3a-42f0-8736-cc6b57755ecd", "Quinton Hoover"),
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
    &SEDGE_TROLL,
    &WRATH_OF_GOD,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&PLAINS, 1),
    PrintingRecord::alternate(&ISLAND, 1),
    PrintingRecord::alternate(&SWAMP, 1),
    PrintingRecord::alternate(&MOUNTAIN, 1),
    PrintingRecord::alternate(&FOREST, 1),
];
