//! Runtime choices that determine a spell's locked stack characteristics.
//!
//! Playing a particular card part, choosing rules-text modes, and selecting an
//! alternative cost are independent decisions. [`CastChoices`] carries the
//! proposed decisions; after validation, [`CastSignature`] freezes the choices
//! that copy effects must preserve.

use std::error::Error;
use std::fmt;

use crate::action::Target;
use crate::card::SpellForm;
use crate::ids::{AdditionalCostId, AlternativeCostId, ModeId, PlayOptionId, TargetSlotId};

/// Semantic cost choices made while casting a spell.
///
/// This deliberately does not contain mana sources, sacrificed objects, or
/// discarded cards. Those are payments and must not be performed again when a
/// spell is copied. The vectors preserve order and multiplicity for cards that
/// allow an additional cost to be selected more than once.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct CostConfiguration {
    alternative: Option<AlternativeCostId>,
    additional: Vec<AdditionalCostId>,
}

impl CostConfiguration {
    #[must_use]
    pub fn new(alternative: Option<AlternativeCostId>, additional: Vec<AdditionalCostId>) -> Self {
        Self {
            alternative,
            additional,
        }
    }

    #[must_use]
    pub const fn alternative(&self) -> Option<AlternativeCostId> {
        self.alternative
    }

    #[must_use]
    pub fn additional(&self) -> &[AdditionalCostId] {
        &self.additional
    }
}

/// Targets selected for one independently addressable target slot.
///
/// A slot contains a vector because some effects use one grammatical target
/// instruction with variable cardinality. Different slots may contain the
/// same target; for example, fused Turn // Burn may target one creature with
/// both halves.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TargetSelection {
    slot: TargetSlotId,
    targets: Vec<Target>,
}

impl TargetSelection {
    #[must_use]
    pub fn new(slot: TargetSlotId, targets: Vec<Target>) -> Self {
        Self { slot, targets }
    }

    #[must_use]
    pub fn single(slot: TargetSlotId, target: Target) -> Self {
        Self::new(slot, vec![target])
    }

    #[must_use]
    pub const fn slot(&self) -> TargetSlotId {
        self.slot
    }

    #[must_use]
    pub fn targets(&self) -> &[Target] {
        &self.targets
    }
}

/// Player-supplied choices for a proposed cast action.
///
/// The game engine must validate these against the selected card definition
/// before constructing a [`CastSignature`]. In particular, it derives the
/// authoritative [`SpellForm`] from `play_option` rather than accepting a form
/// from the player.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CastChoices {
    play_option: PlayOptionId,
    modes: Vec<ModeId>,
    costs: CostConfiguration,
    x: u16,
    targets: Vec<TargetSelection>,
}

impl Default for CastChoices {
    fn default() -> Self {
        Self::new(PlayOptionId::DEFAULT)
    }
}

impl CastChoices {
    #[must_use]
    pub fn new(play_option: PlayOptionId) -> Self {
        Self {
            play_option,
            modes: Vec::new(),
            costs: CostConfiguration::default(),
            x: 0,
            targets: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_modes(mut self, modes: Vec<ModeId>) -> Self {
        self.modes = modes;
        self
    }

    #[must_use]
    pub fn with_costs(mut self, costs: CostConfiguration) -> Self {
        self.costs = costs;
        self
    }

    #[must_use]
    pub const fn with_x(mut self, x: u16) -> Self {
        self.x = x;
        self
    }

    #[must_use]
    pub fn with_targets(mut self, targets: Vec<TargetSelection>) -> Self {
        self.targets = targets;
        self
    }

    #[must_use]
    pub const fn play_option(&self) -> PlayOptionId {
        self.play_option
    }

    #[must_use]
    pub fn modes(&self) -> &[ModeId] {
        &self.modes
    }

    #[must_use]
    pub const fn costs(&self) -> &CostConfiguration {
        &self.costs
    }

    #[must_use]
    pub const fn x(&self) -> u16 {
        self.x
    }

    #[must_use]
    pub fn targets(&self) -> &[TargetSelection] {
        &self.targets
    }

    /// Returns the targets in printed slot order for compatibility with rules
    /// code that does not yet need to distinguish the slots.
    pub fn iter_targets(&self) -> impl Iterator<Item = &Target> {
        self.targets
            .iter()
            .flat_map(|selection| selection.targets.iter())
    }
}

/// The immutable casting choices carried by a spell on the stack.
///
/// Modes are an ordered vector rather than a set because order and repetition
/// can matter. Copying a spell clones this complete value; a copy effect such
/// as Fork may then replace only target values through [`Self::copy_with_targets`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CastSignature {
    play_option: PlayOptionId,
    form: SpellForm,
    modes: Vec<ModeId>,
    costs: CostConfiguration,
    x: u16,
    targets: Vec<TargetSelection>,
}

impl CastSignature {
    /// Freezes choices after the engine has validated `choices` and derived
    /// the resulting spell form from its cataloged play option.
    #[must_use]
    pub fn from_validated_choices(form: SpellForm, choices: CastChoices) -> Self {
        Self {
            play_option: choices.play_option,
            form,
            modes: choices.modes,
            costs: choices.costs,
            x: choices.x,
            targets: choices.targets,
        }
    }

