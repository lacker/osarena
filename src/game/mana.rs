use crate::{AbilityOrigin, GameObjectId, ManaColor, ManaRestrictionDef, ManaSpendEffectDef};

/// The ability and object incarnation that produced one mana. Keeping both
/// identities lets spend restrictions and riders remain meaningful after the
/// source changes zones.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ManaSource {
    pub object: GameObjectId,
    pub ability: AbilityOrigin,
}

/// One mana in a player's pool, including the rules attached to spending it.
///
/// Runtime storage uses one value per mana rather than grouping production
/// into lots: three mana from Mishra's Workshop are three identical values,
/// and each can be spent or retained independently.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Mana {
    pub color: ManaColor,
    pub source: Option<ManaSource>,
    pub restrictions: &'static [ManaRestrictionDef],
    pub spend_effects: &'static [ManaSpendEffectDef],
}

impl Mana {
    #[must_use]
    pub const fn unrestricted(color: ManaColor) -> Self {
        Self {
            color,
            source: None,
            restrictions: &[],
            spend_effects: &[],
        }
    }

    #[must_use]
    pub const fn from_ability(
        color: ManaColor,
        source: ManaSource,
        restrictions: &'static [ManaRestrictionDef],
        spend_effects: &'static [ManaSpendEffectDef],
    ) -> Self {
        Self {
            color,
            source: Some(source),
            restrictions,
            spend_effects,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ManaPool {
    pub white: u16,
    pub blue: u16,
    pub black: u16,
    pub red: u16,
    pub green: u16,
    pub colorless: u16,
}

impl ManaPool {
    #[must_use]
    pub const fn total(self) -> u16 {
        self.white + self.blue + self.black + self.red + self.green + self.colorless
    }

    pub(super) fn add(&mut self, other: Self) {
        self.white = self.white.saturating_add(other.white);
        self.blue = self.blue.saturating_add(other.blue);
        self.black = self.black.saturating_add(other.black);
        self.red = self.red.saturating_add(other.red);
        self.green = self.green.saturating_add(other.green);
        self.colorless = self.colorless.saturating_add(other.colorless);
    }

    pub(super) const fn amount(self, color: ManaColor) -> u16 {
        match color {
            ManaColor::White => self.white,
            ManaColor::Blue => self.blue,
            ManaColor::Black => self.black,
            ManaColor::Red => self.red,
            ManaColor::Green => self.green,
            ManaColor::Colorless => self.colorless,
        }
    }

    pub(super) fn add_color(&mut self, color: ManaColor, amount: u16) {
        match color {
            ManaColor::White => self.white = self.white.saturating_add(amount),
            ManaColor::Blue => self.blue = self.blue.saturating_add(amount),
            ManaColor::Black => self.black = self.black.saturating_add(amount),
            ManaColor::Red => self.red = self.red.saturating_add(amount),
            ManaColor::Green => self.green = self.green.saturating_add(amount),
            ManaColor::Colorless => self.colorless = self.colorless.saturating_add(amount),
        }
    }

    pub(super) fn remove_color(&mut self, color: ManaColor, amount: u16) {
        match color {
            ManaColor::White => self.white -= amount,
            ManaColor::Blue => self.blue -= amount,
            ManaColor::Black => self.black -= amount,
            ManaColor::Red => self.red -= amount,
            ManaColor::Green => self.green -= amount,
            ManaColor::Colorless => self.colorless -= amount,
        }
    }
}
