use std::collections::HashSet;
use std::error::Error;

use penta::{Game, HandcraftedPolicy, PlayerId, PlayerObservation, Policy, RandomPolicy};
use penta::{card, decks};

const DEFAULT_SEEDS: u64 = 100;
const ACTION_LIMIT: usize = 50_000;

#[derive(Default)]
struct AuditTotals {
    games: u64,
    actions: u64,
    max_actions: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let seed_count = std::env::args()
        .nth(1)
        .map(|value| value.parse())
        .transpose()?
        .unwrap_or(DEFAULT_SEEDS);
    let catalog = card::catalog()?;
    let decks = [
        ("Goblins", decks::goblins()),
        ("Sligh", decks::sligh()),
        ("Artifacts", decks::artifacts()),
        ("Robots", decks::robots()),
        ("The Deck", decks::the_deck()),
        ("Mono Black", decks::mono_black()),
        ("White Weenie", decks::white_weenie()),
        ("Erhnamgeddon", decks::erhnamgeddon()),
        ("Counterburn", decks::counterburn()),
        ("Lions/Dib", decks::lions_dib()),
        ("BWR Aggro", decks::bwr_aggro()),
        ("GR Aggro", decks::gr_aggro()),
        ("Troll Disk", decks::troll_disk()),
        ("Jeskai Aggro", decks::jeskai_aggro()),
        ("Lion Dib Bolt", decks::lions_dib_bolt()),
    ];
    let mut totals = AuditTotals::default();

    for (first_name, first_deck) in &decks {
        for (second_name, second_deck) in &decks {
            for seed in 0..seed_count {
                let game_seed = matchup_seed(seed, first_name, second_name);

                let mut game = Game::new(
                    catalog.clone(),
                    [first_deck.clone(), second_deck.clone()],
                    game_seed,
                )?;
                audit_game(
                    &mut game,
                    &mut RandomPolicy::new(game_seed ^ 0x51a7_0001),
                    &mut RandomPolicy::new(game_seed ^ 0x51a7_0002),
                    &mut totals,
                    first_name,
                    second_name,
                    seed,
                    "random/random",
                )?;

                let mut game = Game::new(
                    catalog.clone(),
                    [first_deck.clone(), second_deck.clone()],
                    game_seed,
                )?;
                audit_game(
                    &mut game,
                    &mut HandcraftedPolicy::new(catalog.clone()),
                    &mut RandomPolicy::new(game_seed ^ 0xa11c_e5ed),
                    &mut totals,
                    first_name,
                    second_name,
                    seed,
                    "handcrafted/random",
                )?;

                let mut game = Game::new(
                    catalog.clone(),
                    [first_deck.clone(), second_deck.clone()],
                    game_seed,
                )?;
                audit_game(
                    &mut game,
                    &mut RandomPolicy::new(game_seed ^ 0xa11c_e5ed),
                    &mut HandcraftedPolicy::new(catalog.clone()),
                    &mut totals,
                    first_name,
                    second_name,
                    seed,
                    "random/handcrafted",
                )?;
            }
            println!("{first_name:9} vs {second_name:9}: audited {seed_count} seeds");
        }
    }

    println!(
        "Audited {} games and {} actions; longest game was {} actions",
        totals.games, totals.actions, totals.max_actions
    );
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn audit_game(
    game: &mut Game,
    first: &mut dyn Policy,
    second: &mut dyn Policy,
    totals: &mut AuditTotals,
    first_deck: &str,
    second_deck: &str,
    seed: u64,
    policies: &str,
) -> Result<(), Box<dyn Error>> {
    for action_count in 0..ACTION_LIMIT {
        audit_public_state(game).map_err(|problem| {
            format!(
                "{first_deck} vs {second_deck}, seed {seed}, {policies}, \
                 action {action_count}: {problem}"
            )
        })?;

        let Some(player) = game.decision_player() else {
            totals.games += 1;
            totals.actions += u64::try_from(action_count).unwrap_or(u64::MAX);
            totals.max_actions = totals.max_actions.max(action_count);
            return Ok(());
        };
        let observation = game.observe(player);
        let action = match player {
            PlayerId::One => first.choose_action(&observation),
            PlayerId::Two => second.choose_action(&observation),
        }
        .ok_or_else(|| {
            format!(
                "{first_deck} vs {second_deck}, seed {seed}, {policies}, \
                 action {action_count}: policy returned no action"
            )
        })?;
        if !game.is_legal_action(player, &action) {
            return Err(format!(
                "{first_deck} vs {second_deck}, seed {seed}, {policies}, \
                 action {action_count}: policy chose illegal action {action:?}"
            )
            .into());
        }
        game.apply(player, action)?;
    }

    Err(format!(
        "{first_deck} vs {second_deck}, seed {seed}, {policies}: \
         exceeded {ACTION_LIMIT} actions"
    )
    .into())
}

fn audit_public_state(game: &Game) -> Result<(), String> {
    let first = game.observe(PlayerId::One);
    let second = game.observe(PlayerId::Two);
    compare_public_observations(&first, &second)?;

    match (game.result(), game.decision_player()) {
        (Some(_), None) => {
            if !first.legal_actions.is_empty() || !second.legal_actions.is_empty() {
                return Err("finished game still exposes legal actions".into());
            }
        }
        (None, Some(player)) => {
            let deciding = game.observe(player);
            if deciding.legal_actions.is_empty() {
                return Err(format!("{player} must decide but has no legal actions"));
            }
            let unique: HashSet<_> = deciding.legal_actions.iter().collect();
            if unique.len() != deciding.legal_actions.len() {
                return Err(format!("{player} has duplicate legal actions"));
            }
        }
        (Some(_), Some(player)) => {
            return Err(format!("finished game still asks {player} to decide"));
        }
        (None, None) => return Err("unfinished game has no decision player".into()),
    }
    Ok(())
}

fn compare_public_observations(
    first: &PlayerObservation,
    second: &PlayerObservation,
) -> Result<(), String> {
    if first.turn != second.turn
        || first.active_player != second.active_player
        || first.priority != second.priority
        || first.step != second.step
        || first.life_totals != second.life_totals
        || first.mana_pools != second.mana_pools
        || first.library_sizes != second.library_sizes
        || first.graveyards != second.graveyards
        || first.battlefield != second.battlefield
        || first.stack != second.stack
        || first.result != second.result
    {
        return Err("players disagree about public game state".into());
    }
    if first.hand.len() != second.opponent_hand_size
        || second.hand.len() != first.opponent_hand_size
    {
        return Err("hidden hand sizes are inconsistent".into());
    }
    Ok(())
}

fn matchup_seed(seed: u64, first: &str, second: &str) -> u64 {
    first
        .bytes()
        .chain(second.bytes())
        .fold(seed, |value, byte| {
            value
                .wrapping_mul(0x100_0000_01b3)
                .wrapping_add(u64::from(byte))
        })
}
