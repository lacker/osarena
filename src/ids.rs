use std::fmt;

/// Stable identity of a card in the card catalog.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CardDefinitionId(pub u16);

/// Identity of one logical rules component within a card definition.
///
/// Parts include faces of double-faced cards and halves of split cards. The
/// identifier is local to its [`CardDefinitionId`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CardPartId(pub u8);

impl CardPartId {
    /// The sole part of an ordinary card, or the primary/front part of a
    /// structured card.
    pub const PRIMARY: Self = Self(0);
}

/// Identity of one printed or granted ability within a card part.
///
/// The same definition can create many independent ability objects during a
/// game. Those objects receive [`GameObjectId`]s; this identifier continues to
/// name the ability in the card's rules definition.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AbilityId(pub u8);

impl AbilityId {
    /// The first ability in an ordinary single-ability card part.
    pub const PRIMARY: Self = Self(0);
}

/// Identity of one legal way to play a card, local to its card definition.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlayOptionId(pub u8);

impl PlayOptionId {
    /// The ordinary play option synthesized for an unstructured card.
    pub const DEFAULT: Self = Self(0);
}

/// Identity of one rules-text mode, local to its card definition.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModeId(pub u8);

/// Identity of one independently chosen target slot, local to its card
/// definition.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TargetSlotId(pub u8);

/// Identity of an alternative cost choice, local to its card definition.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AlternativeCostId(pub u8);

/// Identity of an additional cost choice, local to its card definition.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdditionalCostId(pub u8);

/// Identity of a recipe that can combine two physical cards into one melded
/// game object. No supported format currently executes meld actions, but card
/// topology can refer to a recipe without conflating it with a card face.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MeldRecipeId(pub u16);

/// Identity of one physical piece of cardboard for the duration of a game.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhysicalCardId(pub u32);

/// Identity of one rules object in its current zone.
///
/// A true zone change creates a new identity. Turning a card face up,
/// transforming it, or phasing it out does not.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GameObjectId(pub u32);

/// Compatibility name for callers written before physical cards and game
/// objects had separate identities. Prefer [`GameObjectId`] in new code.
///
/// This is the same type, not a distinct one, so nothing here stops a
/// protocol 1 caller from carrying an ID across a zone change — which is now
/// wrong. `#[deprecated]` is deliberately absent: on a `use` re-export the
/// attribute never reaches callers, and a `type` alias that would carry it
/// cannot be used as a constructor, so it would break every `CardInstanceId(n)`
/// in the wild. The migration pressure has to come from the changelog.
pub use GameObjectId as CardInstanceId;

/// Compatibility name for callers written before stack objects shared the
/// global game-object identity space. Prefer [`GameObjectId`] in new code.
///
/// The same caveat as [`CardInstanceId`] applies: this is an alias, not a
/// separate type, and it carries no compiler warning.
pub use GameObjectId as StackObjectId;

/// One of the two players in a game.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PlayerId {
    One,
    Two,
}

impl PlayerId {
    #[must_use]
    pub const fn opponent(self) -> Self {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::One,
        }
    }

    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::One => 0,
            Self::Two => 1,
        }
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One => formatter.write_str("player one"),
            Self::Two => formatter.write_str("player two"),
        }
    }
}
