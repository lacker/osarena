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
/// objects had separate identities.
#[deprecated(note = "use GameObjectId for actions and observations")]
pub use GameObjectId as CardInstanceId;

/// Compatibility name for callers written before stack objects shared the
/// global game-object identity space.
#[deprecated(note = "use GameObjectId")]
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
