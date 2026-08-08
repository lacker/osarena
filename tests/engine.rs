use penta::card::{CardBehavior, CardCatalog, CardDefinition, CardPrinting, CardSet};
use penta::deck::{Deck, DeckError};
use penta::game::{GameResult, WinReason};
use penta::poc;
use penta::{
    AbilityOrigin, Action, BasicLandType, CardDefinitionId, CastChoices, Format, Game, GameError,
    GameEvent, PlayOptionId, PlayerId, Step, Target, TargetSelection, TargetSlotId,
};

fn catalog() -> CardCatalog {
    CardCatalog::new([
        CardDefinition::new(
            CardDefinitionId(1),
            "Mountain",
            CardSet::Alpha,
            true,
            CardBehavior::Mountain,
        ),
        CardDefinition::new(
            CardDefinitionId(2),
            "Lightning Bolt",
            CardSet::Alpha,
            false,
            CardBehavior::LightningBolt,
        ),
        CardDefinition::new(
            CardDefinitionId(3),
            "Black Lotus",
            CardSet::Alpha,
            false,
            CardBehavior::Unsupported,
        ),
        CardDefinition::new(
            CardDefinitionId(4),
            "Contract from Below",
            CardSet::Alpha,
            false,
            CardBehavior::Unsupported,
        ),
        CardDefinition::new(
            CardDefinitionId(5),
            "Standard Test Spell",
            CardSet::Innistrad,
            false,
            CardBehavior::Unsupported,
        ),
    ])
    .unwrap()
}

fn valid_deck() -> Deck {
    let mut main = vec![CardDefinitionId(1); 55];
    main.extend([CardDefinitionId(2); 4]);
    main.push(CardDefinitionId(3));
    Deck {
        main,
        sideboard: Vec::new(),
    }
}

fn game_with_mountain_and_bolt() -> Game {
    let catalog = catalog();
    for seed in 0..1_000 {
        let mut game = Game::new(catalog.clone(), [valid_deck(), valid_deck()], seed).unwrap();
        let hand = &game.observe(PlayerId::One).hand;
        let has_mountain = hand
            .iter()
            .any(|(_, definition)| *definition == CardDefinitionId(1));
        let has_bolt = hand
            .iter()
            .any(|(_, definition)| *definition == CardDefinitionId(2));
        if has_mountain && has_bolt {
            keep_both(&mut game);
            return game;
        }
    }
    panic!("expected to find a deterministic seed with Mountain and Lightning Bolt");
}

fn keep_both(game: &mut Game) {
    game.apply(PlayerId::One, Action::KeepHand).unwrap();
    game.apply(PlayerId::Two, Action::KeepHand).unwrap();
}

#[test]
fn decision_player_tracks_pregame_priority_and_turn_based_choices() {
    let mut game = Game::new(catalog(), [valid_deck(), valid_deck()], 0).unwrap();
    assert_eq!(game.decision_player(), Some(PlayerId::One));

    game.apply(PlayerId::One, Action::KeepHand).unwrap();
    assert_eq!(game.decision_player(), Some(PlayerId::Two));
    game.apply(PlayerId::Two, Action::KeepHand).unwrap();
    assert_eq!(game.decision_player(), Some(PlayerId::One));

    pass_priority_pair(&mut game);
    pass_priority_pair(&mut game);
    pass_priority_pair(&mut game);
    pass_priority_pair(&mut game);
    assert_eq!(game.observe(PlayerId::One).step, Step::DeclareAttackers);
    assert_eq!(game.decision_player(), Some(PlayerId::One));

    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.observe(PlayerId::One).step, Step::DeclareBlockers);
    assert_eq!(game.decision_player(), Some(PlayerId::Two));

    game.apply(PlayerId::Two, Action::Concede).unwrap();
    assert_eq!(game.decision_player(), None);
}

fn pass_priority_pair(game: &mut Game) {
    let first = game.observe(PlayerId::One).priority;
    game.apply(first, Action::PassPriority).unwrap();
    game.apply(first.opponent(), Action::PassPriority).unwrap();
}

fn advance_to_first_main(game: &mut Game) {
    assert_eq!(game.observe(PlayerId::One).step, Step::Upkeep);
    pass_priority_pair(game);
    assert_eq!(game.observe(PlayerId::One).step, Step::Draw);
    pass_priority_pair(game);
    assert_eq!(game.observe(PlayerId::One).step, Step::PrecombatMain);
}

