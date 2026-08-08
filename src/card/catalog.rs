use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use super::{
    AbilityDef, AbilityTargetDef, CardDefinition, CardPrinting, CardPrintingId, CardSet,
    CardStructure, PlayActionKind, PlayOptionDef, SpellForm, TargetSlotDef,
};
use crate::{
    AbilityId, AdditionalCostId, AlternativeCostId, CardDefinitionId, CardPartId, Format, ModeId,
    PlayOptionId, TargetSlotId,
};

/// A catalog is immutable once built, and callers pass it around by value —
/// a game, a policy, and the protocol facade each hold one. Sharing the maps
/// behind an `Arc` makes those clones a refcount bump instead of a deep copy
/// of every definition.
#[derive(Clone, Debug, Default)]
pub struct CardCatalog {
    entries: Arc<CatalogEntries>,
}

#[derive(Debug, Default)]
struct CatalogEntries {
    definitions: HashMap<CardDefinitionId, CardDefinition>,
    ids_by_name: HashMap<String, CardDefinitionId>,
    definition_by_printing: HashMap<CardPrintingId, CardDefinitionId>,
}

impl CatalogEntries {
    fn attach_printing(&mut self, printing: CardPrinting) -> Result<(), CatalogError> {
        let definition = printing.id.definition;
        if !self.definitions.contains_key(&definition) {
            return Err(CatalogError::OrphanPrinting(printing.id));
        }
        if self.definition_by_printing.contains_key(&printing.id) {
            return Err(CatalogError::DuplicatePrintingId(printing.id));
        }

        self.definition_by_printing.insert(printing.id, definition);
        self.definitions
            .get_mut(&definition)
            .expect("printing definition was checked above")
            .printings
            .push(printing);
        Ok(())
    }
}

impl CardCatalog {
    /// Builds a catalog whose definition, printing, and case-insensitive name
    /// identities are unique.
    ///
    /// # Errors
    ///
    /// Returns [`CatalogError`] when a definition ID, printing ID, or
    /// normalized card name is repeated; a printing belongs to another
    /// definition; or structured parts, forms, modes, costs, and target slots
    /// are missing or ambiguous.
    pub fn new(
        definitions: impl IntoIterator<Item = CardDefinition>,
    ) -> Result<Self, CatalogError> {
        Self::with_additional_printings(definitions, std::iter::empty())
    }

    /// Builds a catalog and attaches reprints or alternate printings to their
    /// canonical card definitions.
    ///
    /// # Errors
    ///
    /// Returns [`CatalogError`] under the same conditions as [`Self::new`], or
    /// when an additional printing references a definition outside the
    /// supplied catalog.
    pub fn with_additional_printings(
        definitions: impl IntoIterator<Item = CardDefinition>,
        printings: impl IntoIterator<Item = CardPrinting>,
    ) -> Result<Self, CatalogError> {
        let mut entries = CatalogEntries::default();
        let mut definition_printings = Vec::new();
        for mut definition in definitions {
            if entries.definitions.contains_key(&definition.id) {
                return Err(CatalogError::DuplicateId(definition.id));
            }
            let normalized_name = normalize_name(&definition.name);
            if entries.ids_by_name.contains_key(&normalized_name) {
                return Err(CatalogError::DuplicateName(definition.name));
            }
            validate_composition(&definition)?;
            let supplied_printings = std::mem::take(&mut definition.printings);
            definition_printings.extend(
                supplied_printings
                    .into_iter()
                    .map(|printing| (definition.id, printing)),
            );
            entries.ids_by_name.insert(normalized_name, definition.id);
            entries.definitions.insert(definition.id, definition);
        }

        for (definition, printing) in definition_printings {
            if printing.id.definition != definition {
                return Err(CatalogError::MismatchedPrintingDefinition {
                    definition,
                    printing: printing.id,
                });
            }
            entries.attach_printing(printing)?;
        }
        for printing in printings {
            entries.attach_printing(printing)?;
        }
        Ok(Self {
            entries: Arc::new(entries),
        })
    }

    #[must_use]
    pub fn get(&self, id: CardDefinitionId) -> Option<&CardDefinition> {
        self.entries.definitions.get(&id)
    }

    /// Every definition in the catalog, ordered by id so consumers see a
    /// stable listing.
    #[must_use]
    pub fn definitions(&self) -> Vec<&CardDefinition> {
        let mut definitions: Vec<_> = self.entries.definitions.values().collect();
        definitions.sort_by_key(|definition| definition.id);
        definitions
    }

    /// Looks up a card definition ID by its case-insensitive printed name.
    #[must_use]
    pub fn find_by_name(&self, name: &str) -> Option<CardDefinitionId> {
        self.entries.ids_by_name.get(&normalize_name(name)).copied()
    }

    #[must_use]
    pub fn get_printing(&self, id: CardPrintingId) -> Option<&CardPrinting> {
        let definition = self.entries.definition_by_printing.get(&id)?;
        self.entries
            .definitions
            .get(definition)?
            .printings
            .iter()
            .find(|printing| printing.id == id)
    }

    /// Returns every known printing of `id`, or an empty slice for an unknown
    /// definition.
    #[must_use]
    pub fn printings_for(&self, id: CardDefinitionId) -> &[CardPrinting] {
        self.get(id).map_or(&[], |card| card.printings.as_slice())
    }

    #[must_use]
    pub fn has_printing_in(&self, id: CardDefinitionId, set: CardSet) -> bool {
        self.printings_for(id)
            .iter()
            .any(|printing| printing.id.set == set)
    }

    #[must_use]
    pub fn is_banned(&self, id: CardDefinitionId) -> bool {
        self.is_banned_in(id, Format::OldSchool9394)
    }

    #[must_use]
    pub fn is_allowed_in(&self, id: CardDefinitionId, format: Format) -> bool {
        self.get(id).is_some_and(|card| format.allows_card(card))
    }

