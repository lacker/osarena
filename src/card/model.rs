use crate::ids::{
    AbilityId, AdditionalCostId, AlternativeCostId, CardDefinitionId, CardPartId, MeldRecipeId,
    ModeId, PlayOptionId, TargetSlotId,
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

/// Whether a card part has a printed mana cost.
///
/// `Cost(ManaCost::default())` represents a printed `{0}` cost. `None` means
/// that no mana cost exists at all; it is not a cost that can ordinarily be
/// paid. `CardPart::mana_cost` remains an `Option` for compatibility and is
/// the stored, authoritative representation of this distinction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PrintedManaCost {
    None,
    Cost(ManaCost),
}

impl PrintedManaCost {
    #[must_use]
    pub const fn as_option(self) -> Option<ManaCost> {
        match self {
            Self::None => None,
            Self::Cost(cost) => Some(cost),
        }
    }

    /// Both a nonexistent mana cost and a printed `{0}` cost have mana value
    /// zero, even though only the latter is a payable printed cost.
    #[must_use]
    pub const fn mana_value(self) -> u16 {
        match self {
            Self::None => 0,
            Self::Cost(cost) => cost.mana_value(),
        }
    }
}

impl CardPart {
    #[must_use]
    pub fn new(id: CardPartId, name: impl Into<String>, rules: CardRules) -> Self {
        Self::with_printed_mana_cost(id, name, rules, PrintedManaCost::Cost(rules.mana_cost))
    }

    /// Creates a part with an explicit printed-cost characteristic.
    #[must_use]
    pub fn with_printed_mana_cost(
        id: CardPartId,
        name: impl Into<String>,
        rules: CardRules,
        printed_mana_cost: PrintedManaCost,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            rules,
            mana_cost: printed_mana_cost.as_option(),
        }
    }

    #[must_use]
    pub const fn printed_mana_cost(&self) -> PrintedManaCost {
        match self.mana_cost {
            Some(cost) => PrintedManaCost::Cost(cost),
            None => PrintedManaCost::None,
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

/// A zone in which an ability can exist or an object can be selected.
///
/// This is catalog vocabulary. Runtime zones may store objects differently,
/// but card definitions should not need to know those storage details.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ZoneKind {
    Library,
    Hand,
    Battlefield,
    Graveyard,
    Stack,
    Exile,
    Command,
}

/// A player described relative to an ability's controller or triggering
/// event, rather than by a game-specific player identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayerRelation {
    Any,
    You,
    Opponent,
    ActivePlayer,
    NonactivePlayer,
    /// The player identified directly by the event, such as the player whose
    /// upkeep began or who cast a spell.
    EventPlayer,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ColorDef {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl ColorDef {
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::White => 0,
            Self::Blue => 1,
            Self::Black => 2,
            Self::Red => 3,
            Self::Green => 4,
        }
    }
}

/// A composable predicate over a card or game object.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectPredicateDef {
    Any,
    Source,
    Land,
    Creature,
    Artifact,
    Spell,
    NoncreatureSpell,
    CardKind(CardKind),
    Color(ColorDef),
    Subtype(&'static str),
    All(&'static [ObjectPredicateDef]),
    AnyOf(&'static [ObjectPredicateDef]),
    Not(&'static ObjectPredicateDef),
    /// A narrow, named predicate that cannot yet be expressed by the common
    /// vocabulary. The engine owns the meaning of each supported name.
    Special(&'static str),
}

/// The legal subject of one ability target slot.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AbilityTargetPredicate {
    AnyTarget,
    Player(PlayerRelation),
    Object {
        object: ObjectPredicateDef,
        zones: &'static [ZoneKind],
        /// Relation of the object's controller, when the zone supplies one.
        controller: Option<PlayerRelation>,
        /// Relation of the physical card's owner. This is the relevant
        /// relation for private zones such as a graveyard.
        owner: Option<PlayerRelation>,
    },
}

/// A const-friendly target declaration kept beside a printed ability.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AbilityTargetDef {
    pub id: TargetSlotId,
    pub label: &'static str,
    pub predicate: AbilityTargetPredicate,
    pub minimum: u8,
    pub maximum: u8,
}

impl AbilityTargetDef {
    #[must_use]
    pub const fn exactly_one(
        id: TargetSlotId,
        label: &'static str,
        predicate: AbilityTargetPredicate,
    ) -> Self {
        Self {
            id,
            label,
            predicate,
            minimum: 1,
            maximum: 1,
        }
    }
}

/// A cost paid to activate an ability. The ability category, rather than the
/// presence of an `AddMana` effect, determines whether it is a mana ability.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AbilityCostDef {
    Mana(ManaCost),
    TapSource,
    UntapSource,
    SacrificeSource,
    PayLife(u16),
    DiscardCards(u8),
    SacrificePermanent {
        object: ObjectPredicateDef,
        controller: PlayerRelation,
    },
    ExileSource,
    Special(&'static str),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManaKindDef {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

/// A basic land subtype used by type-changing effects and mana provenance.
///
/// Card definitions intentionally list their printed mana abilities even when
/// a basic land type would grant the same ability under the comprehensive
/// rules. This keeps the catalog self-describing. The runtime only synthesizes
/// an intrinsic ability when an effect such as Blood Moon changes a land's
/// subtype without also supplying an explicit ability clause.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BasicLandType {
    Plains,
    Island,
    Swamp,
    Mountain,
    Forest,
}

impl BasicLandType {
    pub const ALL: [Self; 5] = [
        Self::Plains,
        Self::Island,
        Self::Swamp,
        Self::Mountain,
        Self::Forest,
    ];

    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::Plains => 0,
            Self::Island => 1,
            Self::Swamp => 2,
            Self::Mountain => 3,
            Self::Forest => 4,
        }
    }

    #[must_use]
    pub const fn mana_kind(self) -> ManaKindDef {
        match self {
            Self::Plains => ManaKindDef::White,
            Self::Island => ManaKindDef::Blue,
            Self::Swamp => ManaKindDef::Black,
            Self::Mountain => ManaKindDef::Red,
            Self::Forest => ManaKindDef::Green,
        }
    }

    #[must_use]
    pub const fn ability_text(self) -> &'static str {
        match self {
            Self::Plains => "{T}: Add {W}.",
            Self::Island => "{T}: Add {U}.",
            Self::Swamp => "{T}: Add {B}.",
            Self::Mountain => "{T}: Add {R}.",
            Self::Forest => "{T}: Add {G}.",
        }
    }

    #[must_use]
    pub const fn subtype(self) -> &'static str {
        match self {
            Self::Plains => "Plains",
            Self::Island => "Island",
            Self::Swamp => "Swamp",
            Self::Mountain => "Mountain",
            Self::Forest => "Forest",
        }
    }
}

