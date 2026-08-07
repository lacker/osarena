use super::{CardBehavior, CardDefinition, CardRules, CardSet};
use crate::CardDefinitionId;

/// Internal source record from which the runtime catalog is built.
pub(super) struct CardRecord {
    pub(super) id: CardDefinitionId,
    pub(super) name: &'static str,
    pub(super) set: CardSet,
    pub(super) is_basic_land: bool,
    pub(super) behavior: CardBehavior,
    pub(super) rules: CardRules,
}

impl CardRecord {
    pub(super) const fn new(
        id: CardDefinitionId,
        name: &'static str,
        set: CardSet,
        is_basic_land: bool,
        behavior: CardBehavior,
        rules: CardRules,
    ) -> Self {
        Self {
            id,
            name,
            set,
            is_basic_land,
            behavior,
            rules,
        }
    }

    pub(super) fn definition(&self) -> CardDefinition {
        CardDefinition {
            id: self.id,
            name: self.name.into(),
            set: self.set,
            is_basic_land: self.is_basic_land,
            behavior: self.behavior,
            rules: self.rules,
        }
    }
}
