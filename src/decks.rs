//! Built-in decklists compiled from the YAML files in the repository's `decks/` directory.

use std::error::Error;
use std::fmt;

use crate::Deck;
use crate::card::{self, CardCatalog};

#[derive(Clone, Copy)]
enum Zone {
    Main,
    Sideboard,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum BuiltinDeckError {
    DuplicateSection { line: usize, section: &'static str },
    EntryOutsideSection { line: usize },
    InvalidEntry { line: usize },
    InvalidCount { line: usize, value: String },
    UnknownCard { line: usize, name: String },
    MissingSection(&'static str),
}

impl fmt::Display for BuiltinDeckError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateSection { line, section } => {
                write!(formatter, "duplicate {section} section on line {line}")
            }
            Self::EntryOutsideSection { line } => {
                write!(formatter, "deck entry outside a section on line {line}")
            }
            Self::InvalidEntry { line } => write!(formatter, "invalid deck entry on line {line}"),
            Self::InvalidCount { line, value } => {
                write!(formatter, "invalid card count {value:?} on line {line}")
            }
            Self::UnknownCard { line, name } => {
                write!(formatter, "unknown card {name:?} on line {line}")
            }
            Self::MissingSection(section) => write!(formatter, "missing {section} section"),
        }
    }
}

impl Error for BuiltinDeckError {}

fn parse(yaml: &str, catalog: &CardCatalog) -> Result<Deck, BuiltinDeckError> {
    let mut deck = Deck {
        main: Vec::new(),
        sideboard: Vec::new(),
    };
    let mut zone = None;
    let mut saw_main = false;
    let mut saw_sideboard = false;

    for (index, raw_line) in yaml.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match line {
            "main:" => {
                if saw_main {
                    return Err(BuiltinDeckError::DuplicateSection {
                        line: line_number,
                        section: "main",
                    });
                }
                saw_main = true;
                zone = Some(Zone::Main);
                continue;
            }
            "sideboard:" => {
                if saw_sideboard {
                    return Err(BuiltinDeckError::DuplicateSection {
                        line: line_number,
                        section: "sideboard",
                    });
                }
                saw_sideboard = true;
                zone = Some(Zone::Sideboard);
                continue;
            }
            _ => {}
        }

        let Some(current_zone) = zone else {
            return Err(BuiltinDeckError::EntryOutsideSection { line: line_number });
        };
        let Some((name, raw_count)) = line.rsplit_once(':') else {
            return Err(BuiltinDeckError::InvalidEntry { line: line_number });
        };
        let name = name.trim();
        if name.is_empty() {
            return Err(BuiltinDeckError::InvalidEntry { line: line_number });
        }
        let count =
            raw_count
                .trim()
                .parse::<usize>()
                .map_err(|_| BuiltinDeckError::InvalidCount {
                    line: line_number,
                    value: raw_count.trim().into(),
                })?;
        let id = catalog
            .find_by_name(name)
            .ok_or_else(|| BuiltinDeckError::UnknownCard {
                line: line_number,
                name: name.into(),
            })?;
        let cards = match current_zone {
            Zone::Main => &mut deck.main,
            Zone::Sideboard => &mut deck.sideboard,
        };
        cards.extend(std::iter::repeat_n(id, count));
    }

    if !saw_main {
        return Err(BuiltinDeckError::MissingSection("main"));
    }
    if !saw_sideboard {
        return Err(BuiltinDeckError::MissingSection("sideboard"));
    }
    Ok(deck)
}

fn builtin(yaml: &str) -> Deck {
    let catalog = card::catalog().expect("built-in card catalog must be valid");
    parse(yaml, &catalog).expect("built-in deck YAML must be valid")
}

macro_rules! deck {
    ($name:ident, $file:literal, $description:literal) => {
        #[doc = $description]
        #[must_use]
        pub fn $name() -> Deck {
            builtin(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/decks/",
                $file
            )))
        }
    };
}

deck!(
    goblins,
    "goblins.yaml",
    "Returns a representative powered EC Goblins deck."
);
deck!(
    sligh,
    "sligh.yaml",
    "Returns a representative powered EC Sligh deck."
);
deck!(
    artifacts,
    "artifacts.yaml",
    "Returns a representative powered EC Atog artifact deck."
);
deck!(
    robots,
    "robots.yaml",
    "Returns a representative powered EC mono-red Robots deck."
);
deck!(
    the_deck,
    "the_deck.yaml",
    "Returns the classic powered EC control deck known as The Deck."
);
deck!(
    mono_black,
    "mono_black.yaml",
    "Returns a representative powered EC Mono Black deck."
);
deck!(
    white_weenie,
    "white_weenie.yaml",
    "Returns a representative powered EC White Weenie deck."
);
deck!(
    erhnamgeddon,
    "erhnamgeddon.yaml",
    "Returns a representative powered EC Erhnamgeddon deck."
);
deck!(
    counterburn,
    "counterburn.yaml",
    "Returns a representative powered EC Counterburn deck."
);
deck!(
    lions_dib,
    "lions_dib.yaml",
    "Returns a representative powered EC Lions/Dib deck."
);
deck!(
    bwr_aggro,
    "bwr_aggro.yaml",
    "Returns a representative powered BWR aggro deck."
);
deck!(
    gr_aggro,
    "gr_aggro.yaml",
    "Returns a representative powered green-red aggro deck."
);
deck!(
    troll_disk,
    "troll_disk.yaml",
    "Returns a representative powered Sedge Troll / Disk deck."
);
deck!(
    jeskai_aggro,
    "jeskai_aggro.yaml",
    "Returns a representative powered Jeskai tempo deck."
);
deck!(
    lions_dib_bolt,
    "lions_dib_bolt.yaml",
    "Returns the Lion/Dib shell with its burn package."
);

/// Backwards-compatible name for the built-in artifact deck.
#[must_use]
pub fn mono_red_atog() -> Deck {
    artifacts()
}

#[cfg(test)]
mod tests {
    use super::{BuiltinDeckError, parse};
    use crate::card;

    #[test]
    fn parser_reports_unknown_cards_with_their_line() {
        let catalog = card::catalog().unwrap();
        let error = parse("main:\n  Not a Card: 60\nsideboard:\n", &catalog).unwrap_err();

        assert_eq!(
            error,
            BuiltinDeckError::UnknownCard {
                line: 2,
                name: "Not a Card".into(),
            }
        );
    }
}
