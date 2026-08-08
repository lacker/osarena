use penta::game::PermanentObservation;
use penta::poc;
use penta::{
    AbilityId, AbilityOrigin, Action, BasicLandType, CardInstanceId, CardPartId, CastChoices, Game,
    GameResult, HandcraftedPolicy, ManaPool, PlayOptionId, PlayerId, PlayerObservation, Policy,
    RandomPolicy, Step, Target, TargetSelection, TargetSlotId, play_game,
};

const ACTION_LIMIT: usize = 50_000;
const PRIMARY_PRINTED_ABILITY: AbilityOrigin = AbilityOrigin::Printed {
    definition: penta::CardDefinitionId(0),
    part: CardPartId::PRIMARY,
    ability: AbilityId::PRIMARY,
};

fn activated_targets(target: Target) -> Vec<TargetSelection> {
    vec![TargetSelection::single(TargetSlotId(0), target)]
}

fn policy_observation(
    battlefield: Vec<PermanentObservation>,
    legal_actions: Vec<Action>,
) -> PlayerObservation {
    PlayerObservation {
        viewer: PlayerId::One,
        turn: 3,
        active_turn: 2,
        active_player: PlayerId::One,
        priority: PlayerId::One,
        step: Step::PrecombatMain,
        life_totals: [20, 20],
        mana_pools: [ManaPool::default(), ManaPool::default()],
        hand: Vec::new(),
        opponent_hand_size: 0,
        last_seen_hand: None,
        library_sizes: [50, 50],
        graveyards: [Vec::new(), Vec::new()],
        exiles: [Vec::new(), Vec::new()],
        battlefield,
        stack: Vec::new(),
        decision: None,
        result: None,
        legal_actions,
    }
}

fn permanent(
    id: u32,
    definition: penta::CardDefinitionId,
    controller: PlayerId,
    power: Option<i16>,
    toughness: Option<i16>,
) -> PermanentObservation {
    PermanentObservation {
        id: CardInstanceId(id),
        definition,
        presented: CardPartId::PRIMARY,
        controller,
        tapped: false,
        power,
        toughness,
        damage: 0,
        attacking: false,
        blocking: None,
        flying: false,
        can_attack: false,
        entered_this_turn: false,
    }
}

#[test]
fn random_policy_is_seeded_and_avoids_conceding() {
    let catalog = poc::catalog().unwrap();
    let game = Game::new(catalog, [poc::goblins(), poc::goblins()], 17).unwrap();
    let observation = game.observe(PlayerId::One);
    let mut first = RandomPolicy::new(99);
    let mut second = RandomPolicy::new(99);

    for _ in 0..20 {
        let first_action = first.choose_action(&observation);
        let second_action = second.choose_action(&observation);
        assert_eq!(first_action, second_action);
        assert!(!matches!(first_action, Some(Action::Concede)));
    }
}

#[test]
fn handcrafted_does_not_float_unneeded_mana_in_its_main_phase() {
    let catalog = poc::catalog().unwrap();
    let mountain = permanent(1, poc::cards::MOUNTAIN, PlayerId::One, None, None);
    let observation = policy_observation(
        vec![mountain],
        vec![
            Action::PassPriority,
            Action::ActivateManaAbility {
                source: CardInstanceId(1),
                ability: AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain),
                color: penta::ManaColor::Red,
            },
        ],
    );
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::PassPriority)
    );
}

#[test]
fn handcrafted_does_not_feed_a_creature_to_a_superior_blocker() {
    let catalog = poc::catalog().unwrap();
    let attacker = permanent(
        1,
        poc::cards::GOBLIN_BALLOON_BRIGADE,
        PlayerId::One,
        Some(1),
        Some(1),
    );
    let blocker = permanent(2, poc::cards::SU_CHI, PlayerId::Two, Some(4), Some(4));
    let mut observation = policy_observation(
        vec![attacker, blocker],
        vec![
            Action::FinishDeclaringAttackers,
            Action::DeclareAttacker {
                attacker: CardInstanceId(1),
            },
        ],
    );
    observation.step = Step::DeclareAttackers;
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::FinishDeclaringAttackers)
    );
}

