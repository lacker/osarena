use crate::ids::{
    AdditionalCostId, AlternativeCostId, CardDefinitionId, CardPartId, MeldRecipeId, ModeId,
    PlayOptionId, TargetSlotId,
};

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
    Innistrad,
    DarkAscension,
    AvacynRestored,
    Magic2013,
    ReturnToRavnica,
    Gatecrash,
    DragonsMaze,
    Magic2014,
}

/// Stable identity of one exact printing of a card.
///
/// A card may have several printings in one set, such as basic lands with
/// different art. Variant zero is the primary printing when no alternate is
/// specified.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CardPrintingId {
    pub definition: CardDefinitionId,
    pub set: CardSet,
    pub variant: u16,
}

impl CardPrintingId {
    #[must_use]
    pub const fn new(definition: CardDefinitionId, set: CardSet) -> Self {
        Self {
            definition,
            set,
            variant: 0,
        }
    }

    #[must_use]
    pub const fn with_variant(definition: CardDefinitionId, set: CardSet, variant: u16) -> Self {
        Self {
            definition,
            set,
            variant,
        }
    }
}

/// One cataloged set-and-variant printing of a canonical card definition.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CardPrinting {
    pub id: CardPrintingId,
}

impl CardPrinting {
    #[must_use]
    pub const fn new(definition: CardDefinitionId, set: CardSet) -> Self {
        Self {
            id: CardPrintingId::new(definition, set),
        }
    }

    #[must_use]
    pub const fn with_variant(definition: CardDefinitionId, set: CardSet, variant: u16) -> Self {
        Self {
            id: CardPrintingId::with_variant(definition, set, variant),
        }
    }
}

/// One independently addressable bundle of printed characteristics.
///
/// A part is broader than a physical face: the two halves of a split card are
/// separate parts printed on one face, while a transforming card has one part
/// on each physical face.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardPart {
    pub id: CardPartId,
    pub name: String,
    pub rules: CardRules,
    /// `None` distinguishes a face with no mana cost from a printed `{0}` cost.
    /// `CardRules::mana_cost` remains available as a compatibility value.
    pub mana_cost: Option<ManaCost>,
}

impl CardPart {
    #[must_use]
    pub fn new(id: CardPartId, name: impl Into<String>, rules: CardRules) -> Self {
        Self {
            id,
            name: name.into(),
            rules,
            mana_cost: Some(rules.mana_cost),
        }
    }

    /// Marks a back face or land part as having no printed mana cost.
    #[must_use]
    pub const fn without_mana_cost(mut self) -> Self {
        self.mana_cost = None;
        self
    }
}

/// The rules family used by a two-faced card.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DoubleFacedKind {
    Transforming,
    Modal,
}

/// A secondary spell frame printed alongside a card's ordinary characteristics.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AlternateSpellKind {
    Adventure,
    Omen,
}

/// The physical/logical topology of a canonical card definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CardStructure {
    Single {
        main: CardPartId,
    },
    Split {
        parts: Vec<CardPartId>,
        /// The play option that combines the parts, if the card has one.
        fused: Option<PlayOptionId>,
    },
    Flip {
        normal: CardPartId,
        flipped: CardPartId,
    },
    DoubleFaced {
        front: CardPartId,
        back: CardPartId,
        kind: DoubleFacedKind,
    },
    AlternateSpell {
        main: CardPartId,
        alternate: CardPartId,
        kind: AlternateSpellKind,
    },
    /// A physical card that can participate in a separately cataloged meld
    /// recipe. The recipe, rather than either component definition, supplies
    /// the combined object's result characteristics.
    MeldPart {
        front: CardPartId,
        recipe: MeldRecipeId,
    },
}

/// One named-object condition and one physical-card requirement in a future
/// meld recipe.
///
/// These are deliberately separate. An object's effective name can satisfy
/// `required_name` even when it is a token or copy, while a successful meld
/// must ultimately be backed by the physical `required_card`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeldComponentDef {
    pub required_name: String,
    pub required_card: CardDefinitionId,
}

/// Characteristics of the combined object produced by a meld recipe.
///
/// This is not a printing and does not pretend to be either component card.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeldResultDef {
    pub name: String,
    pub rules: CardRules,
}