#[test]
fn restricted_cards_are_limited_across_deck_and_sideboard() {
    let catalog = catalog();
    let mut main = vec![CardDefinitionId(1); 58];
    main.extend([CardDefinitionId(3); 2]);
    let error = Deck {
        main,
        sideboard: Vec::new(),
    }
    .validate(&catalog)
    .unwrap_err();

    assert_eq!(
        error,
        DeckError::TooManyCopies {
            card: "Black Lotus".into(),
            count: 2,
            limit: 1,
        }
    );
}

#[test]
fn banned_cards_are_rejected() {
    let catalog = catalog();
    let mut main = vec![CardDefinitionId(1); 59];
    main.push(CardDefinitionId(4));

    assert_eq!(
        Deck {
            main,
            sideboard: Vec::new(),
        }
        .validate(&catalog)
        .unwrap_err(),
        DeckError::BannedCard("Contract from Below".into())
    );
}

#[test]
fn deck_validation_uses_the_selected_formats_card_pool() {
    let catalog = catalog();
    let mut main = vec![CardDefinitionId(1); 59];
    main.push(CardDefinitionId(5));
    let standard_deck = Deck {
        main,
        sideboard: Vec::new(),
    };

    assert_eq!(
        standard_deck.clone().validate(&catalog).unwrap_err(),
        DeckError::CardNotAllowed {
            card: "Standard Test Spell".into(),
            format: Format::OldSchool9394,
        }
    );
    standard_deck
        .validate_for_format(&catalog, Format::IsdRtrStandard)
        .unwrap();

    let mut main = vec![CardDefinitionId(1); 59];
    main.push(CardDefinitionId(2));
    assert_eq!(
        Deck {
            main,
            sideboard: Vec::new(),
        }
        .validate_for_format(&catalog, Format::IsdRtrStandard)
        .unwrap_err(),
        DeckError::CardNotAllowed {
            card: "Lightning Bolt".into(),
            format: Format::IsdRtrStandard,
        }
    );
}

#[test]
fn deck_validation_uses_reprints_without_splitting_copy_identity() {
    let mountain = CardDefinition::new(
        CardDefinitionId(1),
        "Mountain",
        CardSet::Alpha,
        true,
        CardBehavior::Mountain,
    );
    let bolt = CardDefinition::new(
        CardDefinitionId(2),
        "Lightning Bolt",
        CardSet::Alpha,
        false,
        CardBehavior::LightningBolt,
    );
    let catalog = CardCatalog::with_additional_printings(
        [mountain, bolt],
        [
            CardPrinting::new(CardDefinitionId(2), CardSet::Magic2014),
            CardPrinting::with_variant(CardDefinitionId(2), CardSet::Magic2014, 1),
        ],
    )
    .unwrap();

    let legal = Deck {
        main: [vec![CardDefinitionId(1); 56], vec![CardDefinitionId(2); 4]].concat(),
        sideboard: Vec::new(),
    };
    legal
        .validate_for_format(&catalog, Format::IsdRtrStandard)
        .unwrap();

    let too_many = Deck {
        main: [vec![CardDefinitionId(1); 55], vec![CardDefinitionId(2); 5]].concat(),
        sideboard: Vec::new(),
    };
    assert_eq!(
        too_many
            .validate_for_format(&catalog, Format::IsdRtrStandard)
            .unwrap_err(),
        DeckError::TooManyCopies {
            card: "Lightning Bolt".into(),
            count: 5,
            limit: 4,
        }
    );
    assert_eq!(
        catalog.find_by_name("lightning bolt"),
        Some(CardDefinitionId(2))
    );
}

#[test]
fn setup_is_deterministic_and_hides_the_opponents_hand() {
    let catalog = catalog();
    let game_a = Game::new(catalog.clone(), [valid_deck(), valid_deck()], 0xdeca_fbad).unwrap();
    let game_b = Game::new(catalog.clone(), [valid_deck(), valid_deck()], 0xdeca_fbad).unwrap();

    assert_eq!(game_a.format(), Format::OldSchool9394);
    assert_eq!(game_a.observe(PlayerId::One), game_b.observe(PlayerId::One));
    let observation = game_a.observe(PlayerId::One);
    assert_eq!(observation.hand.len(), 7);
    assert_eq!(observation.opponent_hand_size, 7);
    assert_eq!(observation.library_sizes, [53, 53]);
}