    #[must_use]
    pub const fn play_option(&self) -> PlayOptionId {
        self.play_option
    }

    #[must_use]
    pub const fn form(&self) -> &SpellForm {
        &self.form
    }

    #[must_use]
    pub fn modes(&self) -> &[ModeId] {
        &self.modes
    }

    #[must_use]
    pub const fn costs(&self) -> &CostConfiguration {
        &self.costs
    }

    #[must_use]
    pub const fn x(&self) -> u16 {
        self.x
    }

    #[must_use]
    pub fn targets(&self) -> &[TargetSelection] {
        &self.targets
    }

    /// Returns the targets in printed slot order.
    pub fn iter_targets(&self) -> impl Iterator<Item = &Target> {
        self.targets
            .iter()
            .flat_map(|selection| selection.targets.iter())
    }

    /// Produces a spell-copy signature with replacement target values.
    ///
    /// The replacement must retain the same ordered slots and target counts.
    /// Target legality is deliberately checked by the game engine: retaining
    /// an original target that has since become illegal is permitted when a
    /// spell is copied, and it will be rechecked when the copy resolves.
    ///
    /// # Errors
    ///
    /// Returns [`TargetReplacementError`] when the replacement changes the
    /// number, order, identity, or cardinality of the original target slots.
    pub fn copy_with_targets(
        &self,
        targets: Vec<TargetSelection>,
    ) -> Result<Self, TargetReplacementError> {
        if self.targets.len() != targets.len() {
            return Err(TargetReplacementError::SelectionCount {
                expected: self.targets.len(),
                actual: targets.len(),
            });
        }
        for (index, (original, replacement)) in self.targets.iter().zip(&targets).enumerate() {
            if original.slot != replacement.slot {
                return Err(TargetReplacementError::Slot {
                    index,
                    expected: original.slot,
                    actual: replacement.slot,
                });
            }
            if original.targets.len() != replacement.targets.len() {
                return Err(TargetReplacementError::TargetCount {
                    slot: original.slot,
                    expected: original.targets.len(),
                    actual: replacement.targets.len(),
                });
            }
        }

        let mut copied = self.clone();
        copied.targets = targets;
        Ok(copied)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TargetReplacementError {
    SelectionCount {
        expected: usize,
        actual: usize,
    },
    Slot {
        index: usize,
        expected: TargetSlotId,
        actual: TargetSlotId,
    },
    TargetCount {
        slot: TargetSlotId,
        expected: usize,
        actual: usize,
    },
}

impl fmt::Display for TargetReplacementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SelectionCount { expected, actual } => write!(
                formatter,
                "a spell copy must keep {expected} target selections, but received {actual}"
            ),
            Self::Slot {
                index,
                expected,
                actual,
            } => write!(
                formatter,
                "target selection {index} must keep slot {expected:?}, but used {actual:?}"
            ),
            Self::TargetCount {
                slot,
                expected,
                actual,
            } => write!(
                formatter,
                "target slot {slot:?} must keep {expected} targets, but received {actual}"
            ),
        }
    }
}

