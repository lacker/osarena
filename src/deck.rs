use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use crate::CardDefinitionId;
use crate::Format;
use crate::card::CardCatalog;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deck {
    pub main: Vec<CardDefinitionId>,
    pub sideboard: Vec<CardDefinitionId>,
}

impl Deck {
    /// Checks this deck against the default Eternal Central construction rules.
    ///
    /// # Errors
    ///
    /// Returns [`DeckError`] when the deck size, card identities, banned list,
    /// restricted list, or copy limits are invalid.
    pub fn validate(self, catalog: &CardCatalog) -> Result<ValidatedDeck, DeckError> {
        self.validate_for_format(catalog, Format::OldSchool9394)
    }

    /// Checks this deck against the construction rules and card legality of
    /// `format`.
    ///
    /// # Errors
    ///
    /// Returns [`DeckError`] when the deck size, card identities, format
    /// legality, banned list, restricted list, or copy limits are invalid.
    pub fn validate_for_format(
        self,
        catalog: &CardCatalog,
        format: Format,
    ) -> Result<ValidatedDeck, DeckError> {
        let format_rules = format.rules();
        if self.main.len() < format_rules.minimum_main_deck_size {
            return Err(DeckError::MainDeckTooSmall {
                actual: self.main.len(),
                minimum: format_rules.minimum_main_deck_size,
            });
        }
        if self.sideboard.len() > format_rules.maximum_sideboard_size {
            return Err(DeckError::SideboardTooLarge {
                actual: self.sideboard.len(),
                maximum: format_rules.maximum_sideboard_size,
            });
        }

        let mut counts = HashMap::<CardDefinitionId, usize>::new();
        for id in self.main.iter().chain(&self.sideboard) {
            let Some(card) = catalog.get(*id) else {
                return Err(DeckError::UnknownCard(*id));
            };
            if !catalog.is_allowed_in(*id, format) {
                return Err(DeckError::CardNotAllowed {
                    card: card.name.clone(),
                    format,
                });
            }
            if catalog.is_banned_in(*id, format) {
                return Err(DeckError::BannedCard(card.name.clone()));
            }
            *counts.entry(*id).or_default() += 1;
        }

        for (id, count) in counts {
            let Some(card) = catalog.get(id) else {
                return Err(DeckError::UnknownCard(id));
            };
            let limit = if card.is_basic_land() {
                usize::MAX
            } else if catalog.is_restricted_in(id, format) {
                1
            } else {
                format_rules.maximum_copies
            };
            if count > limit {
                return Err(DeckError::TooManyCopies {
                    card: card.name.clone(),
                    count,
                    limit,
                });
            }
        }

        Ok(ValidatedDeck(self))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedDeck(Deck);

impl ValidatedDeck {
    pub(crate) fn into_main(self) -> Vec<CardDefinitionId> {
        self.0.main
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeckError {
    MainDeckTooSmall {
        actual: usize,
        minimum: usize,
    },
    SideboardTooLarge {
        actual: usize,
        maximum: usize,
    },
    UnknownCard(CardDefinitionId),
    CardNotAllowed {
        card: String,
        format: Format,
    },
    BannedCard(String),
    TooManyCopies {
        card: String,
        count: usize,
        limit: usize,
    },
}

impl fmt::Display for DeckError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MainDeckTooSmall { actual, minimum } => write!(
                formatter,
                "main deck has {actual} cards; at least {minimum} are required"
            ),
            Self::SideboardTooLarge { actual, maximum } => write!(
                formatter,
                "sideboard has {actual} cards; at most {maximum} are allowed"
            ),
            Self::UnknownCard(id) => write!(formatter, "unknown card definition ID {id:?}"),
            Self::CardNotAllowed { card, format } => {
                write!(formatter, "{card} is not legal in {format}")
            }
            Self::BannedCard(card) => write!(formatter, "{card} is banned"),
            Self::TooManyCopies { card, count, limit } => {
                write!(
                    formatter,
                    "{card} appears {count} times; the limit is {limit}"
                )
            }
        }
    }
}

impl Error for DeckError {}