#[test]
fn concession_ends_the_game() {
    let catalog = catalog();
    let mut game = Game::new(catalog.clone(), [valid_deck(), valid_deck()], 123).unwrap();

    game.apply(PlayerId::One, Action::Concede).unwrap();

    let observation = game.observe(PlayerId::Two);
    assert_eq!(
        observation.result,
        Some(GameResult::Winner {
            winner: PlayerId::Two,
            reason: WinReason::OpponentConceded,
        })
    );
    assert!(observation.legal_actions.is_empty());
}

#[test]
fn only_the_priority_player_can_take_game_actions() {
    let game = game_with_mountain_and_bolt();

    assert_eq!(game.legal_actions(PlayerId::Two), vec![Action::Concede]);
    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&Action::PassPriority)
    );
}

#[test]
fn first_player_skips_the_first_draw() {
    let mut game = game_with_mountain_and_bolt();
    let initial_hand_size = game.observe(PlayerId::One).hand.len();

    pass_priority_pair(&mut game);

    let observation = game.observe(PlayerId::One);
    assert_eq!(observation.step, Step::Draw);
    assert_eq!(observation.hand.len(), initial_hand_size);
}

#[test]
fn london_mulligan_redraws_seven_then_bottoms_one() {
    let catalog = catalog();
    let mut game = Game::new(catalog, [valid_deck(), valid_deck()], 77).unwrap();

    game.apply(PlayerId::One, Action::TakeMulligan).unwrap();
    assert_eq!(game.observe(PlayerId::One).hand.len(), 7);
    game.apply(PlayerId::One, Action::KeepHand).unwrap();

    let bottom = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::BottomCards { .. }))
        .unwrap();
    game.apply(PlayerId::One, bottom).unwrap();
    assert_eq!(game.observe(PlayerId::One).hand.len(), 6);

    game.apply(PlayerId::Two, Action::KeepHand).unwrap();
    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&Action::PassPriority)
    );
}

#[test]
fn mountain_casts_and_resolves_lightning_bolt() {
    let mut game = game_with_mountain_and_bolt();
    advance_to_first_main(&mut game);

    let observation = game.observe(PlayerId::One);
    let mountain = observation
        .hand
        .iter()
        .find(|(_, definition)| *definition == CardDefinitionId(1))
        .unwrap()
        .0;
    let bolt = observation
        .hand
        .iter()
        .find(|(_, definition)| *definition == CardDefinitionId(2))
        .unwrap()
        .0;

    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: mountain,
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();
    let mountain = game
        .observe(PlayerId::One)
        .battlefield
        .iter()
        .find(|permanent| permanent.definition == CardDefinitionId(1))
        .unwrap()
        .id;
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: mountain,
            ability: AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain),
            color: penta::ManaColor::Red,
        },
    )
    .unwrap();
    game.apply(
        PlayerId::One,
        Action::CastSpell {
            card: bolt,
            choices: CastChoices::default()
                .with_x(0)
                .with_targets(vec![TargetSelection::new(
                    TargetSlotId(0),
                    vec![Target::Player(PlayerId::Two)],
                )]),
            sacrifices: Vec::new(),
        },
    )
    .unwrap();

    let on_stack = game.observe(PlayerId::One);
    assert_eq!(on_stack.stack.len(), 1);
    let spell_id = on_stack.stack[0].id;
    assert_ne!(spell_id, bolt);
    assert_eq!(on_stack.life_totals, [20, 20]);
    assert_eq!(on_stack.mana_pools[0].red, 0);

    pass_priority_pair(&mut game);

    let resolved = game.observe(PlayerId::One);
    assert!(resolved.stack.is_empty());
    assert_eq!(resolved.life_totals, [20, 17]);
    assert_eq!(resolved.graveyards[0][0].1, CardDefinitionId(2));
    assert_ne!(resolved.graveyards[0][0].0, spell_id);
    assert!(game.events().contains(&GameEvent::DamageDealt {
        player: PlayerId::Two,
        amount: 3,
    }));
}

