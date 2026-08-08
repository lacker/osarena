use super::{CardArt, CardComposition, CardDefinition, CardPrinting, CardRules, CardSet};
use crate::CardDefinitionId;

type CompositionBuilder = fn() -> CardComposition;

/// Internal source record from which the runtime catalog is built.
pub(super) struct CardRecord {
    pub(super) id: CardDefinitionId,
    pub(super) name: &'static str,
    pub(super) art: CardArt,
    pub(super) set: CardSet,
    pub(super) rules: CardRules,
    composition: Option<CompositionBuilder>,
}

impl CardRecord {
    #[allow(clippy::large_types_passed_by_value)]
    pub(super) const fn new(
        id: CardDefinitionId,
        name: &'static str,
        art: CardArt,
        set: CardSet,
        rules: CardRules,
    ) -> Self {
        Self {
            id,
            name,
            art,
            set,
            rules,
            composition: None,
        }
    }

    /// Supplies logical parts and play options for a structured or modal card.
    #[must_use]
    pub(super) const fn with_composition(mut self, builder: CompositionBuilder) -> Self {
        self.composition = Some(builder);
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