#[test]
fn handcrafted_does_not_add_a_redundant_lethal_blocker() {
    let catalog = poc::catalog().unwrap();
    let mut attacker = permanent(
        1,
        poc::cards::GOBLIN_BALLOON_BRIGADE,
        PlayerId::Two,
        Some(1),
        Some(1),
    );
    attacker.attacking = true;
    let mut first_blocker = permanent(
        2,
        poc::cards::GOBLIN_BALLOON_BRIGADE,
        PlayerId::One,
        Some(1),
        Some(1),
    );
    first_blocker.blocking = Some(CardInstanceId(1));
    let second_blocker = permanent(
        3,
        poc::cards::GOBLIN_BALLOON_BRIGADE,
        PlayerId::One,
        Some(1),
        Some(1),
    );
    let mut observation = policy_observation(
        vec![attacker, first_blocker, second_blocker],
        vec![
            Action::FinishDeclaringBlockers,
            Action::DeclareBlocker {
                blocker: CardInstanceId(3),
                attacker: CardInstanceId(1),
            },
        ],
    );
    observation.step = Step::DeclareBlockers;
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::FinishDeclaringBlockers)
    );
}

#[test]
fn handcrafted_deploys_a_creature_before_burning_a_nonlethal_player() {
    let catalog = poc::catalog().unwrap();
    let bolt = CardInstanceId(1);
    let goblin = CardInstanceId(2);
    let cast_bolt = Action::CastSpell {
        card: bolt,
        choices: CastChoices::default()
            .with_x(0)
            .with_targets(vec![TargetSelection::new(
                TargetSlotId(0),
                vec![Target::Player(PlayerId::Two)],
            )]),
        sacrifices: Vec::new(),
    };
    let cast_goblin = Action::CastSpell {
        card: goblin,
        choices: CastChoices::default().with_x(0).with_targets(Vec::new()),
        sacrifices: Vec::new(),
    };
    let mut observation = policy_observation(
        Vec::new(),
        vec![Action::PassPriority, cast_bolt, cast_goblin.clone()],
    );
    observation.hand = vec![
        (bolt, poc::cards::LIGHTNING_BOLT),
        (goblin, poc::cards::GOBLIN_BALLOON_BRIGADE),
    ];
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(policy.choose_action(&observation), Some(cast_goblin));
}

#[test]
fn handcrafted_never_burns_itself_when_the_opponent_is_a_legal_target() {
    let catalog = poc::catalog().unwrap();
    let bolt = CardInstanceId(1);
    let hit_self = Action::CastSpell {
        card: bolt,
        choices: CastChoices::default()
            .with_x(0)
            .with_targets(vec![TargetSelection::new(
                TargetSlotId(0),
                vec![Target::Player(PlayerId::One)],
            )]),
        sacrifices: Vec::new(),
    };
    let hit_opponent = Action::CastSpell {
        card: bolt,
        choices: CastChoices::default()
            .with_x(0)
            .with_targets(vec![TargetSelection::new(
                TargetSlotId(0),
                vec![Target::Player(PlayerId::Two)],
            )]),
        sacrifices: Vec::new(),
    };
    let mut observation = policy_observation(
        Vec::new(),
        vec![Action::PassPriority, hit_opponent.clone(), hit_self],
    );
    observation.hand = vec![(bolt, poc::cards::LIGHTNING_BOLT)];
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(policy.choose_action(&observation), Some(hit_opponent));
}

#[test]
fn handcrafted_plays_a_mountain_before_a_colorless_land() {
    let catalog = poc::catalog().unwrap();
    let strip = CardInstanceId(1);
    let mountain = CardInstanceId(2);
    let mut observation = policy_observation(
        Vec::new(),
        vec![
            Action::PlayLand {
                card: strip,
                option: PlayOptionId::DEFAULT,
            },
            Action::PlayLand {
                card: mountain,
                option: PlayOptionId::DEFAULT,
            },
        ],
    );
    observation.hand = vec![
        (strip, poc::cards::STRIP_MINE),
        (mountain, poc::cards::MOUNTAIN),
    ];
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::PlayLand {
            card: mountain,
            option: PlayOptionId::DEFAULT,
        })
    );
}