/// Catalog data needed to implement meld later without conflating its name
/// predicate with its physical-card validation.
///
/// No supported format executes meld today; this type is intentionally not
/// wired into game actions or resolution yet.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeldRecipeDef {
    pub id: MeldRecipeId,
    pub components: [MeldComponentDef; 2],
    pub result: MeldResultDef,
}

/// The characteristic parts used by an object while it is a spell.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SpellForm {
    Part(CardPartId),
    /// Combined parts retain printed order, which is also resolution order for
    /// a fused split spell.
    Combined(Vec<CardPartId>),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayActionKind {
    CastSpell,
    PlayLand,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayRestriction {
    Normal,
    FromHandOnly,
}

/// A catalog-level description of what can occupy one target slot.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TargetPredicate {
    AnyTarget,
    Player,
    Permanent,
    CreaturePermanent,
    Spell,
    NoncreatureSpell,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetSlotDef {
    pub id: TargetSlotId,
    pub label: String,
    pub predicate: TargetPredicate,
    pub minimum: u8,
    pub maximum: u8,
}

impl TargetSlotDef {
    #[must_use]
    pub fn exactly_one(
        id: TargetSlotId,
        label: impl Into<String>,
        predicate: TargetPredicate,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            predicate,
            minimum: 1,
            maximum: 1,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModeDef {
    pub id: ModeId,
    pub label: String,
    pub targets: Vec<TargetSlotDef>,
    pub effect_status: CardEffectStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModeSetDef {
    pub minimum: u8,
    pub maximum: u8,
    /// Some cards explicitly allow the same mode to be chosen more than once.
    pub may_repeat: bool,
    pub modes: Vec<ModeDef>,
}

impl ModeSetDef {
    #[must_use]
    pub fn choose_one(modes: Vec<ModeDef>) -> Self {
        Self {
            minimum: 1,
            maximum: 1,
            may_repeat: false,
            modes,
        }
    }
}

/// A named alternative to the cost supplied by a play option.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlternativeCostDef {
    pub id: AlternativeCostId,
    pub label: String,
    pub mana_cost: ManaCost,
}

/// A named additional cost. Some additional costs are nonmana costs, so the
/// mana component is optional and the authoritative rules remain in `label`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdditionalCostDef {
    pub id: AdditionalCostId,
    pub label: String,
    pub mana_cost: Option<ManaCost>,
}

/// One legal way to play a card. This is distinct from rules-text modes and
/// from alternative/additional cost choices.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayOptionDef {
    pub id: PlayOptionId,
    pub label: String,
    pub action: PlayActionKind,
    pub form: SpellForm,
    pub mana_cost: Option<ManaCost>,
    pub restriction: PlayRestriction,
    pub modes: Option<ModeSetDef>,
    pub targets: Vec<TargetSlotDef>,
    pub alternative_costs: Vec<AlternativeCostDef>,
    pub additional_costs: Vec<AdditionalCostDef>,
    pub effect_status: CardEffectStatus,
}

impl PlayOptionDef {
    #[must_use]
    pub fn cast(
        id: PlayOptionId,
        label: impl Into<String>,
        form: SpellForm,
        mana_cost: ManaCost,
        effect_status: CardEffectStatus,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            action: PlayActionKind::CastSpell,
            form,
            mana_cost: Some(mana_cost),
            restriction: PlayRestriction::Normal,
            modes: None,
            targets: Vec::new(),
            alternative_costs: Vec::new(),
            additional_costs: Vec::new(),
            effect_status,
        }
    }

    #[must_use]
    pub fn play_land(
        id: PlayOptionId,
        label: impl Into<String>,
        part: CardPartId,
        effect_status: CardEffectStatus,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            action: PlayActionKind::PlayLand,
            form: SpellForm::Part(part),
            mana_cost: None,
            restriction: PlayRestriction::Normal,
            modes: None,
            targets: Vec::new(),
            alternative_costs: Vec::new(),
            additional_costs: Vec::new(),
            effect_status,
        }
    }

    #[must_use]
    pub fn with_targets(mut self, targets: Vec<TargetSlotDef>) -> Self {
        self.targets = targets;
        self
    }

    #[must_use]
    pub fn with_modes(mut self, modes: ModeSetDef) -> Self {
        self.modes = Some(modes);
        self
    }

    #[must_use]
    pub const fn restricted_to_hand(mut self) -> Self {
        self.restriction = PlayRestriction::FromHandOnly;
        self
    }
}

