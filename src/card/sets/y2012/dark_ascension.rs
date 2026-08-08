//! Dark Ascension card records used by the built-in ISD–RTR Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityImplementationDef, AddManaEffectDef, CardArt,
    CardComposition, CardEffectStatus, CardKind, CardPart, CardRules, CardSet, CardStructure,
    DoubleFacedKind, EffectDef, EvergreenAbility, LandEntry, ManaCost, ManaKindDef, PlayOptionDef,
    SpellForm, cards,
};
use crate::ids::{AbilityId, CardPartId, PlayOptionId};

pub(in crate::card::sets) static HELLRIDER: CardRecord = CardRecord::new(
    cards::HELLRIDER,
    "Hellrider",
    CardArt::new("0ec8d800-7f06-44e0-b22d-cdff0a9b153d", "Svetlin Velinov"),
    CardSet::DarkAscension,
        CardRules::new(CardKind::Creature, ManaCost::colored(2, 0, 0, 0, 2, 0), "")
    .with_subtypes(&["Devil"])
    .creature(3, 3)
    .with_abilities(&[
        AbilityDef::evergreen(AbilityId::PRIMARY, "Haste", EvergreenAbility::Haste),
        AbilityDef::not_implemented(
            AbilityId(1),
            "Whenever a creature you control attacks, this creature deals 1 damage to the player or planeswalker it's attacking.",
            "The attack trigger is not executed.",
        ),
    ]),
);

const fn huntmaster_front_rules() -> CardRules {
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(2, 0, 0, 0, 1, 1),
        "Whenever this creature enters or transforms into Huntmaster of the Fells, create a 2/2 green Wolf creature token and you gain 2 life.\nAt the beginning of each upkeep, if no spells were cast last turn, transform this creature.",
    )
    .with_subtypes(&["Human", "Werewolf"])
    .creature(2, 2)
    .metadata_only()
}

static HUNTMASTER_BACK_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::evergreen(AbilityId::PRIMARY, "Trample", EvergreenAbility::Trample),
    AbilityDef::not_implemented(
        AbilityId(1),
        "Whenever this creature transforms into Ravager of the Fells, it deals 2 damage to target opponent or planeswalker and 2 damage to up to one target creature that player or that planeswalker's controller controls.\nAt the beginning of each upkeep, if a player cast two or more spells last turn, transform this creature.",
        "The transform trigger, damage trigger, and transformation procedure are not executed.",
    ),
];

const fn huntmaster_back_rules() -> CardRules {
    CardRules::new(CardKind::Creature, ManaCost::new(0, 0), "")
        .with_subtypes(&["Werewolf"])
        .printed_colors([false, false, false, true, true])
        .creature(4, 4)
        .with_abilities(&HUNTMASTER_BACK_ABILITIES)
}

fn huntmaster_composition() -> CardComposition {
    let front = huntmaster_front_rules();
    let back = huntmaster_back_rules();
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

pub(in crate::card::sets) static HUNTMASTER_OF_THE_FELLS: CardRecord = CardRecord::new(
    cards::HUNTMASTER_OF_THE_FELLS,
    "Huntmaster of the Fells",
    CardArt::new("aae6fb12-b252-453b-bca7-1ea2a0d6c8dc", "Chris Rahn"),
    CardSet::DarkAscension,
    huntmaster_front_rules(),
)
.with_composition(huntmaster_composition);

pub(in crate::card::sets) static RAY_OF_REVELATION: CardRecord = CardRecord::new(
    cards::RAY_OF_REVELATION,
    "Ray of Revelation",
    CardArt::new("d7e2c5a4-cf92-46bd-9033-8036436488cb", "Cliff Childs"),
    CardSet::DarkAscension,
        CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 1, 0, 0, 0, 0),
        "Destroy target enchantment.\nFlashback {G} (You may cast this card from your graveyard for its flashback cost. Then exile it.)",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static STRANGLEROOT_GEIST: CardRecord = CardRecord::new(
    cards::STRANGLEROOT_GEIST,
    "Strangleroot Geist",
    CardArt::new("bf1fb137-205c-480f-b6dc-dfa137793ae3", "Jason Chan"),
    CardSet::DarkAscension,
        CardRules::new(CardKind::Creature, ManaCost::colored(0, 0, 0, 0, 0, 2), "")
    .with_subtypes(&["Spirit"])
    .creature(2, 1)
    .with_abilities(&[
        AbilityDef::evergreen(AbilityId::PRIMARY, "Haste", EvergreenAbility::Haste),
        AbilityDef::evergreen(
            AbilityId(1),
            "Undying (When this creature dies, if it had no +1/+1 counters on it, return it to the battlefield under its owner's control with a +1/+1 counter on it.)",
            EvergreenAbility::Undying,
        )
        .with_implementation(AbilityImplementationDef::NotImplemented {
            explanation: "The undying trigger is not executed.",
        }),
    ]),
);

pub(in crate::card::sets) static TRAGIC_SLIP: CardRecord = CardRecord::new(
    cards::TRAGIC_SLIP,
    "Tragic Slip",
    CardArt::new("09666671-601e-4fca-bdfb-fb288bf2672c", "Christopher Moeller"),
    CardSet::DarkAscension,
        CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 0, 1, 0, 0),
        "Target creature gets -1/-1 until end of turn.\nMorbid — That creature gets -13/-13 until end of turn instead if a creature died this turn.",
    )
    .metadata_only(),
);

pub(in crate::card::sets) static VAULT_OF_THE_ARCHANGEL: CardRecord = CardRecord::new(
    cards::VAULT_OF_THE_ARCHANGEL,
    "Vault of the Archangel",
    CardArt::new("35a65437-430a-42ef-854f-6e66f8e1a04a", "John Avon"),
    CardSet::DarkAscension,
        CardRules::new(CardKind::Land, ManaCost::colored(0, 0, 0, 0, 0, 0), "")
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
    ]),
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
