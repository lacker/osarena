//! C ABI over [`penta::protocol`].
//!
//! Every function here is a thin translation of the protocol module: strings
//! crossing the boundary are UTF-8, NUL-terminated JSON in the canonical
//! protocol shapes, so a bot written against this header reads the same bytes
//! as one written against the Python bindings or a future tournament server.
//!
//! Ownership rules, matching `include/penta.h`:
//! - Strings returned as `char *` are owned by the caller; free each with
//!   [`penta_string_free`].
//! - Strings returned as `const char *` are borrowed; do not free them.
//!   [`penta_last_error`] stays valid until the next failing call on the
//!   same thread.
//! - Games from [`penta_new`] and [`penta_clone`] are freed with
//!   [`penta_free`].

use std::cell::RefCell;
use std::ffi::{CStr, CString, c_char};

use penta::protocol::{BotGame, catalog_json_for_format, deck_names_for_format, parse_format_slug};
use penta::{Format, GameResult, PlayerId, card};

thread_local! {
    static LAST_ERROR: RefCell<CString> = RefCell::new(CString::default());
}

fn set_error(message: &str) {
    let stored = CString::new(message.replace('\0', " "))
        .unwrap_or_else(|_| CString::new("error message contained NUL").expect("static is clean"));
    LAST_ERROR.with(|slot| *slot.borrow_mut() = stored);
}

/// Converts a Rust string into a caller-owned C string.
fn give_string(value: String) -> *mut c_char {
    let Ok(string) = CString::new(value) else {
        set_error("string contained an interior NUL");
        return std::ptr::null_mut();
    };
    string.into_raw()
}

/// Parses a borrowed C format slug, reporting errors through
/// [`penta_last_error`].
///
/// # Safety
///
/// `format` must point to a valid NUL-terminated string.
unsafe fn format_from_c(format: *const c_char) -> Option<Format> {
    if format.is_null() {
        set_error("format is null");
        return None;
    }
    let Ok(slug) = (unsafe { CStr::from_ptr(format) }).to_str() else {
        set_error("format is not valid UTF-8");
        return None;
    };
    match parse_format_slug(slug) {
        Ok(format) => Some(format),
        Err(message) => {
            set_error(&message);
            None
        }
    }
}

fn catalog_json_for(format: Format) -> *mut c_char {
    match card::catalog() {
        Ok(catalog) => give_string(catalog_json_for_format(&catalog, format).to_string()),
        Err(error) => {
            set_error(&error.to_string());
            std::ptr::null_mut()
        }
    }
}

const fn seat_code(seat: PlayerId) -> i32 {
    match seat {
        PlayerId::One => 0,
        PlayerId::Two => 1,
    }
}

const fn seat_from_code(code: i32) -> Option<PlayerId> {
    match code {
        0 => Some(PlayerId::One),
        1 => Some(PlayerId::Two),
        _ => None,
    }
}

/// The engine crate version as a static string. Never freed.
#[unsafe(no_mangle)]
pub extern "C" fn penta_engine_version() -> *const c_char {
    static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");
    VERSION.as_ptr().cast()
}

/// The protocol version the JSON shapes follow.
#[unsafe(no_mangle)]
pub extern "C" fn penta_protocol_version() -> u32 {
    penta::protocol::PROTOCOL_VERSION
}

/// The most recent error on this thread, as a borrowed string. Empty until
/// something fails; valid until the next failing call.
#[unsafe(no_mangle)]
pub extern "C" fn penta_last_error() -> *const c_char {
    LAST_ERROR.with(|slot| slot.borrow().as_ptr())
}

/// Every card definition as Old School protocol JSON. Caller frees.
#[unsafe(no_mangle)]
pub extern "C" fn penta_catalog_json() -> *mut c_char {
    catalog_json_for(Format::OldSchool9394)
}

/// Every card definition with legality for `format`, as protocol JSON.
/// Caller frees. Returns null on error; see [`penta_last_error`].
///
/// # Safety
///
/// `format` must be a valid NUL-terminated UTF-8 format slug.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn penta_catalog_json_for_format(format: *const c_char) -> *mut c_char {
    let Some(format) = (unsafe { format_from_c(format) }) else {
        return std::ptr::null_mut();
    };
    catalog_json_for(format)
}

/// The Old School built-in deck names as a JSON array of strings. Caller frees.
#[unsafe(no_mangle)]
pub extern "C" fn penta_deck_names_json() -> *mut c_char {
    give_string(serde_json_names(&deck_names_for_format(
        Format::OldSchool9394,
    )))
}

/// The built-in deck names for `format` as a JSON array. Caller frees.
/// Returns null on error; see [`penta_last_error`].
///
/// # Safety
///
/// `format` must be a valid NUL-terminated UTF-8 format slug.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn penta_deck_names_for_format_json(format: *const c_char) -> *mut c_char {
    let Some(format) = (unsafe { format_from_c(format) }) else {
        return std::ptr::null_mut();
    };
    give_string(serde_json_names(&deck_names_for_format(format)))
}

fn serde_json_names(names: &[&str]) -> String {
    let mut out = String::from("[");
    for (index, name) in names.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push('"');
        out.push_str(name);
        out.push('"');
    }
    out.push(']');
    out
}