/// The structured portion of a card definition supplied by a set record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardComposition {
    pub parts: Vec<CardPart>,
    pub structure: CardStructure,
    pub play_options: Vec<PlayOptionDef>,
}

impl CardComposition {
    #[must_use]
    pub fn single(name: impl Into<String>, rules: CardRules) -> Self {
        let name = name.into();
        let is_land = rules.kind == CardKind::Land;
        let mut part = CardPart::new(CardPartId::PRIMARY, name.clone(), rules);
        if is_land {
            part = part.without_mana_cost();
        }
        let option = if is_land {
            PlayOptionDef::play_land(
                PlayOptionId::DEFAULT,
                name,
                CardPartId::PRIMARY,
                rules.effect_status,
            )
        } else {
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                name,
                SpellForm::Part(CardPartId::PRIMARY),
                rules.mana_cost,
                rules.effect_status,
            )
        };
        Self {
            parts: vec![part],
            structure: CardStructure::Single {
                main: CardPartId::PRIMARY,
            },
            play_options: vec![option],
        }
    }
}

/// Canonical artwork metadata used when no exact printing is selected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardArt {
    pub scryfall_id: String,
    pub artist: String,
}

impl CardArt {
    #[must_use]
    pub fn new(scryfall_id: impl Into<String>, artist: impl Into<String>) -> Self {
        Self {
            scryfall_id: scryfall_id.into(),
            artist: artist.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardDefinition {
    pub id: CardDefinitionId,
    pub name: String,
    pub art: Option<CardArt>,
    /// The canonical record's debut set within this catalog.
    ///
    /// Rules that care where a card debuted, such as City in a Bottle, use
    /// this field. Format legality instead considers every known `printing`.
    pub set: CardSet,
    pub printings: Vec<CardPrinting>,
    pub is_basic_land: bool,
    pub behavior: CardBehavior,
    /// Compatibility view of the primary/front part. Contextual rules should
    /// use `parts` once the game engine is part-aware.
    pub rules: CardRules,
    pub parts: Vec<CardPart>,
    pub structure: CardStructure,
    pub play_options: Vec<PlayOptionDef>,
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
        let name = name.into();
        let rules = *behavior.rules();
        let composition = CardComposition::single(name.clone(), rules);
        Self {
            id,
            name,
            art: None,
            set,
            printings: vec![CardPrinting::new(id, set)],
            is_basic_land,
            behavior,
            rules,
            parts: composition.parts,
            structure: composition.structure,
            play_options: composition.play_options,
        }
    }

    #[must_use]
    pub fn part(&self, id: CardPartId) -> Option<&CardPart> {
        self.parts.iter().find(|part| part.id == id)
    }

    #[must_use]
    pub fn play_option(&self, id: PlayOptionId) -> Option<&PlayOptionDef> {
        self.play_options.iter().find(|option| option.id == id)
    }

    #[must_use]
    pub fn primary_part_id(&self) -> CardPartId {
        match &self.structure {
            CardStructure::Single { main } | CardStructure::AlternateSpell { main, .. } => *main,
            CardStructure::Split { parts, .. } => {
                parts.first().copied().unwrap_or(CardPartId::PRIMARY)
            }
            CardStructure::Flip { normal, .. } => *normal,
            CardStructure::DoubleFaced { front, .. } | CardStructure::MeldPart { front, .. } => {
                *front
            }
        }
    }

    #[must_use]
    pub fn primary_part(&self) -> Option<&CardPart> {
        self.part(self.primary_part_id())
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
    AbruptDecay,
    Aetherling,
    AngelOfSerenity,
    ArborElf,
    ArchangelOfThune,
    AssembleTheLegion,
    AugurOfBolas,
    AureliasFury,
    AureliaTheWarleader,
    AvacynsPilgrim,
    AzoriusCharm,
    BlasphemousAct,
    BlindObedience,
    BloodBaronOfVizkopa,
    BonfireOfTheDamned,
    BorosCharm,
    BorosReckoner,
    BurningEarth,
    CavernOfSouls,
    CelestialFlare,
    ClifftopRetreat,
    Counterflux,
    DemonicRising,
    DesecrationDemon,
    DetentionSphere,
    DiscipleOfBolas,
    Dispel,
    Dissipate,
    DomriRade,
    DoomBlade,
    Duress,
    ElvishMystic,
    EncroachingWastes,
    EssenceScatter,
    FlamesOfTheFirebrand,
    FlinthoofBoar,
    GarrukRelentless,
    GavonyTownship,
    GazeOfGranite,
    GhorClanRampager,
    GhostQuarter,
    GlacialFortress,
    GodlessShrine,
    GolgariGuildgate,
    GrislySalvage,
    HallowedFountain,
    Hellrider,
    HuntmasterOfTheFells,
    IsolatedChapel,
    IzzetCharm,
    IzzetStaticaster,
    JaceArchitectOfThought,
    JaceMemoryAdept,
    KessigWolfRun,
    LifebaneZombie,
    LilianaOfTheVeil,
    LoxodonSmiter,
    MizziumMortars,
    MoorlandHaunt,
    Mulch,
    Mutavault,
    Mutilate,
    Negate,
    OblivionRing,
    ObzedatGhostCouncil,
    OvergrownTomb,
    PillarOfFlame,
    PithingNeedle,
    PrimevalBounty,
    Putrefy,
    Quicken,
    RatchetBomb,
    RayOfRevelation,
    RestInPeace,
    RestorationAngel,
    RhoxFaithmender,
    RootboundCrag,
    RuricTharTheUnbowed,
    SacredFoundry,
    ScavengingOoze,
    SelesnyaCharm,
    SepulchralPrimordial,
    ShadowbornDemon,
    SigardaHostOfHerons,
    SignInBlood,
    SinCollector,
    SnapcasterMage,
    SphinxsRevelation,
    SteamVents,
    StompingGround,
    StranglerootGeist,
    SulfurFalls,
    SunpetalGrove,
    SupremeVerdict,
    Syncopate,
    TempleGarden,
    Terminus,
    ThinkTwice,
    Thragtusk,
    ThundermawHellkite,
    TragicSlip,
    TurnBurn,
    UltimatePrice,
    UnburialRites,
    UnderworldConnections,
    UnflinchingCourage,
    UrgentExorcism,
    VampireNighthawk,
    VaultOfTheArchangel,
    VoiceOfResurgence,
    VolcanicStrength,
    VraskaTheUnseen,
    WarPriestOfThune,
    WarleadersHelix,
    WoodlandCemetery,
    ZealousConscripts,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardKind {
    Land,
    Creature,
    Artifact,
    ArtifactCreature,
    Enchantment,
    Planeswalker,
    Instant,
    Sorcery,
}

/// Whether the engine executes a card's printed non-baseline effects.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardEffectStatus {
    Implemented,
    MetadataOnly,
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
                | Self::Planeswalker
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
    /// Number of `{R/W}` hybrid symbols in this cost.
    pub white_red_hybrid: u16,
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
            white_red_hybrid: 0,
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
            white_red_hybrid: 0,
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
            white_red_hybrid: 0,
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
            white_red_hybrid: 0,
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
            white_red_hybrid: 0,
            variable_x: true,
            x_multiplier,
        }
    }

    #[must_use]
    pub const fn white_red_hybrid(count: u16) -> Self {
        Self {
            generic: 0,
            white: 0,
            blue: 0,
            black: 0,
            red: 0,
            green: 0,
            white_red_hybrid: count,
            variable_x: false,
            x_multiplier: 0,
        }
    }
}

/// Mana a permanent can produce through its ordinary tap ability.
///
/// Colors use `[white, blue, black, red, green, colorless]` order. Restricted
/// mana abilities remain described in rules text until the payment engine can
/// model their restriction; the unrestricted ability is recorded here.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ManaProduction {
    pub colors: [bool; 6],
    pub amount: u16,
}

