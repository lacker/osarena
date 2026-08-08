//! Dark Ascension card records used by the built-in ISD–RTR Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, CardArt, CardComposition, CardEffectStatus,
    CardKind, CardPart, CardRules, CardSet, CardStructure, DoubleFacedKind, EffectDef, LandEntry,
    ManaCost, ManaKindDef, PlayOptionDef, SpellForm, cards,
};
use crate::ids::{AbilityId, CardPartId, PlayOptionId};

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static HELLRIDER: CardRecord = CardRecord::new(
    cards::HELLRIDER,
    "Hellrider",
    CardArt::new("0ec8d800-7f06-44e0-b22d-cdff0a9b153d", "Svetlin Velinov"),
    CardSet::DarkAscension,
    false,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(2, 0, 0, 0, 2, 0),
        "Haste\nWhenever a creature you control attacks, this creature deals 1 damage to the player or planeswalker it's attacking.",
    )
    .type_line("Creature — Devil")
    .creature(3, 3)
    .haste()
    .metadata_only(),
);

const fn huntmaster_front_rules() -> CardRules {
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(2, 0, 0, 0, 1, 1),
        "Whenever this creature enters or transforms into Huntmaster of the Fells, create a 2/2 green Wolf creature token and you gain 2 life.\nAt the beginning of each upkeep, if no spells were cast last turn, transform this creature.",
    )
    .type_line("Creature — Human Werewolf")
    .creature(2, 2)
    .metadata_only()
}

fn huntmaster_composition() -> CardComposition {
    let front = huntmaster_front_rules();
    let back = CardRules::new(
        CardKind::Creature,
        ManaCost::default(),
        "Trample\nWhenever this creature transforms into Ravager of the Fells, it deals 2 damage to target opponent or planeswalker and 2 damage to up to one target creature that player or that planeswalker's controller controls.\nAt the beginning of each upkeep, if a player cast two or more spells last turn, transform this creature.",
    )
    .type_line("Creature — Werewolf")
    .printed_colors([false, false, false, true, true])
    .creature(4, 4)
    .trample()
    .metadata_only();
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Huntmaster of the Fells", front),
            CardPart::new(CardPartId(1), "Ravager of the Fells", back).without_mana_cost(),
        ],
        structure: CardStructure::DoubleFaced {
            front: CardPartId::PRIMARY,
            back: CardPartId(1),
            kind: DoubleFacedKind::Transforming,
        },
        play_options: vec![PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Huntmaster of the Fells",
            SpellForm::Part(CardPartId::PRIMARY),
            front.mana_cost,
            CardEffectStatus::MetadataOnly,
        )],
    }
}

// Implementation status: Baseline front-face creature is playable; both faces and transformation topology are cataloged, while triggers are pending.
pub(in crate::card::sets) static HUNTMASTER_OF_THE_FELLS: CardRecord = CardRecord::new(
    cards::HUNTMASTER_OF_THE_FELLS,
    "Huntmaster of the Fells",
    CardArt::new("aae6fb12-b252-453b-bca7-1ea2a0d6c8dc", "Chris Rahn"),
    CardSet::DarkAscension,
    false,
    huntmaster_front_rules(),
)
.with_composition(huntmaster_composition);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static RAY_OF_REVELATION: CardRecord = CardRecord::new(
    cards::RAY_OF_REVELATION,
    "Ray of Revelation",
    CardArt::new("d7e2c5a4-cf92-46bd-9033-8036436488cb", "Cliff Childs"),
    CardSet::DarkAscension,
    false,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 1, 0, 0, 0, 0),
        "Destroy target enchantment.\nFlashback {G} (You may cast this card from your graveyard for its flashback cost. Then exile it.)",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static STRANGLEROOT_GEIST: CardRecord = CardRecord::new(
    cards::STRANGLEROOT_GEIST,
    "Strangleroot Geist",
    CardArt::new("bf1fb137-205c-480f-b6dc-dfa137793ae3", "Jason Chan"),
    CardSet::DarkAscension,
    false,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(0, 0, 0, 0, 0, 2),
        "Haste\nUndying (When this creature dies, if it had no +1/+1 counters on it, return it to the battlefield under its owner's control with a +1/+1 counter on it.)",
    )
    .type_line("Creature — Spirit")
    .creature(2, 1)
    .undying()
    .haste()
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static TRAGIC_SLIP: CardRecord = CardRecord::new(
    cards::TRAGIC_SLIP,
    "Tragic Slip",
    CardArt::new("09666671-601e-4fca-bdfb-fb288bf2672c", "Christopher Moeller"),
    CardSet::DarkAscension,
    false,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 0, 1, 0, 0),
        "Target creature gets -1/-1 until end of turn.\nMorbid — That creature gets -13/-13 until end of turn instead if a creature died this turn.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Land entry and modeled mana production are active; other printed abilities are pending.
pub(in crate::card::sets) static VAULT_OF_THE_ARCHANGEL: CardRecord = CardRecord::new(
    cards::VAULT_OF_THE_ARCHANGEL,
    "Vault of the Archangel",
    CardArt::new("35a65437-430a-42ef-854f-6e66f8e1a04a", "John Avon"),
    CardSet::DarkAscension,
    false,
    CardRules::new(CardKind::Land, ManaCost::colored(0, 0, 0, 0, 0, 0), "")
    .type_line("Land")
    .land_entry(LandEntry::Untapped)
    .with_abilities(&[
        AbilityDef::activated_mana(
            AbilityId::PRIMARY,
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Colorless)),
        ),
        AbilityDef::not_implemented(
            AbilityId(1),
            "{2}{W}{B}, {T}: Creatures you control gain deathtouch and lifelink until end of turn.",
            "The deathtouch- and lifelink-granting activated ability is not executed.",
        ),
    ])
    .with_effect_status(CardEffectStatus::MetadataOnly),
);
pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &HELLRIDER,
    &HUNTMASTER_OF_THE_FELLS,
    &RAY_OF_REVELATION,
    &STRANGLEROOT_GEIST,
    &TRAGIC_SLIP,
    &VAULT_OF_THE_ARCHANGEL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