    #[must_use]
    pub fn is_banned_in(&self, id: CardDefinitionId, format: Format) -> bool {
        self.get(id)
            .is_some_and(|card| format.is_banned(&card.name))
    }

    #[must_use]
    pub fn is_restricted(&self, id: CardDefinitionId) -> bool {
        self.is_restricted_in(id, Format::OldSchool9394)
    }

    #[must_use]
    pub fn is_restricted_in(&self, id: CardDefinitionId, format: Format) -> bool {
        self.get(id)
            .is_some_and(|card| format.is_restricted(&card.name))
    }
}

/// Folds a printed card name to the key both the catalog and a lookup use.
///
/// Magic prints accented names — Juzám Djinn, Márton Stromgald, Lim-Dûl —
/// and decklists, search boxes, and bot authors overwhelmingly type them
/// without the accents. Folding here means the catalog can store the name as
/// printed while every spelling still resolves to the same card.
fn normalize_name(name: &str) -> String {
    let mut normalized = String::with_capacity(name.len());

    for lowered in name.trim().chars().flat_map(char::to_lowercase) {
        match ascii_fold(lowered) {
            Some(replacement) => normalized.push_str(replacement),
            None => normalized.push(lowered),
        }
    }

    normalized
}

/// The ASCII spelling of one lowercase Latin-1 letter, or `None` when the
/// character is already the key form.
///
/// Ligatures and the letters that conventionally spell out are handled as
/// strings rather than single characters, so Æther folds to `aether` and not
/// to `ather` — otherwise typing the unaccented name would stop matching,
/// which is the whole point of folding.
fn ascii_fold(lowered: char) -> Option<&'static str> {
    Some(match lowered {
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => "a",
        'æ' => "ae",
        'ç' => "c",
        'è' | 'é' | 'ê' | 'ë' => "e",
        'ì' | 'í' | 'î' | 'ï' => "i",
        'ð' => "d",
        'ñ' => "n",
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' => "o",
        'ù' | 'ú' | 'û' | 'ü' => "u",
        'ý' | 'ÿ' => "y",
        'þ' => "th",
        'ß' => "ss",
        _ => return None,
    })
}

fn validate_composition(definition: &CardDefinition) -> Result<(), CatalogError> {
    if definition
        .implementation_status
        .explanation()
        .is_some_and(|explanation| explanation.trim().is_empty())
    {
        return Err(CatalogError::MissingImplementationExplanation(
            definition.id,
        ));
    }
    let mut defined_parts = HashSet::new();
    for part in &definition.parts {
        if !defined_parts.insert(part.id) {
            return Err(CatalogError::DuplicatePartId {
                definition: definition.id,
                part: part.id,
            });
        }
        validate_abilities(definition, part.id, part.rules.abilities)?;
    }

    let structure_parts = structure_parts(definition)?;
    for part in &structure_parts {
        if !defined_parts.contains(part) {
            return Err(CatalogError::UndefinedStructurePart {
                definition: definition.id,
                part: *part,
            });
        }
    }
    for part in &definition.parts {
        if !structure_parts.contains(&part.id) {
            return Err(CatalogError::PartOutsideStructure {
                definition: definition.id,
                part: part.id,
            });
        }
    }

    let mut play_options = HashSet::new();
    let mut modes = HashSet::new();
    let mut alternative_costs = HashSet::new();
    let mut additional_costs = HashSet::new();
    for option in &definition.play_options {
        if !play_options.insert(option.id) {
            return Err(CatalogError::DuplicatePlayOptionId {
                definition: definition.id,
                option: option.id,
            });
        }
        validate_spell_form(definition, option, &defined_parts, &structure_parts)?;
        validate_cost_ids(
            definition,
            option,
            &mut alternative_costs,
            &mut additional_costs,
        )?;
        validate_modes_and_targets(definition, option, &mut modes)?;
    }

    validate_fused_option(definition)
}

fn validate_abilities(
    definition: &CardDefinition,
    part: CardPartId,
    abilities: &[AbilityDef],
) -> Result<(), CatalogError> {
    let mut ids = HashSet::new();
    for ability in abilities {
        if !ids.insert(ability.id()) {
            return Err(CatalogError::DuplicateAbilityId {
                definition: definition.id,
                part,
                ability: ability.id(),
            });
        }

        let (source_zones, targets, is_mana_ability) = match ability {
            AbilityDef::Spell(spell) => (None, spell.targets, false),
            AbilityDef::ActivatedMana(activated) => {
                (Some(activated.source_zones), activated.targets, true)
            }
            AbilityDef::TriggeredMana(triggered) => {
                (Some(triggered.source_zones), triggered.targets, true)
            }
            AbilityDef::Activated(activated) => {
                (Some(activated.source_zones), activated.targets, false)
            }
            AbilityDef::Triggered(triggered) => {
                (Some(triggered.source_zones), triggered.targets, false)
            }
            AbilityDef::Static(static_ability) => {
                (Some(static_ability.source_zones), &[][..], false)
            }
            AbilityDef::SpecialAction(special_action) => {
                (Some(special_action.source_zones), &[][..], false)
            }
        };

        if source_zones.is_some_and(<[super::ZoneKind]>::is_empty) {
            return Err(CatalogError::AbilityHasNoSourceZone {
                definition: definition.id,
                part,
                ability: ability.id(),
            });
        }
        if is_mana_ability && !targets.is_empty() {
            return Err(CatalogError::ManaAbilityHasTargets {
                definition: definition.id,
                part,
                ability: ability.id(),
            });
        }
        validate_ability_targets(definition, part, ability.id(), targets)?;
    }
    Ok(())
}

fn validate_ability_targets(
    definition: &CardDefinition,
    part: CardPartId,
    ability: AbilityId,
    targets: &[AbilityTargetDef],
) -> Result<(), CatalogError> {
    let mut ids = HashSet::new();
    for target in targets {
        if target.minimum > target.maximum {
            return Err(CatalogError::InvalidAbilityTargetBounds {
                definition: definition.id,
                part,
                ability,
                target: target.id,
                minimum: target.minimum,
                maximum: target.maximum,
            });
        }
        if !ids.insert(target.id) {
            return Err(CatalogError::DuplicateAbilityTargetId {
                definition: definition.id,
                part,
                ability,
                target: target.id,
            });
        }
    }
    Ok(())
}

