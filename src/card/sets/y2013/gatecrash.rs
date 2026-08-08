//! Gatecrash card records used by the built-in ISD–RTR Standard decks.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    CardArt, CardBehavior, CardKind, CardRules, CardSet, LandEntry, ManaCost, cards,
};

// Implementation status: Metadata only; this spell is withheld from legal actions.
pub(in crate::card::sets) static ASSEMBLE_THE_LEGION: CardRecord = CardRecord::new(
    cards::ASSEMBLE_THE_LEGION,
    "Assemble the Legion",
    CardArt::new("43675ed7-ece1-4414-965e-9ebadcbf3dfb", "Eric Deschamps"),
    CardSet::Gatecrash,
    false,
    CardBehavior::AssembleTheLegion,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(3, 1, 0, 0, 1, 0),
        "At the beginning of your upkeep, put a muster counter on this enchantment. Then create a 1/1 red and white Soldier creature token with haste for each muster counter on this enchantment.",
    )
    .type_line("Enchantment")
    .metadata_only(),
);

// Implementation status: Metadata only; this spell is withheld from legal actions.
pub(in crate::card::sets) static AURELIAS_FURY: CardRecord = CardRecord::new(
    cards::AURELIAS_FURY,
    "Aurelia's Fury",
    CardArt::new("1a3465b6-ee7f-4553-bbf1-85fae9734b67", "Tyler Jacobson"),
    CardSet::Gatecrash,
    false,
    CardBehavior::AureliasFury,
    CardRules::new(
        CardKind::Instant,
        ManaCost::variable(0, 1, 0, 0, 1, 0, 1),
        "Aurelia's Fury deals X damage divided as you choose among any number of targets. Tap each creature dealt damage this way. Players dealt damage this way can't cast noncreature spells this turn.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Baseline creature casting/combat and declaratively modeled traits are active; remaining printed abilities are pending.
pub(in crate::card::sets) static AURELIA_THE_WARLEADER: CardRecord = CardRecord::new(
    cards::AURELIA_THE_WARLEADER,
    "Aurelia, the Warleader",
    CardArt::new("4ec18e35-05e4-4bfc-b32b-c3e71c95a71d", "Slawomir Maniak"),
    CardSet::Gatecrash,
    false,
    CardBehavior::AureliaTheWarleader,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(2, 2, 0, 0, 2, 0),
        "Flying, vigilance, haste\nWhenever Aurelia attacks for the first time each turn, untap all creatures you control. After this phase, there is an additional combat phase.",
    )
    .type_line("Legendary Creature — Angel")
    .creature(3, 4)
    .legendary()
    .flying()
    .vigilance()
    .haste()
    .metadata_only(),
);

// Implementation status: Metadata only; this spell is withheld from legal actions.
pub(in crate::card::sets) static BLIND_OBEDIENCE: CardRecord = CardRecord::new(
    cards::BLIND_OBEDIENCE,
    "Blind Obedience",
    CardArt::new("07c3e78d-d917-4552-842f-feff99c059e0", "Seb McKinnon"),
    CardSet::Gatecrash,
    false,
    CardBehavior::BlindObedience,
    CardRules::new(
        CardKind::Enchantment,
        ManaCost::colored(1, 1, 0, 0, 0, 0),
        "Extort (Whenever you cast a spell, you may pay {W/B}. If you do, each opponent loses 1 life and you gain that much life.)\nArtifacts and creatures your opponents control enter tapped.",
    )
    .type_line("Enchantment")
    .metadata_only(),
);

// Implementation status: Metadata only; this spell is withheld from legal actions.
pub(in crate::card::sets) static BOROS_CHARM: CardRecord = CardRecord::new(
    cards::BOROS_CHARM,
    "Boros Charm",
    CardArt::new("d4ddf9cc-40a7-4b4f-bb51-b08171453c9a", "Zoltan Boros"),
    CardSet::Gatecrash,
    false,
    CardBehavior::BorosCharm,
    CardRules::new(
        CardKind::Instant,
        ManaCost::colored(0, 1, 0, 0, 1, 0),
        "Choose one —\n• Boros Charm deals 4 damage to target player or planeswalker.\n• Permanents you control gain indestructible until end of turn.\n• Target creature gains double strike until end of turn.",
    )
    .type_line("Instant")
    .metadata_only(),
);

// Implementation status: Baseline creature casting/combat and declaratively modeled traits are active; remaining printed abilities are pending.
pub(in crate::card::sets) static BOROS_RECKONER: CardRecord = CardRecord::new(
    cards::BOROS_RECKONER,
    "Boros Reckoner",
    CardArt::new("82a18b07-38b8-4854-9735-3cfe83b11bf1", "Howard Lyon"),
    CardSet::Gatecrash,
    false,
    CardBehavior::BorosReckoner,
    CardRules::new(
        CardKind::Creature,
        ManaCost::white_red_hybrid(3),
        "Whenever this creature is dealt damage, it deals that much damage to any target.\n{R/W}: This creature gains first strike until end of turn.",
    )
    .type_line("Creature — Minotaur Wizard")
    .creature(3, 3)
    .metadata_only(),
);

// Implementation status: Metadata only; this spell is withheld from legal actions.
pub(in crate::card::sets) static DOMRI_RADE: CardRecord = CardRecord::new(
    cards::DOMRI_RADE,
    "Domri Rade",
    CardArt::new("21b48170-99dd-440f-9954-fc229d6094d3", "Tyler Jacobson"),
    CardSet::Gatecrash,
    false,
    CardBehavior::DomriRade,
    CardRules::new(
        CardKind::Planeswalker,
        ManaCost::colored(1, 0, 0, 0, 1, 1),
        "+1: Look at the top card of your library. If it's a creature card, you may reveal it and put it into your hand.\n−2: Target creature you control fights another target creature.\n−7: You get an emblem with \"Creatures you control have double strike, trample, hexproof, and haste.\"",
    )
    .type_line("Legendary Planeswalker — Domri")
    .planeswalker(3)
    .legendary()
    .metadata_only(),
);

// Implementation status: Baseline creature casting/combat and declaratively modeled traits are active; remaining printed abilities are pending.
pub(in crate::card::sets) static GHOR_CLAN_RAMPAGER: CardRecord = CardRecord::new(
    cards::GHOR_CLAN_RAMPAGER,
    "Ghor-Clan Rampager",
    CardArt::new("382048ec-0bf5-49a5-90d5-f80fbda08962", "Charles Urbach"),
    CardSet::Gatecrash,
    false,
    CardBehavior::GhorClanRampager,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(2, 0, 0, 0, 1, 1),
        "Trample\nBloodrush — {R}{G}, Discard this card: Target attacking creature gets +4/+4 and gains trample until end of turn.",
    )
    .type_line("Creature — Beast")
    .creature(4, 4)
    .trample()
    .metadata_only(),
);

