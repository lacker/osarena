//! Dragon's Maze card records used by the built-in ISD–RTR Standard decks.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    CardArt, CardComposition, CardEffectStatus, CardKind, CardPart, CardRules, CardSet,
    CardStructure, ManaCost, PlayOptionDef, SpellForm, TargetPredicate, TargetSlotDef, cards,
};
use crate::ids::{CardPartId, PlayOptionId, TargetSlotId};

// Implementation status: Baseline creature casting/combat and declaratively modeled traits are active; remaining printed abilities are pending.
pub(in crate::card::sets) static AETHERLING: CardRecord = CardRecord::new(
    cards::AETHERLING,
    "Aetherling",
    CardArt::new("9c93313b-cf43-47e9-a911-717b4d14b0b5", "Tyler Jacobson"),
    CardSet::DragonsMaze,
    false,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(4, 0, 2, 0, 0, 0),
        "{U}: Exile this creature. Return it to the battlefield under its owner's control at the beginning of the next end step.\n{U}: This creature can't be blocked this turn.\n{1}: This creature gets +1/-1 until end of turn.\n{1}: This creature gets -1/+1 until end of turn.",
    )
    .type_line("Creature — Shapeshifter")
    .creature(4, 5)
    .metadata_only(),
);

// Implementation status: Baseline creature casting/combat and declaratively modeled traits are active; remaining printed abilities are pending.
pub(in crate::card::sets) static BLOOD_BARON_OF_VIZKOPA: CardRecord = CardRecord::new(
    cards::BLOOD_BARON_OF_VIZKOPA,
    "Blood Baron of Vizkopa",
    CardArt::new("e4edad09-bf7b-40e9-ac2a-100da8a43274", "Anthony Palumbo"),
    CardSet::DragonsMaze,
    false,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(3, 1, 0, 1, 0, 0),
        "Lifelink, protection from white and from black\nAs long as you have 30 or more life and an opponent has 10 or less life, this creature gets +6/+6 and has flying.",
    )
    .type_line("Creature — Vampire")
    .creature(4, 4)
    .lifelink()
    .protection([true, false, true, false, false])
    .metadata_only(),
);

// Implementation status: Metadata only; this spell is withheld from legal actions.
pub(in crate::card::sets) static GAZE_OF_GRANITE: CardRecord = CardRecord::new(
    cards::GAZE_OF_GRANITE,
    "Gaze of Granite",
    CardArt::new("96c9ac10-d114-4aa5-87ac-f1069cde8e40", "Nils Hamm"),
    CardSet::DragonsMaze,
    false,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::variable(0, 0, 0, 2, 0, 1, 1),
        "Destroy each nonland permanent with mana value X or less.",
    )
    .type_line("Sorcery")
    .metadata_only(),
);

// Implementation status: Metadata only; this spell is withheld from legal actions.
pub(in crate::card::sets) static PUTREFY: CardRecord = CardRecord::new(
    cards::PUTREFY,
    "Putrefy",
    CardArt::new("0d43a0b6-2a5c-4959-96ee-6e570949dfed", "Igor Kieryluk"),
    CardSet::DragonsMaze,
    false,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 0, 0, 1, 0, 1),
        "Destroy target artifact or creature. It can't be regenerated.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Baseline creature casting/combat and declaratively modeled traits are active; remaining printed abilities are pending.
pub(in crate::card::sets) static RURIC_THAR_THE_UNBOWED: CardRecord = CardRecord::new(
    cards::RURIC_THAR_THE_UNBOWED,
    "Ruric Thar, the Unbowed",
    CardArt::new("84dd3586-7c3b-4f9c-a1eb-7745b75339b0", "Tyler Jacobson"),
    CardSet::DragonsMaze,
    false,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(4, 0, 0, 0, 1, 1),
        "Vigilance, reach\nRuric Thar attacks each combat if able.\nWhenever a player casts a noncreature spell, Ruric Thar deals 6 damage to that player.",
    )
    .type_line("Legendary Creature — Ogre Warrior")
    .creature(6, 6)
    .legendary()
    .vigilance()
    .reach()
    .metadata_only(),
);

// Implementation status: Baseline creature casting/combat and declaratively modeled traits are active; remaining printed abilities are pending.
pub(in crate::card::sets) static SIN_COLLECTOR: CardRecord = CardRecord::new(
    cards::SIN_COLLECTOR,
    "Sin Collector",
    CardArt::new("305a3feb-df49-486c-a3b4-ff2721d60019", "Mike Bierek"),
    CardSet::DragonsMaze,
    false,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 1, 0, 1, 0, 0),
        "When this creature enters, target opponent reveals their hand. You choose an instant or sorcery card from it and exile that card.",
    )
    .type_line("Creature — Human Cleric")
    .creature(2, 1)
    .metadata_only(),
);