/// How a land enters the battlefield before replacement effects are applied.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LandEntry {
    Untapped,
    Tapped,
    TappedUnlessControlsLandType([bool; 5]),
    PayLifeOrTapped(u8),
}

/// A named alternative to a card's primary printed mana cost.
///
/// This covers split-card halves and their fused cost without forcing the
/// initial game implementation to expose every casting mode immediately.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AlternateManaCost {
    pub label: &'static str,
    pub cost: ManaCost,
}

impl AlternateManaCost {
    #[must_use]
    pub const fn new(label: &'static str, cost: ManaCost) -> Self {
        Self { label, cost }
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
    pub effect_status: CardEffectStatus,
    pub type_line: &'static str,
    pub mana_cost: ManaCost,
    pub alternate_mana_costs: &'static [AlternateManaCost],
    pub mana_production: Option<ManaProduction>,
    /// Basic land types in `[Plains, Island, Swamp, Mountain, Forest]` order.
    pub land_types: [bool; 5],
    pub land_entry: LandEntry,
    pub starting_loyalty: Option<u16>,
    pub creature_stats: Option<CreatureStats>,
    pub text: &'static str,
    pub activated_ability_text: Option<ActivatedAbilityText>,
    pub is_legendary: bool,
    pub is_goblin: bool,
    pub has_flying: bool,
    pub has_mountainwalk: bool,
    pub has_vigilance: bool,
    pub has_deathtouch: bool,
    pub has_flash: bool,
    pub has_hexproof: bool,
    pub has_intimidate: bool,
    pub has_lifelink: bool,
    pub has_reach: bool,
    pub has_undying: bool,
    /// Protection colors in `[white, blue, black, red, green]` order.
    pub protection_colors: [bool; 5],
    /// Printed colors in `[white, blue, black, red, green]` order.
    pub colors: [bool; 5],
}

impl CardRules {
    #[must_use]
    pub const fn new(kind: CardKind, mana_cost: ManaCost, text: &'static str) -> Self {
        Self {
            kind,
            effect_status: CardEffectStatus::Implemented,
            type_line: match kind {
                CardKind::Land => "Land",
                CardKind::Creature => "Creature",
                CardKind::Artifact => "Artifact",
                CardKind::ArtifactCreature => "Artifact Creature",
                CardKind::Enchantment => "Enchantment",
                CardKind::Planeswalker => "Planeswalker",
                CardKind::Instant => "Instant",
                CardKind::Sorcery => "Sorcery",
            },
            mana_cost,
            alternate_mana_costs: &[],
            mana_production: None,
            land_types: [false; 5],
            land_entry: LandEntry::Untapped,
            starting_loyalty: None,
            creature_stats: None,
            text,
            activated_ability_text: None,
            is_legendary: false,
            is_goblin: false,
            has_flying: false,
            has_mountainwalk: false,
            has_vigilance: false,
            has_deathtouch: false,
            has_flash: false,
            has_hexproof: false,
            has_intimidate: false,
            has_lifelink: false,
            has_reach: false,
            has_undying: false,
            protection_colors: [false; 5],
            colors: [
                mana_cost.white > 0 || mana_cost.white_red_hybrid > 0,
                mana_cost.blue > 0,
                mana_cost.black > 0,
                mana_cost.red > 0 || mana_cost.white_red_hybrid > 0,
                mana_cost.green > 0,
            ],
        }
    }