// Implementation status: Mana production is active; the land currently takes the tapped/no-life branch, and the choice is pending.
pub(in crate::card::sets) static GODLESS_SHRINE: CardRecord = CardRecord::new(
    cards::GODLESS_SHRINE,
    "Godless Shrine",
    CardArt::new("6fd672bb-18cf-44e3-8dda-5310b1e0fffe", "Cliff Childs"),
    CardSet::Gatecrash,
    false,
    CardBehavior::GodlessShrine,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "({T}: Add {W} or {B}.)\nAs this land enters, you may pay 2 life. If you don't, it enters tapped.",
    )
    .type_line("Land — Plains Swamp")
    .produces([true, false, true, false, false, false])
    .land_types([true, false, true, false, false])
    .land_entry(LandEntry::PayLifeOrTapped(2))
    .metadata_only(),
);

// Implementation status: Baseline creature casting/combat and declaratively modeled traits are active; remaining printed abilities are pending.
pub(in crate::card::sets) static OBZEDAT_GHOST_COUNCIL: CardRecord = CardRecord::new(
    cards::OBZEDAT_GHOST_COUNCIL,
    "Obzedat, Ghost Council",
    CardArt::new("4cc198d8-1f27-482d-8f5d-21e02c59797a", "Svetlin Velinov"),
    CardSet::Gatecrash,
    false,
    CardBehavior::ObzedatGhostCouncil,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(1, 2, 0, 2, 0, 0),
        "When Obzedat enters, target opponent loses 2 life and you gain 2 life.\nAt the beginning of your end step, you may exile Obzedat. If you do, return it to the battlefield under its owner's control at the beginning of your next upkeep. It gains haste.",
    )
    .type_line("Legendary Creature — Spirit Advisor")
    .creature(5, 5)
    .legendary()
    .metadata_only(),
);

