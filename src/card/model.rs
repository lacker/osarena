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
    TriggeringPlayer,
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
    TriggeringPlayer,
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
    Special(&'static str),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpellAbilityDef {
    pub id: AbilityId,
    pub text: &'static str,
    pub targets: &'static [AbilityTargetDef],
    pub effect: EffectDef,
}

impl SpellAbilityDef {
    #[must_use]
    pub const fn new(id: AbilityId, text: &'static str, effect: EffectDef) -> Self {
        Self {
            id,
            text,
            targets: &[],
            effect,
        }
    }

    #[must_use]
    pub const fn with_targets(mut self, targets: &'static [AbilityTargetDef]) -> Self {
        self.targets = targets;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActivatedAbilityDef {
    pub id: AbilityId,
    pub text: &'static str,
    pub source_zones: &'static [ZoneKind],
    pub costs: &'static [AbilityCostDef],
    pub targets: &'static [AbilityTargetDef],
    pub effect: EffectDef,
}

impl ActivatedAbilityDef {
    #[must_use]
    pub const fn new(
        id: AbilityId,
        text: &'static str,
        costs: &'static [AbilityCostDef],
        effect: EffectDef,
    ) -> Self {
        Self {
            id,
            text,
            source_zones: &[ZoneKind::Battlefield],
            costs,
            targets: &[],
            effect,
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
    pub id: AbilityId,
    pub text: &'static str,
    pub source_zones: &'static [ZoneKind],
    pub event: TriggerEventDef,
    pub targets: &'static [AbilityTargetDef],
    pub effect: EffectDef,
}

impl TriggeredAbilityDef {
    #[must_use]
    pub const fn new(
        id: AbilityId,
        text: &'static str,
        event: TriggerEventDef,
        effect: EffectDef,
    ) -> Self {
        Self {
            id,
            text,
            source_zones: &[ZoneKind::Battlefield],
            event,
            targets: &[],
            effect,
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
    pub id: AbilityId,
    pub text: &'static str,
    pub source_zones: &'static [ZoneKind],
    pub effect: EffectDef,
}

/// A rules-defined action a player may take without using the stack, such as
/// turning a face-down permanent face up. This is deliberately distinct from
/// both activated abilities and mana abilities; its timing category is never
/// inferred from its cost or effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpecialActionDef {
    pub id: AbilityId,
    pub text: &'static str,
    pub source_zones: &'static [ZoneKind],
    pub costs: &'static [AbilityCostDef],
    pub effect: EffectDef,
}

impl SpecialActionDef {
    #[must_use]
    pub const fn new(
        id: AbilityId,
        text: &'static str,
        source_zones: &'static [ZoneKind],
        costs: &'static [AbilityCostDef],
        effect: EffectDef,
    ) -> Self {
        Self {
            id,
            text,
            source_zones,
            costs,
            effect,
        }
    }
}

impl StaticAbilityDef {
    #[must_use]
    pub const fn new(id: AbilityId, text: &'static str, effect: EffectDef) -> Self {
        Self {
            id,
            text,
            source_zones: &[ZoneKind::Battlefield],
            effect,
        }
    }

    #[must_use]
    pub const fn with_source_zones(mut self, source_zones: &'static [ZoneKind]) -> Self {
        self.source_zones = source_zones;
        self
    }
}

/// The rules category of an ability is explicit. In particular, adding mana
/// does not cause the engine to infer that an ability is a mana ability.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AbilityDef {
    Spell(SpellAbilityDef),
    ActivatedMana(ActivatedAbilityDef),
    TriggeredMana(TriggeredAbilityDef),
    Activated(ActivatedAbilityDef),
    Triggered(TriggeredAbilityDef),
    Static(StaticAbilityDef),
    SpecialAction(SpecialActionDef),
}

impl AbilityDef {
    #[must_use]
    pub const fn spell(id: AbilityId, text: &'static str, effect: EffectDef) -> Self {
        Self::Spell(SpellAbilityDef::new(id, text, effect))
    }

    #[must_use]
    pub const fn activated_mana(
        id: AbilityId,
        text: &'static str,
        costs: &'static [AbilityCostDef],
        effect: EffectDef,
    ) -> Self {
        Self::ActivatedMana(ActivatedAbilityDef::new(id, text, costs, effect))
    }

    #[must_use]
    pub const fn triggered_mana(
        id: AbilityId,
        text: &'static str,
        event: TriggerEventDef,
        effect: EffectDef,
    ) -> Self {
        Self::TriggeredMana(TriggeredAbilityDef::new(id, text, event, effect))
    }

    #[must_use]
    pub const fn activated(
        id: AbilityId,
        text: &'static str,
        costs: &'static [AbilityCostDef],
        effect: EffectDef,
    ) -> Self {
        Self::Activated(ActivatedAbilityDef::new(id, text, costs, effect))
    }

    #[must_use]
    pub const fn triggered(
        id: AbilityId,
        text: &'static str,
        event: TriggerEventDef,
        effect: EffectDef,
    ) -> Self {
        Self::Triggered(TriggeredAbilityDef::new(id, text, event, effect))
    }

    #[must_use]
    pub const fn static_ability(id: AbilityId, text: &'static str, effect: EffectDef) -> Self {
        Self::Static(StaticAbilityDef::new(id, text, effect))
    }

    #[must_use]
    pub const fn special_action(
        id: AbilityId,
        text: &'static str,
        source_zones: &'static [ZoneKind],
        costs: &'static [AbilityCostDef],
        effect: EffectDef,
    ) -> Self {
        Self::SpecialAction(SpecialActionDef::new(id, text, source_zones, costs, effect))
    }

    #[must_use]
    pub const fn id(self) -> AbilityId {
        match self {
            Self::Spell(definition) => definition.id,
            Self::ActivatedMana(definition) | Self::Activated(definition) => definition.id,
            Self::TriggeredMana(definition) | Self::Triggered(definition) => definition.id,
            Self::Static(definition) => definition.id,
            Self::SpecialAction(definition) => definition.id,
        }
    }

    #[must_use]
    pub const fn text(self) -> &'static str {
        match self {
            Self::Spell(definition) => definition.text,
            Self::ActivatedMana(definition) | Self::Activated(definition) => definition.text,
            Self::TriggeredMana(definition) | Self::Triggered(definition) => definition.text,
            Self::Static(definition) => definition.text,
            Self::SpecialAction(definition) => definition.text,
        }
    }

    #[must_use]
    pub const fn uses_stack(self) -> bool {
        matches!(
            self,
            Self::Spell(_) | Self::Activated(_) | Self::Triggered(_)
        )
    }
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
                rules.effect_status,
            )
        } else {
            PlayOptionDef::cast_with_printed_mana_cost(
                PlayOptionId::DEFAULT,
                name,
                SpellForm::Part(CardPartId::PRIMARY),
                printed_mana_cost,
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
    pub is_basic_land: bool,
    pub behavior: CardBehavior,
    /// Card-level implementation coverage. This intentionally lives outside
    /// `CardRules`: multipart cards have one audit status rather than mutable
    /// copies on each part.
    pub implementation_status: ImplementationStatus,
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
            implementation_status: ImplementationStatus::for_effect_status(rules.effect_status),
            rules,
            parts: composition.parts,
            structure: composition.structure,
            play_options: composition.play_options,
        }
    }

    /// Overrides the default card-level implementation coverage.
    #[must_use]
    pub fn with_implementation_status(mut self, status: ImplementationStatus) -> Self {
        self.implementation_status = status;
        self
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

/// How completely the engine implements a card or independently modeled part.
///
/// Ordinary construction defaults to [`Self::Complete`]. Both non-complete
/// states carry an explanation so catalog inspection can say what remains
/// rather than relying on a source-code comment beside the record.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ImplementationStatus {
    #[default]
    Complete,
    Partial {
        explanation: &'static str,
    },
    MetadataOnly {
        explanation: &'static str,
    },
}

impl ImplementationStatus {
    const METADATA_ONLY_EXPLANATION: &'static str =
        "Printed rules are cataloged but are not executed by the engine.";

    pub(crate) const fn for_effect_status(status: CardEffectStatus) -> Self {
        match status {
            CardEffectStatus::Implemented => Self::Complete,
            CardEffectStatus::MetadataOnly => Self::MetadataOnly {
                explanation: Self::METADATA_ONLY_EXPLANATION,
            },
        }
    }

    #[must_use]
    pub const fn partial(explanation: &'static str) -> Self {
        Self::Partial { explanation }
    }

    #[must_use]
    pub const fn metadata_only(explanation: &'static str) -> Self {
        Self::MetadataOnly { explanation }
    }

    #[must_use]
    pub const fn explanation(self) -> Option<&'static str> {
        match self {
            Self::Complete => None,
            Self::Partial { explanation } | Self::MetadataOnly { explanation } => Some(explanation),
        }
    }

    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Whether the current game engine may execute a play option or mode.
///
/// This remains as a compatibility gate while [`ImplementationStatus`]
/// records richer catalog-level coverage. New catalog code should use the
/// latter to describe completeness.
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
    /// Printed and characteristic-defining abilities declared by this part.
    /// Intrinsic abilities supplied by the rules (for example, basic land
    /// types) are derived separately from these printed definitions.
    pub abilities: &'static [AbilityDef],
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
            abilities: &[],
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
    pub const fn with_abilities(mut self, abilities: &'static [AbilityDef]) -> Self {
        self.abilities = abilities;
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
            ImplementationStatus::default(),
            ImplementationStatus::Complete
        );

        let metadata_only = implemented.metadata_only();
        assert_eq!(metadata_only.effect_status, CardEffectStatus::MetadataOnly);
        let metadata_definition = CardDefinition::new(
            CardDefinitionId(8),
            "Unsupported",
            CardSet::Alpha,
            false,
            CardBehavior::Unsupported,
        );
        assert!(matches!(
            metadata_definition.implementation_status,
            ImplementationStatus::MetadataOnly { explanation }
                if !explanation.is_empty()
        ));

        let partial = CardDefinition::new(
            CardDefinitionId(9),
            "Mountain",
            CardSet::Alpha,
            true,
            CardBehavior::Mountain,
        )
        .with_implementation_status(ImplementationStatus::partial("One rider is deferred."));
        assert_eq!(
            partial.implementation_status.explanation(),
            Some("One rider is deferred.")
        );
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
        assert_eq!(rules.abilities[0].id(), AbilityId::PRIMARY);
        assert_eq!(rules.abilities[1].id(), AbilityId(1));
        assert_eq!(rules.abilities[2].id(), AbilityId(2));
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
