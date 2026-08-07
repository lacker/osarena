use crate::CardDefinitionId;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardSet {
    Alpha,
    Beta,
    Unlimited,
    CollectorsEdition,
    InternationalCollectorsEdition,
    ArabianNights,
    Antiquities,
    Revised,
    Legends,
    TheDark,
    FallenEmpires,
    Promo1994,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardDefinition {
    pub id: CardDefinitionId,
    pub name: String,
    pub set: CardSet,
    pub is_basic_land: bool,
    pub behavior: CardBehavior,
    pub rules: CardRules,
}

impl CardDefinition {
    /// Creates a definition using the built-in metadata for `behavior`.
    #[must_use]
    pub fn new(
        id: CardDefinitionId,
        name: impl Into<String>,
        set: CardSet,
        is_basic_land: bool,
        behavior: CardBehavior,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            set,
            is_basic_land,
            behavior,
            rules: *behavior.rules(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardBehavior {
    AncestralRecall,
    AnkhOfMishra,
    ArgothianPixies,
    Armageddon,
    Atog,
    Badlands,
    BallLightning,
    Balance,
    Berserk,
    Bayou,
    BlackLotus,
    BlackKnight,
    BlackVise,
    BirdsOfParadise,
    BlueElementalBlast,
    Braingeyser,
    BloodMoon,
    ChainLightning,
    Channel,
    ChaosOrb,
    CityOfBrass,
    CityInABottle,
    CopperTablet,
    CopyArtifact,
    Counterspell,
    Crusade,
    DarkRitual,
    DemonicTutor,
    Detonate,
    DivineOffering,
    DrainLife,
    DragonWhelp,
    Disenchant,
    DustToDust,
    EnergyFlux,
    Earthquake,
    ErhnamDjinn,
    Forest,
    Fireball,
    Fork,
    GiantGrowth,
    GlassesOfUrza,
    GoblinBalloonBrigade,
    GoblinDiggingTeam,
    GoblinGrenade,
    GoblinKing,
    GoblinsOfTheFlarg,
    GraniteGargoyle,
    HurkylsRecall,
    HymnToTourach,
    HypnoticSpecter,
    IcyManipulator,
    IcatianJavelineers,
    IronStar,
    IronclawOrcs,
    Island,
    IvoryTower,
    JayemdaeTome,
    Juggernaut,
    JuzamDjinn,
    KirdApe,
    LlanowarElves,
    LibraryOfAlexandria,
    ManaDrain,
    ManaVault,
    MazeOfIth,
    MindTwist,
    MishrasWorkshop,
    Moat,
    NevinyrralsDisk,
    OrderOfLeitbur,
    OrderOfTheEbonHand,
    Pendelhaven,
    Plateau,
    PsionicBlast,
    Recall,
    Regrowth,
    RelicBarrier,
    SageOfLatNam,
    Savannah,
    SavannahLions,
    Scrubland,
    SerendibEfreet,
    SedgeTroll,
    SengirVampire,
    ScrybSprites,
    Sinkhole,
    StoneRain,
    Swamp,
    SylvanLibrary,
    Taiga,
    Terror,
    ThunderSpirit,
    TimeVault,
    Timetwister,
    TropicalIsland,
    UndergroundSea,
    VolcanicIsland,
    FellwarStone,
    Mountain,
    LightningBolt,
    MishrasFactory,
    MoxEmerald,
    MoxJet,
    MoxPearl,
    MoxRuby,
    MoxSapphire,
    OrcishMechanics,
    Plains,
    RedElementalBlast,
    Shatter,
    Smoke,
    SolRing,
    SerraAngel,
    StoneGiant,
    StripMine,
    SuChi,
    SwordsToPlowshares,
    TimeWalk,
    Tundra,
    Triskelion,
    Tetravus,
    TheAbyss,
    WheelOfFortune,
    WhirlingDervish,
    WhiteKnight,
    WinterOrb,
    WrathOfGod,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardKind {
    Land,
    Creature,
    Artifact,
    ArtifactCreature,
    Enchantment,
    Instant,
    Sorcery,
}

impl CardKind {
    #[must_use]
    pub const fn is_creature(self) -> bool {
        matches!(self, Self::Creature | Self::ArtifactCreature)
    }

    #[must_use]
    pub const fn is_artifact(self) -> bool {
        matches!(self, Self::Artifact | Self::ArtifactCreature)
    }

    #[must_use]
    pub const fn is_permanent(self) -> bool {
        matches!(
            self,
            Self::Land
                | Self::Creature
                | Self::Artifact
                | Self::ArtifactCreature
                | Self::Enchantment
        )
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ManaCost {
    pub generic: u16,
    pub white: u16,
    pub blue: u16,
    pub black: u16,
    pub red: u16,
    pub green: u16,
    pub variable_x: bool,
    pub x_multiplier: u16,
}

impl ManaCost {
    #[must_use]
    pub const fn new(generic: u16, red: u16) -> Self {
        Self {
            generic,
            white: 0,
            blue: 0,
            black: 0,
            red,
            green: 0,
            variable_x: false,
            x_multiplier: 0,
        }
    }

    #[must_use]
    pub const fn colored(
        generic: u16,
        white: u16,
        blue: u16,
        black: u16,
        red: u16,
        green: u16,
    ) -> Self {
        Self {
            generic,
            white,
            blue,
            black,
            red,
            green,
            variable_x: false,
            x_multiplier: 0,
        }
    }

    #[must_use]
    pub const fn with_x(red: u16) -> Self {
        Self {
            generic: 0,
            white: 0,
            blue: 0,
            black: 0,
            red,
            green: 0,
            variable_x: true,
            x_multiplier: 1,
        }
    }

    #[must_use]
    pub const fn colored_x(white: u16, blue: u16, black: u16, red: u16, green: u16) -> Self {
        Self {
            generic: 0,
            white,
            blue,
            black,
            red,
            green,
            variable_x: true,
            x_multiplier: 1,
        }
    }

    #[must_use]
    pub const fn variable(
        generic: u16,
        white: u16,
        blue: u16,
        black: u16,
        red: u16,
        green: u16,
        x_multiplier: u16,
    ) -> Self {
        Self {
            generic,
            white,
            blue,
            black,
            red,
            green,
            variable_x: true,
            x_multiplier,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CreatureStats {
    pub power: i16,
    pub toughness: i16,
    pub haste: bool,
    pub trample: bool,
}

/// How a client should describe activating a permanent's targeted ability.
///
/// `targeted` is a template with `{}` where the target's name goes, so a menu
/// can name the effect instead of the card; `summary` is the same effect with
/// no particular target picked yet.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActivatedAbilityText {
    pub targeted: &'static str,
    pub summary: &'static str,
}

/// Declarative rules metadata kept beside a card's catalog identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct CardRules {
    pub kind: CardKind,
    pub mana_cost: ManaCost,
    pub creature_stats: Option<CreatureStats>,
    pub text: &'static str,
    pub activated_ability_text: Option<ActivatedAbilityText>,
    pub is_legendary: bool,
    pub is_goblin: bool,
    pub has_flying: bool,
    pub has_mountainwalk: bool,
    pub has_vigilance: bool,
    /// Printed colors in `[white, blue, black, red, green]` order.
    pub colors: [bool; 5],
}

impl CardRules {
    #[must_use]
    pub const fn new(kind: CardKind, mana_cost: ManaCost, text: &'static str) -> Self {
        Self {
            kind,
            mana_cost,
            creature_stats: None,
            text,
            activated_ability_text: None,
            is_legendary: false,
            is_goblin: false,
            has_flying: false,
            has_mountainwalk: false,
            has_vigilance: false,
            colors: [
                mana_cost.white > 0,
                mana_cost.blue > 0,
                mana_cost.black > 0,
                mana_cost.red > 0,
                mana_cost.green > 0,
            ],
        }
    }

    #[must_use]
    pub const fn creature(mut self, power: i16, toughness: i16) -> Self {
        self.creature_stats = Some(CreatureStats {
            power,
            toughness,
            haste: false,
            trample: false,
        });
        self
    }

    #[must_use]
    pub const fn haste(mut self) -> Self {
        if let Some(mut stats) = self.creature_stats {
            stats.haste = true;
            self.creature_stats = Some(stats);
        }
        self
    }

    #[must_use]
    pub const fn trample(mut self) -> Self {
        if let Some(mut stats) = self.creature_stats {
            stats.trample = true;
            self.creature_stats = Some(stats);
        }
        self
    }

    #[must_use]
    pub const fn legendary(mut self) -> Self {
        self.is_legendary = true;
        self
    }

    #[must_use]
    pub const fn goblin(mut self) -> Self {
        self.is_goblin = true;
        self
    }

    #[must_use]
    pub const fn flying(mut self) -> Self {
        self.has_flying = true;
        self
    }

    #[must_use]
    pub const fn mountainwalk(mut self) -> Self {
        self.has_mountainwalk = true;
        self
    }

    #[must_use]
    pub const fn vigilance(mut self) -> Self {
        self.has_vigilance = true;
        self
    }

    #[must_use]
    pub const fn activated(mut self, targeted: &'static str, summary: &'static str) -> Self {
        self.activated_ability_text = Some(ActivatedAbilityText { targeted, summary });
        self
    }

    pub(super) const fn unsupported() -> Self {
        let mut rules = Self::new(
            CardKind::Artifact,
            ManaCost::new(u16::MAX, u16::MAX),
            "Rules text is not implemented.",
        );
        rules.colors = [false; 5];
        rules
    }
}