    /// Marks printed effects that are cataloged but not executed by the game
    /// engine yet. Lands can still use declarative entry/mana metadata and
    /// creatures can still be cast as their baseline bodies.
    #[must_use]
    pub const fn metadata_only(mut self) -> Self {
        self.effect_status = CardEffectStatus::MetadataOnly;
        self
    }

    #[must_use]
    pub const fn type_line(mut self, type_line: &'static str) -> Self {
        self.type_line = type_line;
        self
    }

    /// Overrides colors supplied by a color indicator or another printed
    /// characteristic that cannot be derived from the mana cost.
    #[must_use]
    pub const fn printed_colors(mut self, colors: [bool; 5]) -> Self {
        self.colors = colors;
        self
    }

    #[must_use]
    pub const fn alternate_costs(mut self, costs: &'static [AlternateManaCost]) -> Self {
        self.alternate_mana_costs = costs;
        let mut index = 0;
        while index < costs.len() {
            let cost = costs[index].cost;
            self.colors[0] = self.colors[0] || cost.white > 0 || cost.white_red_hybrid > 0;
            self.colors[1] = self.colors[1] || cost.blue > 0;
            self.colors[2] = self.colors[2] || cost.black > 0;
            self.colors[3] = self.colors[3] || cost.red > 0 || cost.white_red_hybrid > 0;
            self.colors[4] = self.colors[4] || cost.green > 0;
            index += 1;
        }
        self
    }

    #[must_use]
    pub const fn produces(mut self, colors: [bool; 6]) -> Self {
        self.mana_production = Some(ManaProduction { colors, amount: 1 });
        self
    }

