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
    ($name:ident, $format:literal, $file:literal, $description:literal) => {
        #[doc = $description]
        #[must_use]
        pub fn $name() -> crate::Deck {
            super::builtin(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/decks/",
                $format,
                "/",
                $file
            )))
        }
    };
}

/// Built-in Eternal Central Old School 93/94 decklists.
pub mod old_school_93_94 {
    deck!(
        goblins,
        "old_school_93_94",
        "goblins.yaml",
        "Returns a representative powered EC Goblins deck."
    );
    deck!(
        sligh,
        "old_school_93_94",
        "sligh.yaml",
        "Returns a representative powered EC Sligh deck."
    );
    deck!(
        artifacts,
        "old_school_93_94",
        "artifacts.yaml",
        "Returns a representative powered EC Atog artifact deck."
    );
    deck!(
        robots,
        "old_school_93_94",
        "robots.yaml",
        "Returns a representative powered EC mono-red Robots deck."
    );
    deck!(
        the_deck,
        "old_school_93_94",
        "the_deck.yaml",
        "Returns the classic powered EC control deck known as The Deck."
    );
    deck!(
        mono_black,
        "old_school_93_94",
        "mono_black.yaml",
        "Returns a representative powered EC Mono Black deck."
    );
    deck!(
        white_weenie,
        "old_school_93_94",
        "white_weenie.yaml",
        "Returns a representative powered EC White Weenie deck."
    );
    deck!(
        erhnamgeddon,
        "old_school_93_94",
        "erhnamgeddon.yaml",
        "Returns a representative powered EC Erhnamgeddon deck."
    );
    deck!(
        counterburn,
        "old_school_93_94",
        "counterburn.yaml",
        "Returns a representative powered EC Counterburn deck."
    );
    deck!(
        lions_dib,
        "old_school_93_94",
        "lions_dib.yaml",
        "Returns a representative powered EC Lions/Dib deck."
    );
    deck!(
        bwr_aggro,
        "old_school_93_94",
        "bwr_aggro.yaml",
        "Returns a representative powered BWR aggro deck."
    );
    deck!(
        gr_aggro,
        "old_school_93_94",
        "gr_aggro.yaml",
        "Returns a representative powered green-red aggro deck."
    );
    deck!(
        troll_disk,
        "old_school_93_94",
        "troll_disk.yaml",
        "Returns a representative powered Sedge Troll / Disk deck."
    );
    deck!(
        jeskai_aggro,
        "old_school_93_94",
        "jeskai_aggro.yaml",
        "Returns a representative powered Jeskai tempo deck."
    );
    deck!(
        lions_dib_bolt,
        "old_school_93_94",
        "lions_dib_bolt.yaml",
        "Returns the Lion/Dib shell with its burn package."
    );

    /// Backwards-compatible name for the built-in artifact deck.
    #[must_use]
    pub fn mono_red_atog() -> crate::Deck {
        artifacts()
    }
}

/// Built-in decks from the September 2013 ISD–RTR Standard card pool.
pub mod isd_rtr_standard {
    deck!(
        naya_midrange_rudy_briksza,
        "isd_rtr_standard",
        "naya_midrange_rudy_briksza.yaml",
        "Returns Rudy Briksza's first-place Naya Midrange deck from SCG Open Atlanta."
    );
    deck!(
        gr_aggro_joseph_greer,
        "isd_rtr_standard",
        "gr_aggro_joseph_greer.yaml",
        "Returns Joseph Greer's second-place G/R Aggro deck from SCG Open Atlanta."
    );
    deck!(
        bg_midrange_mike_fyrberg,
        "isd_rtr_standard",
        "bg_midrange_mike_fyrberg.yaml",
        "Returns Mike Fyrberg's third-place B/G Midrange deck from SCG Open Atlanta."
    );
    deck!(
        naya_midrange_jimmie_smith,
        "isd_rtr_standard",
        "naya_midrange_jimmie_smith.yaml",
        "Returns Jimmie Smith's fourth-place Naya Midrange deck from SCG Open Atlanta."
    );
    deck!(
        uwr_flash_korey_mcduffie,
        "isd_rtr_standard",
        "uwr_flash_korey_mcduffie.yaml",
        "Returns the fifth-place U/W/R Flash deck piloted by Korey `McDuffie` at SCG Open Atlanta."
    );
    deck!(
        uw_flash_phillip_lorren,
        "isd_rtr_standard",
        "uw_flash_phillip_lorren.yaml",
        "Returns Phillip Lorren's sixth-place U/W Flash deck from SCG Open Atlanta."
    );
    deck!(
        uw_flash_clayton_arch,
        "isd_rtr_standard",
        "uw_flash_clayton_arch.yaml",
        "Returns a legality-corrected version of Clayton Arch's seventh-place U/W Flash deck from SCG Open Atlanta."
    );
    deck!(
        junk_reanimator_drew_kuenzinger,
        "isd_rtr_standard",
        "junk_reanimator_drew_kuenzinger.yaml",
        "Returns Drew Kuenzinger's eighth-place Junk Reanimator deck from SCG Open Atlanta."
    );
}