/// Which kind of mana an effect adds. A choice is made as the mana ability
/// resolves; it is not modeled as several interchangeable colors already in
/// the pool.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManaSelectionDef {
    One(ManaKindDef),
    Choice(&'static [ManaKindDef]),
}

/// A restriction carried by produced mana until that mana is spent.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManaRestrictionDef {
    CastSpell(ObjectPredicateDef),
    CastCreatureSpellOfChosenType,
    ActivateAbility(ObjectPredicateDef),
    Special(&'static str),
}

/// An effect applied to the spell or ability paid for with a mana unit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManaSpendEffectDef {
    ApplyToPaidSpell(AppliedEffectDef),
    ApplyToPaidAbility(AppliedEffectDef),
    Special(&'static str),
}

/// One set of indistinguishable mana units created by an effect. The runtime
/// pool may store `amount` as a count keyed by the remaining fields.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AddManaEffectDef {
    pub mana: ManaSelectionDef,
    pub amount: u16,
    pub restrictions: &'static [ManaRestrictionDef],
    pub spend_effects: &'static [ManaSpendEffectDef],
}

impl AddManaEffectDef {
    #[must_use]
    pub const fn one(mana: ManaKindDef) -> Self {
        Self {
            mana: ManaSelectionDef::One(mana),
            amount: 1,
            restrictions: &[],
            spend_effects: &[],
        }
    }

    #[must_use]
    pub const fn choice(mana: &'static [ManaKindDef]) -> Self {
        Self {
            mana: ManaSelectionDef::Choice(mana),
            amount: 1,
            restrictions: &[],
            spend_effects: &[],
        }
    }

    #[must_use]
    pub const fn with_amount(mut self, amount: u16) -> Self {
        self.amount = amount;
        self
    }

    #[must_use]
    pub const fn with_restrictions(mut self, restrictions: &'static [ManaRestrictionDef]) -> Self {
        self.restrictions = restrictions;
        self
    }

    #[must_use]
    pub const fn with_spend_effects(
        mut self,
        spend_effects: &'static [ManaSpendEffectDef],
    ) -> Self {
        self.spend_effects = spend_effects;
        self
    }
}

/// A value evaluated from the resolving spell or ability and its captured
/// event. `SourcePower` and `SourceToughness` deliberately leave current-versus
/// last-known-information selection to the runtime source reference.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ValueDef {
    Constant(i32),
    ChosenX,
    SourcePower,
    SourceToughness,
    TriggerEventAmount,
    CardsInHandAbove {
        player: PlayerRelation,
        threshold: u8,
    },
}

/// An object or player affected by an effect. Targets are chosen when a spell
/// or stack ability is formed; triggering subjects come from captured events.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EffectRecipientDef {
    Source,
    Controller,
    Opponent,
    Target(TargetSlotId),
    TriggeringObject,
    /// The triggering object's controller when this effect resolves, using
    /// last-known information if that object is no longer live.
    ControllerOfTriggeringObject,
    /// The player named directly by the event, such as the player whose
    /// upkeep began or who cast the triggering spell.
    EventPlayer,
    MatchingObjects {
        object: ObjectPredicateDef,
        zones: &'static [ZoneKind],
        controller: PlayerRelation,
    },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EffectDurationDef {
    Permanent,
    UntilEndOfTurn,
    WhileSourceRemainsInZone,
    UntilSourceLeavesZone,
}