fn structure_parts(definition: &CardDefinition) -> Result<Vec<CardPartId>, CatalogError> {
    let parts = match &definition.structure {
        CardStructure::Single { main } => vec![*main],
        CardStructure::Split { parts, .. } => {
            if parts.len() < 2 {
                return Err(CatalogError::InvalidSplitPartCount {
                    definition: definition.id,
                    actual: parts.len(),
                });
            }
            parts.clone()
        }
        CardStructure::Flip { normal, flipped } => vec![*normal, *flipped],
        CardStructure::DoubleFaced { front, back, .. } => vec![*front, *back],
        CardStructure::AlternateSpell {
            main, alternate, ..
        } => vec![*main, *alternate],
        CardStructure::MeldPart { front, .. } => vec![*front],
    };

    let mut seen = HashSet::new();
    for part in &parts {
        if !seen.insert(*part) {
            return Err(CatalogError::DuplicateStructurePart {
                definition: definition.id,
                part: *part,
            });
        }
    }
    Ok(parts)
}

fn validate_spell_form(
    definition: &CardDefinition,
    option: &PlayOptionDef,
    defined_parts: &HashSet<CardPartId>,
    structure_parts: &[CardPartId],
) -> Result<(), CatalogError> {
    let form_parts = match &option.form {
        SpellForm::Part(part) => vec![*part],
        SpellForm::Combined(parts) => {
            if parts.is_empty() {
                return Err(CatalogError::EmptySpellForm {
                    definition: definition.id,
                    option: option.id,
                });
            }
            let mut seen = HashSet::new();
            for part in parts {
                if !seen.insert(*part) {
                    return Err(CatalogError::DuplicateSpellFormPart {
                        definition: definition.id,
                        option: option.id,
                        part: *part,
                    });
                }
            }
            parts.clone()
        }
    };

    for part in form_parts {
        if !defined_parts.contains(&part) {
            return Err(CatalogError::UndefinedSpellFormPart {
                definition: definition.id,
                option: option.id,
                part,
            });
        }
        if !structure_parts.contains(&part) {
            return Err(CatalogError::SpellFormPartOutsideStructure {
                definition: definition.id,
                option: option.id,
                part,
            });
        }
    }
    Ok(())
}

fn validate_cost_ids(
    definition: &CardDefinition,
    option: &PlayOptionDef,
    alternative_costs: &mut HashSet<AlternativeCostId>,
    additional_costs: &mut HashSet<AdditionalCostId>,
) -> Result<(), CatalogError> {
    for cost in &option.alternative_costs {
        if !alternative_costs.insert(cost.id) {
            return Err(CatalogError::DuplicateAlternativeCostId {
                definition: definition.id,
                cost: cost.id,
            });
        }
    }
    for cost in &option.additional_costs {
        if !additional_costs.insert(cost.id) {
            return Err(CatalogError::DuplicateAdditionalCostId {
                definition: definition.id,
                cost: cost.id,
            });
        }
    }
    Ok(())
}

fn validate_modes_and_targets(
    definition: &CardDefinition,
    option: &PlayOptionDef,
    definition_modes: &mut HashSet<ModeId>,
) -> Result<(), CatalogError> {
    let mut option_slots = HashMap::new();
    validate_target_slots(definition, option, None, &option.targets, &mut option_slots)?;

    let Some(mode_set) = &option.modes else {
        return Ok(());
    };
    if mode_set.modes.is_empty() {
        return Err(CatalogError::EmptyModeSet {
            definition: definition.id,
            option: option.id,
        });
    }
    if mode_set.minimum > mode_set.maximum {
        return Err(CatalogError::InvalidModeBounds {
            definition: definition.id,
            option: option.id,
            minimum: mode_set.minimum,
            maximum: mode_set.maximum,
        });
    }
    if mode_set.maximum == 0 {
        return Err(CatalogError::ZeroModeMaximum {
            definition: definition.id,
            option: option.id,
        });
    }
    if !mode_set.may_repeat && usize::from(mode_set.maximum) > mode_set.modes.len() {
        return Err(CatalogError::TooManyModesWithoutRepetition {
            definition: definition.id,
            option: option.id,
            maximum: mode_set.maximum,
            available: mode_set.modes.len(),
        });
    }

    let modes_can_coexist = mode_set.maximum > 1;
    let mut coexisting_mode_slots = HashMap::new();
    for mode in &mode_set.modes {
        if !definition_modes.insert(mode.id) {
            return Err(CatalogError::DuplicateModeId {
                definition: definition.id,
                mode: mode.id,
            });
        }

        let mut mode_slots = HashMap::new();
        validate_target_slots(
            definition,
            option,
            Some(mode.id),
            &mode.targets,
            &mut mode_slots,
        )?;
        for slot in mode_slots.keys() {
            if option_slots.contains_key(slot) {
                return Err(CatalogError::AmbiguousTargetSlot {
                    definition: definition.id,
                    option: option.id,
                    slot: *slot,
                    first_mode: None,
                    second_mode: Some(mode.id),
                });
            }
        }
        if mode_set.may_repeat && modes_can_coexist && !mode.targets.is_empty() {
            return Err(CatalogError::RepeatableModeHasAmbiguousTargets {
                definition: definition.id,
                option: option.id,
                mode: mode.id,
            });
        }
        if modes_can_coexist {
            for slot in mode_slots.keys() {
                if let Some(first_mode) = coexisting_mode_slots.insert(*slot, mode.id) {
                    return Err(CatalogError::AmbiguousTargetSlot {
                        definition: definition.id,
                        option: option.id,
                        slot: *slot,
                        first_mode: Some(first_mode),
                        second_mode: Some(mode.id),
                    });
                }
            }
        }
    }
    Ok(())
}

