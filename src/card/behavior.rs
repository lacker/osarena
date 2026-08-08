use super::{CardBehavior, CardKind, CardRules, CreatureStats, ManaCost};
use crate::card::sets;

impl CardBehavior {
    /// Returns all declarative rules metadata for this card behavior.
    #[must_use]
    pub const fn rules(self) -> &'static CardRules {
        sets::rules(self)
    }

    #[must_use]
    pub const fn is_legendary(self) -> bool {
        self.rules().is_legendary
    }

    #[must_use]
    pub fn rules_text(self) -> std::borrow::Cow<'static, str> {
        self.rules().rules_text()
    }

    #[must_use]
    pub const fn kind(self) -> CardKind {
        self.rules().kind
    }

    #[must_use]
    pub const fn mana_cost(self) -> ManaCost {
        self.rules().mana_cost
    }

    #[must_use]
    pub const fn creature_stats(self) -> Option<CreatureStats> {
        self.rules().creature_stats
    }

    #[must_use]
    pub const fn is_goblin(self) -> bool {
        self.rules().is_goblin
    }

    #[must_use]
    pub const fn has_flying(self) -> bool {
        self.rules().has_flying
    }

    #[must_use]
    pub const fn has_mountainwalk(self) -> bool {
        self.rules().has_mountainwalk
    }

    /// Returns the printed color flags in `[white, blue, black, red, green]` order.
    #[must_use]
    pub const fn color_identity(self) -> [bool; 5] {
        self.rules().colors
    }

    #[must_use]
    pub const fn is_red(self) -> bool {
        self.rules().colors[3]
    }

    #[must_use]
    pub const fn is_blue(self) -> bool {
        self.rules().colors[1]
    }

    #[must_use]
    pub const fn is_white(self) -> bool {
        self.rules().colors[0]
    }

    #[must_use]
    pub const fn is_black(self) -> bool {
        self.rules().colors[2]
    }

    #[must_use]
    pub const fn is_green(self) -> bool {
        self.rules().colors[4]
    }

    #[must_use]
    pub const fn has_vigilance(self) -> bool {
        self.rules().has_vigilance
    }
}