/// A continuous or rules-modifying effect applied to a game object.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AppliedEffectDef {
    CannotBeCountered,
    ModifyPowerToughness {
        power: ValueDef,
        toughness: ValueDef,
    },
    GrantAbility(&'static AbilityDef),
    Special(&'static str),
}

/// Declarative effect primitives interpreted by the rules engine.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EffectDef {
    None,
    Sequence(&'static [EffectDef]),
    AddMana(AddManaEffectDef),
    DealDamage {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    GainLife {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    DrawCards {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    LoseLife {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    Tap {
        object: EffectRecipientDef,
    },
    Destroy {
        object: EffectRecipientDef,
        can_regenerate: bool,
    },
    Sacrifice {
        object: EffectRecipientDef,
    },
    Counter {
        object: EffectRecipientDef,
    },
    AddPlusOneCounters {
        object: EffectRecipientDef,
        amount: ValueDef,
    },
    OptionalManaPayment {
        cost: ManaCost,
        effect: &'static EffectDef,
    },
    EntersTapped,
    MoveToZone {
        object: EffectRecipientDef,
        zone: ZoneKind,
    },
    Apply {
        recipient: EffectRecipientDef,
        effect: AppliedEffectDef,
        duration: EffectDurationDef,
    },
    /// A narrow escape hatch for effects that genuinely cannot be composed
    /// from shared primitives. The surrounding costs, targets, and timing can
    /// still remain declarative.
    Special(&'static str),
}

/// Turn structure used by beginning/end-of-step trigger declarations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TurnStepDef {
    Untap,
    Upkeep,
    Draw,
    PrecombatMain,
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    EndOfCombat,
    PostcombatMain,
    End,
    Cleanup,
}

/// The committed event observed by a triggered ability.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TriggerEventDef {
    ZoneChanged {
        object: ObjectPredicateDef,
        from: Option<ZoneKind>,
        to: Option<ZoneKind>,
    },
    BecomesTapped(ObjectPredicateDef),
    SpellCast(ObjectPredicateDef),
    AbilityActivated(ObjectPredicateDef),
    StepBegins {
        step: TurnStepDef,
        player: PlayerRelation,
    },
    DamageDealt {
        source: ObjectPredicateDef,
        recipient: EffectRecipientDef,
    },
    ManaAdded(PlayerRelation),
    /// A creature dealt damage by this ability's source this turn died.
    DamagedCreatureDied,
    Special(&'static str),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpellAbilityDef {
    pub targets: &'static [AbilityTargetDef],
}

impl SpellAbilityDef {
    #[must_use]
    pub const fn new() -> Self {
        Self { targets: &[] }
    }

    #[must_use]
    pub const fn with_targets(mut self, targets: &'static [AbilityTargetDef]) -> Self {
        self.targets = targets;
        self
    }
}

impl Default for SpellAbilityDef {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActivatedAbilityDef {
    pub source_zones: &'static [ZoneKind],
    pub costs: &'static [AbilityCostDef],
    pub targets: &'static [AbilityTargetDef],
}

impl ActivatedAbilityDef {
    #[must_use]
    pub const fn new(costs: &'static [AbilityCostDef]) -> Self {
        Self {
            source_zones: &[ZoneKind::Battlefield],
            costs,
            targets: &[],
        }
    }

    #[must_use]
    pub const fn with_source_zones(mut self, source_zones: &'static [ZoneKind]) -> Self {
        self.source_zones = source_zones;
        self
    }

    #[must_use]
    pub const fn with_targets(mut self, targets: &'static [AbilityTargetDef]) -> Self {
        self.targets = targets;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TriggeredAbilityDef {
    pub source_zones: &'static [ZoneKind],
    pub event: TriggerEventDef,
    pub targets: &'static [AbilityTargetDef],
}

impl TriggeredAbilityDef {
    #[must_use]
    pub const fn new(event: TriggerEventDef) -> Self {
        Self {
            source_zones: &[ZoneKind::Battlefield],
            event,
            targets: &[],
        }
    }

    #[must_use]
    pub const fn with_source_zones(mut self, source_zones: &'static [ZoneKind]) -> Self {
        self.source_zones = source_zones;
        self
    }

    #[must_use]
    pub const fn with_targets(mut self, targets: &'static [AbilityTargetDef]) -> Self {
        self.targets = targets;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StaticAbilityDef {
    pub source_zones: &'static [ZoneKind],
}

/// A replacement ability changes how an event happens and never uses the
/// stack. It is modeled separately from a triggered ability even when both
/// watch the same event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ReplacementAbilityDef {
    pub source_zones: &'static [ZoneKind],
}

impl ReplacementAbilityDef {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            source_zones: &[ZoneKind::Battlefield],
        }
    }
}

impl Default for ReplacementAbilityDef {
    fn default() -> Self {
        Self::new()
    }
}

/// A rules-defined action a player may take without using the stack, such as
/// turning a face-down permanent face up. This is deliberately distinct from
/// both activated abilities and mana abilities; its timing category is never
/// inferred from its cost or effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpecialActionDef {
    pub source_zones: &'static [ZoneKind],
    pub costs: &'static [AbilityCostDef],
}

impl SpecialActionDef {
    #[must_use]
    pub const fn new(source_zones: &'static [ZoneKind], costs: &'static [AbilityCostDef]) -> Self {
        Self {
            source_zones,
            costs,
        }
    }
}

impl StaticAbilityDef {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            source_zones: &[ZoneKind::Battlefield],
        }
    }

    #[must_use]
    pub const fn with_source_zones(mut self, source_zones: &'static [ZoneKind]) -> Self {
        self.source_zones = source_zones;
        self
    }
}

impl Default for StaticAbilityDef {
    fn default() -> Self {
        Self::new()
    }
}

/// A keyword ability carried as an ordinary, ordered rules clause.
///
/// The clause's [`AbilityImplementationDef`] says whether the engine currently
/// executes the keyword. This keeps unimplemented keywords such as first
/// strike visible and accurately reflected in aggregate coverage without
/// hiding them in card-level booleans.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EvergreenAbility {
    Flying,
    Trample,
    Haste,
    FirstStrike,
    DoubleStrike,
    Banding,
    Vigilance,
    Deathtouch,
    Lifelink,
    Reach,
    Flash,
    Hexproof,
    Intimidate,
    Undying,
    Mountainwalk,
    ProtectionFrom(ColorDef),
}

/// The rules category and structural procedure of an ability. Text, identity,
/// and implementation coverage live on [`AbilityDef`] so every printed clause
/// has one canonical text string regardless of how it executes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DeclarativeAbilityDef {
    Spell(SpellAbilityDef),
    ActivatedMana(ActivatedAbilityDef),
    TriggeredMana(TriggeredAbilityDef),
    Activated(ActivatedAbilityDef),
    Triggered(TriggeredAbilityDef),
    Static(StaticAbilityDef),
    Replacement(ReplacementAbilityDef),
    SpecialAction(SpecialActionDef),
    Evergreen(EvergreenAbility),
    /// Transitional structural marker for a clause still dispatched through
    /// the owning card's legacy custom behavior.
    Legacy,
}

/// How completely one printed rules clause is implemented.
///
/// Fully declarative clauses need no explanation: their structure and effect
/// are the implementation. Every other variant explains the escape hatch or
/// remaining gap beside the clause that owns it.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AbilityImplementationDef {
    Definition,
    CustomFull {
        behavior: Option<CardBehavior>,
        explanation: &'static str,
    },
    CustomPartial {
        behavior: Option<CardBehavior>,
        explanation: &'static str,
    },
    NotImplemented {
        explanation: &'static str,
    },
}

impl AbilityImplementationDef {
    #[must_use]
    pub const fn explanation(self) -> Option<&'static str> {
        match self {
            Self::Definition => None,
            Self::CustomFull { explanation, .. }
            | Self::CustomPartial { explanation, .. }
            | Self::NotImplemented { explanation } => Some(explanation),
        }
    }

    #[must_use]
    pub const fn custom_behavior(self) -> Option<CardBehavior> {
        match self {
            Self::CustomFull { behavior, .. } | Self::CustomPartial { behavior, .. } => behavior,
            Self::Definition | Self::NotImplemented { .. } => None,
        }
    }

    #[must_use]
    pub const fn with_custom_behavior(self, behavior: CardBehavior) -> Self {
        match self {
            Self::CustomFull { explanation, .. } => Self::CustomFull {
                behavior: Some(behavior),
                explanation,
            },
            Self::CustomPartial { explanation, .. } => Self::CustomPartial {
                behavior: Some(behavior),
                explanation,
            },
            Self::Definition | Self::NotImplemented { .. } => Self::CustomFull {
                behavior: Some(behavior),
                explanation: "Implemented by the named card-local special behavior.",
            },
        }
    }

    #[must_use]
    pub const fn is_executable(self) -> bool {
        !matches!(self, Self::NotImplemented { .. })
    }
}

/// One printed rules clause and its implementation.
///
/// The category is explicit even when the implementation remains custom; the
/// engine never infers stack behavior from costs, targets, or effects.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AbilityDef {
    pub id: AbilityId,
    pub text: &'static str,
    /// Optional action-menu wording for a targeted activated ability. This is
    /// presentation attached to the exact ability, not a second rules text.
    pub activation_text: Option<ActivatedAbilityText>,
    pub definition: DeclarativeAbilityDef,
    pub effect: EffectDef,
    pub implementation: AbilityImplementationDef,
}

impl AbilityDef {
    const LEGACY_EXPLANATION: &'static str =
        "Legacy aggregate rules text has not been assigned an implementation.";

    #[must_use]
    pub const fn spell(id: AbilityId, text: &'static str, effect: EffectDef) -> Self {
        Self::defined(
            id,
            text,
            DeclarativeAbilityDef::Spell(SpellAbilityDef::new()),
            effect,
        )
    }