/// Starts a game from the protocol config JSON (see `penta.h` for the
/// shape). Returns null on error; see [`penta_last_error`].
///
/// # Safety
///
/// `config_json` must be a valid NUL-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn penta_new(config_json: *const c_char) -> *mut BotGame {
    if config_json.is_null() {
        set_error("config_json is null");
        return std::ptr::null_mut();
    }
    let Ok(config) = unsafe { CStr::from_ptr(config_json) }.to_str() else {
        set_error("config_json is not valid UTF-8");
        return std::ptr::null_mut();
    };
    match BotGame::from_config_json(config) {
        Ok(game) => Box::into_raw(Box::new(game)),
        Err(message) => {
            set_error(&message);
            std::ptr::null_mut()
        }
    }
}

/// An independent copy of a game: same state, same future for the same
/// actions — the built-in opponent's state included — so a bot can fork a
/// game, roll out a candidate line, and discard the copy. Freed with
/// [`penta_free`], like a game from [`penta_new`]. Returns null when `game`
/// is null.
///
/// # Safety
///
/// `game` must be a live pointer from [`penta_new`] or [`penta_clone`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn penta_clone(game: *const BotGame) -> *mut BotGame {
    let Some(game) = (unsafe { game.as_ref() }) else {
        set_error("game is null");
        return std::ptr::null_mut();
    };
    Box::into_raw(Box::new(game.clone()))
}

/// The seat that must act: 0 for p1, 1 for p2, -1 when the game is over.
///
/// # Safety
///
/// `game` must be a live pointer from [`penta_new`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn penta_decision_seat(game: *const BotGame) -> i32 {
    let Some(game) = (unsafe { game.as_ref() }) else {
        set_error("game is null");
        return -1;
    };
    game.decision_seat().map_or(-1, seat_code)
}

/// The number of legal actions for the acting seat; 0 when the game is over.
///
/// # Safety
///
/// `game` must be a live pointer from [`penta_new`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn penta_legal_action_count(game: *const BotGame) -> u32 {
    let Some(game) = (unsafe { game.as_ref() }) else {
        set_error("game is null");
        return 0;
    };
    u32::try_from(game.legal_action_count()).unwrap_or(u32::MAX)
}

/// One seat's observation as protocol JSON. Caller frees. Returns null for a
/// bad seat code.
///
/// # Safety
///
/// `game` must be a live pointer from [`penta_new`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn penta_observe_json(game: *const BotGame, seat: i32) -> *mut c_char {
    let Some(game) = (unsafe { game.as_ref() }) else {
        set_error("game is null");
        return std::ptr::null_mut();
    };
    let Some(seat) = seat_from_code(seat) else {
        set_error("seat must be 0 (p1) or 1 (p2)");
        return std::ptr::null_mut();
    };
    give_string(game.observe_json(seat))
}

/// Plays `action_index` from the acting seat's `legalActions`. Returns 0 on
/// success, -1 on error (see [`penta_last_error`]).
///
/// # Safety
///
/// `game` must be a live pointer from [`penta_new`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn penta_act(game: *mut BotGame, action_index: u32) -> i32 {
    let Some(game) = (unsafe { game.as_mut() }) else {
        set_error("game is null");
        return -1;
    };
    match game.act(action_index as usize) {
        Ok(()) => 0,
        Err(message) => {
            set_error(&message);
            -1
        }
    }
}

/// Answers a pending decision with explicit option ids, for multi-pick
/// decisions where the default expansion in `legalActions` is not wanted.
/// Returns 0 on success, -1 on error (see [`penta_last_error`]).
///
/// # Safety
///
/// `game` must be a live pointer from [`penta_new`]; `option_ids` must point
/// to `count` readable `uint32_t`s (it may be null when `count` is 0).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn penta_choose_decision(
    game: *mut BotGame,
    option_ids: *const u32,
    count: u32,
) -> i32 {
    let Some(game) = (unsafe { game.as_mut() }) else {
        set_error("game is null");
        return -1;
    };
    let options = if count == 0 {
        &[]
    } else if option_ids.is_null() {
        set_error("option_ids is null but count is nonzero");
        return -1;
    } else {
        unsafe { std::slice::from_raw_parts(option_ids, count as usize) }
    };
    match game.choose_decision(options) {
        Ok(()) => 0,
        Err(message) => {
            set_error(&message);
            -1
        }
    }
}

/// The result: -1 while the game is running, 0 for a draw, 1 when p1 won,
/// 2 when p2 won.
///
/// # Safety
///
/// `game` must be a live pointer from [`penta_new`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn penta_result(game: *const BotGame) -> i32 {
    let Some(game) = (unsafe { game.as_ref() }) else {
        set_error("game is null");
        return -1;
    };
    match game.result() {
        None => -1,
        Some(GameResult::Draw) => 0,
        Some(GameResult::Winner {
            winner: PlayerId::One,
            ..
        }) => 1,
        Some(GameResult::Winner {
            winner: PlayerId::Two,
            ..
        }) => 2,
    }
}

/// Frees a string returned by any `*_json` function. Null is a no-op.
///
/// # Safety
///
/// `string` must have come from this library and not been freed before.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn penta_string_free(string: *mut c_char) {
    if !string.is_null() {
        drop(unsafe { CString::from_raw(string) });
    }
}

/// Frees a game from [`penta_new`] or [`penta_clone`]. Null is a no-op.
///
/// # Safety
///
/// `game` must have come from [`penta_new`] or [`penta_clone`] and not been
/// freed before.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn penta_free(game: *mut BotGame) {
    if !game.is_null() {
        drop(unsafe { Box::from_raw(game) });
    }
}
