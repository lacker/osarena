//! Built-in card records, grouped by release year and set.
//!
//! Each canonical card is defined in one set module and has an adjacent
//! `Implementation status` comment describing which rules the engine executes.
//! Reprints and alternate-art variants point back to that canonical record from
//! their own set module.

mod y1993;
mod y1994;
mod y2011;
mod y2012;
mod y2013;

use super::record::{CardRecord, PrintingRecord};
use crate::card::{CardBehavior, CardDefinition, CardPrinting, CardRules, CardSet};

static UNSUPPORTED_RULES: CardRules = CardRules::unsupported();

struct SetModule {
    set: CardSet,
    cards: &'static [&'static CardRecord],
    additional_printings: &'static [PrintingRecord],
}

impl SetModule {
    const fn new(
        set: CardSet,
        cards: &'static [&'static CardRecord],
        additional_printings: &'static [PrintingRecord],
    ) -> Self {
        Self {
            set,
            cards,
            additional_printings,
        }
    }
}

/// Every supported set has one source module. `cards` contains definitions
/// introduced by that module; `additional_printings` contains reprints and
/// further variants of definitions introduced elsewhere.
const SET_MODULES: &[SetModule] = &[
    SetModule::new(
        CardSet::Alpha,
        y1993::alpha::CARDS,
        y1993::alpha::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Beta,
        y1993::beta::CARDS,
        y1993::beta::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Unlimited,
        y1993::unlimited::CARDS,
        y1993::unlimited::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::CollectorsEdition,
        y1993::collectors_edition::CARDS,
        y1993::collectors_edition::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::InternationalCollectorsEdition,
        y1993::international_collectors_edition::CARDS,
        y1993::international_collectors_edition::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ArabianNights,
        y1993::arabian_nights::CARDS,
        y1993::arabian_nights::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Antiquities,
        y1994::antiquities::CARDS,
        y1994::antiquities::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Revised,
        y1994::revised::CARDS,
        y1994::revised::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Legends,
        y1994::legends::CARDS,
        y1994::legends::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::TheDark,
        y1994::the_dark::CARDS,
        y1994::the_dark::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::FallenEmpires,
        y1994::fallen_empires::CARDS,
        y1994::fallen_empires::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Promo1994,
        y1994::promo_1994::CARDS,
        y1994::promo_1994::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Innistrad,
        y2011::innistrad::CARDS,
        y2011::innistrad::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::DarkAscension,
        y2012::dark_ascension::CARDS,
        y2012::dark_ascension::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::AvacynRestored,
        y2012::avacyn_restored::CARDS,
        y2012::avacyn_restored::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Magic2013,
        y2012::magic_2013::CARDS,
        y2012::magic_2013::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ReturnToRavnica,
        y2012::return_to_ravnica::CARDS,
        y2012::return_to_ravnica::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Gatecrash,
        y2013::gatecrash::CARDS,
        y2013::gatecrash::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::DragonsMaze,
        y2013::dragons_maze::CARDS,
        y2013::dragons_maze::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Magic2014,
        y2013::magic_2014::CARDS,
        y2013::magic_2014::ADDITIONAL_PRINTINGS,
    ),
];

pub(super) fn definitions() -> Vec<CardDefinition> {
    let mut definitions = Vec::with_capacity(244);
    for module in SET_MODULES {
        definitions.extend(module.cards.iter().map(|record| record.definition()));
    }
    definitions
}

pub(super) fn additional_printings() -> Vec<CardPrinting> {
    SET_MODULES
        .iter()
        .flat_map(|module| {
            module
                .additional_printings
                .iter()
                .map(|record| record.printing(module.set))
        })
        .collect()
}