    #[must_use]
    pub const fn activated_mana(
        id: AbilityId,
        text: &'static str,
        costs: &'static [AbilityCostDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            id,
            text,
            DeclarativeAbilityDef::ActivatedMana(ActivatedAbilityDef::new(costs)),
            effect,
        )
    }

    #[must_use]
    pub const fn triggered_mana(
        id: AbilityId,
        text: &'static str,
        event: TriggerEventDef,
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            id,
            text,
            DeclarativeAbilityDef::TriggeredMana(TriggeredAbilityDef::new(event)),
            effect,
        )
    }

    #[must_use]
    pub const fn activated(
        id: AbilityId,
        text: &'static str,
        costs: &'static [AbilityCostDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            id,
            text,
            DeclarativeAbilityDef::Activated(ActivatedAbilityDef::new(costs)),
            effect,
        )
    }

    #[must_use]
    pub const fn triggered(
        id: AbilityId,
        text: &'static str,
        event: TriggerEventDef,
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            id,
            text,
            DeclarativeAbilityDef::Triggered(TriggeredAbilityDef::new(event)),
            effect,
        )
    }

    #[must_use]
    pub const fn static_ability(id: AbilityId, text: &'static str, effect: EffectDef) -> Self {
        Self::defined(
            id,
            text,
            DeclarativeAbilityDef::Static(StaticAbilityDef::new()),
            effect,
        )
    }

    #[must_use]
    pub const fn evergreen(id: AbilityId, text: &'static str, ability: EvergreenAbility) -> Self {
        Self::defined(
            id,
            text,
            DeclarativeAbilityDef::Evergreen(ability),
            EffectDef::None,
        )
    }

    #[must_use]
    pub const fn replacement(id: AbilityId, text: &'static str, effect: EffectDef) -> Self {
        Self::defined(
            id,
            text,
            DeclarativeAbilityDef::Replacement(ReplacementAbilityDef::new()),
            effect,
        )
    }

    #[must_use]
    pub const fn special_action(
        id: AbilityId,
        text: &'static str,
        source_zones: &'static [ZoneKind],
        costs: &'static [AbilityCostDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            id,
            text,
            DeclarativeAbilityDef::SpecialAction(SpecialActionDef::new(source_zones, costs)),
            effect,
        )
    }

    #[must_use]
    pub const fn defined(
        id: AbilityId,
        text: &'static str,
        definition: DeclarativeAbilityDef,
        effect: EffectDef,
    ) -> Self {
        Self {
            id,
            text,
            activation_text: None,
            definition,
            effect,
            implementation: AbilityImplementationDef::Definition,
        }
    }

    #[must_use]
    pub const fn custom_full(
        id: AbilityId,
        text: &'static str,
        behavior: CardBehavior,
        explanation: &'static str,
    ) -> Self {
        Self {
            id,
            text,
            activation_text: None,
            definition: DeclarativeAbilityDef::Legacy,
            effect: EffectDef::None,
            implementation: AbilityImplementationDef::CustomFull {
                behavior: Some(behavior),
                explanation,
            },
        }
    }

    #[must_use]
    pub const fn legacy(id: AbilityId, text: &'static str) -> Self {
        Self {
            id,
            text,
            activation_text: None,
            definition: DeclarativeAbilityDef::Legacy,
            effect: EffectDef::None,
            implementation: AbilityImplementationDef::NotImplemented {
                explanation: Self::LEGACY_EXPLANATION,
            },
        }
    }

    #[must_use]
    pub const fn custom_partial(
        id: AbilityId,
        text: &'static str,
        behavior: CardBehavior,
        explanation: &'static str,
    ) -> Self {
        Self {
            id,
            text,
            activation_text: None,
            definition: DeclarativeAbilityDef::Legacy,
            effect: EffectDef::None,
            implementation: AbilityImplementationDef::CustomPartial {
                behavior: Some(behavior),
                explanation,
            },
        }
    }

    #[must_use]
    pub const fn not_implemented(
        id: AbilityId,
        text: &'static str,
        explanation: &'static str,
    ) -> Self {
        Self {
            id,
            text,
            activation_text: None,
            definition: DeclarativeAbilityDef::Legacy,
            effect: EffectDef::None,
            implementation: AbilityImplementationDef::NotImplemented { explanation },
        }
    }

    #[must_use]
    pub const fn with_implementation(mut self, implementation: AbilityImplementationDef) -> Self {
        self.implementation = implementation;
        self
    }

    #[must_use]
    pub const fn with_activation_text(
        mut self,
        targeted: &'static str,
        summary: &'static str,
    ) -> Self {
        self.activation_text = Some(ActivatedAbilityText { targeted, summary });
        self
    }

    #[must_use]
    pub const fn with_targets(mut self, targets: &'static [AbilityTargetDef]) -> Self {
        match &mut self.definition {
            DeclarativeAbilityDef::Spell(definition) => definition.targets = targets,
            DeclarativeAbilityDef::ActivatedMana(definition)
            | DeclarativeAbilityDef::Activated(definition) => definition.targets = targets,
            DeclarativeAbilityDef::TriggeredMana(definition)
            | DeclarativeAbilityDef::Triggered(definition) => definition.targets = targets,
            DeclarativeAbilityDef::Static(_)
            | DeclarativeAbilityDef::Replacement(_)
            | DeclarativeAbilityDef::SpecialAction(_)
            | DeclarativeAbilityDef::Evergreen(_)
            | DeclarativeAbilityDef::Legacy => {}
        }
        self
    }

    #[must_use]
    pub const fn with_source_zones(mut self, source_zones: &'static [ZoneKind]) -> Self {
        match &mut self.definition {
            DeclarativeAbilityDef::ActivatedMana(definition)
            | DeclarativeAbilityDef::Activated(definition) => {
                definition.source_zones = source_zones;
            }
            DeclarativeAbilityDef::TriggeredMana(definition)
            | DeclarativeAbilityDef::Triggered(definition) => {
                definition.source_zones = source_zones;
            }
            DeclarativeAbilityDef::Static(definition) => {
                definition.source_zones = source_zones;
            }
            DeclarativeAbilityDef::Replacement(definition) => {
                definition.source_zones = source_zones;
            }
            DeclarativeAbilityDef::SpecialAction(definition) => {
                definition.source_zones = source_zones;
            }
            DeclarativeAbilityDef::Spell(_)
            | DeclarativeAbilityDef::Evergreen(_)
            | DeclarativeAbilityDef::Legacy => {}
        }
        self
    }

    #[must_use]
    pub const fn uses_stack(self) -> bool {
        matches!(
            self.definition,
            DeclarativeAbilityDef::Spell(_)
                | DeclarativeAbilityDef::Activated(_)
                | DeclarativeAbilityDef::Triggered(_)
        )
    }
}