#[test]
fn nonbasic_lands_cannot_be_cast_as_spells() {
    use poc::cards::{MISHRA_S_FACTORY, STRIP_MINE};

    let catalog = poc::catalog().unwrap();
    for definition in [MISHRA_S_FACTORY, STRIP_MINE] {
        let mut checked = false;
        for seed in 0..1_000 {
            let deck = poc::artifacts();
            let mut game = Game::new(catalog.clone(), [deck.clone(), deck], seed).unwrap();
            if !game
                .observe(PlayerId::One)
                .hand
                .iter()
                .any(|(_, card)| *card == definition)
            {
                continue;
            }

            keep_both(&mut game);
            advance_to_first_main(&mut game);
            let observation = game.observe(PlayerId::One);
            let card = observation
                .hand
                .iter()
                .find(|(_, card)| *card == definition)
                .unwrap()
                .0;
            let actions = game.legal_actions(PlayerId::One);

            assert!(actions.contains(&Action::PlayLand {
                card,
                option: PlayOptionId::DEFAULT,
            }));
            assert!(!actions.iter().any(
                |action| matches!(action, Action::CastSpell { card: cast, .. } if *cast == card)
            ));
            checked = true;
            break;
        }
        assert!(checked, "expected a seed with nonbasic land {definition:?}");
    }
}

#[test]
fn unspent_mana_burns_at_the_end_of_a_phase() {
    let mut game = game_with_mountain_and_bolt();
    advance_to_first_main(&mut game);
    let mountain = game
        .observe(PlayerId::One)
        .hand
        .iter()
        .find(|(_, definition)| *definition == CardDefinitionId(1))
        .unwrap()
        .0;

    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: mountain,
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();
    let mountain = game
        .observe(PlayerId::One)
        .battlefield
        .iter()
        .find(|permanent| permanent.definition == CardDefinitionId(1))
        .unwrap()
        .id;
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: mountain,
            ability: AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain),
            color: penta::ManaColor::Red,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let observation = game.observe(PlayerId::One);
    assert_eq!(observation.step, Step::BeginningOfCombat);
    assert_eq!(observation.life_totals, [19, 20]);
    assert_eq!(observation.mana_pools[0].red, 0);
    assert!(game.events().contains(&GameEvent::ManaBurn {
        player: PlayerId::One,
        amount: 1,
    }));
}

#[test]
fn mana_emptying_and_burn_follow_the_games_format() {
    let catalog = catalog();
    let deck = Deck {
        main: vec![CardDefinitionId(1); 60],
        sideboard: Vec::new(),
    };

    for (format, expected_mana) in [(Format::OldSchool9394, 1), (Format::IsdRtrStandard, 0)] {
        let mut game =
            Game::new_with_format(format, catalog.clone(), [deck.clone(), deck.clone()], 0)
                .unwrap();
        assert_eq!(game.format(), format);
        keep_both(&mut game);
        advance_to_first_main(&mut game);

        let mountain = game.observe(PlayerId::One).hand[0].0;
        game.apply(
            PlayerId::One,
            Action::PlayLand {
                card: mountain,
                option: PlayOptionId::DEFAULT,
            },
        )
        .unwrap();
        let mountain = game.observe(PlayerId::One).battlefield.first().unwrap().id;
        pass_priority_pair(&mut game);
        assert_eq!(game.observe(PlayerId::One).step, Step::BeginningOfCombat);

        game.apply(
            PlayerId::One,
            Action::ActivateManaAbility {
                source: mountain,
                ability: AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain),
                color: penta::ManaColor::Red,
            },
        )
        .unwrap();
        pass_priority_pair(&mut game);

        let observation = game.observe(PlayerId::One);
        assert_eq!(observation.step, Step::DeclareAttackers);
        assert_eq!(observation.mana_pools[0].red, expected_mana);
        assert_eq!(observation.life_totals, [20, 20]);
        assert!(
            !game
                .events()
                .iter()
                .any(|event| matches!(event, GameEvent::ManaBurn { .. }))
        );
    }
}

#[test]
fn game_validates_decks_against_its_own_catalog() {
    let catalog = catalog();
    let short_deck = Deck {
        main: vec![CardDefinitionId(1); 59],
        sideboard: Vec::new(),
    };

    assert!(matches!(
        Game::new(catalog, [short_deck, valid_deck()], 0),
        Err(GameError::InvalidDeck {
            player: PlayerId::One,
            error: DeckError::MainDeckTooSmall {
                actual: 59,
                minimum: 60,
            },
        })
    ));
}

