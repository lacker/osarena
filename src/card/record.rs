use super::{
    CardArt, CardBehavior, CardComposition, CardDefinition, CardPrinting, CardRules, CardSet,
    ImplementationStatus,
};
use crate::CardDefinitionId;

type CompositionBuilder = fn() -> CardComposition;

/// Internal source record from which the runtime catalog is built.
pub(super) struct CardRecord {
    pub(super) id: CardDefinitionId,
    pub(super) name: &'static str,
    pub(super) art: CardArt,
    pub(super) set: CardSet,
    pub(super) is_basic_land: bool,
    pub(super) behavior: CardBehavior,
    pub(super) rules: CardRules,
    pub(super) implementation_status: ImplementationStatus,
    composition: Option<CompositionBuilder>,
}

impl CardRecord {
    pub(super) const fn new(
        id: CardDefinitionId,
        name: &'static str,
        art: CardArt,
        set: CardSet,
        is_basic_land: bool,
        behavior: CardBehavior,
        rules: CardRules,
    ) -> Self {
        Self {
            id,
            name,
            art,
            set,
            is_basic_land,
            behavior,
            rules,
            implementation_status: ImplementationStatus::for_effect_status(rules.effect_status),
            composition: None,
        }
    }

    /// Supplies logical parts and play options for a structured or modal card.
    #[must_use]
    pub(super) const fn with_composition(mut self, builder: CompositionBuilder) -> Self {
        self.composition = Some(builder);
        self
    }

    /// Overrides the default complete status without adding a required
    /// argument to [`Self::new`].
    #[must_use]
    pub(super) const fn with_implementation_status(mut self, status: ImplementationStatus) -> Self {
        self.implementation_status = status;
        self
    }

    pub(super) fn definition(&self) -> CardDefinition {
        let composition = self.composition.map_or_else(
            || CardComposition::single(self.name, self.rules),
            |builder| builder(),
        );
        CardDefinition {
            id: self.id,
            name: self.name.into(),
            art: Some(self.art),
            set: self.set,
            printings: vec![CardPrinting::new(self.id, self.set)],
            is_basic_land: self.is_basic_land,
            behavior: self.behavior,
            implementation_status: self.implementation_status,
            rules: self.rules,
            parts: composition.parts,
            structure: composition.structure,
            play_options: composition.play_options,
        }
    }
}

/// A reprint or alternate-art printing whose rules come from `card`.
pub(super) struct PrintingRecord {
    pub(super) card: &'static CardRecord,
    pub(super) variant: u16,
}

impl PrintingRecord {
    /// Adds the default variant of `card` to another set.
    pub(super) const fn reprint(card: &'static CardRecord) -> Self {
        Self { card, variant: 0 }
    }

    /// Adds another distinguishable printing of `card` within the same set.
    pub(super) const fn alternate(card: &'static CardRecord, variant: u16) -> Self {
        assert!(variant > 0, "alternate printing variants start at one");
        Self { card, variant }
    }

    pub(super) const fn printing(&self, set: CardSet) -> CardPrinting {
        CardPrinting::with_variant(self.card.id, set, self.variant)
    }
}
