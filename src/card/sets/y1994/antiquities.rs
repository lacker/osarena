use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, CardArt, CardBehavior, CardKind, CardRules,
    CardSet, EffectDef, ImplementationStatus, ManaCost, ManaKindDef, ManaRestrictionDef,
    ObjectPredicateDef, TriggerEventDef, ZoneKind, cards,
};
use crate::ids::AbilityId;

pub(in crate::card::sets) static ATOG: CardRecord = CardRecord::new(
    cards::ATOG,
    "Atog",
    CardArt::new("2249fc40-4412-48fd-800a-7ea3678aee3f", "Jesper Myrfors"),
    CardSet::Antiquities,
    false,
    CardBehavior::Atog,
    CardRules::new(
        CardKind::Creature,
        ManaCost::new(1, 1),
        "Sacrifice an artifact: Atog gets +2/+2 until end of turn.",
    )
    .creature(1, 2),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The activated ability currently resolves immediately instead of using the stack.",
});

pub(in crate::card::sets) static DETONATE: CardRecord = CardRecord::new(
    cards::DETONATE,
    "Detonate",
    CardArt::new(
        "ffd7eb90-ae95-49df-898a-9510187bce1c",
        "Randy Asplund-Faith",
    ),
    CardSet::Antiquities,
    false,
    CardBehavior::Detonate,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::with_x(1),
        "Destroy target artifact with mana value X. Its controller takes X damage.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The no-regeneration clause is absent from both the catalog text and resolution behavior.",
    });

static SU_CHI_ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered(
    AbilityId::PRIMARY,
    "When Su-Chi dies, add {C}{C}{C}{C}.",
    TriggerEventDef::ZoneChanged {
        object: ObjectPredicateDef::Source,
        from: Some(ZoneKind::Battlefield),
        to: Some(ZoneKind::Graveyard),
    },
    EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Colorless).with_amount(4)),
)];

pub(in crate::card::sets) static SU_CHI: CardRecord = CardRecord::new(
    cards::SU_CHI,
    "Su-Chi",
    CardArt::new("a64d4f93-0c04-4078-aec0-7e9de92f260f", "Christopher Rush"),
    CardSet::Antiquities,
    false,
    CardBehavior::SuChi,
    CardRules::new(
        CardKind::ArtifactCreature,
        ManaCost::new(4, 0),
        "When Su-Chi dies, add 4.",
    )
    .creature(4, 4)
    .with_abilities(&SU_CHI_ABILITIES),
);

pub(in crate::card::sets) static MISHRA_S_FACTORY: CardRecord = CardRecord::new(
    cards::MISHRA_S_FACTORY,
    "Mishra's Factory",
    CardArt::new("a696c5b6-f216-454d-8029-74e84bbd1428", "Kaja Foglio & Phil Foglio"),
    CardSet::Antiquities,
    false,
    CardBehavior::MishrasFactory,
    CardRules::new(
        CardKind::Land,
        ManaCost::new(0, 0),
        "Tap: Add 1. 1: Becomes a 2/2 Assembly-Worker artifact creature until end of turn. Tap: Target Assembly-Worker gets +1/+1 until end of turn.",
    )
    .activated("Give {} +1/+1 with Mishra's Factory", "Give an Assembly-Worker +1/+1"),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "Its animation and pump activated abilities currently resolve immediately instead of using the stack.",
    });

pub(in crate::card::sets) static ORCISH_MECHANICS: CardRecord = CardRecord::new(
    cards::ORCISH_MECHANICS,
    "Orcish Mechanics",
    CardArt::new("5e34fc6b-5f00-4a22-9ee2-afc1caf99961", "Pete Venters"),
    CardSet::Antiquities,
    false,
    CardBehavior::OrcishMechanics,
    CardRules::new(
        CardKind::Creature,
        ManaCost::new(1, 1),
        "Tap, sacrifice an artifact: Deal 2 damage to any target.",
    )
    .creature(1, 1)
    .activated("Deal 2 damage to {} with Orcish Mechanics", "Deal 2 damage"),
);

pub(in crate::card::sets) static STRIP_MINE: CardRecord = CardRecord::new(
    cards::STRIP_MINE,
    "Strip Mine",
    CardArt::new("e7880157-7f27-4f1b-9cdc-ab36a6252376", "Daniel Gelon"),
    CardSet::Antiquities,
    false,
    CardBehavior::StripMine,
    CardRules::new(
        CardKind::Land,
        ManaCost::new(0, 0),
        "Tap, sacrifice Strip Mine: Destroy target land.",
    )
    .activated("Destroy {} with Strip Mine", "Destroy a land"),
);

pub(in crate::card::sets) static TRISKELION: CardRecord = CardRecord::new(
    cards::TRISKELION,
    "Triskelion",
    CardArt::new("a79c99e1-722a-44b6-8fa3-2be3f0c193d8", "Douglas Shuler"),
    CardSet::Antiquities,
    false,
    CardBehavior::Triskelion,
    CardRules::new(
        CardKind::ArtifactCreature,
        ManaCost::new(6, 0),
        "Enters with three +1/+1 counters. Remove a +1/+1 counter: Deal 1 damage to any target.",
    )
    .creature(1, 1)
    .activated("Deal 1 damage to {} with Triskelion", "Deal 1 damage"),
);