#[test]
fn drawing_from_an_empty_library_loses_the_game() {
    let catalog = catalog();
    let mut game = Game::new(catalog, [valid_deck(), valid_deck()], 0).unwrap();
    keep_both(&mut game);

    for _ in 0..3_000 {
        if game.result().is_some() {
            break;
        }
        let (actor, action) = [PlayerId::One, PlayerId::Two]
            .into_iter()
            .find_map(|actor| {
                game.legal_actions(actor)
                    .into_iter()
                    .find(|action| !matches!(action, Action::Concede))
                    .map(|action| (actor, action))
            })
            .unwrap();
        game.apply(actor, action).unwrap();
    }

    assert_eq!(
        game.result(),
        Some(GameResult::Winner {
            winner: PlayerId::One,
            reason: WinReason::OpponentTriedToDrawFromEmptyLibrary,
        })
    );
}

#[test]
#[ignore = "slow simulation sweep"]
fn all_builtin_deck_matchups_complete_under_deterministic_greedy_bots() {
    let decks = [
        ("Goblins", penta::poc::goblins()),
        ("Sligh", penta::poc::sligh()),
        ("Artifacts", penta::poc::artifacts()),
        ("Robots", penta::poc::robots()),
        ("The Deck", penta::poc::the_deck()),
        ("Mono Black", penta::poc::mono_black()),
        ("White Weenie", penta::poc::white_weenie()),
        ("Erhnamgeddon", penta::poc::erhnamgeddon()),
        ("Counterburn", penta::poc::counterburn()),
        ("Lions/Dib", penta::poc::lions_dib()),
    ];
    for (first_name, first_deck) in &decks {
        for (second_name, second_deck) in &decks {
            let mut game = Game::new(
                penta::poc::catalog().unwrap(),
                [first_deck.clone(), second_deck.clone()],
                2_026,
            )
            .unwrap();
            for _ in 0..50_000 {
                let Some(player) = game.decision_player() else {
                    break;
                };
                let action = choose_greedy_action(&game, player).unwrap();
                game.apply(player, action).unwrap();
            }
            assert!(
                game.result().is_some(),
                "{first_name} versus {second_name} did not terminate"
            );
        }
    }
}

fn choose_greedy_action(game: &Game, player: PlayerId) -> Option<Action> {
    if let Some(decision) = game.observe(player).decision {
        return Some(Action::ChooseDecision {
            decision: decision.id,
            options: decision
                .options
                .iter()
                .take(decision.minimum)
                .map(|option| option.id)
                .collect(),
        });
    }
    let actions = game.legal_actions(player);
    let choose = |predicate: &dyn Fn(&Action) -> bool| {
        actions.iter().find(|action| predicate(action)).cloned()
    };

    choose(&|action| matches!(action, Action::KeepHand))
        .or_else(|| choose(&|action| matches!(action, Action::BottomCards { .. })))
        .or_else(|| choose(&|action| matches!(action, Action::DiscardCards { .. })))
        .or_else(|| {
            actions
                .iter()
                .filter_map(|action| match action {
                    Action::ChooseUntap { permanents } => Some((permanents.len(), action.clone())),
                    _ => None,
                })
                .max_by_key(|(count, _)| *count)
                .map(|(_, action)| action)
        })
        .or_else(|| choose(&|action| matches!(action, Action::PlayLand { .. })))
        .or_else(|| choose(&|action| matches!(action, Action::ActivateManaAbility { .. })))
        .or_else(|| {
            actions
                .iter()
                .filter_map(|action| match action {
                    Action::CastSpell { choices, .. } => {
                        let attacks_opponent = choices
                            .iter_targets()
                            .any(|target| *target == Target::Player(player.opponent()));
                        Some((attacks_opponent, choices.x(), action.clone()))
                    }
                    _ => None,
                })
                .max_by_key(|(attacks_opponent, x, _)| (*attacks_opponent, *x))
                .map(|(_, _, action)| action)
        })
        .or_else(|| choose(&|action| matches!(action, Action::ActivateAbility { .. })))
        .or_else(|| choose(&|action| matches!(action, Action::DeclareAttacker { .. })))
        .or_else(|| choose(&|action| matches!(action, Action::FinishDeclaringAttackers)))
        .or_else(|| choose(&|action| matches!(action, Action::DeclareBlocker { .. })))
        .or_else(|| choose(&|action| matches!(action, Action::FinishDeclaringBlockers)))
        .or_else(|| choose(&|action| matches!(action, Action::AssignCombatDamage { .. })))
        .or_else(|| choose(&|action| matches!(action, Action::PassPriority)))
}