/// Returns the fully declarative mana ability intrinsically granted by one
/// basic land subtype. Its runtime origin, rather than this local ID, is what
/// distinguishes multiple intrinsic abilities on a dual land.
#[must_use]
pub const fn intrinsic_basic_land_mana_ability(land_type: BasicLandType) -> AbilityDef {
    AbilityDef::activated_mana(
        AbilityId::PRIMARY,
        land_type.ability_text(),
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::one(land_type.mana_kind())),
    )
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
        Self::cast_with_printed_mana_cost(
            id,
            label,
            form,
            PrintedManaCost::Cost(mana_cost),
            effect_status,
        )
    }

    /// Defines a cast action without collapsing a nonexistent printed cost
    /// into `{0}`. A spell with `PrintedManaCost::None` ordinarily needs a
    /// separate casting permission or alternative cost before it is legal.
    #[must_use]
    pub fn cast_with_printed_mana_cost(
        id: PlayOptionId,
        label: impl Into<String>,
        form: SpellForm,
        printed_mana_cost: PrintedManaCost,
        effect_status: CardEffectStatus,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            action: PlayActionKind::CastSpell,
            form,
            mana_cost: printed_mana_cost.as_option(),
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
        let printed_mana_cost = if rules.kind == CardKind::Land {
            PrintedManaCost::None
        } else {
            PrintedManaCost::Cost(rules.mana_cost)
        };
        Self::single_with_printed_mana_cost(name, rules, printed_mana_cost)
    }

    /// Builds an ordinary card whose printed-cost characteristic is explicit.
    /// This is required for nonland cards with no mana cost and keeps them
    /// distinct from cards with a printed `{0}` cost.
    #[must_use]
    pub fn single_with_printed_mana_cost(
        name: impl Into<String>,
        rules: CardRules,
        printed_mana_cost: PrintedManaCost,
    ) -> Self {
        let name = name.into();
        let is_land = rules.kind == CardKind::Land;
        let effect_status = match rules.implementation_status() {
            ImplementationStatus::MetadataOnly => CardEffectStatus::MetadataOnly,
            ImplementationStatus::Complete | ImplementationStatus::Partial => {
                CardEffectStatus::Implemented
            }
        };
        let part = CardPart::with_printed_mana_cost(
            CardPartId::PRIMARY,
            name.clone(),
            rules,
            printed_mana_cost,
        );
        let option = if is_land {
            PlayOptionDef::play_land(
                PlayOptionId::DEFAULT,
                name,
                CardPartId::PRIMARY,
                effect_status,
            )
        } else {
            PlayOptionDef::cast_with_printed_mana_cost(
                PlayOptionId::DEFAULT,
                name,
                SpellForm::Part(CardPartId::PRIMARY),
                printed_mana_cost,
                effect_status,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CardArt {
    pub scryfall_id: &'static str,
    pub artist: &'static str,
}

impl CardArt {
    #[must_use]
    pub const fn new(scryfall_id: &'static str, artist: &'static str) -> Self {
        Self {
            scryfall_id,
            artist,
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
        let rules = if is_basic_land {
            (*behavior.rules()).with_supertype(CardSupertype::Basic)
        } else {
            *behavior.rules()
        };
        let composition = CardComposition::single(name.clone(), rules);
        Self {
            id,
            name,
            art: None,
            set,
            printings: vec![CardPrinting::new(id, set)],
            rules,
            parts: composition.parts,
            structure: composition.structure,
            play_options: composition.play_options,
        }
    }

    #[must_use]
    pub const fn is_basic_land(&self) -> bool {
        matches!(self.rules.kind, CardKind::Land) && self.rules.has_supertype(CardSupertype::Basic)
    }

    #[must_use]
    pub fn part(&self, id: CardPartId) -> Option<&CardPart> {
        self.parts.iter().find(|part| part.id == id)
    }

    #[must_use]
    pub fn play_option(&self, id: PlayOptionId) -> Option<&PlayOptionDef> {
        self.play_options.iter().find(|option| option.id == id)
    }

    /// Derives card-level coverage from every ordered clause on every part.
    /// A mix of complete and unimplemented parts is partial; a card is
    /// metadata-only only when every represented clause is unimplemented.
    #[must_use]
    pub fn implementation_status(&self) -> ImplementationStatus {
        let mut statuses = self
            .parts
            .iter()
            .map(|part| part.rules.implementation_status());
        statuses
            .next()
            .map_or(ImplementationStatus::Complete, |first| {
                statuses.fold(first, ImplementationStatus::combine)
            })
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
    ArgothianPixies,
    Atog,
    Balance,
    Berserk,
    BlackVise,
    BlueElementalBlast,
    BloodMoon,
    ChainLightning,
    Channel,
    ChaosOrb,
    CityInABottle,
    CopyArtifact,
    Crusade,
    DemonicTutor,
    Detonate,
    DivineOffering,
    DrainLife,
    DragonWhelp,
    DustToDust,
    Earthquake,
    ErhnamDjinn,
    Fireball,
    Fork,
    GiantGrowth,
    GlassesOfUrza,
    GoblinGrenade,
    HurkylsRecall,
    HymnToTourach,
    HypnoticSpecter,
    IcyManipulator,
    IcatianJavelineers,
    IronclawOrcs,
    Juggernaut,
    KirdApe,
    LibraryOfAlexandria,
    ManaDrain,
    ManaVault,
    MazeOfIth,
    MindTwist,
    Moat,
    NevinyrralsDisk,
    Pendelhaven,
    Recall,
    Regrowth,
    SedgeTroll,
    SylvanLibrary,
    Terror,
    TimeVault,
    Timetwister,
    FellwarStone,
    LightningBolt,
    MishrasFactory,
    OrcishMechanics,
    RedElementalBlast,
    Smoke,
    StoneGiant,
    SwordsToPlowshares,
    TimeWalk,
    Triskelion,
    Tetravus,
    TheAbyss,
    WheelOfFortune,
    WhirlingDervish,
    WinterOrb,
    // Compatibility rules keys retained while CardDefinition::new still
    // accepts CardBehavior instead of CardRules directly.
    Mountain,
    Plains,
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardSupertype {
    Basic,
    Legendary,
    Snow,
    World,
}

impl CardSupertype {
    const COUNT: usize = 4;

    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::Basic => 0,
            Self::Legendary => 1,
            Self::Snow => 2,
            Self::World => 3,
        }
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Basic => "Basic",
            Self::Legendary => "Legendary",
            Self::Snow => "Snow",
            Self::World => "World",
        }
    }
}

/// How completely the engine implements a card or independently modeled part.
///
/// Ordinary construction defaults to [`Self::Complete`]. Explanations live on
/// the non-declarative clause implementations that caused a non-complete
/// aggregate status, rather than being duplicated at card level.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ImplementationStatus {
    #[default]
    Complete,
    Partial,
    MetadataOnly,
}

impl ImplementationStatus {
    #[must_use]
    pub const fn combine(self, other: Self) -> Self {
        match (self, other) {
            (Self::Partial, _)
            | (_, Self::Partial)
            | (Self::Complete, Self::MetadataOnly)
            | (Self::MetadataOnly, Self::Complete) => Self::Partial,
            (Self::MetadataOnly, Self::MetadataOnly) => Self::MetadataOnly,
            (Self::Complete, Self::Complete) => Self::Complete,
        }
    }

    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Whether the current game engine may execute an independently modeled play
/// option or mode. Ordinary single-card options derive this gate from their
/// clause-local [`ImplementationStatus`] instead of storing another status on
/// [`CardRules`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardEffectStatus {
    Implemented,
    MetadataOnly,
}

impl CardKind {
    #[must_use]
    pub const fn type_name(self) -> &'static str {
        match self {
            Self::Land => "Land",
            Self::Creature => "Creature",
            Self::Artifact => "Artifact",
            Self::ArtifactCreature => "Artifact Creature",
            Self::Enchantment => "Enchantment",
            Self::Planeswalker => "Planeswalker",
            Self::Instant => "Instant",
            Self::Sorcery => "Sorcery",
        }
    }

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
    /// Mana value with each `{X}` treated as zero.
    #[must_use]
    pub const fn mana_value(self) -> u16 {
        self.generic
            .saturating_add(self.white)
            .saturating_add(self.blue)
            .saturating_add(self.black)
            .saturating_add(self.red)
            .saturating_add(self.green)
            .saturating_add(self.white_red_hybrid)
    }

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
}

/// How a client should describe activating a permanent's targeted ability.
///
/// `targeted` is a template with `{}` where the target's name goes, so a menu
/// can name the effect instead of the card; `summary` is the same effect with
/// no particular target picked yet.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActivatedAbilityText {
    pub targeted: &'static str,
    pub summary: &'static str,
}