pub(in crate::card::sets) static IVORY_TOWER: CardRecord = CardRecord::new(
    cards::IVORY_TOWER,
    "Ivory Tower",
    CardArt::new(
        "a5f23039-45ca-4c15-af50-bfd40ea26453",
        "Margaret Organ-Kean",
    ),
    CardSet::Antiquities,
    false,
    CardBehavior::IvoryTower,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::new(1, 0),
        "At the beginning of your upkeep, gain 1 life for each card in your hand beyond four.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The upkeep trigger currently resolves immediately instead of using the stack.",
});

static MISHRA_S_WORKSHOP_RESTRICTIONS: [ManaRestrictionDef; 1] =
    [ManaRestrictionDef::CastSpell(ObjectPredicateDef::Artifact)];

static MISHRA_S_WORKSHOP_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_mana(
    AbilityId::PRIMARY,
    "{T}: Add {C}{C}{C}. Spend this mana only to cast artifact spells.",
    &[AbilityCostDef::TapSource],
    EffectDef::AddMana(
        AddManaEffectDef::one(ManaKindDef::Colorless)
            .with_amount(3)
            .with_restrictions(&MISHRA_S_WORKSHOP_RESTRICTIONS),
    ),
)];

pub(in crate::card::sets) static MISHRA_S_WORKSHOP: CardRecord = CardRecord::new(
    cards::MISHRA_S_WORKSHOP,
    "Mishra's Workshop",
    CardArt::new("135de5c7-6ac9-4b68-8f1a-97f120a4b125", "Kaja Foglio"),
    CardSet::Antiquities,
    false,
    CardBehavior::MishrasWorkshop,
    CardRules::new(
        CardKind::Land,
        ManaCost::new(0, 0),
        "Tap: Add 3. Spend this mana only to cast artifact spells.",
    )
    .with_abilities(&MISHRA_S_WORKSHOP_ABILITIES),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "Its produced mana carries the artifact-spell restriction, but payment does not enforce that restriction yet.",
});

pub(in crate::card::sets) static ARGOTHIAN_PIXIES: CardRecord = CardRecord::new(
    cards::ARGOTHIAN_PIXIES,
    "Argothian Pixies",
    CardArt::new("5712e87a-2381-4f5b-a853-6973841f9bf1", "Amy Weber"),
    CardSet::Antiquities,
    false,
    CardBehavior::ArgothianPixies,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 0, 0, 0, 0, 1),
        "Argothian Pixies can't be blocked by artifact creatures. Prevent all damage that would be dealt to Argothian Pixies by artifact creatures.",
    )
    .creature(2, 1),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The artifact-creature blocking restriction works, but damage from artifact creatures is not prevented.",
});

pub(in crate::card::sets) static HURKYLS_RECALL: CardRecord = CardRecord::new(
    cards::HURKYLS_RECALL,
    "Hurkyl's Recall",
    CardArt::new("f32373dd-06d8-45d1-8777-3b1411bcb30a", "NéNé Thomas"),
    CardSet::Antiquities,
    false,
    CardBehavior::HurkylsRecall,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 0, 1, 0, 0, 0),
        "Return all artifacts target player owns to their hand.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The resolver currently returns artifacts the targeted player controls instead of artifacts they own.",
});

pub(in crate::card::sets) static SAGE_OF_LAT_NAM: CardRecord = CardRecord::new(
    cards::SAGE_OF_LAT_NAM,
    "Sage of Lat-Nam",
    CardArt::new("b4ff60ce-073c-46b8-807c-8b40467b960c", "Pete Venters"),
    CardSet::Antiquities,
    false,
    CardBehavior::SageOfLatNam,
    CardRules::new(
        CardKind::ArtifactCreature,
        ManaCost::colored(1, 0, 1, 0, 0, 0),
        "Tap, sacrifice an artifact: Draw a card.",
    )
    .creature(1, 1),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The card is incorrectly typed as an artifact creature.",
});

pub(in crate::card::sets) static TETRAVUS: CardRecord = CardRecord::new(
    cards::TETRAVUS,
    "Tetravus",
    CardArt::new("23eb19f9-2e8f-4bf0-9bf8-868e6da70e2d", "Mark Tedin"),
    CardSet::Antiquities,
    false,
    CardBehavior::Tetravus,
    CardRules::new(
        CardKind::ArtifactCreature,
        ManaCost::new(6, 0),
        "Flying. Tetravus enters with three +1/+1 counters on it.",
    )
    .creature(1, 1)
    .flying(),
)
.with_implementation_status(ImplementationStatus::Partial {
        explanation: "The upkeep abilities that create and absorb Tetravite tokens are not implemented or cataloged.",
    });

pub(in crate::card::sets) static ENERGY_FLUX: CardRecord = CardRecord::new(
    cards::ENERGY_FLUX,
    "Energy Flux",
    CardArt::new("bd1f624b-e8f2-462f-838a-7cb9e8fda988", "Kaja Foglio"),
    CardSet::Antiquities,
    false,
    CardBehavior::EnergyFlux,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(2, 0, 1, 0, 0, 0),
        "At the beginning of each player's upkeep, sacrifice each artifact unless you pay 2 for it.",
    ),
)
.with_implementation_status(ImplementationStatus::Partial {
    explanation: "The per-artifact upkeep triggers and payment choices currently resolve outside the stack.",
});

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ATOG,
    &DETONATE,
    &SU_CHI,
    &MISHRA_S_FACTORY,
    &ORCISH_MECHANICS,
    &STRIP_MINE,
    &TRISKELION,
    &IVORY_TOWER,
    &MISHRA_S_WORKSHOP,
    &ARGOTHIAN_PIXIES,
    &HURKYLS_RECALL,
    &SAGE_OF_LAT_NAM,
    &TETRAVUS,
    &ENERGY_FLUX,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
