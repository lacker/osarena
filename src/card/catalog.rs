use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use super::CardDefinition;
use crate::CardDefinitionId;
use crate::rules;

#[derive(Clone, Debug, Default)]
pub struct CardCatalog {
    by_id: HashMap<CardDefinitionId, CardDefinition>,
    by_name: HashMap<String, CardDefinitionId>,
}

impl CardCatalog {
    /// Builds a catalog whose card IDs and case-insensitive names are unique.
    ///
    /// # Errors
    ///
    /// Returns [`CatalogError`] when an ID or normalized card name is repeated.
    pub fn new(
        definitions: impl IntoIterator<Item = CardDefinition>,
    ) -> Result<Self, CatalogError> {
        let mut catalog = Self::default();
        for definition in definitions {
            if catalog.by_id.contains_key(&definition.id) {
                return Err(CatalogError::DuplicateId(definition.id));
            }
            let normalized_name = normalize_name(&definition.name);
            if catalog.by_name.contains_key(&normalized_name) {
                return Err(CatalogError::DuplicateName(definition.name));
            }
            catalog.by_name.insert(normalized_name, definition.id);
            catalog.by_id.insert(definition.id, definition);
        }
        Ok(catalog)
    }

    #[must_use]
    pub fn get(&self, id: CardDefinitionId) -> Option<&CardDefinition> {
        self.by_id.get(&id)
    }

    /// Every definition in the catalog, ordered by id so consumers see a
    /// stable listing.
    #[must_use]
    pub fn definitions(&self) -> Vec<&CardDefinition> {
        let mut definitions: Vec<_> = self.by_id.values().collect();
        definitions.sort_by_key(|definition| definition.id);
        definitions
    }

    /// Looks up a card definition ID by its case-insensitive printed name.
    #[must_use]
    pub fn find_by_name(&self, name: &str) -> Option<CardDefinitionId> {
        self.by_name.get(&normalize_name(name)).copied()
    }

    #[must_use]
    pub fn is_banned(&self, id: CardDefinitionId) -> bool {
        self.get(id)
            .is_some_and(|card| rules::is_banned(&card.name))
    }

    #[must_use]
    pub fn is_restricted(&self, id: CardDefinitionId) -> bool {
        self.get(id)
            .is_some_and(|card| rules::is_restricted(&card.name))
    }
}

fn normalize_name(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CatalogError {
    DuplicateId(CardDefinitionId),
    DuplicateName(String),
}

impl fmt::Display for CatalogError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateId(id) => write!(formatter, "duplicate card definition ID {id:?}"),
            Self::DuplicateName(name) => write!(formatter, "duplicate card name {name:?}"),
        }
    }
}

impl Error for CatalogError {}