// Implementation status: Mana production is active; the land currently takes the tapped/no-life branch, and the choice is pending.
pub(in crate::card::sets) static SACRED_FOUNDRY: CardRecord = CardRecord::new(
    cards::SACRED_FOUNDRY,
    "Sacred Foundry",
    CardArt::new("0a26d900-c652-4f9c-8681-a35c5f8b1937", "Sam Burley"),
    CardSet::Gatecrash,
    false,
    CardBehavior::SacredFoundry,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "({T}: Add {R} or {W}.)\nAs this land enters, you may pay 2 life. If you don't, it enters tapped.",
    )
    .type_line("Land — Mountain Plains")
    .produces([true, false, false, true, false, false])
    .land_types([true, false, false, true, false])
    .land_entry(LandEntry::PayLifeOrTapped(2))
    .metadata_only(),
);

// Implementation status: Baseline creature casting/combat and declaratively modeled traits are active; remaining printed abilities are pending.
pub(in crate::card::sets) static SEPULCHRAL_PRIMORDIAL: CardRecord = CardRecord::new(
    cards::SEPULCHRAL_PRIMORDIAL,
    "Sepulchral Primordial",
    CardArt::new("eb0865cd-d9b4-43ea-87d2-ad5c65fc0459", "Stephan Martiniere"),
    CardSet::Gatecrash,
    false,
    CardBehavior::SepulchralPrimordial,
    CardRules::new(
        CardKind::Creature,
        ManaCost::colored(5, 0, 0, 2, 0, 0),
        "Intimidate (This creature can't be blocked except by artifact creatures and/or creatures that share a color with it.)\nWhen this creature enters, for each opponent, you may put up to one target creature card from that player's graveyard onto the battlefield under your control.",
    )
    .type_line("Creature — Avatar")
    .creature(5, 4)
    .intimidate()
    .metadata_only(),
);

// Implementation status: Mana production is active; the land currently takes the tapped/no-life branch, and the choice is pending.
pub(in crate::card::sets) static STOMPING_GROUND: CardRecord = CardRecord::new(
    cards::STOMPING_GROUND,
    "Stomping Ground",
    CardArt::new("f29f3415-971c-4a5d-aae9-3893f4bdab1e", "David Palumbo"),
    CardSet::Gatecrash,
    false,
    CardBehavior::StompingGround,
    CardRules::new(
        CardKind::Land,
        ManaCost::colored(0, 0, 0, 0, 0, 0),
        "({T}: Add {R} or {G}.)\nAs this land enters, you may pay 2 life. If you don't, it enters tapped.",
    )
    .type_line("Land — Mountain Forest")
    .produces([false, false, false, true, true, false])
    .land_types([false, false, false, true, true])
    .land_entry(LandEntry::PayLifeOrTapped(2))
    .metadata_only(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ASSEMBLE_THE_LEGION,
    &AURELIAS_FURY,
    &AURELIA_THE_WARLEADER,
    &BLIND_OBEDIENCE,
    &BOROS_CHARM,
    &BOROS_RECKONER,
    &DOMRI_RADE,
    &GHOR_CLAN_RAMPAGER,
    &GODLESS_SHRINE,
    &OBZEDAT_GHOST_COUNCIL,
    &SACRED_FOUNDRY,
    &SEPULCHRAL_PRIMORDIAL,
    &STOMPING_GROUND,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