#[test]
fn handcrafted_only_uses_orcish_mechanics_on_a_player_for_lethal() {
    let catalog = poc::catalog().unwrap();
    let mechanics = CardInstanceId(1);
    let vise = CardInstanceId(2);
    let observation = policy_observation(
        vec![
            permanent(
                1,
                poc::cards::ORCISH_MECHANICS,
                PlayerId::One,
                Some(1),
                Some(1),
            ),
            permanent(2, poc::cards::BLACK_VISE, PlayerId::One, None, None),
        ],
        vec![
            Action::PassPriority,
            Action::ActivateAbility {
                source: mechanics,
                ability: PRIMARY_PRINTED_ABILITY,
                targets: activated_targets(Target::Player(PlayerId::Two)),
                sacrifice: Some(vise),
            },
        ],
    );
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::PassPriority)
    );
}

#[test]
fn handcrafted_sacrifices_artifacts_to_atog_for_an_unblocked_lethal_attack() {
    let catalog = poc::catalog().unwrap();
    let atog = CardInstanceId(1);
    let vise = CardInstanceId(2);
    let mox = CardInstanceId(3);
    let mut attacking_atog = permanent(1, poc::cards::ATOG, PlayerId::One, Some(1), Some(2));
    attacking_atog.attacking = true;
    let mut observation = policy_observation(
        vec![
            attacking_atog,
            permanent(2, poc::cards::BLACK_VISE, PlayerId::One, None, None),
            permanent(3, poc::cards::MOX_RUBY, PlayerId::One, None, None),
        ],
        vec![
            Action::PassPriority,
            Action::ActivateAbility {
                source: atog,
                ability: PRIMARY_PRINTED_ABILITY,
                targets: Vec::new(),
                sacrifice: Some(vise),
            },
            Action::ActivateAbility {
                source: atog,
                ability: PRIMARY_PRINTED_ABILITY,
                targets: Vec::new(),
                sacrifice: Some(mox),
            },
        ],
    );
    observation.life_totals = [20, 5];
    observation.step = Step::DeclareBlockers;
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::ActivateAbility {
            source: atog,
            ability: PRIMARY_PRINTED_ABILITY,
            targets: Vec::new(),
            sacrifice: Some(vise),
        })
    );
}

#[test]
#[ignore = "slow simulation sweep"]
fn handcrafted_policy_decisively_beats_random_across_builtin_decks_and_seats() {
    let catalog = poc::catalog().unwrap();
    let decks = [
        poc::goblins(),
        poc::sligh(),
        poc::artifacts(),
        poc::robots(),
        poc::the_deck(),
    ];
    let mut wins = 0;
    let mut decided_games = 0;

    for deck in decks {
        for seed in 0..10 {
            for handcrafted_seat in [PlayerId::One, PlayerId::Two] {
                let mut game =
                    Game::new(catalog.clone(), [deck.clone(), deck.clone()], seed).unwrap();
                let mut handcrafted = HandcraftedPolicy::new(catalog.clone());
                let mut random = RandomPolicy::new(seed ^ 0xa11c_e5ed);
                let result = match handcrafted_seat {
                    PlayerId::One => {
                        play_game(&mut game, &mut handcrafted, &mut random, ACTION_LIMIT)
                    }
                    PlayerId::Two => {
                        play_game(&mut game, &mut random, &mut handcrafted, ACTION_LIMIT)
                    }
                }
                .unwrap();

                if let GameResult::Winner { winner, .. } = result {
                    decided_games += 1;
                    wins += usize::from(winner == handcrafted_seat);
                }
            }
        }
    }

    assert_eq!(decided_games, 100);
    assert!(
        wins >= 90,
        "handcrafted policy won only {wins} of {decided_games} games"
    );
}

