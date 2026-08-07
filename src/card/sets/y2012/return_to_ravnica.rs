//! Return to Ravnica card records used by the built-in ISD–RTR Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    CardBehavior, CardComposition, CardEffectStatus, CardKind, CardPart, CardRules, CardSet,
    CardStructure, LandEntry, ManaCost, ModeDef, ModeSetDef, PlayOptionDef, SpellForm,
    TargetPredicate, TargetSlotDef, cards,
};
use crate::ids::{CardPartId, ModeId, PlayOptionId, TargetSlotId};

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static ABRUPT_DECAY: CardRecord = CardRecord::new_with_art(
    cards::ABRUPT_DECAY,
    "Abrupt Decay",
    "3b1e92b4-6e53-4dba-a572-c67e01965ac5",
    "Svetlin Velinov",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::AbruptDecay,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 0, 1, 0, 1),
        "This spell can't be countered.\nDestroy target nonland permanent with mana value 3 or less.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static ANGEL_OF_SERENITY: CardRecord = CardRecord::new_with_art(
    cards::ANGEL_OF_SERENITY,
    "Angel of Serenity",
    "f10d82f7-7759-457e-a9bb-f9a5bd968f82",
    "Aleksi Briclot",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::AngelOfSerenity,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(4, 3, 0, 0, 0, 0),
        "Flying\nWhen this creature enters, you may exile up to three other target creatures from the battlefield and/or creature cards from graveyards.\nWhen this creature leaves the battlefield, return the exiled cards to their owners' hands.",
    )
    .type_line("Creature — Angel")
    .creature(5, 6)
    .flying()
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static AZORIUS_CHARM: CardRecord = CardRecord::new_with_art(
    cards::AZORIUS_CHARM,
    "Azorius Charm",
    "26adc211-d089-4102-91e5-225bbeb5f382",
    "Zoltan Boros",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::AzoriusCharm,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 1, 1, 0, 0, 0),
        "Choose one —\n• Creatures you control gain lifelink until end of turn.\n• Draw a card.\n• Put target attacking or blocking creature on top of its owner's library.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static COUNTERFLUX: CardRecord = CardRecord::new_with_art(
    cards::COUNTERFLUX,
    "Counterflux",
    "94e4b773-40a4-4272-85dd-f728ada22748",
    "Scott M. Fischer",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::Counterflux,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 2, 0, 1, 0),
        "This spell can't be countered.\nCounter target spell you don't control.\nOverload {1}{U}{U}{R} (You may cast this spell for its overload cost. If you do, change \"target\" in its text to \"each.\")",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static DESECRATION_DEMON: CardRecord = CardRecord::new_with_art(
    cards::DESECRATION_DEMON,
    "Desecration Demon",
    "8242fade-754c-4404-b3fb-f3cccf84b3b6",
    "Jason Chan",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::DesecrationDemon,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(2, 0, 0, 2, 0, 0),
        "Flying\nAt the beginning of each combat, any opponent may sacrifice a creature of their choice. If a player does, tap this creature and put a +1/+1 counter on it.",
    )
    .type_line("Creature — Demon")
    .creature(6, 6)
    .flying()
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static DETENTION_SPHERE: CardRecord = CardRecord::new_with_art(
    cards::DETENTION_SPHERE,
    "Detention Sphere",
    "afee5464-83b7-4d7a-b407-9ee7de21535b",
    "Kev Walker",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::DetentionSphere,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(1, 1, 1, 0, 0, 0),
        "When this enchantment enters, you may exile target nonland permanent not named Detention Sphere and all other permanents with the same name as that permanent.\nWhen this enchantment leaves the battlefield, return the exiled cards to the battlefield under their owner's control.",
    )
    .type_line("Enchantment")
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static DISPEL: CardRecord = CardRecord::new_with_art(
    cards::DISPEL,
    "Dispel",
    "08d4a8d7-c136-472f-8146-a1100701ca4f",
    "Chase Stone",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::Dispel,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 1, 0, 0, 0),
        "Counter target instant spell.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Land entry and mana production are active.
pub(in crate::card::sets) static GOLGARI_GUILDGATE: CardRecord = CardRecord::new_with_art(
    cards::GOLGARI_GUILDGATE,
    "Golgari Guildgate",
    "8fe2fd1a-f7d3-48b4-bad8-be5ee45d6121",
    "Eytan Zana",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::GolgariGuildgate,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "This land enters tapped.\n{T}: Add {B} or {G}.",
    )
    .type_line("Land — Gate")
    .produces([false, false, true, false, true, false])
    .land_entry(LandEntry::Tapped)
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static GRISLY_SALVAGE: CardRecord = CardRecord::new_with_art(
    cards::GRISLY_SALVAGE,
    "Grisly Salvage",
    "dcb5eb2a-ae7a-4416-970c-6e9306689c88",
    "Dave Kendall",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::GrislySalvage,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 0, 1, 0, 1),
        "Reveal the top five cards of your library. You may put a creature or land card from among them into your hand. Put the rest into your graveyard.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Land types and mana production are active; entry currently takes the tapped/no-life branch.