fn validate_target_slots(
    definition: &CardDefinition,
    option: &PlayOptionDef,
    mode: Option<ModeId>,
    slots: &[TargetSlotDef],
    seen: &mut HashMap<TargetSlotId, Option<ModeId>>,
) -> Result<(), CatalogError> {
    for slot in slots {
        if slot.minimum > slot.maximum {
            return Err(CatalogError::InvalidTargetBounds {
                definition: definition.id,
                option: option.id,
                mode,
                slot: slot.id,
                minimum: slot.minimum,
                maximum: slot.maximum,
            });
        }
        if let Some(first_mode) = seen.insert(slot.id, mode) {
            return Err(CatalogError::AmbiguousTargetSlot {
                definition: definition.id,
                option: option.id,
                slot: slot.id,
                first_mode,
                second_mode: mode,
            });
        }
    }
    Ok(())
}

fn validate_fused_option(definition: &CardDefinition) -> Result<(), CatalogError> {
    let CardStructure::Split { parts, fused } = &definition.structure else {
        if let Some(option) = definition
            .play_options
            .iter()
            .find(|option| matches!(option.form, SpellForm::Combined(_)))
        {
            return Err(CatalogError::UnexpectedCombinedSpellForm {
                definition: definition.id,
                option: option.id,
            });
        }
        return Ok(());
    };

    for option in &definition.play_options {
        if matches!(option.form, SpellForm::Combined(_)) && Some(option.id) != *fused {
            return Err(CatalogError::UnexpectedCombinedSpellForm {
                definition: definition.id,
                option: option.id,
            });
        }
    }

    let Some(fused) = fused else {
        return Ok(());
    };
    let Some(option) = definition.play_option(*fused) else {
        return Err(CatalogError::MissingFusedPlayOption {
            definition: definition.id,
            option: *fused,
        });
    };
    if option.action != PlayActionKind::CastSpell
        || option.form != SpellForm::Combined(parts.clone())
    {
        return Err(CatalogError::InvalidFusedPlayOption {
            definition: definition.id,
            option: *fused,
            expected: parts.clone(),
            actual: option.form.clone(),
            actual_action: option.action,
        });
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CatalogError {
    DuplicateId(CardDefinitionId),
    DuplicateName(String),
    DuplicatePrintingId(CardPrintingId),
    MismatchedPrintingDefinition {
        definition: CardDefinitionId,
        printing: CardPrintingId,
    },
    OrphanPrinting(CardPrintingId),
    MissingImplementationExplanation(CardDefinitionId),
    DuplicatePartId {
        definition: CardDefinitionId,
        part: CardPartId,
    },
    DuplicateAbilityId {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
    },
    AbilityHasNoSourceZone {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
    },
    ManaAbilityHasTargets {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
    },
    InvalidAbilityTargetBounds {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        target: TargetSlotId,
        minimum: u8,
        maximum: u8,
    },
    DuplicateAbilityTargetId {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
        target: TargetSlotId,
    },
    DuplicateStructurePart {
        definition: CardDefinitionId,
        part: CardPartId,
    },
    InvalidSplitPartCount {
        definition: CardDefinitionId,
        actual: usize,
    },
    UndefinedStructurePart {
        definition: CardDefinitionId,
        part: CardPartId,
    },
    PartOutsideStructure {
        definition: CardDefinitionId,
        part: CardPartId,
    },
    DuplicatePlayOptionId {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    EmptySpellForm {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    DuplicateSpellFormPart {
        definition: CardDefinitionId,
        option: PlayOptionId,
        part: CardPartId,
    },
    UndefinedSpellFormPart {
        definition: CardDefinitionId,
        option: PlayOptionId,
        part: CardPartId,
    },
    SpellFormPartOutsideStructure {
        definition: CardDefinitionId,
        option: PlayOptionId,
        part: CardPartId,
    },
    MissingFusedPlayOption {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    InvalidFusedPlayOption {
        definition: CardDefinitionId,
        option: PlayOptionId,
        expected: Vec<CardPartId>,
        actual: SpellForm,
        actual_action: PlayActionKind,
    },
    UnexpectedCombinedSpellForm {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    DuplicateModeId {
        definition: CardDefinitionId,
        mode: ModeId,
    },
    EmptyModeSet {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    InvalidModeBounds {
        definition: CardDefinitionId,
        option: PlayOptionId,
        minimum: u8,
        maximum: u8,
    },
    ZeroModeMaximum {
        definition: CardDefinitionId,
        option: PlayOptionId,
    },
    TooManyModesWithoutRepetition {
        definition: CardDefinitionId,
        option: PlayOptionId,
        maximum: u8,
        available: usize,
    },
    DuplicateAlternativeCostId {
        definition: CardDefinitionId,
        cost: AlternativeCostId,
    },
    DuplicateAdditionalCostId {
        definition: CardDefinitionId,
        cost: AdditionalCostId,
    },
    InvalidTargetBounds {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: Option<ModeId>,
        slot: TargetSlotId,
        minimum: u8,
        maximum: u8,
    },
    AmbiguousTargetSlot {
        definition: CardDefinitionId,
        option: PlayOptionId,
        slot: TargetSlotId,
        first_mode: Option<ModeId>,
        second_mode: Option<ModeId>,
    },
    RepeatableModeHasAmbiguousTargets {
        definition: CardDefinitionId,
        option: PlayOptionId,
        mode: ModeId,
    },
}

impl fmt::Display for CatalogError {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateId(id) => write!(formatter, "duplicate card definition ID {id:?}"),
            Self::DuplicateName(name) => write!(formatter, "duplicate card name {name:?}"),
            Self::DuplicatePrintingId(id) => write!(formatter, "duplicate card printing ID {id:?}"),
            Self::MismatchedPrintingDefinition {
                definition,
                printing,
            } => write!(
                formatter,
                "card printing {printing:?} was supplied by definition {definition:?}"
            ),
            Self::OrphanPrinting(id) => write!(
                formatter,
                "card printing {id:?} references an unknown definition"
            ),
            Self::MissingImplementationExplanation(definition) => write!(
                formatter,
                "card definition {definition:?} has a non-complete implementation status without an explanation"
            ),
            Self::DuplicatePartId { definition, part } => write!(
                formatter,
                "card definition {definition:?} defines part {part:?} more than once"
            ),
            Self::DuplicateAbilityId {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "part {part:?} of card definition {definition:?} defines ability {ability:?} more than once"
            ),
            Self::AbilityHasNoSourceZone {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} has no source zone"
            ),
            Self::ManaAbilityHasTargets {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "mana ability {ability:?} on part {part:?} of card definition {definition:?} declares targets"
            ),
            Self::InvalidAbilityTargetBounds {
                definition,
                part,
                ability,
                target,
                minimum,
                maximum,
            } => write!(
                formatter,
                "target {target:?} of ability {ability:?} on part {part:?} of card definition {definition:?} requires at least {minimum} targets but allows at most {maximum}"
            ),
            Self::DuplicateAbilityTargetId {
                definition,
                part,
                ability,
                target,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} defines target {target:?} more than once"
            ),
            Self::DuplicateStructurePart { definition, part } => write!(
                formatter,
                "card definition {definition:?}'s structure references part {part:?} more than once"
            ),
            Self::InvalidSplitPartCount { definition, actual } => write!(
                formatter,
                "split card definition {definition:?} must contain at least two ordered parts, but contains {actual}"
            ),
            Self::UndefinedStructurePart { definition, part } => write!(
                formatter,
                "card definition {definition:?}'s structure references undefined part {part:?}"
            ),
            Self::PartOutsideStructure { definition, part } => write!(
                formatter,
                "card definition {definition:?} defines part {part:?}, but its structure does not contain that part"
            ),
            Self::DuplicatePlayOptionId { definition, option } => write!(
                formatter,
                "card definition {definition:?} defines play option {option:?} more than once"
            ),
            Self::EmptySpellForm { definition, option } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} has an empty combined spell form"
            ),
            Self::DuplicateSpellFormPart {
                definition,
                option,
                part,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} references part {part:?} more than once in its spell form"
            ),
            Self::UndefinedSpellFormPart {
                definition,
                option,
                part,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} references undefined spell-form part {part:?}"
            ),
            Self::SpellFormPartOutsideStructure {
                definition,
                option,
                part,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} uses part {part:?}, which is not in the card's structure"
            ),
            Self::MissingFusedPlayOption { definition, option } => write!(
                formatter,
                "split card definition {definition:?} names missing fused play option {option:?}"
            ),
            Self::InvalidFusedPlayOption {
                definition,
                option,
                expected,
                actual,
                actual_action,
            } => write!(
                formatter,
                "fused play option {option:?} of card definition {definition:?} must cast combined parts {expected:?} in printed order, but has action {actual_action:?} and form {actual:?}"
            ),
            Self::UnexpectedCombinedSpellForm { definition, option } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} has a combined spell form but is not its declared fused split option"
            ),
            Self::DuplicateModeId { definition, mode } => write!(
                formatter,
                "card definition {definition:?} defines mode {mode:?} more than once"
            ),
            Self::EmptyModeSet { definition, option } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} has a mode set with no modes"
            ),
            Self::InvalidModeBounds {
                definition,
                option,
                minimum,
                maximum,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} requires at least {minimum} modes but allows at most {maximum}"
            ),
            Self::ZeroModeMaximum { definition, option } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} has modes but allows none to be selected"
            ),
            Self::TooManyModesWithoutRepetition {
                definition,
                option,
                maximum,
                available,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} allows {maximum} modes without repetition but defines only {available}"
            ),
            Self::DuplicateAlternativeCostId { definition, cost } => write!(
                formatter,
                "card definition {definition:?} defines alternative cost {cost:?} more than once"
            ),
            Self::DuplicateAdditionalCostId { definition, cost } => write!(
                formatter,
                "card definition {definition:?} defines additional cost {cost:?} more than once"
            ),
            Self::InvalidTargetBounds {
                definition,
                option,
                mode,
                slot,
                minimum,
                maximum,
            } => {
                if let Some(mode) = mode {
                    write!(
                        formatter,
                        "target slot {slot:?} in mode {mode:?} of play option {option:?} on card definition {definition:?} requires at least {minimum} targets but allows at most {maximum}"
                    )
                } else {
                    write!(
                        formatter,
                        "target slot {slot:?} in play option {option:?} of card definition {definition:?} requires at least {minimum} targets but allows at most {maximum}"
                    )
                }
            }
            Self::AmbiguousTargetSlot {
                definition,
                option,
                slot,
                first_mode,
                second_mode,
            } => write!(
                formatter,
                "target slot {slot:?} is ambiguous within one casting of play option {option:?} on card definition {definition:?} (origins {first_mode:?} and {second_mode:?})"
            ),
            Self::RepeatableModeHasAmbiguousTargets {
                definition,
                option,
                mode,
            } => write!(
                formatter,
                "repeatable mode {mode:?} of play option {option:?} on card definition {definition:?} has target slots that cannot distinguish repeated selections"
            ),
        }
    }
}

