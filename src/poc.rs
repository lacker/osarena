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
    use std::collections::HashSet;

    use super::cards;
    use super::{
        artifacts, bwr_aggro, catalog, counterburn, erhnamgeddon, goblins, gr_aggro, jeskai_aggro,
        lions_dib, lions_dib_bolt, mono_black, robots, sligh, the_deck, troll_disk, white_weenie,
    };
    use crate::rules;
    use crate::{CardBehavior, CardDefinitionId, CardSet, CreatureStats, ManaCost};

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
        let mut scryfall_ids = HashSet::new();
        for raw_id in 1..=128 {
            let card = catalog.get(CardDefinitionId(raw_id)).unwrap();
            assert_ne!(card.behavior, CardBehavior::Unsupported, "{}", card.name);
            assert_eq!(card.rules, *card.behavior.rules(), "{}", card.name);
            assert!(
                !card.rules.text.is_empty(),
                "{} is missing rules text",
                card.name
            );
            assert!(
                card.scryfall_id.as_deref().is_some_and(is_uuid),
                "{} has an invalid Scryfall ID",
                card.name
            );
            let scryfall_id = card.scryfall_id.as_deref().unwrap();
            assert!(
                scryfall_ids.insert(scryfall_id),
                "{} repeats Scryfall ID {}",
                card.name,
                scryfall_id
            );
            assert!(
                card.artist
                    .as_deref()
                    .is_some_and(|artist| !artist.trim().is_empty()),
                "{} is missing its artist",
                card.name
            );
        }
    }

    fn is_uuid(value: &str) -> bool {
        value.len() == 36
            && value.bytes().enumerate().all(|(index, byte)| match index {
                8 | 13 | 18 | 23 => byte == b'-',
                _ => byte.is_ascii_hexdigit(),
            })
    }

    #[test]
    fn canonical_art_ids_and_debut_sets_are_correct() {
        let catalog = catalog().unwrap();
        for (id, scryfall_id, set) in [
            (
                cards::VOLCANIC_ISLAND,
                "0324641d-af55-4c53-b4dc-c8262e967da5",
                CardSet::Beta,
            ),
            (
                cards::ENERGY_FLUX,
                "bd1f624b-e8f2-462f-838a-7cb9e8fda988",
                CardSet::Antiquities,
            ),
            (
                cards::SEDGE_TROLL,
                "02ec317b-52a6-4490-80e5-a56826b06771",
                CardSet::Alpha,
            ),
        ] {
            let card = catalog.get(id).unwrap();
            assert_eq!(card.scryfall_id.as_deref(), Some(scryfall_id));
            assert_eq!(card.set, set);
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