macro_rules! old_school_compatibility_wrapper {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[must_use]
        pub fn $name() -> Deck {
            old_school_93_94::$name()
        }
    };
}

old_school_compatibility_wrapper!(goblins, "Returns a representative powered EC Goblins deck.");
old_school_compatibility_wrapper!(sligh, "Returns a representative powered EC Sligh deck.");
old_school_compatibility_wrapper!(
    artifacts,
    "Returns a representative powered EC Atog artifact deck."
);
old_school_compatibility_wrapper!(
    robots,
    "Returns a representative powered EC mono-red Robots deck."
);
old_school_compatibility_wrapper!(
    the_deck,
    "Returns the classic powered EC control deck known as The Deck."
);
old_school_compatibility_wrapper!(
    mono_black,
    "Returns a representative powered EC Mono Black deck."
);
old_school_compatibility_wrapper!(
    white_weenie,
    "Returns a representative powered EC White Weenie deck."
);
old_school_compatibility_wrapper!(
    erhnamgeddon,
    "Returns a representative powered EC Erhnamgeddon deck."
);
old_school_compatibility_wrapper!(
    counterburn,
    "Returns a representative powered EC Counterburn deck."
);
old_school_compatibility_wrapper!(
    lions_dib,
    "Returns a representative powered EC Lions/Dib deck."
);
old_school_compatibility_wrapper!(
    bwr_aggro,
    "Returns a representative powered BWR aggro deck."
);
old_school_compatibility_wrapper!(
    gr_aggro,
    "Returns a representative powered green-red aggro deck."
);
old_school_compatibility_wrapper!(
    troll_disk,
    "Returns a representative powered Sedge Troll / Disk deck."
);
old_school_compatibility_wrapper!(
    jeskai_aggro,
    "Returns a representative powered Jeskai tempo deck."
);
old_school_compatibility_wrapper!(
    lions_dib_bolt,
    "Returns the Lion/Dib shell with its burn package."
);

/// Backwards-compatible name for the built-in artifact deck.
#[must_use]
pub fn mono_red_atog() -> Deck {
    old_school_93_94::mono_red_atog()
}

#[cfg(test)]
mod tests {
    use super::{
        BuiltinDeckError, artifacts, bwr_aggro, counterburn, erhnamgeddon, goblins, gr_aggro,
        isd_rtr_standard, jeskai_aggro, lions_dib, lions_dib_bolt, mono_black, mono_red_atog,
        old_school_93_94, parse, robots, sligh, the_deck, troll_disk, white_weenie,
    };
    use crate::card;
    use crate::{Deck, Format};

    type DeckBuilder = fn() -> Deck;

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

    #[test]
    fn old_school_top_level_builders_remain_compatible() {
        let builders: &[(DeckBuilder, DeckBuilder)] = &[
            (goblins, old_school_93_94::goblins),
            (sligh, old_school_93_94::sligh),
            (artifacts, old_school_93_94::artifacts),
            (robots, old_school_93_94::robots),
            (the_deck, old_school_93_94::the_deck),
            (mono_black, old_school_93_94::mono_black),
            (white_weenie, old_school_93_94::white_weenie),
            (erhnamgeddon, old_school_93_94::erhnamgeddon),
            (counterburn, old_school_93_94::counterburn),
            (lions_dib, old_school_93_94::lions_dib),
            (bwr_aggro, old_school_93_94::bwr_aggro),
            (gr_aggro, old_school_93_94::gr_aggro),
            (troll_disk, old_school_93_94::troll_disk),
            (jeskai_aggro, old_school_93_94::jeskai_aggro),
            (lions_dib_bolt, old_school_93_94::lions_dib_bolt),
            (mono_red_atog, old_school_93_94::mono_red_atog),
        ];

        for (top_level, namespaced) in builders {
            assert_eq!(top_level(), namespaced());
        }
    }

    #[test]
    fn standard_decks_parse_from_the_union_catalog_and_are_legal() {
        let catalog = card::catalog().unwrap();
        let builders: &[fn() -> Deck] = &[
            isd_rtr_standard::naya_midrange_rudy_briksza,
            isd_rtr_standard::gr_aggro_joseph_greer,
            isd_rtr_standard::bg_midrange_mike_fyrberg,
            isd_rtr_standard::naya_midrange_jimmie_smith,
            isd_rtr_standard::uwr_flash_korey_mcduffie,
            isd_rtr_standard::uw_flash_phillip_lorren,
            isd_rtr_standard::uw_flash_clayton_arch,
            isd_rtr_standard::junk_reanimator_drew_kuenzinger,
        ];

        for build in builders {
            let deck = build();
            assert_eq!(deck.main.len(), 60);
            assert_eq!(deck.sideboard.len(), 15);
            deck.validate_for_format(&catalog, Format::IsdRtrStandard)
                .unwrap();
        }
    }
}