pub(in crate::card::sets) static HALLOWED_FOUNTAIN: CardRecord = CardRecord::new_with_art(
    cards::HALLOWED_FOUNTAIN,
    "Hallowed Fountain",
    "af7091c9-5f98-4078-a42b-c9e057346d9b",
    "Jung Park",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::HallowedFountain,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "({T}: Add {W} or {U}.)\nAs this land enters, you may pay 2 life. If you don't, it enters tapped.",
    )
    .type_line("Land — Plains Island")
    .produces([true, true, false, false, false, false])
    .land_types([true, true, false, false, false])
    .land_entry(LandEntry::PayLifeOrTapped(2))
    .metadata_only(),
);

const fn izzet_charm_rules() -> CardRules {
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 0, 1, 0, 1, 0),
        "Choose one —\n• Counter target noncreature spell unless its controller pays {2}.\n• Izzet Charm deals 2 damage to target creature.\n• Draw two cards, then discard two cards.",
    )
    .type_line("Instant")
    .metadata_only()
}

fn izzet_charm_composition() -> CardComposition {
    let rules = izzet_charm_rules();
    let modes = ModeSetDef::choose_one(vec![
        ModeDef {
            id: ModeId(0),
            label: "Counter a noncreature spell unless its controller pays {2}".into(),
            targets: vec![TargetSlotDef::exactly_one(
                TargetSlotId(0),
                "noncreature spell",
                TargetPredicate::NoncreatureSpell,
            )],
            effect_status: CardEffectStatus::MetadataOnly,
        },
        ModeDef {
            id: ModeId(1),
            label: "Deal 2 damage to a creature".into(),
            targets: vec![TargetSlotDef::exactly_one(
                TargetSlotId(1),
                "creature",
                TargetPredicate::CreaturePermanent,
            )],
            effect_status: CardEffectStatus::MetadataOnly,
        },
        ModeDef {
            id: ModeId(2),
            label: "Draw two cards, then discard two cards".into(),
            targets: Vec::new(),
            effect_status: CardEffectStatus::MetadataOnly,
        },
    ]);
    CardComposition {
        parts: vec![CardPart::new(CardPartId::PRIMARY, "Izzet Charm", rules)],
        structure: CardStructure::Single {
            main: CardPartId::PRIMARY,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Izzet Charm",
                SpellForm::Part(CardPartId::PRIMARY),
                rules.mana_cost,
                CardEffectStatus::MetadataOnly,
            )
            .with_modes(modes),
        ],
    }
}

// Implementation status: Spell form, three modes, and their target slots are cataloged; effect execution is pending.
pub(in crate::card::sets) static IZZET_CHARM: CardRecord = CardRecord::new_with_art(
    cards::IZZET_CHARM,
    "Izzet Charm",
    "1e3a5af6-5423-442b-a207-364e97a871d8",
    "Zoltan Boros",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::IzzetCharm,
    izzet_charm_rules(),
)
.with_composition(izzet_charm_composition);

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static IZZET_STATICASTER: CardRecord = CardRecord::new_with_art(
    cards::IZZET_STATICASTER,
    "Izzet Staticaster",
    "190ac2fe-532d-4d7e-9d74-07ae6850aac8",
    "Scott M. Fischer",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::IzzetStaticaster,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 0, 1, 0, 1, 0),
        "Flash (You may cast this spell any time you could cast an instant.)\nHaste\n{T}: This creature deals 1 damage to target creature and each other creature with the same name as that creature.",
    )
    .type_line("Creature — Human Wizard")
    .creature(0, 3)
    .flash()
    .haste()
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static JACE_ARCHITECT_OF_THOUGHT: CardRecord = CardRecord::new_with_art(
    cards::JACE_ARCHITECT_OF_THOUGHT,
    "Jace, Architect of Thought",
    "d4df3a38-678e-42dc-a3fd-d1d399368f07",
    "Jaime Jones",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::JaceArchitectOfThought,
    CardRules::new(
        CardKind::Planeswalker,
        ManaCost::colored(2, 0, 2, 0, 0, 0),
        "+1: Until your next turn, whenever a creature an opponent controls attacks, it gets -1/-0 until end of turn.\n−2: Reveal the top three cards of your library. An opponent separates those cards into two piles. Put one pile into your hand and the other on the bottom of your library in any order.\n−8: For each player, search that player's library for a nonland card and exile it, then that player shuffles. You may cast those cards without paying their mana costs.",
    )
    .type_line("Legendary Planeswalker — Jace")
    .planeswalker(4)
    .legendary()
    .metadata_only(),
);