#[test]
fn handcrafted_never_aims_removal_at_its_own_permanents() {
    let catalog = poc::catalog().unwrap();
    let own_angel = permanent(1, poc::cards::SERRA_ANGEL, PlayerId::One, Some(4), Some(4));
    let own_mox = permanent(2, poc::cards::MOX_PEARL, PlayerId::One, None, None);
    let swords = CardInstanceId(10);
    let disenchant = CardInstanceId(11);
    let mut observation = policy_observation(
        vec![own_angel, own_mox],
        vec![
            Action::PassPriority,
            Action::CastSpell {
                card: swords,
                choices: CastChoices::default()
                    .with_x(0)
                    .with_targets(vec![TargetSelection::new(
                        TargetSlotId(0),
                        vec![Target::Permanent(CardInstanceId(1))],
                    )]),
                sacrifices: Vec::new(),
            },
            Action::CastSpell {
                card: disenchant,
                choices: CastChoices::default()
                    .with_x(0)
                    .with_targets(vec![TargetSelection::new(
                        TargetSlotId(0),
                        vec![Target::Permanent(CardInstanceId(2))],
                    )]),
                sacrifices: Vec::new(),
            },
        ],
    );
    observation.hand = vec![
        (swords, poc::cards::SWORDS_TO_PLOWSHARES),
        (disenchant, poc::cards::DISENCHANT),
    ];
    let mut policy = HandcraftedPolicy::new(catalog);

    // Removal carries a large base score, so a merely-unattractive friendly
    // target used to stay far above passing.
    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::PassPriority),
        "the only targets on offer are its own board, so it should hold the removal",
    );
}

#[test]
fn handcrafted_still_spends_removal_on_the_opponent() {
    let catalog = poc::catalog().unwrap();
    let own_angel = permanent(1, poc::cards::SERRA_ANGEL, PlayerId::One, Some(4), Some(4));
    let their_angel = permanent(3, poc::cards::SERRA_ANGEL, PlayerId::Two, Some(4), Some(4));
    let swords = CardInstanceId(10);
    let cast_at_theirs = Action::CastSpell {
        card: swords,
        choices: CastChoices::default()
            .with_x(0)
            .with_targets(vec![TargetSelection::new(
                TargetSlotId(0),
                vec![Target::Permanent(CardInstanceId(3))],
            )]),
        sacrifices: Vec::new(),
    };
    let mut observation = policy_observation(
        vec![own_angel, their_angel],
        vec![
            Action::PassPriority,
            Action::CastSpell {
                card: swords,
                choices: CastChoices::default()
                    .with_x(0)
                    .with_targets(vec![TargetSelection::new(
                        TargetSlotId(0),
                        vec![Target::Permanent(CardInstanceId(1))],
                    )]),
                sacrifices: Vec::new(),
            },
            cast_at_theirs.clone(),
        ],
    );
    observation.hand = vec![(swords, poc::cards::SWORDS_TO_PLOWSHARES)];
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(policy.choose_action(&observation), Some(cast_at_theirs));
}

#[test]
fn handcrafted_animates_a_factory_once_rather_than_every_priority() {
    let catalog = poc::catalog().unwrap();
    let animate = Action::ActivateAbility {
        source: CardInstanceId(1),
        ability: PRIMARY_PRINTED_ABILITY,
        targets: Vec::new(),
        sacrifice: None,
    };

    let dormant = permanent(1, poc::cards::MISHRA_S_FACTORY, PlayerId::One, None, None);
    let mut observation =
        policy_observation(vec![dormant], vec![Action::PassPriority, animate.clone()]);
    observation.step = Step::BeginningOfCombat;
    let mut policy = HandcraftedPolicy::new(catalog.clone());
    assert_eq!(
        policy.choose_action(&observation),
        Some(animate.clone()),
        "a dormant Factory is still worth animating",
    );

    // Same board, except the Factory is already a 2/2.
    let awake = permanent(
        1,
        poc::cards::MISHRA_S_FACTORY,
        PlayerId::One,
        Some(2),
        Some(2),
    );
    let mut observation = policy_observation(vec![awake], vec![Action::PassPriority, animate]);
    observation.step = Step::BeginningOfCombat;
    let mut policy = HandcraftedPolicy::new(catalog);
    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::PassPriority),
        "animating a Factory that is already a creature only burns mana",
    );
}
