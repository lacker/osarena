//! Python bindings over [`penta::protocol`].
//!
//! Observations and the catalog cross into Python as canonical protocol JSON
//! strings — the same bytes every other consumer of the protocol sees — and
//! bots answer with an index into `legalActions`. Parse with `json.loads`;
//! see BOTS.md at the repository root for the schema.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use engine::protocol::{
    BotGame, Opponent, catalog_json_for_format, deck_names_for_format, parse_format_slug,
};
use engine::{Format, PlayerId};

fn seat_from_name(name: &str) -> PyResult<PlayerId> {
    match name {
        "p1" => Ok(PlayerId::One),
        "p2" => Ok(PlayerId::Two),
        other => Err(PyValueError::new_err(format!(
            "seat must be \"p1\" or \"p2\", got {other:?}"
        ))),
    }
}

fn format_from_slug(slug: &str) -> PyResult<Format> {
    parse_format_slug(slug).map_err(PyValueError::new_err)
}

/// One game of a supported Magic format, driven from Python.
///
/// With a built-in opponent, `act` plays your action and then lets the
/// opponent play until you have a real choice again. With
/// `opponent="external"` the game stops at every decision for either seat,
/// so one loop can drive both sides for self-play.
#[pyclass]
struct Game {
    inner: BotGame,
}

#[pymethods]
impl Game {
    /// Starts a game. `opponent` is `"handcrafted"`, `"random"`, or
    /// `"external"`; `opponent_seat` is `"p1"` or `"p2"`. `format`
    /// defaults to Old School for compatibility.
    #[new]
    #[pyo3(signature = (
        p1_deck,
        p2_deck,
        opponent="handcrafted",
        opponent_seat="p2",
        seed=0,
        format="old-school-93-94"
    ))]
    fn new(
        p1_deck: &str,
        p2_deck: &str,
        opponent: &str,
        opponent_seat: &str,
        seed: u64,
        format: &str,
    ) -> PyResult<Self> {
        let opponent = match opponent {
            "external" => Opponent::External,
            "random" => Opponent::Random,
            "handcrafted" => Opponent::Handcrafted,
            other => {
                return Err(PyValueError::new_err(format!(
                    "opponent must be \"handcrafted\", \"random\", or \"external\", got {other:?}"
                )));
            }
        };
        let seat = seat_from_name(opponent_seat)?;
        let format = format_from_slug(format)?;
        BotGame::new_with_format(format, p1_deck, p2_deck, opponent, seat, seed)
            .map(|inner| Self { inner })
            .map_err(PyValueError::new_err)
    }

    /// The seat that must act next (`"p1"`/`"p2"`), or `None` when the game
    /// is over.
    fn decision_seat(&self) -> Option<&'static str> {
        self.inner.decision_seat_name()
    }

    /// One seat's observation as protocol JSON. Defaults to the seat that
    /// must act.
    #[pyo3(signature = (seat=None))]
    fn observe(&self, seat: Option<&str>) -> PyResult<String> {
        let seat = match seat {
            Some(name) => seat_from_name(name)?,
            None => self
                .inner
                .decision_seat()
                .ok_or_else(|| PyValueError::new_err("the game is over; pass a seat explicitly"))?,
        };
        Ok(self.inner.observe_json(seat))
    }

    /// How many legal actions the acting seat has; 0 when the game is over.
    fn legal_action_count(&self) -> usize {
        self.inner.legal_action_count()
    }

    /// Plays one index from the acting seat's `legalActions`.
    fn act(&mut self, action_index: usize) -> PyResult<()> {
        self.inner.act(action_index).map_err(PyValueError::new_err)
    }

    /// Answers a pending decision with explicit option ids, for multi-pick
    /// decisions where the default expansion in `legalActions` is not what
    /// you want. The observation's `decision` object lists the options.
    fn choose_decision(&mut self, option_ids: Vec<u32>) -> PyResult<()> {
        self.inner
            .choose_decision(&option_ids)
            .map_err(PyValueError::new_err)
    }

    /// `None` while the game runs; `"p1"`/`"p2"` for the winner, `"draw"`
    /// otherwise.
    fn result(&self) -> Option<&'static str> {
        use engine::GameResult;
        match self.inner.result()? {
            GameResult::Draw => Some("draw"),
            GameResult::Winner {
                winner: PlayerId::One,
                ..
            } => Some("p1"),
            GameResult::Winner {
                winner: PlayerId::Two,
                ..
            } => Some("p2"),
        }
    }
}

/// Every card definition with legality for `format`, as protocol JSON.
#[pyfunction]
#[pyo3(signature = (format = "old-school-93-94"))]
fn catalog(format: &str) -> PyResult<String> {
    let format = format_from_slug(format)?;
    let catalog =
        engine::card::catalog().map_err(|error| PyValueError::new_err(error.to_string()))?;
    Ok(catalog_json_for_format(&catalog, format).to_string())
}

/// The built-in deck names for `format`.
#[pyfunction]
#[pyo3(signature = (format = "old-school-93-94"))]
fn deck_names(format: &str) -> PyResult<Vec<&'static str>> {
    Ok(deck_names_for_format(format_from_slug(format)?))
}

/// The engine crate version, for pinning trained bots to rules behavior.
#[pyfunction]
fn engine_version() -> &'static str {
    engine::protocol::ENGINE_VERSION
}

/// The protocol version the JSON shapes follow.
#[pyfunction]
fn protocol_version() -> u32 {
    engine::protocol::PROTOCOL_VERSION
}

#[pymodule]
fn penta(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Game>()?;
    module.add_function(wrap_pyfunction!(catalog, module)?)?;
    module.add_function(wrap_pyfunction!(deck_names, module)?)?;
    module.add_function(wrap_pyfunction!(engine_version, module)?)?;
    module.add_function(wrap_pyfunction!(protocol_version, module)?)?;
    Ok(())
}