/// Const-friendly storage for the ordered rules clauses of one card part.
///
/// Most legacy records begin as one custom clause. Migrated cards use an
/// inline promoted slice, preserving source order without heap allocation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
// `One` keeps legacy card construction const-friendly without preserving a
// second card-level rules-text field. Migrated records use the slice variant.
#[allow(clippy::large_enum_variant)]
pub enum CardAbilityList {
    None,
    One(AbilityDef),
    Many(&'static [AbilityDef]),
}

impl CardAbilityList {
    #[must_use]
    pub fn as_slice(&self) -> &[AbilityDef] {
        match self {
            Self::None => &[],
            Self::One(ability) => std::slice::from_ref(ability),
            Self::Many(abilities) => abilities,
        }
    }
}

/// Declarative rules metadata kept beside a card's catalog identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct CardRules {
    pub kind: CardKind,
    pub supertypes: [bool; CardSupertype::COUNT],
    pub subtypes: &'static [&'static str],
    pub mana_cost: ManaCost,
    pub alternate_mana_costs: &'static [AlternateManaCost],
    pub land_entry: LandEntry,
    pub starting_loyalty: Option<u16>,
    pub creature_stats: Option<CreatureStats>,
    /// Ordered printed rules clauses. Definitions repeat abilities granted by
    /// basic land subtypes so a card's behavior is visible in one place.
    pub abilities: CardAbilityList,
    /// Printed colors in `[white, blue, black, red, green]` order.
    pub colors: [bool; 5],
}