// Implementation status: Baseline creature is playable; card-specific printed abilities are pending.
pub(in crate::card::sets) static LOXODON_SMITER: CardRecord = CardRecord::new_with_art(
    cards::LOXODON_SMITER,
    "Loxodon Smiter",
    "69247168-2bfb-4cce-a2a6-61459a0fbce4",
    "Ryan Barger",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::LoxodonSmiter,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 1, 0, 0, 0, 1),
        "This spell can't be countered.\nIf a spell or ability an opponent controls causes you to discard this card, put it onto the battlefield instead of putting it into your graveyard.",
    )
    .type_line("Creature — Elephant Soldier")
    .creature(4, 4)
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static MIZZIUM_MORTARS: CardRecord = CardRecord::new_with_art(
    cards::MIZZIUM_MORTARS,
    "Mizzium Mortars",
    "d4ded88d-2688-4f5e-a8b2-16216cf9c792",
    "Noah Bradley",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::MizziumMortars,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(1, 0, 0, 0, 1, 0),
        "Mizzium Mortars deals 4 damage to target creature you don't control.\nOverload {3}{R}{R}{R} (You may cast this spell for its overload cost. If you do, change \"target\" in its text to \"each.\")",
    )
    .type_line("Sorcery")
    .metadata_only(),
);