impl Error for CatalogError {}

#[cfg(test)]
mod tests {
    use super::{CardCatalog, CatalogError};
    use crate::card::{
        AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivatedAbilityDef,
        AdditionalCostDef, AlternateSpellKind, AlternativeCostDef, CardBehavior, CardDefinition,
        CardEffectStatus, CardPart, CardPrinting, CardPrintingId, CardSet, CardStructure,
        DoubleFacedKind, EffectDef, ManaCost, ModeDef, ModeSetDef, PlayOptionDef, PlayerRelation,
        SpellForm, TargetPredicate, TargetSlotDef,
    };
    use crate::{
        AbilityId, AdditionalCostId, AlternativeCostId, CardDefinitionId, CardPartId, Format,
        MeldRecipeId, ModeId, PlayOptionId, TargetSlotId,
    };

    fn definition(id: u16, name: &str, set: CardSet) -> CardDefinition {
        CardDefinition::new(
            CardDefinitionId(id),
            name,
            set,
            false,
            CardBehavior::Unsupported,
        )
    }

    fn target(id: u8, minimum: u8, maximum: u8) -> TargetSlotDef {
        TargetSlotDef {
            id: TargetSlotId(id),
            label: format!("target {id}"),
            predicate: TargetPredicate::AnyTarget,
            minimum,
            maximum,
        }
    }