impl Error for TargetReplacementError {}

#[cfg(test)]
mod tests {
    use super::{
        CastChoices, CastSignature, CostConfiguration, TargetReplacementError, TargetSelection,
    };
    use crate::action::Target;
    use crate::card::SpellForm;
    use crate::ids::{
        AdditionalCostId, AlternativeCostId, CardPartId, GameObjectId, ModeId, PlayOptionId,
        TargetSlotId,
    };

    fn fused_signature(targets: Vec<TargetSelection>) -> CastSignature {
        CastSignature::from_validated_choices(
            SpellForm::Combined(vec![CardPartId(0), CardPartId(1)]),
            CastChoices::new(PlayOptionId(2))
                .with_modes(vec![ModeId(2), ModeId(0)])
                .with_costs(CostConfiguration::new(
                    Some(AlternativeCostId(3)),
                    vec![AdditionalCostId(4), AdditionalCostId(4)],
                ))
                .with_x(7)
                .with_targets(targets),
        )
    }

    #[test]
    fn fork_copy_changes_only_targets() {
        let original = fused_signature(vec![
            TargetSelection::single(TargetSlotId(0), Target::Permanent(GameObjectId(10))),
            TargetSelection::single(TargetSlotId(1), Target::Player(crate::PlayerId::One)),
        ]);
        let copied = original
            .copy_with_targets(vec![
                TargetSelection::single(TargetSlotId(0), Target::Permanent(GameObjectId(20))),
                TargetSelection::single(TargetSlotId(1), Target::Player(crate::PlayerId::Two)),
            ])
            .unwrap();

        assert_eq!(copied.play_option(), original.play_option());
        assert_eq!(copied.form(), original.form());
        assert_eq!(copied.modes(), original.modes());
        assert_eq!(copied.costs(), original.costs());
        assert_eq!(copied.x(), original.x());
        assert_ne!(copied.targets(), original.targets());
        assert_eq!(
            original.targets()[0].targets(),
            &[Target::Permanent(GameObjectId(10))]
        );
    }

    #[test]
    fn two_slots_may_select_the_same_target() {
        let same_creature = Target::Permanent(GameObjectId(10));
        let original = fused_signature(vec![
            TargetSelection::single(TargetSlotId(0), same_creature),
            TargetSelection::single(TargetSlotId(1), same_creature),
        ]);
        let copied = original
            .copy_with_targets(original.targets().to_vec())
            .unwrap();

        assert_eq!(copied.targets()[0].targets(), &[same_creature]);
        assert_eq!(copied.targets()[1].targets(), &[same_creature]);
    }

    #[test]
    fn spell_copy_cannot_change_target_slots_or_cardinality() {
        let original = fused_signature(vec![
            TargetSelection::single(TargetSlotId(0), Target::Permanent(GameObjectId(10))),
            TargetSelection::single(TargetSlotId(1), Target::Player(crate::PlayerId::One)),
        ]);

        assert!(matches!(
            original.copy_with_targets(vec![TargetSelection::single(
                TargetSlotId(0),
                Target::Permanent(GameObjectId(20)),
            )]),
            Err(TargetReplacementError::SelectionCount { .. })
        ));
        assert!(matches!(
            original.copy_with_targets(vec![
                TargetSelection::single(TargetSlotId(9), Target::Permanent(GameObjectId(20)),),
                TargetSelection::single(TargetSlotId(1), Target::Player(crate::PlayerId::Two)),
            ]),
            Err(TargetReplacementError::Slot { .. })
        ));
        assert!(matches!(
            original.copy_with_targets(vec![
                TargetSelection::new(
                    TargetSlotId(0),
                    vec![
                        Target::Permanent(GameObjectId(20)),
                        Target::Permanent(GameObjectId(21)),
                    ],
                ),
                TargetSelection::single(TargetSlotId(1), Target::Player(crate::PlayerId::Two)),
            ]),
            Err(TargetReplacementError::TargetCount { .. })
        ));
    }
}
