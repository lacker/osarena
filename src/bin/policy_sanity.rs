use penta::{Game, GameResult, HandcraftedPolicy, PlayerId, RandomPolicy, play_game};
use penta::{card, decks};

const GAMES_PER_SEAT: u64 = 25;
const ACTION_LIMIT: usize = 50_000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let mut wins = 0_u64;
    let mut losses = 0_u64;
    let mut draws = 0_u64;

    for (deck_name, deck) in decks {
        let mut deck_wins = 0_u64;
        let mut deck_losses = 0_u64;
        let mut deck_draws = 0_u64;
        for seed in 0..GAMES_PER_SEAT {
            for handcrafted_seat in [PlayerId::One, PlayerId::Two] {
                let mut game = Game::new(catalog.clone(), [deck.clone(), deck.clone()], seed)?;
                let mut handcrafted = HandcraftedPolicy::new(catalog.clone());
                let mut random = RandomPolicy::new(seed ^ 0xa11c_e5ed);
                let result = match handcrafted_seat {
                    PlayerId::One => {
                        play_game(&mut game, &mut handcrafted, &mut random, ACTION_LIMIT)?
                    }
                    PlayerId::Two => {
                        play_game(&mut game, &mut random, &mut handcrafted, ACTION_LIMIT)?
                    }
                };
                match result {
                    GameResult::Winner { winner, .. } if winner == handcrafted_seat => {
                        deck_wins += 1;
                    }
                    GameResult::Winner { .. } => deck_losses += 1,
                    GameResult::Draw => deck_draws += 1,
                }
            }
        }
        wins += deck_wins;
        losses += deck_losses;
        draws += deck_draws;
        println!(
            "{deck_name:9}: {deck_wins:>3} wins, {deck_losses:>3} losses, \
             {deck_draws:>3} draws"
        );
    }

    let decided = wins + losses;
    let win_rate_tenths = wins * 1_000 / decided;
    let win_rate_whole = win_rate_tenths / 10;
    let win_rate_fraction = win_rate_tenths % 10;
    println!(
        "Overall  : {wins:>3} wins, {losses:>3} losses, {draws:>3} draws \
         ({win_rate_whole}.{win_rate_fraction}% of decided games)"
    );
    Ok(())
}