impl CardRules {
    #[must_use]
    pub const fn new(kind: CardKind, mana_cost: ManaCost, text: &'static str) -> Self {
        Self {
            kind,
            supertypes: [false; CardSupertype::COUNT],
            subtypes: &[],
            mana_cost,
            alternate_mana_costs: &[],
            land_entry: LandEntry::Untapped,
            starting_loyalty: None,
            creature_stats: None,
            abilities: if text.is_empty() {
                CardAbilityList::None
            } else {
                CardAbilityList::One(AbilityDef::legacy(AbilityId::PRIMARY, text))
            },
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
    ///
    /// # Panics
    ///
    /// Panics unless this rule set contains the one legacy aggregate clause
    /// created from `CardRules::new` text. Explicit clause lists must declare
    /// implementation coverage on each clause instead.
    #[must_use]
    pub const fn metadata_only(mut self) -> Self {
        self.abilities = match self.abilities {
            CardAbilityList::None => panic!(
                "metadata_only() requires legacy rules text; explicit clauses own their coverage"
            ),
            CardAbilityList::One(ability) => CardAbilityList::One(ability.with_implementation(
                AbilityImplementationDef::NotImplemented {
                    explanation: "Printed rules are cataloged but are not executed by the engine.",
                },
            )),
            CardAbilityList::Many(_) => panic!(
                "metadata_only() cannot follow with_abilities(); set implementation coverage on each clause"
            ),
        };
        self
    }

    #[must_use]
    pub const fn with_abilities(mut self, abilities: &'static [AbilityDef]) -> Self {
        self.abilities = if abilities.is_empty() {
            CardAbilityList::None
        } else {
            CardAbilityList::Many(abilities)
        };
        self
    }

    /// Compatibility bridge for legacy one-clause records. New definitions
    /// put the behavior directly in the implementation of the clause it
    /// executes.
    ///
    /// # Panics
    ///
    /// Panics unless exactly one custom clause can own the behavior.
    #[must_use]
    pub const fn with_special_behavior(mut self, behavior: CardBehavior) -> Self {
        self.abilities = match self.abilities {
            CardAbilityList::One(mut ability) => {
                ability.implementation = ability.implementation.with_custom_behavior(behavior);
                CardAbilityList::One(ability)
            }
            CardAbilityList::Many(abilities) if abilities.len() == 1 => {
                let mut ability = abilities[0];
                ability.implementation = ability.implementation.with_custom_behavior(behavior);
                CardAbilityList::One(ability)
            }
            CardAbilityList::None | CardAbilityList::Many(_) => {
                panic!("with_special_behavior() requires exactly one custom ability clause")
            }
        };
        self
    }

    #[must_use]
    pub fn special_behavior(&self) -> Option<CardBehavior> {
        self.ability_clauses()
            .iter()
            .find_map(|ability| ability.implementation.custom_behavior())
    }

    /// Marks the part's one legacy aggregate clause as only partially
    /// implemented. Migrated multi-clause cards put coverage on each clause.
    ///
    /// # Panics
    ///
    /// Panics unless this rule set contains the one legacy aggregate clause
    /// created from `CardRules::new` text. Explicit clause lists must declare
    /// implementation coverage on each clause instead.
    #[must_use]
    pub const fn partial(mut self, explanation: &'static str) -> Self {
        self.abilities = match self.abilities {
            CardAbilityList::One(ability) => CardAbilityList::One(ability.with_implementation(
                AbilityImplementationDef::CustomPartial {
                    behavior: ability.implementation.custom_behavior(),
                    explanation,
                },
            )),
            CardAbilityList::None => {
                panic!("partial() requires legacy rules text; explicit clauses own their coverage")
            }
            CardAbilityList::Many(_) => panic!(
                "partial() cannot follow with_abilities(); set implementation coverage on each clause"
            ),
        };
        self
    }

    #[must_use]
    pub fn ability_clauses(&self) -> &[AbilityDef] {
        self.abilities.as_slice()
    }

    /// Renders the ordered card text from the same clauses used by execution
    /// and implementation auditing.
    #[must_use]
    pub fn rules_text(&self) -> std::borrow::Cow<'static, str> {
        match self.abilities {
            CardAbilityList::None => std::borrow::Cow::Borrowed(""),
            CardAbilityList::One(ability) => std::borrow::Cow::Borrowed(ability.text),
            CardAbilityList::Many(abilities) => std::borrow::Cow::Owned(
                abilities
                    .iter()
                    .map(|ability| ability.text)
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        }
    }

    #[must_use]
    pub fn implementation_status(&self) -> ImplementationStatus {
        // Playing a land and casting/using a modeled creature body are shared,
        // executable rules even when every card-specific clause is deferred.
        let mut has_full = self.kind == CardKind::Land || self.creature_stats.is_some();
        let mut has_partial = false;
        let mut has_unimplemented = false;
        for ability in self.ability_clauses() {
            match ability.implementation {
                AbilityImplementationDef::Definition
                | AbilityImplementationDef::CustomFull { .. } => has_full = true,
                AbilityImplementationDef::CustomPartial { .. } => has_partial = true,
                AbilityImplementationDef::NotImplemented { .. } => has_unimplemented = true,
            }
        }
        if has_partial || (has_full && has_unimplemented) {
            ImplementationStatus::Partial
        } else if has_unimplemented {
            ImplementationStatus::MetadataOnly
        } else {
            ImplementationStatus::Complete
        }
    }

    #[must_use]
    pub const fn with_supertype(mut self, supertype: CardSupertype) -> Self {
        self.supertypes[supertype.index()] = true;
        self
    }

    #[must_use]
    pub const fn with_subtypes(mut self, subtypes: &'static [&'static str]) -> Self {
        self.subtypes = subtypes;
        self
    }

    #[must_use]
    pub const fn has_supertype(&self, supertype: CardSupertype) -> bool {
        self.supertypes[supertype.index()]
    }

    #[must_use]
    pub fn has_subtype(&self, subtype: &str) -> bool {
        self.subtypes.contains(&subtype)
    }

    #[must_use]
    pub fn type_line(&self) -> String {
        let mut words = [
            CardSupertype::Basic,
            CardSupertype::Legendary,
            CardSupertype::Snow,
            CardSupertype::World,
        ]
        .into_iter()
        .filter(|supertype| self.has_supertype(*supertype))
        .map(CardSupertype::name)
        .collect::<Vec<_>>();
        words.push(self.kind.type_name());
        let mut line = words.join(" ");
        if !self.subtypes.is_empty() {
            line.push_str(" — ");
            line.push_str(&self.subtypes.join(" "));
        }
        line
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
        self.creature_stats = Some(CreatureStats { power, toughness });
        self
    }

    #[must_use]
    pub fn has_evergreen(&self, expected: EvergreenAbility) -> bool {
        self.ability_clauses().iter().any(|ability| {
            ability.implementation.is_executable()
                && matches!(ability.definition, DeclarativeAbilityDef::Evergreen(actual) if actual == expected)
        })
    }

    pub(super) const fn unsupported() -> Self {
        let mut rules = Self::new(
            CardKind::Artifact,
            ManaCost::new(u16::MAX, u16::MAX),
            "Rules text is not implemented.",
        )
        .metadata_only();
        rules.colors = [false; 5];
        rules
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AbilityCostDef, AbilityDef, AddManaEffectDef, AlternateManaCost, CardBehavior,
        CardComposition, CardDefinition, CardEffectStatus, CardKind, CardPart, CardPrinting,
        CardPrintingId, CardRules, CardSet, EffectDef, ImplementationStatus, ManaCost, ManaKindDef,
        ManaRestrictionDef, ObjectPredicateDef, PrintedManaCost, TriggerEventDef,
    };
    use crate::{AbilityId, CardDefinitionId, CardPartId};

    static DEFERRED_CLAUSE: [AbilityDef; 1] = [AbilityDef::not_implemented(
        AbilityId::PRIMARY,
        "A deferred card-specific ability.",
        "The card-specific ability is not executed.",
    )];

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
    fn clause_implementation_drives_the_ordinary_play_option_gate() {
        let implemented = CardRules::new(CardKind::Instant, ManaCost::default(), "");
        assert_eq!(
            ImplementationStatus::default(),
            ImplementationStatus::Complete
        );
        assert_eq!(
            CardComposition::single("Implemented", implemented).play_options[0].effect_status,
            CardEffectStatus::Implemented
        );

        let uncategorized = CardRules::new(
            CardKind::Instant,
            ManaCost::default(),
            "Legacy text with no assigned implementation.",
        );
        assert_eq!(
            uncategorized.implementation_status(),
            ImplementationStatus::MetadataOnly
        );
        let custom = CardRules::new(
            CardKind::Instant,
            ManaCost::default(),
            "A card-local effect.",
        )
        .with_special_behavior(CardBehavior::LightningBolt);
        assert_eq!(
            custom.implementation_status(),
            ImplementationStatus::Complete
        );
        assert_eq!(custom.special_behavior(), Some(CardBehavior::LightningBolt));

        let metadata_only = CardRules::new(
            CardKind::Instant,
            ManaCost::default(),
            "A deferred spell effect.",
        )
        .metadata_only();
        assert_eq!(
            metadata_only.implementation_status(),
            ImplementationStatus::MetadataOnly
        );
        assert_eq!(
            CardComposition::single("Deferred", metadata_only).play_options[0].effect_status,
            CardEffectStatus::MetadataOnly
        );
        let metadata_definition = CardDefinition::new(
            CardDefinitionId(8),
            "Unsupported",
            CardSet::Alpha,
            false,
            CardBehavior::Unsupported,
        );
        assert_eq!(
            metadata_definition.implementation_status(),
            ImplementationStatus::MetadataOnly
        );

        let partial = CardRules::new(
            CardKind::Enchantment,
            ManaCost::default(),
            "A custom clause with one deferred rider.",
        )
        .partial("One rider is deferred.");
        assert_eq!(
            partial.ability_clauses()[0].implementation.explanation(),
            Some("One rider is deferred.")
        );
        assert_eq!(
            partial.implementation_status(),
            ImplementationStatus::Partial
        );
        assert_eq!(
            CardComposition::single("Partial", partial).play_options[0].effect_status,
            CardEffectStatus::Implemented
        );
    }

    #[test]
    fn vanilla_creature_body_is_complete() {
        let rules = CardRules::new(CardKind::Creature, ManaCost::default(), "").creature(2, 2);

        assert_eq!(
            rules.implementation_status(),
            ImplementationStatus::Complete
        );
    }

    #[test]
    fn creature_body_with_an_unimplemented_clause_is_partial() {
        let rules = CardRules::new(CardKind::Creature, ManaCost::default(), "")
            .creature(2, 2)
            .with_abilities(&DEFERRED_CLAUSE);

        assert_eq!(rules.implementation_status(), ImplementationStatus::Partial);
        assert_eq!(
            CardComposition::single("Partial creature", rules).play_options[0].effect_status,
            CardEffectStatus::Implemented
        );
    }

    #[test]
    fn noncreature_with_only_an_unimplemented_clause_is_metadata_only() {
        let rules = CardRules::new(CardKind::Enchantment, ManaCost::default(), "")
            .with_abilities(&DEFERRED_CLAUSE);

        assert_eq!(
            rules.implementation_status(),
            ImplementationStatus::MetadataOnly
        );
        assert_eq!(
            CardComposition::single("Deferred enchantment", rules).play_options[0].effect_status,
            CardEffectStatus::MetadataOnly
        );
    }

    #[test]
    #[should_panic(expected = "metadata_only() cannot follow with_abilities()")]
    fn metadata_only_rejects_explicit_clause_lists() {
        let _ = CardRules::new(CardKind::Enchantment, ManaCost::default(), "")
            .with_abilities(&DEFERRED_CLAUSE)
            .metadata_only();
    }

    #[test]
    #[should_panic(expected = "metadata_only() requires legacy rules text")]
    fn metadata_only_rejects_missing_legacy_clause() {
        let _ = CardRules::new(CardKind::Enchantment, ManaCost::default(), "").metadata_only();
    }

    #[test]
    #[should_panic(expected = "partial() cannot follow with_abilities()")]
    fn partial_rejects_explicit_clause_lists() {
        let _ = CardRules::new(CardKind::Enchantment, ManaCost::default(), "")
            .with_abilities(&DEFERRED_CLAUSE)
            .partial("This explanation belongs on the clause.");
    }

    #[test]
    #[should_panic(expected = "partial() requires legacy rules text")]
    fn partial_rejects_missing_legacy_clause() {
        let _ = CardRules::new(CardKind::Enchantment, ManaCost::default(), "")
            .partial("There is no clause to receive this explanation.");
    }

    #[test]
    fn no_mana_cost_is_distinct_from_a_printed_zero_cost() {
        let rules = CardRules::new(CardKind::Sorcery, ManaCost::default(), "");
        let zero = CardPart::new(CardPartId::PRIMARY, "Zero", rules);
        let none = CardPart::with_printed_mana_cost(
            CardPartId::PRIMARY,
            "None",
            rules,
            PrintedManaCost::None,
        );

        assert_eq!(
            zero.printed_mana_cost(),
            PrintedManaCost::Cost(ManaCost::default())
        );
        assert_eq!(none.printed_mana_cost(), PrintedManaCost::None);
        assert_eq!(zero.printed_mana_cost().mana_value(), 0);
        assert_eq!(none.printed_mana_cost().mana_value(), 0);

        let composition = CardComposition::single_with_printed_mana_cost(
            "No-cost spell",
            rules,
            PrintedManaCost::None,
        );
        assert_eq!(composition.parts[0].mana_cost, None);
        assert_eq!(composition.play_options[0].mana_cost, None);
    }

    #[test]
    fn ability_category_is_explicit_and_not_inferred_from_effect() {
        const COSTS: &[AbilityCostDef] = &[AbilityCostDef::TapSource];
        const ADD_MANA: EffectDef = EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Green));
        const MANA_ABILITY: AbilityDef =
            AbilityDef::activated_mana(AbilityId::PRIMARY, "Add green.", COSTS, ADD_MANA);
        const ORDINARY_TRIGGER: AbilityDef = AbilityDef::triggered(
            AbilityId(1),
            "Add green when this dies.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(super::ZoneKind::Battlefield),
                to: Some(super::ZoneKind::Graveyard),
            },
            ADD_MANA,
        );
        const TURN_FACE_UP: AbilityDef = AbilityDef::special_action(
            AbilityId(2),
            "Turn this face up.",
            &[super::ZoneKind::Battlefield],
            &[AbilityCostDef::Mana(ManaCost::new(3, 0))],
            EffectDef::Special("turn face up"),
        );
        static ABILITIES: [AbilityDef; 3] = [MANA_ABILITY, ORDINARY_TRIGGER, TURN_FACE_UP];

        assert!(!MANA_ABILITY.uses_stack());
        assert!(ORDINARY_TRIGGER.uses_stack());
        assert!(!TURN_FACE_UP.uses_stack());

        let rules =
            CardRules::new(CardKind::Creature, ManaCost::default(), "").with_abilities(&ABILITIES);
        assert_eq!(rules.ability_clauses()[0].id, AbilityId::PRIMARY);
        assert_eq!(rules.ability_clauses()[1].id, AbilityId(1));
        assert_eq!(rules.ability_clauses()[2].id, AbilityId(2));
    }

    #[test]
    fn mana_effects_keep_restrictions_attached_to_each_counted_unit() {
        const RESTRICTIONS: &[ManaRestrictionDef] =
            &[ManaRestrictionDef::CastSpell(ObjectPredicateDef::Artifact)];
        let workshop_mana = AddManaEffectDef::one(ManaKindDef::Colorless)
            .with_amount(3)
            .with_restrictions(RESTRICTIONS);

        assert_eq!(workshop_mana.amount, 3);
        assert_eq!(workshop_mana.restrictions, RESTRICTIONS);
    }
}