// Implementation status: Land types and mana production are active; entry currently takes the tapped/no-life branch.
pub(in crate::card::sets) static OVERGROWN_TOMB: CardRecord = CardRecord::new_with_art(
    cards::OVERGROWN_TOMB,
    "Overgrown Tomb",
    "1c7d50d6-b63a-4d8c-88fa-1d78ae693a45",
    "Steven Belledin",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::OvergrownTomb,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "({T}: Add {B} or {G}.)\nAs this land enters, you may pay 2 life. If you don't, it enters tapped.",
    )
    .type_line("Land — Swamp Forest")
    .produces([false, false, true, false, true, false])
    .land_types([false, false, true, false, true])
    .land_entry(LandEntry::PayLifeOrTapped(2))
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static PITHING_NEEDLE: CardRecord = CardRecord::new_with_art(
    cards::PITHING_NEEDLE,
    "Pithing Needle",
    "786c1e91-9d75-46a3-9e0d-56d29fcb01a7",
    "Anthony Palumbo",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::PithingNeedle,
    CardRules::new(
        CardKind::Artifact,
        ManaCost::colored(1, 0, 0, 0, 0, 0),
        "As this artifact enters, choose a card name.\nActivated abilities of sources with the chosen name can't be activated unless they're mana abilities.",
    )
    .type_line("Artifact")
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static REST_IN_PEACE: CardRecord = CardRecord::new_with_art(
    cards::REST_IN_PEACE,
    "Rest in Peace",
    "37c2b1d1-faa0-40fd-82f4-216604ce7635",
    "Terese Nielsen",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::RestInPeace,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(1, 1, 0, 0, 0, 0),
        "When this enchantment enters, exile all graveyards.\nIf a card or token would be put into a graveyard from anywhere, exile it instead.",
    )
    .type_line("Enchantment")
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static SELESNYA_CHARM: CardRecord = CardRecord::new_with_art(
    cards::SELESNYA_CHARM,
    "Selesnya Charm",
    "a9848eab-1d3a-4ab0-adf6-c20858aa3afb",
    "Zoltan Boros",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::SelesnyaCharm,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 1, 0, 0, 0, 1),
        "Choose one —\n• Target creature gets +2/+2 and gains trample until end of turn.\n• Exile target creature with power 5 or greater.\n• Create a 2/2 white Knight creature token with vigilance.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static SPHINXS_REVELATION: CardRecord = CardRecord::new_with_art(
    cards::SPHINXS_REVELATION,
    "Sphinx's Revelation",
    "404d9413-ef57-4b6e-8584-48a1dc7fe6f1",
    "Slawomir Maniak",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::SphinxsRevelation,
    CardRules::new(
        CardKind::Instant,
        ManaCost::variable(0, 1, 2, 0, 0, 0, 1),
        "You gain X life and draw X cards.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Land types and mana production are active; entry currently takes the tapped/no-life branch.
pub(in crate::card::sets) static STEAM_VENTS: CardRecord = CardRecord::new_with_art(
    cards::STEAM_VENTS,
    "Steam Vents",
    "de911c88-f5c8-4955-9fa5-1f28a9b17236",
    "Yeong-Hao Han",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::SteamVents,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "({T}: Add {U} or {R}.)\nAs this land enters, you may pay 2 life. If you don't, it enters tapped.",
    )
    .type_line("Land — Island Mountain")
    .produces([false, true, false, true, false, false])
    .land_types([false, true, false, true, false])
    .land_entry(LandEntry::PayLifeOrTapped(2))
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static SUPREME_VERDICT: CardRecord = CardRecord::new_with_art(
    cards::SUPREME_VERDICT,
    "Supreme Verdict",
    "4e9648f9-7a67-4717-bca1-861d1f7fed43",
    "Sam Burley",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::SupremeVerdict,
    CardRules::new(
        CardKind::Sorcery,
        ManaCost::colored(1, 2, 1, 0, 0, 0),
        "This spell can't be countered.\nDestroy all creatures.",
    )
    .type_line("Sorcery")
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static SYNCOPATE: CardRecord = CardRecord::new_with_art(
    cards::SYNCOPATE,
    "Syncopate",
    "ba6f218f-83b0-4b68-a00f-0327cd79f32a",
    "Clint Cearley",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::Syncopate,
    CardRules::new(
        CardKind::Instant,
        ManaCost::variable(0, 0, 1, 0, 0, 0, 1),
        "Counter target spell unless its controller pays {X}. If that spell is countered this way, exile it instead of putting it into its owner's graveyard.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Land types and mana production are active; entry currently takes the tapped/no-life branch.
pub(in crate::card::sets) static TEMPLE_GARDEN: CardRecord = CardRecord::new_with_art(
    cards::TEMPLE_GARDEN,
    "Temple Garden",
    "b821e604-f9fd-47a4-b5ff-bfb5022834c2",
    "Volkan Baǵa",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::TempleGarden,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "({T}: Add {G} or {W}.)\nAs this land enters, you may pay 2 life. If you don't, it enters tapped.",
    )
    .type_line("Land — Forest Plains")
    .produces([true, false, false, false, true, false])
    .land_types([true, false, false, false, true])
    .land_entry(LandEntry::PayLifeOrTapped(2))
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static ULTIMATE_PRICE: CardRecord = CardRecord::new_with_art(
    cards::ULTIMATE_PRICE,
    "Ultimate Price",
    "d2b4912a-83a2-4870-8fac-81fa79da2830",
    "Karl Kopinski",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::UltimatePrice,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(1, 0, 0, 1, 0, 0),
        "Destroy target monocolored creature.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static UNDERWORLD_CONNECTIONS: CardRecord = CardRecord::new_with_art(
    cards::UNDERWORLD_CONNECTIONS,
    "Underworld Connections",
    "19c52e3b-b3b8-4243-96fe-fa4c8eea7c59",
    "Yeong-Hao Han",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::UnderworldConnections,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(1, 0, 0, 2, 0, 0),
        "Enchant land\nEnchanted land has \"{T}, Pay 1 life: Draw a card.\"",
    )
    .type_line("Enchantment — Aura")
    .metadata_only(),
);

// Implementation status: Spell is withheld from play; printed effects are pending.
pub(in crate::card::sets) static VRASKA_THE_UNSEEN: CardRecord = CardRecord::new_with_art(
    cards::VRASKA_THE_UNSEEN,
    "Vraska the Unseen",
    "8971938c-cd26-4b83-96d7-1408cd0b0de6",
    "Aleksi Briclot",
    CardSet::ReturnToRavnica,
    false,
    CardBehavior::VraskaTheUnseen,
    CardRules::new(
        CardKind::Planeswalker,
        ManaCost::colored(3, 0, 0, 1, 0, 1),
        "+1: Until your next turn, whenever a creature deals combat damage to Vraska, destroy that creature.\n−3: Destroy target nonland permanent.\n−7: Create three 1/1 black Assassin creature tokens with \"Whenever this token deals combat damage to a player, that player loses the game.\"",
    )
    .type_line("Legendary Planeswalker — Vraska")
    .planeswalker(5)
    .legendary()
    .metadata_only(),
);
pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ABRUPT_DECAY,
    &ANGEL_OF_SERENITY,
    &AZORIUS_CHARM,
    &COUNTERFLUX,
    &DESECRATION_DEMON,
    &DETENTION_SPHERE,
    &DISPEL,
    &GOLGARI_GUILDGATE,
    &GRISLY_SALVAGE,
    &HALLOWED_FOUNTAIN,
    &IZZET_CHARM,
    &IZZET_STATICASTER,
    &JACE_ARCHITECT_OF_THOUGHT,
    &LOXODON_SMITER,
    &MIZZIUM_MORTARS,
    &OVERGROWN_TOMB,
    &PITHING_NEEDLE,
    &REST_IN_PEACE,
    &SELESNYA_CHARM,
    &SPHINXS_REVELATION,
    &STEAM_VENTS,
    &SUPREME_VERDICT,
    &SYNCOPATE,
    &TEMPLE_GARDEN,
    &ULTIMATE_PRICE,
    &UNDERWORLD_CONNECTIONS,
    &VRASKA_THE_UNSEEN,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
