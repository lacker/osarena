//! Backwards-compatible constants for Eternal Central Old School 93/94.
//!
//! New code that can run more than one format should use [`crate::Format`]
//! instead of these fixed-format aliases.

use crate::Format;

pub use crate::format::{
    OLD_SCHOOL_BANNED_CARDS as BANNED_CARDS, OLD_SCHOOL_RESTRICTED_CARDS as RESTRICTED_CARDS,
};

pub const STARTING_LIFE: u8 = Format::OldSchool9394.rules().starting_life;
pub const OPENING_HAND_SIZE: usize = Format::OldSchool9394.rules().opening_hand_size;
pub const MINIMUM_MAIN_DECK_SIZE: usize = Format::OldSchool9394.rules().minimum_main_deck_size;
pub const MAXIMUM_SIDEBOARD_SIZE: usize = Format::OldSchool9394.rules().maximum_sideboard_size;
pub const MAXIMUM_COPIES: usize = Format::OldSchool9394.rules().maximum_copies;

#[must_use]
pub fn is_banned(name: &str) -> bool {
    Format::OldSchool9394.is_banned(name)
}

#[must_use]
pub fn is_restricted(name: &str) -> bool {
    Format::OldSchool9394.is_restricted(name)
}