// This is deliberately only an index: every rules value lives with its card.
#[allow(clippy::too_many_lines)]
pub(super) const fn rules(behavior: CardBehavior) -> &'static CardRules {
    match behavior {
        CardBehavior::AnkhOfMishra => &y1993::alpha::ANKH_OF_MISHRA.rules,
        CardBehavior::Atog => &y1994::antiquities::ATOG.rules,
        CardBehavior::BallLightning => &y1994::the_dark::BALL_LIGHTNING.rules,
        CardBehavior::BlackVise => &y1993::alpha::BLACK_VISE.rules,
        CardBehavior::BloodMoon => &y1994::the_dark::BLOOD_MOON.rules,
        CardBehavior::ChainLightning => &y1994::legends::CHAIN_LIGHTNING.rules,
        CardBehavior::CopperTablet => &y1993::alpha::COPPER_TABLET.rules,
        CardBehavior::Detonate => &y1994::antiquities::DETONATE.rules,
        CardBehavior::Fireball => &y1993::alpha::FIREBALL.rules,
        CardBehavior::Fork => &y1993::alpha::FORK.rules,
        CardBehavior::GlassesOfUrza => &y1993::alpha::GLASSES_OF_URZA.rules,
        CardBehavior::IronStar => &y1993::alpha::IRON_STAR.rules,
        CardBehavior::LightningBolt => &y1993::alpha::LIGHTNING_BOLT.rules,
        CardBehavior::Mountain => &y1993::alpha::MOUNTAIN.rules,
        CardBehavior::RedElementalBlast => &y1993::alpha::RED_ELEMENTAL_BLAST.rules,
        CardBehavior::Shatter => &y1993::alpha::SHATTER.rules,
        CardBehavior::Smoke => &y1993::alpha::SMOKE.rules,
        CardBehavior::StoneGiant => &y1993::alpha::STONE_GIANT.rules,
        CardBehavior::SuChi => &y1994::antiquities::SU_CHI.rules,
        CardBehavior::WinterOrb => &y1993::alpha::WINTER_ORB.rules,
        CardBehavior::BlackLotus => &y1993::alpha::BLACK_LOTUS.rules,
        CardBehavior::ChaosOrb => &y1993::alpha::CHAOS_ORB.rules,
        CardBehavior::DragonWhelp => &y1993::alpha::DRAGON_WHELP.rules,
        CardBehavior::GoblinBalloonBrigade => &y1993::alpha::GOBLIN_BALLOON_BRIGADE.rules,
        CardBehavior::GoblinDiggingTeam => &y1994::the_dark::GOBLIN_DIGGING_TEAM.rules,
        CardBehavior::GoblinGrenade => &y1994::fallen_empires::GOBLIN_GRENADE.rules,
        CardBehavior::GoblinKing => &y1993::alpha::GOBLIN_KING.rules,
        CardBehavior::GoblinsOfTheFlarg => &y1994::the_dark::GOBLINS_OF_THE_FLARG.rules,
        CardBehavior::GraniteGargoyle => &y1993::alpha::GRANITE_GARGOYLE.rules,
        CardBehavior::IronclawOrcs => &y1993::alpha::IRONCLAW_ORCS.rules,
        CardBehavior::MishrasFactory => &y1994::antiquities::MISHRA_S_FACTORY.rules,
        CardBehavior::MoxEmerald => &y1993::alpha::MOX_EMERALD.rules,
        CardBehavior::MoxJet => &y1993::alpha::MOX_JET.rules,
        CardBehavior::MoxPearl => &y1993::alpha::MOX_PEARL.rules,
        CardBehavior::MoxRuby => &y1993::alpha::MOX_RUBY.rules,
        CardBehavior::MoxSapphire => &y1993::alpha::MOX_SAPPHIRE.rules,
        CardBehavior::OrcishMechanics => &y1994::antiquities::ORCISH_MECHANICS.rules,
        CardBehavior::SolRing => &y1993::alpha::SOL_RING.rules,
        CardBehavior::StripMine => &y1994::antiquities::STRIP_MINE.rules,
        CardBehavior::WheelOfFortune => &y1993::alpha::WHEEL_OF_FORTUNE.rules,
        CardBehavior::Juggernaut => &y1993::alpha::JUGGERNAUT.rules,
        CardBehavior::ManaVault => &y1993::alpha::MANA_VAULT.rules,
        CardBehavior::Triskelion => &y1994::antiquities::TRISKELION.rules,
        CardBehavior::AncestralRecall => &y1993::alpha::ANCESTRAL_RECALL.rules,
        CardBehavior::Braingeyser => &y1993::alpha::BRAINGEYSER.rules,
        CardBehavior::Counterspell => &y1993::alpha::COUNTERSPELL.rules,
        CardBehavior::Disenchant => &y1993::alpha::DISENCHANT.rules,
        CardBehavior::FellwarStone => &y1994::the_dark::FELLWAR_STONE.rules,
        CardBehavior::Island => &y1993::alpha::ISLAND.rules,
        CardBehavior::IvoryTower => &y1994::antiquities::IVORY_TOWER.rules,
        CardBehavior::JayemdaeTome => &y1993::alpha::JAYEMDAE_TOME.rules,
        CardBehavior::Plains => &y1993::alpha::PLAINS.rules,
        CardBehavior::SerraAngel => &y1993::alpha::SERRA_ANGEL.rules,
        CardBehavior::SwordsToPlowshares => &y1993::alpha::SWORDS_TO_PLOWSHARES.rules,
        CardBehavior::TimeWalk => &y1993::alpha::TIME_WALK.rules,
        CardBehavior::Tundra => &y1993::alpha::TUNDRA.rules,
        CardBehavior::VolcanicIsland => &y1993::beta::VOLCANIC_ISLAND.rules,
        CardBehavior::Armageddon => &y1993::alpha::ARMAGEDDON.rules,
        CardBehavior::Badlands => &y1993::alpha::BADLANDS.rules,
        CardBehavior::Balance => &y1993::alpha::BALANCE.rules,
        CardBehavior::Bayou => &y1993::alpha::BAYOU.rules,
        CardBehavior::BlackKnight => &y1993::alpha::BLACK_KNIGHT.rules,
        CardBehavior::BirdsOfParadise => &y1993::alpha::BIRDS_OF_PARADISE.rules,
        CardBehavior::BlueElementalBlast => &y1993::alpha::BLUE_ELEMENTAL_BLAST.rules,
        CardBehavior::Channel => &y1993::alpha::CHANNEL.rules,
        CardBehavior::CityOfBrass => &y1993::arabian_nights::CITY_OF_BRASS.rules,
        CardBehavior::Crusade => &y1993::alpha::CRUSADE.rules,
        CardBehavior::DarkRitual => &y1993::alpha::DARK_RITUAL.rules,
        CardBehavior::DemonicTutor => &y1993::alpha::DEMONIC_TUTOR.rules,
        CardBehavior::DivineOffering => &y1994::legends::DIVINE_OFFERING.rules,
        CardBehavior::DrainLife => &y1993::alpha::DRAIN_LIFE.rules,
        CardBehavior::Earthquake => &y1993::alpha::EARTHQUAKE.rules,
        CardBehavior::ErhnamDjinn => &y1993::arabian_nights::ERHNAM_DJINN.rules,
        CardBehavior::Forest => &y1993::alpha::FOREST.rules,
        CardBehavior::HymnToTourach => &y1994::fallen_empires::HYMN_TO_TOURACH.rules,
        CardBehavior::HypnoticSpecter => &y1993::alpha::HYPNOTIC_SPECTER.rules,
        CardBehavior::IcatianJavelineers => &y1994::fallen_empires::ICATIAN_JAVELINEERS.rules,
        CardBehavior::JuzamDjinn => &y1993::arabian_nights::JUZAM_DJINN.rules,
        CardBehavior::LibraryOfAlexandria => &y1993::arabian_nights::LIBRARY_OF_ALEXANDRIA.rules,
        CardBehavior::ManaDrain => &y1994::legends::MANA_DRAIN.rules,
        CardBehavior::MazeOfIth => &y1994::the_dark::MAZE_OF_ITH.rules,
        CardBehavior::MindTwist => &y1993::alpha::MIND_TWIST.rules,
        CardBehavior::MishrasWorkshop => &y1994::antiquities::MISHRA_S_WORKSHOP.rules,
        CardBehavior::NevinyrralsDisk => &y1993::alpha::NEVINYRRALS_DISK.rules,
        CardBehavior::OrderOfLeitbur => &y1994::fallen_empires::ORDER_OF_LEITBUR.rules,
        CardBehavior::OrderOfTheEbonHand => &y1994::fallen_empires::ORDER_OF_THE_EBON_HAND.rules,
        CardBehavior::Plateau => &y1993::alpha::PLATEAU.rules,
        CardBehavior::PsionicBlast => &y1993::alpha::PSIONIC_BLAST.rules,
        CardBehavior::Recall => &y1994::legends::RECALL.rules,
        CardBehavior::Regrowth => &y1993::alpha::REGROWTH.rules,
        CardBehavior::Savannah => &y1993::alpha::SAVANNAH.rules,
        CardBehavior::SavannahLions => &y1993::alpha::SAVANNAH_LIONS.rules,
        CardBehavior::Scrubland => &y1993::alpha::SCRUBLAND.rules,
        CardBehavior::SerendibEfreet => &y1993::arabian_nights::SERENDIB_EFREET.rules,
        CardBehavior::SengirVampire => &y1993::alpha::SENGIR_VAMPIRE.rules,
        CardBehavior::Sinkhole => &y1993::alpha::SINKHOLE.rules,
        CardBehavior::Swamp => &y1993::alpha::SWAMP.rules,
        CardBehavior::SylvanLibrary => &y1994::legends::SYLVAN_LIBRARY.rules,
        CardBehavior::Taiga => &y1993::alpha::TAIGA.rules,
        CardBehavior::Terror => &y1993::alpha::TERROR.rules,
        CardBehavior::ThunderSpirit => &y1994::legends::THUNDER_SPIRIT.rules,
        CardBehavior::TimeVault => &y1993::alpha::TIME_VAULT.rules,
        CardBehavior::Timetwister => &y1993::alpha::TIMETWISTER.rules,
        CardBehavior::TropicalIsland => &y1993::alpha::TROPICAL_ISLAND.rules,
        CardBehavior::UndergroundSea => &y1993::alpha::UNDERGROUND_SEA.rules,
        CardBehavior::WhirlingDervish => &y1994::legends::WHIRLING_DERVISH.rules,
        CardBehavior::WhiteKnight => &y1993::alpha::WHITE_KNIGHT.rules,
        CardBehavior::ArgothianPixies => &y1994::antiquities::ARGOTHIAN_PIXIES.rules,
        CardBehavior::Berserk => &y1993::alpha::BERSERK.rules,
        CardBehavior::CityInABottle => &y1993::arabian_nights::CITY_IN_A_BOTTLE.rules,
        CardBehavior::CopyArtifact => &y1993::alpha::COPY_ARTIFACT.rules,
        CardBehavior::DustToDust => &y1994::the_dark::DUST_TO_DUST.rules,
        CardBehavior::EnergyFlux => &y1994::antiquities::ENERGY_FLUX.rules,
        CardBehavior::GiantGrowth => &y1993::alpha::GIANT_GROWTH.rules,
        CardBehavior::HurkylsRecall => &y1994::antiquities::HURKYLS_RECALL.rules,
        CardBehavior::IcyManipulator => &y1993::alpha::ICY_MANIPULATOR.rules,
        CardBehavior::KirdApe => &y1993::arabian_nights::KIRD_APE.rules,
        CardBehavior::LlanowarElves => &y1993::alpha::LLANOWAR_ELVES.rules,
        CardBehavior::Moat => &y1994::legends::MOAT.rules,
        CardBehavior::Pendelhaven => &y1994::legends::PENDELHAVEN.rules,
        CardBehavior::RelicBarrier => &y1994::legends::RELIC_BARRIER.rules,
        CardBehavior::SageOfLatNam => &y1994::antiquities::SAGE_OF_LAT_NAM.rules,
        CardBehavior::SedgeTroll => &y1993::alpha::SEDGE_TROLL.rules,
        CardBehavior::ScrybSprites => &y1993::alpha::SCRYB_SPRITES.rules,
        CardBehavior::StoneRain => &y1993::alpha::STONE_RAIN.rules,
        CardBehavior::Tetravus => &y1994::antiquities::TETRAVUS.rules,
        CardBehavior::TheAbyss => &y1994::legends::THE_ABYSS.rules,
        CardBehavior::WrathOfGod => &y1993::alpha::WRATH_OF_GOD.rules,
        CardBehavior::AbruptDecay => &y2012::return_to_ravnica::ABRUPT_DECAY.rules,
        CardBehavior::Aetherling => &y2013::dragons_maze::AETHERLING.rules,
        CardBehavior::AngelOfSerenity => &y2012::return_to_ravnica::ANGEL_OF_SERENITY.rules,
        CardBehavior::ArborElf => &y2012::magic_2013::ARBOR_ELF.rules,
        CardBehavior::ArchangelOfThune => &y2013::magic_2014::ARCHANGEL_OF_THUNE.rules,
        CardBehavior::AssembleTheLegion => &y2013::gatecrash::ASSEMBLE_THE_LEGION.rules,
        CardBehavior::AugurOfBolas => &y2012::magic_2013::AUGUR_OF_BOLAS.rules,
        CardBehavior::AureliasFury => &y2013::gatecrash::AURELIAS_FURY.rules,
        CardBehavior::AureliaTheWarleader => &y2013::gatecrash::AURELIA_THE_WARLEADER.rules,
        CardBehavior::AvacynsPilgrim => &y2011::innistrad::AVACYNS_PILGRIM.rules,
        CardBehavior::AzoriusCharm => &y2012::return_to_ravnica::AZORIUS_CHARM.rules,
        CardBehavior::BlasphemousAct => &y2011::innistrad::BLASPHEMOUS_ACT.rules,
        CardBehavior::BlindObedience => &y2013::gatecrash::BLIND_OBEDIENCE.rules,
        CardBehavior::BloodBaronOfVizkopa => &y2013::dragons_maze::BLOOD_BARON_OF_VIZKOPA.rules,
        CardBehavior::BonfireOfTheDamned => &y2012::avacyn_restored::BONFIRE_OF_THE_DAMNED.rules,
        CardBehavior::BorosCharm => &y2013::gatecrash::BOROS_CHARM.rules,
        CardBehavior::BorosReckoner => &y2013::gatecrash::BOROS_RECKONER.rules,
        CardBehavior::BurningEarth => &y2013::magic_2014::BURNING_EARTH.rules,
        CardBehavior::CavernOfSouls => &y2012::avacyn_restored::CAVERN_OF_SOULS.rules,
        CardBehavior::CelestialFlare => &y2013::magic_2014::CELESTIAL_FLARE.rules,
        CardBehavior::ClifftopRetreat => &y2011::innistrad::CLIFFTOP_RETREAT.rules,
        CardBehavior::Counterflux => &y2012::return_to_ravnica::COUNTERFLUX.rules,
        CardBehavior::DemonicRising => &y2012::avacyn_restored::DEMONIC_RISING.rules,
        CardBehavior::DesecrationDemon => &y2012::return_to_ravnica::DESECRATION_DEMON.rules,
        CardBehavior::DetentionSphere => &y2012::return_to_ravnica::DETENTION_SPHERE.rules,
        CardBehavior::DiscipleOfBolas => &y2012::magic_2013::DISCIPLE_OF_BOLAS.rules,
        CardBehavior::Dispel => &y2012::return_to_ravnica::DISPEL.rules,
        CardBehavior::Dissipate => &y2011::innistrad::DISSIPATE.rules,
        CardBehavior::DomriRade => &y2013::gatecrash::DOMRI_RADE.rules,
        CardBehavior::DoomBlade => &y2013::magic_2014::DOOM_BLADE.rules,
        CardBehavior::Duress => &y2012::magic_2013::DURESS.rules,
        CardBehavior::ElvishMystic => &y2013::magic_2014::ELVISH_MYSTIC.rules,
        CardBehavior::EncroachingWastes => &y2013::magic_2014::ENCROACHING_WASTES.rules,
        CardBehavior::EssenceScatter => &y2012::magic_2013::ESSENCE_SCATTER.rules,
        CardBehavior::FlamesOfTheFirebrand => &y2012::magic_2013::FLAMES_OF_THE_FIREBRAND.rules,
        CardBehavior::FlinthoofBoar => &y2012::magic_2013::FLINTHOOF_BOAR.rules,
        CardBehavior::GarrukRelentless => &y2011::innistrad::GARRUK_RELENTLESS.rules,
        CardBehavior::GavonyTownship => &y2011::innistrad::GAVONY_TOWNSHIP.rules,
        CardBehavior::GazeOfGranite => &y2013::dragons_maze::GAZE_OF_GRANITE.rules,
        CardBehavior::GhorClanRampager => &y2013::gatecrash::GHOR_CLAN_RAMPAGER.rules,
        CardBehavior::GhostQuarter => &y2011::innistrad::GHOST_QUARTER.rules,
        CardBehavior::GlacialFortress => &y2012::magic_2013::GLACIAL_FORTRESS.rules,
        CardBehavior::GodlessShrine => &y2013::gatecrash::GODLESS_SHRINE.rules,
        CardBehavior::GolgariGuildgate => &y2012::return_to_ravnica::GOLGARI_GUILDGATE.rules,
        CardBehavior::GrislySalvage => &y2012::return_to_ravnica::GRISLY_SALVAGE.rules,
        CardBehavior::HallowedFountain => &y2012::return_to_ravnica::HALLOWED_FOUNTAIN.rules,
        CardBehavior::Hellrider => &y2012::dark_ascension::HELLRIDER.rules,
        CardBehavior::HuntmasterOfTheFells => &y2012::dark_ascension::HUNTMASTER_OF_THE_FELLS.rules,
        CardBehavior::IsolatedChapel => &y2011::innistrad::ISOLATED_CHAPEL.rules,
        CardBehavior::IzzetCharm => &y2012::return_to_ravnica::IZZET_CHARM.rules,
        CardBehavior::IzzetStaticaster => &y2012::return_to_ravnica::IZZET_STATICASTER.rules,
        CardBehavior::JaceArchitectOfThought => {
            &y2012::return_to_ravnica::JACE_ARCHITECT_OF_THOUGHT.rules
        }
        CardBehavior::JaceMemoryAdept => &y2012::magic_2013::JACE_MEMORY_ADEPT.rules,
        CardBehavior::KessigWolfRun => &y2011::innistrad::KESSIG_WOLF_RUN.rules,
        CardBehavior::LifebaneZombie => &y2013::magic_2014::LIFEBANE_ZOMBIE.rules,
        CardBehavior::LilianaOfTheVeil => &y2011::innistrad::LILIANA_OF_THE_VEIL.rules,
        CardBehavior::LoxodonSmiter => &y2012::return_to_ravnica::LOXODON_SMITER.rules,
        CardBehavior::MizziumMortars => &y2012::return_to_ravnica::MIZZIUM_MORTARS.rules,
        CardBehavior::MoorlandHaunt => &y2011::innistrad::MOORLAND_HAUNT.rules,
        CardBehavior::Mulch => &y2011::innistrad::MULCH.rules,
        CardBehavior::Mutavault => &y2013::magic_2014::MUTAVAULT.rules,
        CardBehavior::Mutilate => &y2012::magic_2013::MUTILATE.rules,
        CardBehavior::Negate => &y2012::magic_2013::NEGATE.rules,
        CardBehavior::OblivionRing => &y2012::magic_2013::OBLIVION_RING.rules,
        CardBehavior::ObzedatGhostCouncil => &y2013::gatecrash::OBZEDAT_GHOST_COUNCIL.rules,
        CardBehavior::OvergrownTomb => &y2012::return_to_ravnica::OVERGROWN_TOMB.rules,
        CardBehavior::PillarOfFlame => &y2012::avacyn_restored::PILLAR_OF_FLAME.rules,
        CardBehavior::PithingNeedle => &y2012::return_to_ravnica::PITHING_NEEDLE.rules,
        CardBehavior::PrimevalBounty => &y2013::magic_2014::PRIMEVAL_BOUNTY.rules,
        CardBehavior::Putrefy => &y2013::dragons_maze::PUTREFY.rules,
        CardBehavior::Quicken => &y2013::magic_2014::QUICKEN.rules,
        CardBehavior::RatchetBomb => &y2013::magic_2014::RATCHET_BOMB.rules,
        CardBehavior::RayOfRevelation => &y2012::dark_ascension::RAY_OF_REVELATION.rules,
        CardBehavior::RestInPeace => &y2012::return_to_ravnica::REST_IN_PEACE.rules,
        CardBehavior::RestorationAngel => &y2012::avacyn_restored::RESTORATION_ANGEL.rules,
        CardBehavior::RhoxFaithmender => &y2012::magic_2013::RHOX_FAITHMENDER.rules,
        CardBehavior::RootboundCrag => &y2012::magic_2013::ROOTBOUND_CRAG.rules,
        CardBehavior::RuricTharTheUnbowed => &y2013::dragons_maze::RURIC_THAR_THE_UNBOWED.rules,
        CardBehavior::SacredFoundry => &y2013::gatecrash::SACRED_FOUNDRY.rules,
        CardBehavior::ScavengingOoze => &y2013::magic_2014::SCAVENGING_OOZE.rules,
        CardBehavior::SelesnyaCharm => &y2012::return_to_ravnica::SELESNYA_CHARM.rules,
        CardBehavior::SepulchralPrimordial => &y2013::gatecrash::SEPULCHRAL_PRIMORDIAL.rules,
        CardBehavior::ShadowbornDemon => &y2013::magic_2014::SHADOWBORN_DEMON.rules,
        CardBehavior::SigardaHostOfHerons => &y2012::avacyn_restored::SIGARDA_HOST_OF_HERONS.rules,
        CardBehavior::SignInBlood => &y2012::magic_2013::SIGN_IN_BLOOD.rules,
        CardBehavior::SinCollector => &y2013::dragons_maze::SIN_COLLECTOR.rules,
        CardBehavior::SnapcasterMage => &y2011::innistrad::SNAPCASTER_MAGE.rules,
        CardBehavior::SphinxsRevelation => &y2012::return_to_ravnica::SPHINXS_REVELATION.rules,
        CardBehavior::SteamVents => &y2012::return_to_ravnica::STEAM_VENTS.rules,
        CardBehavior::StompingGround => &y2013::gatecrash::STOMPING_GROUND.rules,
        CardBehavior::StranglerootGeist => &y2012::dark_ascension::STRANGLEROOT_GEIST.rules,
        CardBehavior::SulfurFalls => &y2011::innistrad::SULFUR_FALLS.rules,
        CardBehavior::SunpetalGrove => &y2012::magic_2013::SUNPETAL_GROVE.rules,
        CardBehavior::SupremeVerdict => &y2012::return_to_ravnica::SUPREME_VERDICT.rules,
        CardBehavior::Syncopate => &y2012::return_to_ravnica::SYNCOPATE.rules,
        CardBehavior::TempleGarden => &y2012::return_to_ravnica::TEMPLE_GARDEN.rules,
        CardBehavior::Terminus => &y2012::avacyn_restored::TERMINUS.rules,
        CardBehavior::ThinkTwice => &y2011::innistrad::THINK_TWICE.rules,
        CardBehavior::Thragtusk => &y2012::magic_2013::THRAGTUSK.rules,
        CardBehavior::ThundermawHellkite => &y2012::magic_2013::THUNDERMAW_HELLKITE.rules,
        CardBehavior::TragicSlip => &y2012::dark_ascension::TRAGIC_SLIP.rules,
        CardBehavior::TurnBurn => &y2013::dragons_maze::TURN_BURN.rules,
        CardBehavior::UltimatePrice => &y2012::return_to_ravnica::ULTIMATE_PRICE.rules,
        CardBehavior::UnburialRites => &y2011::innistrad::UNBURIAL_RITES.rules,
        CardBehavior::UnderworldConnections => {
            &y2012::return_to_ravnica::UNDERWORLD_CONNECTIONS.rules
        }
        CardBehavior::UnflinchingCourage => &y2013::dragons_maze::UNFLINCHING_COURAGE.rules,
        CardBehavior::UrgentExorcism => &y2011::innistrad::URGENT_EXORCISM.rules,
        CardBehavior::VampireNighthawk => &y2012::magic_2013::VAMPIRE_NIGHTHAWK.rules,
        CardBehavior::VaultOfTheArchangel => &y2012::dark_ascension::VAULT_OF_THE_ARCHANGEL.rules,
        CardBehavior::VoiceOfResurgence => &y2013::dragons_maze::VOICE_OF_RESURGENCE.rules,
        CardBehavior::VolcanicStrength => &y2012::magic_2013::VOLCANIC_STRENGTH.rules,
        CardBehavior::VraskaTheUnseen => &y2012::return_to_ravnica::VRASKA_THE_UNSEEN.rules,
        CardBehavior::WarPriestOfThune => &y2012::magic_2013::WAR_PRIEST_OF_THUNE.rules,
        CardBehavior::WarleadersHelix => &y2013::dragons_maze::WARLEADERS_HELIX.rules,
        CardBehavior::WoodlandCemetery => &y2011::innistrad::WOODLAND_CEMETERY.rules,
        CardBehavior::ZealousConscripts => &y2012::avacyn_restored::ZEALOUS_CONSCRIPTS.rules,
        CardBehavior::Unsupported => &UNSUPPORTED_RULES,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{CardRecord, SET_MODULES, y1993, y2011, y2012, y2013};
    use crate::card::{
        CardEffectStatus, CardPrinting, CardPrintingId, CardStructure, DoubleFacedKind,
        PlayActionKind, PlayRestriction, SpellForm, TargetPredicate, cards,
    };
    use crate::{CardDefinitionId, CardPartId, CardSet, Format, ModeId, PlayOptionId};

    const SET_SOURCES: &[(&str, &str)] = &[
        ("y1993/alpha.rs", include_str!("y1993/alpha.rs")),
        (
            "y1993/arabian_nights.rs",
            include_str!("y1993/arabian_nights.rs"),
        ),
        ("y1993/beta.rs", include_str!("y1993/beta.rs")),
        (
            "y1993/collectors_edition.rs",
            include_str!("y1993/collectors_edition.rs"),
        ),
        (
            "y1993/international_collectors_edition.rs",
            include_str!("y1993/international_collectors_edition.rs"),
        ),
        ("y1993/unlimited.rs", include_str!("y1993/unlimited.rs")),
        ("y1994/antiquities.rs", include_str!("y1994/antiquities.rs")),
        (
            "y1994/fallen_empires.rs",
            include_str!("y1994/fallen_empires.rs"),
        ),
        ("y1994/legends.rs", include_str!("y1994/legends.rs")),
        ("y1994/promo_1994.rs", include_str!("y1994/promo_1994.rs")),
        ("y1994/revised.rs", include_str!("y1994/revised.rs")),
        ("y1994/the_dark.rs", include_str!("y1994/the_dark.rs")),
        ("y2011/innistrad.rs", include_str!("y2011/innistrad.rs")),
        (
            "y2012/avacyn_restored.rs",
            include_str!("y2012/avacyn_restored.rs"),
        ),
        (
            "y2012/dark_ascension.rs",
            include_str!("y2012/dark_ascension.rs"),
        ),
        ("y2012/magic_2013.rs", include_str!("y2012/magic_2013.rs")),
        (
            "y2012/return_to_ravnica.rs",
            include_str!("y2012/return_to_ravnica.rs"),
        ),
        (
            "y2013/dragons_maze.rs",
            include_str!("y2013/dragons_maze.rs"),
        ),
        ("y2013/gatecrash.rs", include_str!("y2013/gatecrash.rs")),
        ("y2013/magic_2014.rs", include_str!("y2013/magic_2014.rs")),
    ];

    fn standard_records() -> Vec<&'static CardRecord> {
        let mut records = SET_MODULES
            .iter()
            .filter(|module| Format::IsdRtrStandard.allows_set(module.set))
            .flat_map(|module| module.cards.iter().copied())
            .collect::<Vec<_>>();
        records.sort_unstable_by_key(|record| record.id);
        records
    }

    fn is_uuid(value: &str) -> bool {
        value.len() == 36
            && value.bytes().enumerate().all(|(index, byte)| match index {
                8 | 13 | 18 | 23 => byte == b'-',
                _ => byte.is_ascii_hexdigit(),
            })
    }

    fn printings_for_set(set: CardSet) -> Vec<CardPrinting> {
        let module = SET_MODULES.iter().find(|module| module.set == set).unwrap();
        module
            .cards
            .iter()
            .map(|record| CardPrinting::new(record.id, set))
            .chain(
                module
                    .additional_printings
                    .iter()
                    .map(|record| record.printing(set)),
            )
            .collect()
    }

    #[test]
    fn every_supported_set_has_one_matching_module() {
        let expected_sets = Format::OldSchool9394
            .rules()
            .allowed_sets
            .iter()
            .chain(Format::IsdRtrStandard.rules().allowed_sets)
            .copied()
            .collect::<Vec<_>>();
        let registered_sets = SET_MODULES
            .iter()
            .map(|module| module.set)
            .collect::<Vec<_>>();

        assert_eq!(registered_sets, expected_sets);
        assert_eq!(registered_sets.len(), 20);
        assert_eq!(
            registered_sets
                .iter()
                .copied()
                .collect::<HashSet<_>>()
                .len(),
            20
        );

        for module in SET_MODULES {
            for record in module.cards {
                assert_eq!(
                    record.set, module.set,
                    "{} is registered in the wrong set",
                    record.name
                );
            }
        }
    }

    #[test]
    fn built_in_records_keep_stable_dense_ids_and_unique_identity() {
        let records = SET_MODULES
            .iter()
            .flat_map(|module| module.cards.iter().copied())
            .collect::<Vec<_>>();
        assert_eq!(records.len(), 244);

        let mut ids = records.iter().map(|record| record.id).collect::<Vec<_>>();
        ids.sort_unstable();
        assert_eq!(
            ids.iter().map(|id| id.0).collect::<Vec<_>>(),
            (1..=244).collect::<Vec<_>>()
        );
        assert_eq!(
            records
                .iter()
                .map(|record| record.name)
                .collect::<HashSet<_>>()
                .len(),
            records.len()
        );
        assert_eq!(
            records
                .iter()
                .map(|record| record.behavior)
                .collect::<HashSet<_>>()
                .len(),
            records.len()
        );
    }

    #[test]
    fn built_in_catalog_indexes_definitions_and_printings_separately() {
        let catalog = crate::card::catalog().unwrap();
        let printing_count = (1..=244)
            .map(|id| catalog.printings_for(CardDefinitionId(id)).len())
            .sum::<usize>();

        assert_eq!(printing_count, 624);
        for variant in 0..3 {
            assert!(
                catalog
                    .get_printing(CardPrintingId::with_variant(
                        cards::PLAINS,
                        CardSet::Beta,
                        variant,
                    ))
                    .is_some()
            );
        }
        assert_eq!(catalog.find_by_name("Plains"), Some(cards::PLAINS));
    }

    #[test]
    fn every_card_record_has_an_adjacent_implementation_status_comment() {
        let mut record_count = 0;
        let mut status_count = 0;

        for (path, source) in SET_SOURCES {
            let lines = source.lines().collect::<Vec<_>>();
            status_count += lines
                .iter()
                .filter(|line| line.trim_start().starts_with("// Implementation status:"))
                .count();

            for (index, line) in lines.iter().enumerate() {
                if line.starts_with("pub(in crate::card::sets) static ")
                    && line.contains(": CardRecord")
                {
                    record_count += 1;
                    assert!(
                        index > 0 && lines[index - 1].starts_with("// Implementation status:"),
                        "{path}:{} is missing an adjacent implementation-status comment",
                        index + 1
                    );
                }
            }
        }

        assert_eq!(record_count, 244);
        assert_eq!(status_count, record_count);
    }

    #[test]
    fn standard_records_cover_the_top_eight_pool_with_stable_unique_ids() {
        let records = standard_records();
        assert_eq!(records.len(), 116);

        let mut names = HashSet::new();
        let mut behaviors = HashSet::new();
        for (offset, record) in records.iter().enumerate() {
            assert_eq!(usize::from(record.id.0), 129 + offset);
            assert!(names.insert(record.name));
            assert!(behaviors.insert(record.behavior));
            assert!(!record.is_basic_land);
            assert!(Format::IsdRtrStandard.allows_set(record.set));
            assert_eq!(record.behavior.rules(), &record.rules);
            assert_eq!(record.rules.effect_status, CardEffectStatus::MetadataOnly);
        }

        assert!(!names.contains("Celestial Purge"));
        assert!(names.contains("Celestial Flare"));
    }

    #[test]
    fn standard_records_have_complete_unique_scryfall_metadata() {
        let records = standard_records();
        let mut scryfall_ids = HashSet::new();

        for record in records {
            let scryfall_id = record.scryfall_id;
            assert!(
                is_uuid(scryfall_id),
                "{} has an invalid Scryfall ID: {scryfall_id}",
                record.name
            );
            assert!(
                scryfall_ids.insert(scryfall_id),
                "{} repeats Scryfall ID {scryfall_id}",
                record.name
            );
            assert!(
                !record.artist.trim().is_empty(),
                "{} is missing its artist",
                record.name
            );
        }

        assert_eq!(scryfall_ids.len(), 116);
    }

    #[test]
    fn structured_records_expose_parts_and_play_options_without_losing_primary_rules() {
        let garruk = y2011::innistrad::GARRUK_RELENTLESS.definition();
        assert_eq!(garruk.name, "Garruk Relentless");
        assert_eq!(garruk.rules, garruk.primary_part().unwrap().rules);
        assert_eq!(garruk.parts.len(), 2);
        assert_eq!(garruk.parts[1].name, "Garruk, the Veil-Cursed");
        assert_eq!(garruk.parts[1].mana_cost, None);
        assert_eq!(
            garruk.parts[1].rules.colors,
            [false, false, true, false, true]
        );
        assert!(matches!(
            garruk.structure,
            CardStructure::DoubleFaced {
                front: CardPartId(0),
                back: CardPartId(1),
                kind: DoubleFacedKind::Transforming,
            }
        ));

        let huntmaster = y2012::dark_ascension::HUNTMASTER_OF_THE_FELLS.definition();
        assert_eq!(huntmaster.rules, huntmaster.primary_part().unwrap().rules);
        assert_eq!(huntmaster.parts.len(), 2);
        assert_eq!(huntmaster.parts[1].name, "Ravager of the Fells");
        assert_eq!(huntmaster.parts[1].mana_cost, None);
        assert_eq!(huntmaster.parts[1].rules.creature_stats.unwrap().power, 4);
        assert!(huntmaster.parts[1].rules.creature_stats.unwrap().trample);

        let turn_burn = y2013::dragons_maze::TURN_BURN.definition();
        assert_eq!(turn_burn.name, "Turn // Burn");
        assert_eq!(turn_burn.rules, turn_burn.parts[0].rules);
        assert!(turn_burn.rules.alternate_mana_costs.is_empty());
        assert_eq!(turn_burn.parts.len(), 2);
        assert_eq!(turn_burn.parts[0].name, "Turn");
        assert_eq!(turn_burn.parts[1].name, "Burn");
        assert_eq!(
            turn_burn.parts[1].rules.colors,
            [false, false, false, true, false]
        );
        assert!(matches!(
            turn_burn.structure,
            CardStructure::Split {
                ref parts,
                fused: Some(PlayOptionId(2)),
            } if parts == &[CardPartId(0), CardPartId(1)]
        ));
        assert_eq!(turn_burn.play_options.len(), 3);
        assert!(matches!(
            turn_burn.play_options[2].form,
            SpellForm::Combined(ref parts) if parts == &[CardPartId(0), CardPartId(1)]
        ));
        assert_eq!(
            turn_burn.play_options[2].restriction,
            PlayRestriction::FromHandOnly
        );
        assert_eq!(turn_burn.play_options[2].targets.len(), 2);

        let charm = y2012::return_to_ravnica::IZZET_CHARM.definition();
        assert_eq!(charm.parts.len(), 1);
        assert_eq!(charm.play_options.len(), 1);
        let modes = charm.play_options[0].modes.as_ref().unwrap();
        assert_eq!(
            (modes.minimum, modes.maximum, modes.may_repeat),
            (1, 1, false)
        );
        assert_eq!(modes.modes.len(), 3);
        assert_eq!(modes.modes[0].id, ModeId(0));
        assert_eq!(
            modes.modes[0].targets[0].predicate,
            TargetPredicate::NoncreatureSpell
        );
        assert_eq!(
            modes.modes[1].targets[0].predicate,
            TargetPredicate::CreaturePermanent
        );
        assert!(modes.modes[2].targets.is_empty());
        assert_eq!(charm.play_options[0].action, PlayActionKind::CastSpell);
    }

    #[test]
    fn ordinary_records_synthesize_one_primary_part_and_play_option() {
        let bolt = y1993::alpha::LIGHTNING_BOLT.definition();
        assert_eq!(bolt.parts.len(), 1);
        assert_eq!(bolt.primary_part_id(), CardPartId::PRIMARY);
        assert_eq!(bolt.primary_part().unwrap().rules, bolt.rules);
        assert!(matches!(
            bolt.structure,
            CardStructure::Single {
                main: CardPartId::PRIMARY,
            }
        ));
        assert_eq!(bolt.play_options.len(), 1);
        assert_eq!(bolt.play_options[0].id, PlayOptionId::DEFAULT);
        assert_eq!(
            bolt.play_options[0].form,
            SpellForm::Part(CardPartId::PRIMARY)
        );

        let mountain = y1993::alpha::MOUNTAIN.definition();
        assert_eq!(mountain.parts[0].mana_cost, None);
        assert_eq!(mountain.play_options[0].action, PlayActionKind::PlayLand);
        assert_eq!(mountain.play_options[0].mana_cost, None);
    }

    #[test]
    fn cavern_records_only_its_unrestricted_mana_ability() {
        let production = y2012::avacyn_restored::CAVERN_OF_SOULS
            .rules
            .mana_production
            .unwrap();
        assert_eq!(production.colors, [false, false, false, false, false, true]);
        assert_eq!(production.amount, 1);
    }

    #[test]
    fn early_core_sets_reuse_definitions_without_duplicating_identity() {
        let all_definition_ids = SET_MODULES
            .iter()
            .flat_map(|module| module.cards.iter().map(|record| record.id))
            .collect::<HashSet<_>>();
        let basics = [
            cards::PLAINS,
            cards::ISLAND,
            cards::SWAMP,
            cards::MOUNTAIN,
            cards::FOREST,
        ];

        let early_sets = [
            (CardSet::Alpha, 83, 88, 2_u16),
            (CardSet::Beta, 84, 94, 3_u16),
            (CardSet::Unlimited, 84, 94, 3_u16),
            (CardSet::CollectorsEdition, 84, 94, 3_u16),
            (CardSet::InternationalCollectorsEdition, 84, 94, 3_u16),
        ];

        let mut printing_ids = HashSet::new();
        for (set, expected_cards, expected_printings, expected_basic_variants) in early_sets {
            let printings = printings_for_set(set);
            assert_eq!(printings.len(), expected_printings);
            assert_eq!(
                printings
                    .iter()
                    .map(|printing| printing.id.definition)
                    .collect::<HashSet<_>>()
                    .len(),
                expected_cards
            );

            for printing in &printings {
                assert!(all_definition_ids.contains(&printing.id.definition));
                assert_eq!(printing.id.set, set);
                assert!(printing_ids.insert(printing.id));
            }
            for basic in basics {
                let variants = printings
                    .iter()
                    .filter(|printing| printing.id.definition == basic)
                    .map(|printing| printing.id.variant)
                    .collect::<HashSet<_>>();
                assert_eq!(variants.len(), usize::from(expected_basic_variants));
                assert_eq!(variants, (0..expected_basic_variants).collect());
            }
        }

        assert_eq!(y1993::beta::VOLCANIC_ISLAND.id, cards::VOLCANIC_ISLAND);
        assert_eq!(y1993::beta::VOLCANIC_ISLAND.set, CardSet::Beta);
    }
}
