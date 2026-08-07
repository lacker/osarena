//! Backwards-compatible façade for the original proof-of-concept API.
//!
//! New code can use [`crate::card`] for the corpus and [`crate::decks`] for
//! built-in decklists directly.

pub use crate::card::{cards, catalog};
pub use crate::decks::{
    artifacts, bwr_aggro, counterburn, erhnamgeddon, goblins, gr_aggro, jeskai_aggro, lions_dib,
    lions_dib_bolt, mono_black, mono_red_atog, robots, sligh, the_deck, troll_disk, white_weenie,
};

#[cfg(test)]
mod tests {
    use super::cards;
    use super::{
        artifacts, bwr_aggro, catalog, counterburn, erhnamgeddon, goblins, gr_aggro, jeskai_aggro,
        lions_dib, lions_dib_bolt, mono_black, robots, sligh, the_deck, troll_disk, white_weenie,
    };
    use crate::rules;
    use crate::{CardBehavior, CardDefinitionId, CreatureStats, ManaCost};

    #[test]
    fn built_in_decks_have_tournament_sizes() {
        for deck in all_decks() {
            assert_eq!(deck.main.len(), rules::MINIMUM_MAIN_DECK_SIZE);
            assert_eq!(deck.sideboard.len(), rules::MAXIMUM_SIDEBOARD_SIZE);
        }
    }

    #[test]
    fn built_in_decks_are_valid() {
        let catalog = catalog().unwrap();
        for deck in all_decks() {
            deck.validate(&catalog).unwrap();
        }
    }

    #[test]
    fn every_poc_card_has_engine_behavior() {
        let catalog = catalog().unwrap();
        for raw_id in 1..=128 {
            let card = catalog.get(CardDefinitionId(raw_id)).unwrap();
            assert_ne!(card.behavior, CardBehavior::Unsupported, "{}", card.name);
            assert_eq!(card.rules, *card.behavior.rules(), "{}", card.name);
            assert!(
                !card.rules.text.is_empty(),
                "{} is missing rules text",
                card.name
            );
        }
    }

    #[test]
    fn stone_rain_costs_two_generic_and_one_red() {
        let catalog = catalog().unwrap();
        let card = catalog.get(cards::STONE_RAIN).unwrap();

        assert_eq!(card.rules.mana_cost, ManaCost::new(2, 1));
    }

    #[test]
    fn order_of_the_ebon_hand_is_a_two_one() {
        let catalog = catalog().unwrap();
        let card = catalog.get(cards::ORDER_OF_THE_EBON_HAND).unwrap();

        assert_eq!(
            card.rules.creature_stats,
            Some(CreatureStats {
                power: 2,
                toughness: 1,
                haste: false,
                trample: false,
            })
        );
    }

    fn all_decks() -> [crate::Deck; 16] {
        [
            goblins(),
            sligh(),
            artifacts(),
            robots(),
            the_deck(),
            mono_black(),
            white_weenie(),
            erhnamgeddon(),
            counterburn(),
            lions_dib(),
            bwr_aggro(),
            gr_aggro(),
            troll_disk(),
            jeskai_aggro(),
            lions_dib_bolt(),
            super::mono_red_atog(),
        ]
    }
}