const fn turn_rules() -> CardRules {
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(2, 0, 1, 0, 0, 0),
        "Until end of turn, target creature loses all abilities and becomes a red Weird with base power and toughness 0/1.\nFuse (You may cast one or both halves of this card from your hand.)",
    )
    .type_line("Instant")
    .metadata_only()
}

fn turn_burn_composition() -> CardComposition {
    let turn = turn_rules();
    let burn = CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 0, 0, 0, 1, 0),
        "Burn deals 2 damage to any target.\nFuse (You may cast one or both halves of this card from your hand.)",
    )
    .type_line("Instant")
    .metadata_only();
    let turn_target = || {
        TargetSlotDef::exactly_one(
            TargetSlotId(0),
            "creature for Turn",
            TargetPredicate::CreaturePermanent,
        )
    };
    let burn_target = || {
        TargetSlotDef::exactly_one(
            TargetSlotId(1),
            "target for Burn",
            TargetPredicate::AnyTarget,
        )
    };
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Turn", turn),
            CardPart::new(CardPartId(1), "Burn", burn),
        ],
        structure: CardStructure::Split {
            parts: vec![CardPartId::PRIMARY, CardPartId(1)],
            fused: Some(PlayOptionId(2)),
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Turn",
                SpellForm::Part(CardPartId::PRIMARY),
                turn.mana_cost,
                CardEffectStatus::MetadataOnly,
            )
            .with_targets(vec![turn_target()]),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Burn",
                SpellForm::Part(CardPartId(1)),
                burn.mana_cost,
                CardEffectStatus::MetadataOnly,
            )
            .with_targets(vec![burn_target()]),
            PlayOptionDef::cast(
                PlayOptionId(2),
                "Turn // Burn",
                SpellForm::Combined(vec![CardPartId::PRIMARY, CardPartId(1)]),
                ManaCost::colored(3, 0, 1, 0, 1, 0),
                CardEffectStatus::MetadataOnly,
            )
            .with_targets(vec![turn_target(), burn_target()])
            .restricted_to_hand(),
        ],
    }
}

// Implementation status: Both split parts, independent play options, and the hand-only fused form are cataloged; effect execution is pending.
pub(in crate::card::sets) static TURN_BURN: CardRecord = CardRecord::new(
    cards::TURN_BURN,
    "Turn // Burn",
    CardArt::new("8d7fdd59-6d76-4a0c-ac75-816345ef4a39", "Ryan Barger"),
    CardSet::DragonsMaze,
    false,
    turn_rules(),
)
.with_composition(turn_burn_composition);

// Implementation status: Metadata only; this spell is withheld from legal actions.
pub(in crate::card::sets) static UNFLINCHING_COURAGE: CardRecord = CardRecord::new(
    cards::UNFLINCHING_COURAGE,
    "Unflinching Courage",
    CardArt::new("35952c24-d728-4ec6-b0d1-b8183a18554a", "Mike Bierek"),
    CardSet::DragonsMaze,
    false,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(1, 1, 0, 0, 0, 1),
        "Enchant creature\nEnchanted creature gets +2/+2 and has trample and lifelink. (Damage dealt by the creature also causes its controller to gain that much life.)",
    )
    .type_line("Enchantment — Aura")
    .metadata_only(),
);

// Implementation status: Baseline creature casting/combat and declaratively modeled traits are active; remaining printed abilities are pending.
pub(in crate::card::sets) static VOICE_OF_RESURGENCE: CardRecord = CardRecord::new(
    cards::VOICE_OF_RESURGENCE,
    "Voice of Resurgence",
    CardArt::new("07246783-d475-4f61-99ac-e2b574072349", "Winona Nelson"),
    CardSet::DragonsMaze,
    false,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 1, 0, 0, 0, 1),
        "Whenever an opponent casts a spell during your turn and when this creature dies, create a green and white Elemental creature token with \"This token's power and toughness are each equal to the number of creatures you control.\"",
    )
    .type_line("Creature — Elemental")
    .creature(2, 2)
    .metadata_only(),
);

// Implementation status: Metadata only; this spell is withheld from legal actions.
pub(in crate::card::sets) static WARLEADERS_HELIX: CardRecord = CardRecord::new(
    cards::WARLEADERS_HELIX,
    "Warleader's Helix",
    CardArt::new("81e474ac-54f7-43f9-8af9-2f1adf258b15", "Greg Staples"),
    CardSet::DragonsMaze,
    false,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(2, 1, 0, 0, 1, 0),
        "Warleader's Helix deals 4 damage to any target and you gain 4 life.",
    )
    .type_line("Instant")
    .metadata_only(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AETHERLING,
    &BLOOD_BARON_OF_VIZKOPA,
    &GAZE_OF_GRANITE,
    &PUTREFY,
    &RURIC_THAR_THE_UNBOWED,
    &SIN_COLLECTOR,
    &TURN_BURN,
    &UNFLINCHING_COURAGE,
    &VOICE_OF_RESURGENCE,
    &WARLEADERS_HELIX,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