    #[must_use]
    pub const fn produces_amount(mut self, colors: [bool; 6], amount: u16) -> Self {
        self.mana_production = Some(ManaProduction { colors, amount });
        self
    }

    #[must_use]
    pub const fn land_types(mut self, land_types: [bool; 5]) -> Self {
        self.land_types = land_types;
        self
    }

    #[must_use]
    pub const fn land_entry(mut self, land_entry: LandEntry) -> Self {
        self.land_entry = land_entry;
        self
    }

    #[must_use]
    pub const fn planeswalker(mut self, starting_loyalty: u16) -> Self {
        self.starting_loyalty = Some(starting_loyalty);
        self
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
    pub const fn deathtouch(mut self) -> Self {
        self.has_deathtouch = true;
        self
    }

    #[must_use]
    pub const fn flash(mut self) -> Self {
        self.has_flash = true;
        self
    }

    #[must_use]
    pub const fn hexproof(mut self) -> Self {
        self.has_hexproof = true;
        self
    }

    #[must_use]
    pub const fn intimidate(mut self) -> Self {
        self.has_intimidate = true;
        self
    }

    #[must_use]
    pub const fn lifelink(mut self) -> Self {
        self.has_lifelink = true;
        self
    }

    #[must_use]
    pub const fn reach(mut self) -> Self {
        self.has_reach = true;
        self
    }

    #[must_use]
    pub const fn undying(mut self) -> Self {
        self.has_undying = true;
        self
    }

    #[must_use]
    pub const fn protection(mut self, colors: [bool; 5]) -> Self {
        self.protection_colors = colors;
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

#[cfg(test)]
mod tests {
    use super::{
        AlternateManaCost, CardBehavior, CardDefinition, CardEffectStatus, CardKind, CardPrinting,
        CardPrintingId, CardRules, CardSet, ManaCost,
    };
    use crate::CardDefinitionId;

    #[test]
    fn printing_ids_distinguish_variants_within_one_set() {
        let definition = CardDefinitionId(7);
        let primary = CardPrintingId::new(definition, CardSet::Alpha);
        let alternate = CardPrintingId::with_variant(definition, CardSet::Alpha, 1);

        assert_eq!(primary.variant, 0);
        assert_ne!(primary, alternate);
        assert_eq!(
            CardPrinting::with_variant(definition, CardSet::Alpha, 1).id,
            alternate
        );
    }

    #[test]
    fn definitions_start_with_their_primary_printing() {
        let id = CardDefinitionId(7);
        let definition = CardDefinition::new(
            id,
            "Test Card",
            CardSet::Alpha,
            false,
            CardBehavior::Unsupported,
        );

        assert_eq!(
            definition.printings,
            vec![CardPrinting::new(id, CardSet::Alpha)]
        );
    }

    #[test]
    fn planeswalkers_are_permanents() {
        assert!(CardKind::Planeswalker.is_permanent());
        assert!(!CardKind::Planeswalker.is_creature());
    }

    #[test]
    fn white_red_hybrid_costs_have_both_printed_colors() {
        let rules = CardRules::new(CardKind::Creature, ManaCost::white_red_hybrid(3), "");
        assert_eq!(rules.colors, [true, false, false, true, false]);
    }

    #[test]
    fn alternate_costs_extend_the_cards_printed_colors() {
        static ALTERNATES: [AlternateManaCost; 2] = [
            AlternateManaCost::new("Burn", ManaCost::colored(1, 0, 0, 0, 1, 0)),
            AlternateManaCost::new("Fuse", ManaCost::colored(3, 0, 1, 0, 1, 0)),
        ];
        let rules = CardRules::new(CardKind::Instant, ManaCost::colored(2, 0, 1, 0, 0, 0), "")
            .alternate_costs(&ALTERNATES);
        assert_eq!(rules.colors, [false, true, false, true, false]);
    }

    #[test]
    fn metadata_only_is_an_explicit_non_default_status() {
        let implemented = CardRules::new(CardKind::Instant, ManaCost::default(), "");
        assert_eq!(implemented.effect_status, CardEffectStatus::Implemented);
        assert_eq!(
            implemented.metadata_only().effect_status,
            CardEffectStatus::MetadataOnly
        );
    }
}