    fn mode(id: u8, targets: Vec<TargetSlotDef>) -> ModeDef {
        ModeDef {
            id: ModeId(id),
            label: format!("mode {id}"),
            targets,
            effect_status: CardEffectStatus::MetadataOnly,
        }
    }

    fn split_definition(fused: Option<PlayOptionId>) -> CardDefinition {
        let mut card = definition(1, "Left // Right", CardSet::Alpha);
        card.parts
            .push(CardPart::new(CardPartId(1), "Right", card.rules));
        card.structure = CardStructure::Split {
            parts: vec![CardPartId::PRIMARY, CardPartId(1)],
            fused,
        };
        card.play_options[0].label = "Left".into();
        card.play_options.push(PlayOptionDef::cast(
            PlayOptionId(1),
            "Right",
            SpellForm::Part(CardPartId(1)),
            card.rules.mana_cost,
            card.rules.effect_status,
        ));
        card
    }

    fn error(card: CardDefinition) -> CatalogError {
        CardCatalog::new([card]).unwrap_err()
    }

    #[test]
    fn primary_and_additional_printings_are_indexed_by_canonical_definition() {
        let id = CardDefinitionId(1);
        let primary = CardPrintingId::new(id, CardSet::Alpha);
        let beta = CardPrinting::new(id, CardSet::Beta);
        let alternate_beta = CardPrinting::with_variant(id, CardSet::Beta, 1);
        let catalog = CardCatalog::with_additional_printings(
            [definition(1, "Test Card", CardSet::Alpha)],
            [beta, alternate_beta],
        )
        .unwrap();

        assert_eq!(catalog.find_by_name(" test card "), Some(id));
        assert_eq!(catalog.get_printing(primary).unwrap().id, primary);
        assert_eq!(catalog.get_printing(beta.id), Some(&beta));
        assert_eq!(
            catalog.get_printing(alternate_beta.id),
            Some(&alternate_beta)
        );
        assert_eq!(catalog.printings_for(id).len(), 3);
        assert!(catalog.has_printing_in(id, CardSet::Alpha));
        assert!(catalog.has_printing_in(id, CardSet::Beta));
        assert!(!catalog.has_printing_in(id, CardSet::Unlimited));
    }

    #[test]
    fn duplicate_printing_ids_are_rejected() {
        let id = CardDefinitionId(1);
        let duplicate = CardPrinting::new(id, CardSet::Alpha);
        assert_eq!(
            CardCatalog::with_additional_printings(
                [definition(1, "Test Card", CardSet::Alpha)],
                [duplicate],
            )
            .unwrap_err(),
            CatalogError::DuplicatePrintingId(duplicate.id)
        );
    }

    #[test]
    fn an_allowed_reprint_makes_the_canonical_identity_format_legal() {
        let id = CardDefinitionId(1);
        let catalog = CardCatalog::with_additional_printings(
            [definition(1, "Test Card", CardSet::Alpha)],
            [CardPrinting::new(id, CardSet::Magic2014)],
        )
        .unwrap();

        assert_eq!(catalog.get(id).unwrap().set, CardSet::Alpha);
        assert!(catalog.is_allowed_in(id, Format::OldSchool9394));
        assert!(catalog.is_allowed_in(id, Format::IsdRtrStandard));
    }

    #[test]
    fn additional_printings_must_reference_a_cataloged_definition() {
        let orphan = CardPrinting::new(CardDefinitionId(2), CardSet::Beta);
        assert_eq!(
            CardCatalog::with_additional_printings(
                [definition(1, "Test Card", CardSet::Alpha)],
                [orphan],
            )
            .unwrap_err(),
            CatalogError::OrphanPrinting(orphan.id)
        );
    }

    #[test]
    fn definition_supplied_printings_must_belong_to_that_definition() {
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        let mismatched = CardPrinting::new(CardDefinitionId(2), CardSet::Beta);
        card.printings.push(mismatched);

        assert_eq!(
            CardCatalog::new([card]).unwrap_err(),
            CatalogError::MismatchedPrintingDefinition {
                definition: CardDefinitionId(1),
                printing: mismatched.id,
            }
        );
    }

    #[test]
    fn unknown_definitions_have_no_printings() {
        let catalog = CardCatalog::new([definition(1, "Test Card", CardSet::Alpha)]).unwrap();
        assert!(catalog.printings_for(CardDefinitionId(2)).is_empty());
        assert!(
            catalog
                .get_printing(CardPrintingId::new(CardDefinitionId(2), CardSet::Alpha))
                .is_none()
        );
    }

    #[test]
    fn part_and_play_option_ids_are_unique_within_a_definition() {
        let mut duplicate_part = definition(1, "Test Card", CardSet::Alpha);
        duplicate_part.parts.push(duplicate_part.parts[0].clone());
        assert_eq!(
            error(duplicate_part),
            CatalogError::DuplicatePartId {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
            }
        );

        let mut duplicate_option = definition(1, "Test Card", CardSet::Alpha);
        duplicate_option
            .play_options
            .push(duplicate_option.play_options[0].clone());
        assert_eq!(
            error(duplicate_option),
            CatalogError::DuplicatePlayOptionId {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
            }
        );
    }

    #[test]
    fn ability_ids_are_unique_within_each_card_part() {
        static ABILITIES: [AbilityDef; 2] = [
            AbilityDef::spell(AbilityId::PRIMARY, "first", EffectDef::None),
            AbilityDef::spell(AbilityId::PRIMARY, "second", EffectDef::None),
        ];
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        card.parts[0].rules = card.parts[0].rules.with_abilities(&ABILITIES);

        assert_eq!(
            error(card),
            CatalogError::DuplicateAbilityId {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
            }
        );
    }

    #[test]
    fn explicitly_tagged_mana_abilities_cannot_declare_targets() {
        static COSTS: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];
        static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
            TargetSlotId(1),
            "target player",
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )];
        static ABILITIES: [AbilityDef; 1] = [AbilityDef::ActivatedMana(
            ActivatedAbilityDef::new(
                AbilityId::PRIMARY,
                "Target player adds mana.",
                &COSTS,
                EffectDef::None,
            )
            .with_targets(&TARGETS),
        )];
        let mut card = definition(1, "Test Card", CardSet::Alpha);
        card.parts[0].rules = card.parts[0].rules.with_abilities(&ABILITIES);

        assert_eq!(
            error(card),
            CatalogError::ManaAbilityHasTargets {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
            }
        );
    }

    #[test]
    fn every_structure_family_rejects_undefined_or_repeated_parts() {
        let invalid_structures = [
            CardStructure::Single {
                main: CardPartId(9),
            },
            CardStructure::Split {
                parts: vec![CardPartId::PRIMARY, CardPartId(9)],
                fused: None,
            },
            CardStructure::Flip {
                normal: CardPartId::PRIMARY,
                flipped: CardPartId(9),
            },
            CardStructure::DoubleFaced {
                front: CardPartId::PRIMARY,
                back: CardPartId(9),
                kind: DoubleFacedKind::Transforming,
            },
            CardStructure::AlternateSpell {
                main: CardPartId::PRIMARY,
                alternate: CardPartId(9),
                kind: AlternateSpellKind::Adventure,
            },
            CardStructure::MeldPart {
                front: CardPartId(9),
                recipe: MeldRecipeId(1),
            },
        ];
        for structure in invalid_structures {
            let mut card = definition(1, "Test Card", CardSet::Alpha);
            card.structure = structure;
            assert!(matches!(
                error(card),
                CatalogError::UndefinedStructurePart {
                    definition: CardDefinitionId(1),
                    part: CardPartId(9),
                }
            ));
        }

        let mut repeated = definition(1, "Test Card", CardSet::Alpha);
        repeated.structure = CardStructure::Flip {
            normal: CardPartId::PRIMARY,
            flipped: CardPartId::PRIMARY,
        };
        assert_eq!(
            error(repeated),
            CatalogError::DuplicateStructurePart {
                definition: CardDefinitionId(1),
                part: CardPartId::PRIMARY,
            }
        );
    }

    #[test]
    fn spell_forms_must_reference_defined_structural_parts() {
        let mut undefined = definition(1, "Test Card", CardSet::Alpha);
        undefined.play_options[0].form = SpellForm::Part(CardPartId(9));
        assert_eq!(
            error(undefined),
            CatalogError::UndefinedSpellFormPart {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                part: CardPartId(9),
            }
        );

        let mut empty = split_definition(Some(PlayOptionId(2)));
        empty.play_options.push(PlayOptionDef::cast(
            PlayOptionId(2),
            "Left // Right",
            SpellForm::Combined(Vec::new()),
            ManaCost::default(),
            CardEffectStatus::MetadataOnly,
        ));
        assert_eq!(
            error(empty),
            CatalogError::EmptySpellForm {
                definition: CardDefinitionId(1),
                option: PlayOptionId(2),
            }
        );
    }

    #[test]
    fn fused_option_must_exist_and_match_all_split_parts_in_printed_order() {
        assert_eq!(
            error(split_definition(Some(PlayOptionId(2)))),
            CatalogError::MissingFusedPlayOption {
                definition: CardDefinitionId(1),
                option: PlayOptionId(2),
            }
        );

        let mut reversed = split_definition(Some(PlayOptionId(2)));
        reversed.play_options.push(PlayOptionDef::cast(
            PlayOptionId(2),
            "Right // Left",
            SpellForm::Combined(vec![CardPartId(1), CardPartId::PRIMARY]),
            ManaCost::default(),
            CardEffectStatus::MetadataOnly,
        ));
        assert!(matches!(
            error(reversed),
            CatalogError::InvalidFusedPlayOption {
                expected,
                actual: SpellForm::Combined(actual),
                ..
            } if expected == vec![CardPartId::PRIMARY, CardPartId(1)]
                && actual == vec![CardPartId(1), CardPartId::PRIMARY]
        ));

        let mut undeclared = split_definition(None);
        undeclared.play_options.push(PlayOptionDef::cast(
            PlayOptionId(2),
            "Left // Right",
            SpellForm::Combined(vec![CardPartId::PRIMARY, CardPartId(1)]),
            ManaCost::default(),
            CardEffectStatus::MetadataOnly,
        ));
        assert_eq!(
            error(undeclared),
            CatalogError::UnexpectedCombinedSpellForm {
                definition: CardDefinitionId(1),
                option: PlayOptionId(2),
            }
        );
    }

    #[test]
    fn mode_and_cost_ids_are_unique_across_a_definition() {
        let modes = ModeSetDef::choose_one(vec![mode(3, Vec::new()), mode(3, Vec::new())]);
        let mut duplicate_mode = definition(1, "Test Card", CardSet::Alpha);
        duplicate_mode.play_options[0].modes = Some(modes);
        assert_eq!(
            error(duplicate_mode),
            CatalogError::DuplicateModeId {
                definition: CardDefinitionId(1),
                mode: ModeId(3),
            }
        );

        let mut duplicate_alternative = definition(1, "Test Card", CardSet::Alpha);
        duplicate_alternative.play_options[0].alternative_costs = vec![
            AlternativeCostDef {
                id: AlternativeCostId(4),
                label: "first".into(),
                mana_cost: ManaCost::default(),
            },
            AlternativeCostDef {
                id: AlternativeCostId(4),
                label: "second".into(),
                mana_cost: ManaCost::default(),
            },
        ];
        assert_eq!(
            error(duplicate_alternative),
            CatalogError::DuplicateAlternativeCostId {
                definition: CardDefinitionId(1),
                cost: AlternativeCostId(4),
            }
        );

        let mut duplicate_additional = definition(1, "Test Card", CardSet::Alpha);
        duplicate_additional.play_options[0].additional_costs = vec![
            AdditionalCostDef {
                id: AdditionalCostId(5),
                label: "first".into(),
                mana_cost: None,
            },
            AdditionalCostDef {
                id: AdditionalCostId(5),
                label: "second".into(),
                mana_cost: None,
            },
        ];
        assert_eq!(
            error(duplicate_additional),
            CatalogError::DuplicateAdditionalCostId {
                definition: CardDefinitionId(1),
                cost: AdditionalCostId(5),
            }
        );
    }

    #[test]
    fn mode_and_target_cardinality_bounds_are_sane() {
        let mut invalid_modes = definition(1, "Test Card", CardSet::Alpha);
        invalid_modes.play_options[0].modes = Some(ModeSetDef {
            minimum: 2,
            maximum: 1,
            may_repeat: false,
            modes: vec![mode(0, Vec::new()), mode(1, Vec::new())],
        });
        assert_eq!(
            error(invalid_modes),
            CatalogError::InvalidModeBounds {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                minimum: 2,
                maximum: 1,
            }
        );

        let mut too_many_modes = definition(1, "Test Card", CardSet::Alpha);
        too_many_modes.play_options[0].modes = Some(ModeSetDef {
            minimum: 1,
            maximum: 2,
            may_repeat: false,
            modes: vec![mode(0, Vec::new())],
        });
        assert_eq!(
            error(too_many_modes),
            CatalogError::TooManyModesWithoutRepetition {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                maximum: 2,
                available: 1,
            }
        );

        let mut invalid_targets = definition(1, "Test Card", CardSet::Alpha);
        invalid_targets.play_options[0].targets = vec![target(7, 2, 1)];
        assert_eq!(
            error(invalid_targets),
            CatalogError::InvalidTargetBounds {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                mode: None,
                slot: TargetSlotId(7),
                minimum: 2,
                maximum: 1,
            }
        );
    }

    #[test]
    fn target_slot_ids_only_repeat_when_modes_cannot_coexist() {
        let mutually_exclusive = ModeSetDef::choose_one(vec![
            mode(0, vec![target(7, 1, 1)]),
            mode(1, vec![target(7, 1, 1)]),
        ]);
        let mut valid = definition(1, "Test Card", CardSet::Alpha);
        valid.play_options[0].modes = Some(mutually_exclusive);
        CardCatalog::new([valid]).unwrap();

        let mut coexisting = definition(1, "Test Card", CardSet::Alpha);
        coexisting.play_options[0].modes = Some(ModeSetDef {
            minimum: 2,
            maximum: 2,
            may_repeat: false,
            modes: vec![
                mode(0, vec![target(7, 1, 1)]),
                mode(1, vec![target(7, 1, 1)]),
            ],
        });
        assert!(matches!(
            error(coexisting),
            CatalogError::AmbiguousTargetSlot {
                slot: TargetSlotId(7),
                first_mode: Some(ModeId(0)),
                second_mode: Some(ModeId(1)),
                ..
            }
        ));

        let mut repeatable = definition(1, "Test Card", CardSet::Alpha);
        repeatable.play_options[0].modes = Some(ModeSetDef {
            minimum: 2,
            maximum: 2,
            may_repeat: true,
            modes: vec![mode(0, vec![target(8, 1, 1)])],
        });
        assert_eq!(
            error(repeatable),
            CatalogError::RepeatableModeHasAmbiguousTargets {
                definition: CardDefinitionId(1),
                option: PlayOptionId::DEFAULT,
                mode: ModeId(0),
            }
        );
    }
}

#[cfg(test)]
mod name_normalization_tests {
    use super::normalize_name;
    use crate::card;

    #[test]
    fn an_accented_name_is_found_by_either_spelling() {
        let catalog = card::catalog().expect("built-in catalog");
        let printed = catalog
            .find_by_name("Juzám Djinn")
            .expect("the name as printed on the card resolves");
        let typed = catalog
            .find_by_name("Juzam Djinn")
            .expect("the name as players type it resolves");

        assert_eq!(printed, typed);
        assert_eq!(
            catalog.get(printed).expect("definition").name,
            "Juzám Djinn",
            "the catalog stores the printed name; folding only affects lookup"
        );
    }

    #[test]
    fn folding_spells_out_ligatures_instead_of_dropping_them() {
        // Æ is the case that a single-character fold gets wrong: mapping it to
        // "a" would make the unaccented spelling stop matching, which is the
        // opposite of what folding is for.
        assert_eq!(normalize_name("Æther Vial"), "aether vial");
        assert_eq!(normalize_name("Aether Vial"), "aether vial");
    }

    #[test]
    fn folding_covers_the_accents_magic_actually_prints() {
        for (printed, plain) in [
            ("Juzám Djinn", "Juzam Djinn"),
            ("Márton Stromgald", "Marton Stromgald"),
            ("Lim-Dûl's Vault", "Lim-Dul's Vault"),
            ("Séance", "Seance"),
            ("Jötun Grunt", "Jotun Grunt"),
            ("Ærathi Berserker", "Aerathi Berserker"),
        ] {
            assert_eq!(
                normalize_name(printed),
                normalize_name(plain),
                "{printed} and {plain} must resolve to the same card"
            );
        }
    }

    #[test]
    fn normalization_still_trims_and_lowercases() {
        assert_eq!(normalize_name("  Black Lotus  "), "black lotus");
        assert_eq!(normalize_name("BLACK LOTUS"), "black lotus");
    }
}
