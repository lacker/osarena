use super::*;
use crate::poc::{self, cards};
use crate::{
    AbilityTargetDef, AbilityTargetPredicate, AdditionalCostDef, AdditionalCostId,
    AlternativeCostDef, AlternativeCostId, CardComposition, CardDefinition, CardEffectStatus,
    CardInstanceId, CardPart, CardPartId, CardPrinting, CardRules, CardStructure, CastChoices,
    DoubleFacedKind, LandEntry, ManaSpendEffectDef, ModeDef, ModeSetDef, PlayOptionDef,
    PlayOptionId, PlayerRelation, SpellForm, StackObjectId, TargetPredicate, TargetSelection,
    TargetSlotDef, TargetSlotId,
};

static TEST_FLYING_ABILITY: [AbilityDef; 1] = [AbilityDef::evergreen(
    AbilityId::PRIMARY,
    "Flying",
    EvergreenAbility::Flying,
)];
static TEST_FLYING_TRAMPLE_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::evergreen(AbilityId::PRIMARY, "Flying", EvergreenAbility::Flying),
    AbilityDef::evergreen(AbilityId(1), "Trample", EvergreenAbility::Trample),
];

fn ready_game() -> Game {
    let deck = poc::mono_red_atog();
    let mut game = Game::new(poc::catalog().unwrap(), [deck.clone(), deck], 0).unwrap();
    game.pregame = None;
    game.step = Step::PrecombatMain;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.stack.clear();
    game.pending_decisions.clear();
    game.pending_combat_attackers.clear();
    for player in &mut game.players {
        player.hand.clear();
        player.graveyard.clear();
        player.exile.clear();
        player.life = i16::from(rules::STARTING_LIFE);
        player.mana_pool = ManaPool::default();
        player.mana.clear();
    }
    game
}

fn card(id: u32, definition: CardDefinitionId, owner: PlayerId) -> CardInstance {
    CardInstance {
        id: CardInstanceId(id),
        definition,
        owner,
        backing: ObjectBacking::Cards(vec![PhysicalCardId(id)]),
        characteristics: CharacteristicSource::Card(definition),
    }
}

fn creature(id: u32, definition: CardDefinitionId, controller: PlayerId) -> Permanent {
    Permanent {
        card: card(id, definition, controller),
        presented: CardPartId::PRIMARY,
        controller,
        tapped: false,
        entered_controller_turn: 0,
        damage: 0,
        loyalty: None,
        power_bonus: 0,
        toughness_bonus: 0,
        attacking: false,
        blocking: None,
        chosen_player: None,
        destroy_at_end: false,
        temporary_evergreen: Vec::new(),
        factory_animated: false,
        dragon_whelp_activations: 0,
        plus_one_counters: 0,
        javelin_counters: 0,
        combat_damage_assignment: Vec::new(),
        copied_from: None,
        regeneration_shields: 0,
        berserked: false,
        attacked_this_turn: false,
        forestwalk_until_upkeep_of: None,
        damage_sources: Vec::new(),
        deathtouch_damage: false,
    }
}

fn cast_choices(targets: Vec<Target>, x: u16) -> CastChoices {
    let choices = CastChoices::default().with_x(x);
    if targets.is_empty() {
        choices
    } else {
        choices.with_targets(vec![TargetSelection::new(TargetSlotId(0), targets)])
    }
}

fn cast_action(
    card: GameObjectId,
    targets: Vec<Target>,
    sacrifices: Vec<GameObjectId>,
    x: u16,
) -> Action {
    Action::CastSpell {
        card,
        choices: cast_choices(targets, x),
        sacrifices,
    }
}

fn activated_targets(target: Target) -> Vec<TargetSelection> {
    vec![TargetSelection::single(TargetSlotId(0), target)]
}

const fn primary_ability(definition: CardDefinitionId) -> AbilityOrigin {
    AbilityOrigin::Printed {
        definition,
        part: CardPartId::PRIMARY,
        ability: crate::AbilityId::PRIMARY,
    }
}

fn mana_ability_for(game: &Game, source: GameObjectId, color: ManaColor) -> AbilityOrigin {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == source)
        .into_iter()
        .flat_map(|permanent| game.mana_ability_activations(permanent))
        .find(|activation| activation.color == color)
        .expect("source has an effective mana ability for the requested color")
        .ability
}

fn activated_ability_for(game: &Game, source: GameObjectId, index: usize) -> AbilityOrigin {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == source)
        .expect("source is on the battlefield");
    game.activated_ability_origin(permanent, index)
}

fn synchronize_single_part_definition(definition: &mut CardDefinition) {
    let composition = CardComposition::single(definition.name.clone(), definition.rules);
    definition.parts = composition.parts;
    definition.structure = composition.structure;
    definition.play_options = composition.play_options;
}

fn spell(id: u32, definition: CardDefinitionId, controller: PlayerId, x: u16) -> StackObject {
    StackObject {
        id: StackObjectId(id),
        kind: StackObjectKind::Spell,
        card: card(id, definition, controller),
        source: None,
        ability: None,
        controller,
        signature: Some(CastSignature::from_validated_choices(
            SpellForm::Part(CardPartId::PRIMARY),
            cast_choices(Vec::new(), x),
        )),
        chosen_permanents: Vec::new(),
        applied_effects: Vec::new(),
        is_copy: false,
    }
}

fn spell_with_targets(
    id: u32,
    definition: CardDefinitionId,
    controller: PlayerId,
    targets: Vec<Target>,
    x: u16,
) -> StackObject {
    let mut object = spell(id, definition, controller, x);
    object.signature = Some(CastSignature::from_validated_choices(
        SpellForm::Part(CardPartId::PRIMARY),
        cast_choices(targets, x),
    ));
    object
}

fn pass_priority_pair(game: &mut Game) {
    let first = game.priority;
    game.apply(first, Action::PassPriority).unwrap();
    game.apply(first.opponent(), Action::PassPriority).unwrap();
}

#[test]
fn a_physical_card_gets_new_object_identity_in_each_cast_zone() {
    let mut game = ready_game();
    let card = card(10_000, cards::TRISKELION, PlayerId::One);
    let hand_id = card.id;
    let physical = backing_cards(&card.backing);
    game.players[0].hand.push(card);
    game.players[0].mana_pool.colorless = 6;

    game.apply(
        PlayerId::One,
        cast_action(hand_id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    let spell_id = game.stack[0].id;
    assert_ne!(spell_id, hand_id);
    assert_eq!(backing_cards(&game.stack[0].card.backing), physical);

    pass_priority_pair(&mut game);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::TRISKELION)
        .unwrap();
    assert_ne!(permanent.card.id, spell_id);
    assert_ne!(permanent.card.id, hand_id);
    assert_eq!(backing_cards(&permanent.card.backing), physical);
}

#[test]
fn a_forked_spell_has_new_identity_and_no_physical_backing() {
    let mut game = ready_game();
    let original = spell(77, cards::LIGHTNING_BOLT, PlayerId::Two, 0);
    let original_id = original.id;

    game.push_copy(original, PlayerId::One, Vec::new());

    let copied = game.stack.last().unwrap();
    assert_ne!(copied.id, original_id);
    assert_eq!(copied.card.backing, ObjectBacking::None);
    assert_eq!(
        copied.card.characteristics,
        CharacteristicSource::Copy(cards::LIGHTNING_BOLT)
    );
    assert_eq!(copied.card.owner, PlayerId::One);
    assert!(copied.is_copy);
}

#[test]
fn physical_card_metadata_is_separate_from_live_objects() {
    let game = ready_game();
    let physical = game.physical_cards[0].clone();
    assert_eq!(
        game.physical_card_definition(physical.id),
        Some(physical.definition)
    );
    assert_eq!(game.physical_card_owner(physical.id), Some(physical.owner));
}

#[test]
fn spell_events_keep_stack_identity_and_definition_after_the_card_moves() {
    let mut game = ready_game();
    let bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::One);
    let hand_id = bolt.id;
    game.players[0].hand.push(bolt);
    game.players[0].mana_pool.red = 1;
    let event_start = game.events.len();

    game.apply(
        PlayerId::One,
        cast_action(hand_id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    let stack_id = game.stack[0].id;
    assert_ne!(stack_id, hand_id);
    assert!(game.events[event_start..].contains(&GameEvent::SpellCast {
        player: PlayerId::One,
        card: stack_id,
        definition: cards::LIGHTNING_BOLT,
        targets: vec![Target::Player(PlayerId::Two)],
    }));

    pass_priority_pair(&mut game);
    assert!(
        game.events[event_start..].contains(&GameEvent::SpellResolved {
            card: stack_id,
            definition: cards::LIGHTNING_BOLT,
        })
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT && card.id != stack_id),
        "the event still names the former stack object after the card became a new object",
    );
}

#[test]
fn ability_events_distinguish_the_stack_object_from_a_source_that_left_play() {
    let mut game = ready_game();
    let strip = creature(10_000, cards::STRIP_MINE, PlayerId::One);
    let target = creature(10_001, cards::MOUNTAIN, PlayerId::Two);
    let source_id = strip.card.id;
    let target_id = target.card.id;
    game.battlefield = vec![strip, target];
    let event_start = game.events.len();

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: source_id,
            ability: activated_ability_for(&game, source_id, 0),
            targets: activated_targets(Target::Permanent(target_id)),
            sacrifice: None,
        },
    )
    .unwrap();
    let ability_id = game.stack[0].id;
    assert_eq!(
        game.stack[0].ability_origin(),
        Some(AbilityOrigin::Printed {
            definition: cards::STRIP_MINE,
            part: CardPartId::PRIMARY,
            ability: crate::AbilityId(1),
        })
    );
    assert_eq!(
        game.stack[0].ability_text(),
        Some("{T}, Sacrifice this land: Destroy target land.")
    );
    assert_ne!(ability_id, source_id);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != source_id),
        "the source has already left play when its activation is logged",
    );
    assert!(
        game.events[event_start..].contains(&GameEvent::AbilityActivated {
            player: PlayerId::One,
            object: ability_id,
            source: source_id,
            definition: cards::STRIP_MINE,
            chosen_permanents: vec![target_id],
        })
    );

    pass_priority_pair(&mut game);
    assert!(
        game.events[event_start..].contains(&GameEvent::AbilityResolved {
            object: ability_id,
            source: source_id,
            definition: cards::STRIP_MINE,
        })
    );
}

#[test]
fn recall_charges_two_generic_mana_for_each_x() {
    let cost = CardBehavior::Recall.mana_cost();
    assert!(can_pay(
        ManaPool {
            blue: 1,
            colorless: 6,
            ..ManaPool::default()
        },
        cost,
        3,
    ));
    assert!(!can_pay(
        ManaPool {
            blue: 1,
            colorless: 5,
            ..ManaPool::default()
        },
        cost,
        3,
    ));
}

#[test]
fn white_red_hybrid_symbols_accept_either_color_but_not_colorless() {
    let cost = ManaCost::white_red_hybrid(3);
    assert!(can_pay(
        ManaPool {
            white: 2,
            red: 1,
            ..ManaPool::default()
        },
        cost,
        0,
    ));
    assert!(can_pay(
        ManaPool {
            red: 3,
            ..ManaPool::default()
        },
        cost,
        0,
    ));
    assert!(!can_pay(
        ManaPool {
            colorless: 3,
            ..ManaPool::default()
        },
        cost,
        0,
    ));

    let mut pool = ManaPool {
        white: 2,
        red: 1,
        ..ManaPool::default()
    };
    pay_cost(&mut pool, cost, 0);
    assert_eq!(pool, ManaPool::default());
}

#[test]
fn declarative_mana_production_drives_generic_mana_sources() {
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_mana(
        crate::AbilityId::PRIMARY,
        "{T}: Add {U} or {R}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[
            ManaKindDef::Blue,
            ManaKindDef::Red,
        ])),
    )];
    let definition_id = CardDefinitionId(10_000);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test dual land",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules =
        CardRules::new(CardKind::Land, ManaCost::default(), "").with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    game.battlefield
        .push(creature(10_000, definition_id, PlayerId::One));

    let activations = game.mana_ability_activations(&game.battlefield[0]);
    assert_eq!(
        activations
            .iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Blue, ManaColor::Red]
    );
    let ability = mana_ability_for(&game, CardInstanceId(10_000), ManaColor::Blue);
    game.activate_mana_source(
        PlayerId::One,
        CardInstanceId(10_000),
        ability,
        ManaColor::Blue,
    );
    assert_eq!(game.players[0].mana_pool.blue, 1);
    assert!(game.battlefield[0].tapped);
}

#[test]
fn declarative_land_entry_handles_check_tapped_and_shock_lands() {
    let check_id = CardDefinitionId(10_000);
    let gate_id = CardDefinitionId(10_001);
    let shock_id = CardDefinitionId(10_002);
    let mut check = CardDefinition::new(
        check_id,
        "Test check land",
        CardSet::Magic2013,
        false,
        CardBehavior::Unsupported,
    );
    check.rules = CardRules::new(CardKind::Land, ManaCost::default(), "").land_entry(
        LandEntry::TappedUnlessControlsLandType([true, false, false, false, false]),
    );
    synchronize_single_part_definition(&mut check);
    let mut gate = CardDefinition::new(
        gate_id,
        "Test gate",
        CardSet::Gatecrash,
        false,
        CardBehavior::Unsupported,
    );
    gate.rules =
        CardRules::new(CardKind::Land, ManaCost::default(), "").land_entry(LandEntry::Tapped);
    synchronize_single_part_definition(&mut gate);
    let mut shock = CardDefinition::new(
        shock_id,
        "Test shock land",
        CardSet::Gatecrash,
        false,
        CardBehavior::Unsupported,
    );
    shock.rules = CardRules::new(CardKind::Land, ManaCost::default(), "")
        .with_subtypes(&["Plains", "Swamp"])
        .land_entry(LandEntry::PayLifeOrTapped(2));
    synchronize_single_part_definition(&mut shock);

    let plains = CardDefinition::new(
        cards::PLAINS,
        "Plains",
        CardSet::Alpha,
        true,
        CardBehavior::Plains,
    );
    let mut test_game = ready_game();
    test_game.catalog = CardCatalog::new([check, gate, shock, plains]).unwrap();
    test_game
        .battlefield
        .push(creature(9_999, cards::PLAINS, PlayerId::One));

    for (instance, definition) in [(10_000, check_id), (10_001, gate_id), (10_002, shock_id)] {
        test_game.players[0]
            .hand
            .push(card(instance, definition, PlayerId::One));
        test_game.play_land(
            PlayerId::One,
            CardInstanceId(instance),
            PlayOptionId::DEFAULT,
        );
    }

    assert!(!test_game.battlefield[1].tapped);
    assert!(test_game.battlefield[2].tapped);
    assert!(test_game.battlefield[3].tapped);
    assert_eq!(test_game.players[0].life, 20);
}

#[test]
fn a_land_play_option_locks_the_presented_part_on_the_permanent() {
    let definition_id = CardDefinitionId(10_100);
    let land_part = CardPartId(1);
    let land_option = PlayOptionId(1);
    let front_rules = CardRules::new(CardKind::Sorcery, ManaCost::new(1, 0), "Test front");
    let land_rules = CardRules::new(CardKind::Land, ManaCost::default(), "Test back")
        .land_entry(LandEntry::Tapped);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test modal card",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = front_rules;
    definition.parts = vec![
        CardPart::new(CardPartId::PRIMARY, "Test front", front_rules),
        CardPart::new(land_part, "Test back", land_rules).without_mana_cost(),
    ];
    definition.structure = CardStructure::DoubleFaced {
        front: CardPartId::PRIMARY,
        back: land_part,
        kind: DoubleFacedKind::Modal,
    };
    definition.play_options = vec![
        PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Cast Test front",
            SpellForm::Part(CardPartId::PRIMARY),
            front_rules.mana_cost,
            CardEffectStatus::MetadataOnly,
        ),
        PlayOptionDef::play_land(
            land_option,
            "Play Test back",
            land_part,
            CardEffectStatus::Implemented,
        ),
    ];

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    let card = card(10_100, definition_id, PlayerId::One);
    let action = Action::PlayLand {
        card: card.id,
        option: land_option,
    };
    game.players[0].hand.push(card);

    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();

    assert_eq!(game.battlefield[0].presented, land_part);
    assert!(game.battlefield[0].tapped);
}

#[test]
fn a_modal_spell_resolves_by_its_locked_part_instead_of_the_canonical_front() {
    let definition_id = CardDefinitionId(10_150);
    let creature_part = CardPartId(1);
    let creature_option = PlayOptionId(1);
    let front_rules = CardRules::new(CardKind::Instant, ManaCost::new(1, 1), "Test front");
    let creature_rules = CardRules::new(
        CardKind::Creature,
        ManaCost::new(0, 0),
        "Test creature back",
    )
    .creature(3, 4)
    .with_abilities(&TEST_FLYING_ABILITY);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test modal spell",
        CardSet::Magic2014,
        false,
        CardBehavior::LightningBolt,
    );
    definition.rules = front_rules;
    definition.parts = vec![
        CardPart::new(CardPartId::PRIMARY, "Test front", front_rules),
        CardPart::new(creature_part, "Test creature back", creature_rules),
    ];
    definition.structure = CardStructure::DoubleFaced {
        front: CardPartId::PRIMARY,
        back: creature_part,
        kind: DoubleFacedKind::Modal,
    };
    definition.play_options = vec![
        PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Cast Test front",
            SpellForm::Part(CardPartId::PRIMARY),
            front_rules.mana_cost,
            CardEffectStatus::MetadataOnly,
        ),
        PlayOptionDef::cast(
            creature_option,
            "Cast Test creature back",
            SpellForm::Part(creature_part),
            creature_rules.mana_cost,
            CardEffectStatus::Implemented,
        ),
    ];

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    let card = card(10_150, definition_id, PlayerId::One);
    let hand_id = card.id;
    game.players[0].hand.push(card);
    let action = Action::CastSpell {
        card: hand_id,
        choices: CastChoices::new(creature_option),
        sacrifices: Vec::new(),
    };

    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();
    let spell_id = game.stack[0].id;
    pass_priority_pair(&mut game);

    let permanent = &game.battlefield[0];
    assert_ne!(permanent.card.id, spell_id);
    assert_eq!(permanent.presented, creature_part);
    assert_eq!(game.power(permanent), Some(3));
    assert_eq!(game.toughness(permanent), Some(4));
    assert!(game.has_flying(permanent));
}

#[test]
fn changing_a_permanents_presented_face_keeps_its_object_identity() {
    let definition_id = CardDefinitionId(10_101);
    let back = CardPartId(1);
    let front_rules =
        CardRules::new(CardKind::Creature, ManaCost::new(2, 0), "Front-face rules.").creature(2, 2);
    let back_rules = CardRules::new(CardKind::Creature, ManaCost::default(), "")
        .creature(4, 5)
        .with_abilities(&TEST_FLYING_TRAMPLE_ABILITIES);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test Werewolf",
        CardSet::Innistrad,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = front_rules;
    definition.parts = vec![
        CardPart::new(CardPartId::PRIMARY, "Test Werewolf", front_rules),
        CardPart::new(back, "Test Ravager", back_rules).without_mana_cost(),
    ];
    definition.structure = CardStructure::DoubleFaced {
        front: CardPartId::PRIMARY,
        back,
        kind: DoubleFacedKind::Transforming,
    };
    definition.play_options = vec![PlayOptionDef::cast(
        PlayOptionId::DEFAULT,
        "Cast Test Werewolf",
        SpellForm::Part(CardPartId::PRIMARY),
        front_rules.mana_cost,
        CardEffectStatus::MetadataOnly,
    )];

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    let permanent_id = GameObjectId(10_101);
    game.battlefield
        .push(creature(permanent_id.0, definition_id, PlayerId::One));

    let front = &game.observe(PlayerId::One).battlefield[0];
    assert_eq!(front.id, permanent_id);
    assert_eq!(front.presented, CardPartId::PRIMARY);
    assert_eq!(
        (front.power, front.toughness, front.flying),
        (Some(2), Some(2), false)
    );

    game.battlefield[0].presented = back;

    let transformed = &game.observe(PlayerId::One).battlefield[0];
    assert_eq!(transformed.id, permanent_id);
    assert_eq!(transformed.presented, back);
    assert_eq!(
        (transformed.power, transformed.toughness, transformed.flying),
        (Some(4), Some(5), true),
    );
    assert!(game.has_trample(&game.battlefield[0]));

    game.return_permanent_to_hand(permanent_id);
    let returned_id = game.players[0].hand[0].id;
    assert_ne!(returned_id, permanent_id);
}

#[test]
fn city_in_a_bottle_uses_canonical_origin_even_when_a_reprint_exists() {
    let city = CardDefinition::new(
        CardDefinitionId(10_000),
        "City in a Bottle",
        CardSet::ArabianNights,
        false,
        CardBehavior::CityInABottle,
    );
    let kird_ape = CardDefinition::new(
        CardDefinitionId(10_001),
        "Kird Ape",
        CardSet::ArabianNights,
        false,
        CardBehavior::KirdApe,
    );
    let mut game = ready_game();
    game.catalog = CardCatalog::with_additional_printings(
        [city, kird_ape],
        [CardPrinting::new(
            CardDefinitionId(10_001),
            CardSet::Magic2014,
        )],
    )
    .unwrap();
    game.battlefield
        .push(creature(10_000, CardDefinitionId(10_000), PlayerId::One));
    game.battlefield
        .push(creature(10_001, CardDefinitionId(10_001), PlayerId::Two));

    game.handle_upkeep_triggers();

    assert_eq!(game.battlefield.len(), 1);
    assert_eq!(
        game.battlefield[0].card.definition,
        CardDefinitionId(10_000)
    );
    assert_eq!(game.players[1].graveyard.len(), 1);
    assert_eq!(
        game.players[1].graveyard[0].definition,
        CardDefinitionId(10_001)
    );
}

#[test]
fn metadata_only_noncreature_spells_are_hidden_but_baseline_cards_remain_playable() {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    game.players[0].hand.extend([
        card(10_000, crate::card::cards::DOOM_BLADE, PlayerId::One),
        card(10_001, crate::card::cards::PITHING_NEEDLE, PlayerId::One),
        card(10_002, crate::card::cards::DOMRI_RADE, PlayerId::One),
        card(10_003, crate::card::cards::LOXODON_SMITER, PlayerId::One),
        card(10_004, crate::card::cards::CLIFFTOP_RETREAT, PlayerId::One),
        card(10_005, crate::card::cards::IZZET_CHARM, PlayerId::One),
        card(10_006, crate::card::cards::TURN_BURN, PlayerId::One),
    ]);
    game.players[0].mana_pool = ManaPool {
        white: 4,
        blue: 4,
        black: 4,
        red: 4,
        green: 4,
        colorless: 4,
    };

    let actions = game.legal_actions(PlayerId::One);
    let cast_cards = actions
        .iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, .. } => Some(*card),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(cast_cards, vec![CardInstanceId(10_003)]);
    assert!(actions.contains(&Action::PlayLand {
        card: CardInstanceId(10_004),
        option: PlayOptionId::DEFAULT,
    }));
}

#[test]
fn energy_flux_cannot_be_cast_as_an_inert_enchantment() {
    let mut game = ready_game();
    let flux = card(10_000, crate::card::cards::ENERGY_FLUX, PlayerId::One);
    let flux_id = flux.id;
    game.players[0].hand.push(flux);
    game.players[0].mana_pool.colorless = 2;
    game.players[0].mana_pool.blue = 1;

    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .all(|action| !matches!(action, Action::CastSpell { card, .. } if *card == flux_id))
    );

    let result = game.apply(
        PlayerId::One,
        Action::CastSpell {
            card: flux_id,
            choices: CastChoices::default(),
            sacrifices: Vec::new(),
        },
    );
    assert!(result.is_err());
    assert!(game.stack.is_empty());
    assert!(game.players[0].hand.iter().any(|card| card.id == flux_id));
}

#[test]
#[allow(clippy::too_many_lines)]
fn cast_validation_rejects_unrecognized_structured_choices() {
    let definition_id = CardDefinitionId(10_200);
    let option_id = PlayOptionId(7);
    let implemented_mode = ModeId(2);
    let metadata_mode = ModeId(3);
    let slot_id = TargetSlotId(5);
    let alternative_id = AlternativeCostId(11);
    let additional_id = AdditionalCostId(13);
    let mut definition = CardDefinition::new(
        definition_id,
        "Structured Bolt",
        CardSet::Alpha,
        false,
        CardBehavior::LightningBolt,
    );
    definition.rules = CardRules::new(CardKind::Instant, ManaCost::new(0, 1), "");
    synchronize_single_part_definition(&mut definition);
    let mut option = PlayOptionDef::cast(
        option_id,
        "Cast Structured Bolt",
        SpellForm::Part(CardPartId::PRIMARY),
        ManaCost::new(0, 1),
        CardEffectStatus::Implemented,
    )
    .with_modes(ModeSetDef {
        minimum: 1,
        maximum: 2,
        may_repeat: false,
        modes: vec![
            ModeDef {
                id: implemented_mode,
                label: "Target a player".into(),
                targets: vec![TargetSlotDef::exactly_one(
                    slot_id,
                    "target player",
                    TargetPredicate::Player,
                )],
                effect_status: CardEffectStatus::Implemented,
            },
            ModeDef {
                id: metadata_mode,
                label: "Not implemented".into(),
                targets: Vec::new(),
                effect_status: CardEffectStatus::MetadataOnly,
            },
        ],
    });
    option.alternative_costs = vec![AlternativeCostDef {
        id: alternative_id,
        label: "Alternative cost".into(),
        mana_cost: ManaCost::new(1, 0),
    }];
    option.additional_costs = vec![AdditionalCostDef {
        id: additional_id,
        label: "Additional cost".into(),
        mana_cost: Some(ManaCost::new(2, 0)),
    }];
    definition.play_options = vec![option];

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    let card = card(10_200, definition_id, PlayerId::One);
    let card_id = card.id;
    game.players[0].hand.push(card);
    game.players[0].mana_pool.colorless = 20;

    let valid = CastChoices::new(option_id)
        .with_modes(vec![implemented_mode])
        .with_costs(CostConfiguration::new(
            Some(alternative_id),
            vec![additional_id],
        ))
        .with_targets(vec![TargetSelection::single(
            slot_id,
            Target::Player(PlayerId::Two),
        )]);
    let (signature, cost, _) = game
        .validated_cast_signature(PlayerId::One, card_id, &valid)
        .expect("all structured choices are recognized and payable");
    assert_eq!(signature.play_option(), option_id);
    assert_eq!(signature.form(), &SpellForm::Part(CardPartId::PRIMARY));
    assert_eq!(signature.modes(), &[implemented_mode]);
    assert_eq!(signature.costs(), valid.costs());
    assert_eq!(signature.targets(), valid.targets());
    assert_eq!(cost, ManaCost::new(3, 0));

    let invalid = [
        CastChoices::new(PlayOptionId(99)),
        CastChoices::new(option_id),
        CastChoices::new(option_id).with_modes(vec![metadata_mode]),
        CastChoices::new(option_id).with_modes(vec![implemented_mode, implemented_mode]),
        CastChoices::new(option_id)
            .with_modes(vec![implemented_mode])
            .with_costs(CostConfiguration::new(
                Some(AlternativeCostId(99)),
                Vec::new(),
            )),
        CastChoices::new(option_id)
            .with_modes(vec![implemented_mode])
            .with_costs(CostConfiguration::new(None, vec![AdditionalCostId(99)])),
        CastChoices::new(option_id)
            .with_modes(vec![implemented_mode])
            .with_x(1),
        CastChoices::new(option_id)
            .with_modes(vec![implemented_mode])
            .with_targets(vec![TargetSelection::single(
                TargetSlotId(99),
                Target::Player(PlayerId::Two),
            )]),
        CastChoices::new(option_id)
            .with_modes(vec![implemented_mode])
            .with_targets(vec![TargetSelection::single(
                slot_id,
                Target::Permanent(GameObjectId(99_999)),
            )]),
    ];
    for choices in invalid {
        assert!(
            game.validated_cast_signature(PlayerId::One, card_id, &choices)
                .is_none(),
            "invalid structured choices were accepted: {choices:?}",
        );
    }
}

#[test]
fn declarative_dual_lands_cast_and_resolve_a_hybrid_creature() {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    game.battlefield.extend([
        creature(10_000, crate::card::cards::CLIFFTOP_RETREAT, PlayerId::One),
        creature(10_001, crate::card::cards::SACRED_FOUNDRY, PlayerId::One),
        creature(10_002, crate::card::cards::SUNPETAL_GROVE, PlayerId::One),
    ]);
    game.players[0].hand.push(card(
        10_003,
        crate::card::cards::BOROS_RECKONER,
        PlayerId::One,
    ));

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell {
                    card: CardInstanceId(10_003),
                    ..
                }
            )
        })
        .expect("three declarative dual lands can pay {R/W}{R/W}{R/W}");
    assert_eq!(game.mana_sources_for_action(PlayerId::One, &cast).len(), 3);

    game.apply(PlayerId::One, cast).unwrap();
    assert!(game.battlefield.iter().all(|permanent| permanent.tapped));
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
    pass_priority_pair(&mut game);

    let reckoner = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == crate::card::cards::BOROS_RECKONER)
        .unwrap();
    assert_eq!(game.power(reckoner), Some(3));
    assert_eq!(game.toughness(reckoner), Some(3));
}

#[test]
fn flexible_mana_plan_reserves_the_only_green_source_for_a_multicolor_spell() {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    game.battlefield.extend([
        creature(10_000, crate::card::cards::TEMPLE_GARDEN, PlayerId::One),
        creature(10_001, crate::card::cards::GODLESS_SHRINE, PlayerId::One),
        creature(
            10_002,
            crate::card::cards::ENCROACHING_WASTES,
            PlayerId::One,
        ),
    ]);
    game.players[0].hand.push(card(
        10_003,
        crate::card::cards::LOXODON_SMITER,
        PlayerId::One,
    ));

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell {
                    card: CardInstanceId(10_003),
                    ..
                }
            )
        })
        .expect("Godless Shrine can make white while Temple Garden makes green");
    assert_eq!(
        game.mana_sources_for_action(PlayerId::One, &cast),
        vec![
            CardInstanceId(10_001),
            CardInstanceId(10_000),
            CardInstanceId(10_002),
        ],
    );

    game.apply(PlayerId::One, cast).unwrap();
    assert!(game.battlefield.iter().all(|permanent| permanent.tapped));
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
}

#[test]
fn metadata_only_flash_creatures_keep_their_printed_cast_timing() {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    game.step = Step::End;
    game.players[0].mana_pool = ManaPool {
        white: 1,
        colorless: 3,
        ..ManaPool::default()
    };
    game.players[0].hand.extend([
        card(10_000, crate::card::cards::RESTORATION_ANGEL, PlayerId::One),
        card(10_001, crate::card::cards::LOXODON_SMITER, PlayerId::One),
    ]);

    let cast_cards = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, .. } => Some(card),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(cast_cards, vec![CardInstanceId(10_000)]);
}

#[test]
fn city_of_brass_produces_any_color_then_uses_the_stack_for_damage() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::CITY_OF_BRASS, PlayerId::One));

    let ability = mana_ability_for(&game, CardInstanceId(10_000), ManaColor::Blue);
    game.activate_mana_source(
        PlayerId::One,
        CardInstanceId(10_000),
        ability,
        ManaColor::Blue,
    );

    assert_eq!(game.players[0].mana_pool.blue, 1);
    assert_eq!(game.players[0].life, 20);
    assert!(game.stack.is_empty());
    game.finish_rules_procedure();
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::TriggeredAbility);
    assert_eq!(game.stack[0].source, Some(CardInstanceId(10_000)));

    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].life, 19);
}

#[test]
fn trigger_placement_preserves_the_nonactive_players_priority() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::CITY_OF_BRASS, PlayerId::Two));

    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    assert_eq!(game.priority, PlayerId::Two);
    game.apply(
        PlayerId::Two,
        Action::ActivateManaAbility {
            source: CardInstanceId(10_000),
            ability: mana_ability_for(&game, CardInstanceId(10_000), ManaColor::Blue),
            color: ManaColor::Blue,
        },
    )
    .unwrap();

    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.priority, PlayerId::Two);
    assert!(
        game.legal_actions(PlayerId::Two)
            .contains(&Action::PassPriority)
    );
}

#[test]
fn ankh_trigger_can_be_answered_by_bolt_before_it_resolves() {
    let mut game = ready_game();
    game.players[0].life = 2;
    game.players[1].life = 3;
    game.battlefield
        .push(creature(10_000, cards::ANKH_OF_MISHRA, PlayerId::Two));
    let mountain = card(10_001, cards::MOUNTAIN, PlayerId::One);
    let bolt = card(10_002, cards::LIGHTNING_BOLT, PlayerId::One);
    game.players[0]
        .hand
        .extend([mountain.clone(), bolt.clone()]);

    let play_land = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == mountain.id))
        .expect("Mountain is a legal land play");
    game.apply(PlayerId::One, play_land).unwrap();

    assert_eq!(game.players[0].life, 2);
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::TriggeredAbility);
    assert_eq!(game.stack[0].source, Some(CardInstanceId(10_000)));
    assert_eq!(
        game.stack[0].ability_origin(),
        Some(primary_ability(cards::ANKH_OF_MISHRA))
    );

    let cast_bolt = cast_action(bolt.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast_bolt));
    game.apply(PlayerId::One, cast_bolt).unwrap();
    assert_eq!(game.stack.len(), 2);
    assert_eq!(game.stack.last().unwrap().kind, StackObjectKind::Spell);

    pass_priority_pair(&mut game);
    assert_eq!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::One,
            reason: WinReason::OpponentLostAllLife,
        })
    );
    assert_eq!(game.players[0].life, 2);
    assert_eq!(game.stack.len(), 1, "Ankh never got to resolve");
}

#[test]
fn ankh_damages_the_entering_lands_controller_not_its_owner() {
    let mut game = ready_game();
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.battlefield
        .push(creature(10_000, cards::ANKH_OF_MISHRA, PlayerId::One));
    let borrowed_mountain = card(10_001, cards::MOUNTAIN, PlayerId::One);
    game.players[1].hand.push(borrowed_mountain.clone());

    let play_land = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            matches!(action, Action::PlayLand { card, .. } if *card == borrowed_mountain.id)
        })
        .expect("the active player may play the land they currently hold");
    game.apply(PlayerId::Two, play_land).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].life, 20, "the physical owner is unharmed");
    assert_eq!(game.players[1].life, 18, "the land's controller takes 2");
}

#[test]
fn city_trigger_can_be_answered_when_mana_was_floated_first() {
    let mut game = ready_game();
    game.players[0].life = 1;
    game.players[1].life = 3;
    let city = creature(10_000, cards::CITY_OF_BRASS, PlayerId::One);
    let bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::One);
    game.battlefield.push(city);
    game.players[0].hand.push(bolt.clone());

    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: CardInstanceId(10_000),
            ability: mana_ability_for(&game, CardInstanceId(10_000), ManaColor::Red),
            color: ManaColor::Red,
        },
    )
    .unwrap();
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.players[0].life, 1);

    game.apply(
        PlayerId::One,
        cast_action(bolt.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    assert_eq!(game.stack.last().unwrap().kind, StackObjectKind::Spell);
    pass_priority_pair(&mut game);

    assert_eq!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::One,
            reason: WinReason::OpponentLostAllLife,
        })
    );
}

#[test]
fn city_trigger_is_above_a_spell_when_city_pays_during_casting() {
    let mut game = ready_game();
    game.players[0].life = 1;
    game.players[1].life = 3;
    game.battlefield
        .push(creature(10_000, cards::CITY_OF_BRASS, PlayerId::One));
    let bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::One);
    game.players[0].hand.push(bolt.clone());

    let cast = cast_action(bolt.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();

    assert_eq!(game.stack.len(), 2);
    assert_eq!(game.stack[0].kind, StackObjectKind::Spell);
    assert_eq!(game.stack[1].kind, StackObjectKind::TriggeredAbility);
    pass_priority_pair(&mut game);
    assert_eq!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::Two,
            reason: WinReason::OpponentLostAllLife,
        })
    );
    assert_eq!(game.players[1].life, 3, "Bolt never resolved");
}

#[test]
fn a_resolving_tap_effect_uses_the_same_city_trigger_path() {
    let mut game = ready_game();
    game.players[0].mana_pool.colorless = 1;
    game.battlefield.extend([
        creature(10_000, cards::ICY_MANIPULATOR, PlayerId::One),
        creature(10_001, cards::CITY_OF_BRASS, PlayerId::Two),
    ]);
    let activation = Action::ActivateAbility {
        source: CardInstanceId(10_000),
        ability: activated_ability_for(&game, CardInstanceId(10_000), 0),
        targets: activated_targets(Target::Permanent(CardInstanceId(10_001))),
        sacrifice: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&activation));
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(&mut game);

    assert!(game.battlefield[1].tapped);
    assert_eq!(game.players[1].life, 20);
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::TriggeredAbility);
    assert_eq!(game.stack[0].source, Some(CardInstanceId(10_001)));
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 19);
}

#[test]
fn controller_chooses_resolution_order_for_simultaneous_triggers() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::ANKH_OF_MISHRA, PlayerId::One),
        creature(10_001, cards::ANKH_OF_MISHRA, PlayerId::One),
    ]);
    let mountain = card(10_002, cards::MOUNTAIN, PlayerId::One);
    game.players[0].hand.push(mountain.clone());
    let play = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == mountain.id))
        .unwrap();
    game.apply(PlayerId::One, play).unwrap();

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.kind, DecisionKind::TriggerOrder);
    assert_eq!(
        decision.order_semantics,
        Some(DecisionOrderSemantics::Resolution)
    );
    assert!(decision.options.iter().all(|option| {
        option
            .ability_text
            .as_deref()
            .is_some_and(|text| text.contains("Whenever a land enters"))
    }));
    let first = decision.options[0].id;
    let second = decision.options[1].id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![second, first],
        },
    )
    .unwrap();

    assert_eq!(game.stack.len(), 2);
    assert_eq!(
        game.stack.last().unwrap().source,
        Some(CardInstanceId(10_001))
    );
    assert!(game.stack.iter().all(|object| {
        object.ability_origin() == Some(primary_ability(cards::ANKH_OF_MISHRA))
            && object.ability_text().is_some()
    }));
}

#[test]
fn simultaneous_triggers_are_put_on_the_stack_in_apnap_order() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::ANKH_OF_MISHRA, PlayerId::One),
        creature(10_001, cards::ANKH_OF_MISHRA, PlayerId::Two),
    ]);
    let mountain = card(10_002, cards::MOUNTAIN, PlayerId::One);
    game.players[0].hand.push(mountain.clone());
    let play = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == mountain.id))
        .expect("Mountain is a legal land play");
    game.apply(PlayerId::One, play).unwrap();

    assert_eq!(game.stack.len(), 2);
    assert_eq!(
        game.stack[0].source,
        Some(CardInstanceId(10_000)),
        "the active player's trigger is put on the stack first"
    );
    assert_eq!(
        game.stack[1].source,
        Some(CardInstanceId(10_001)),
        "the nonactive player's trigger is on top and resolves first"
    );
}

#[test]
fn targeted_trigger_chooses_public_targets_while_being_put_on_stack() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        TargetSlotId(7),
        "target creature an opponent controls",
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Creature,
            zones: &[ZoneKind::Battlefield],
            controller: Some(PlayerRelation::Opponent),
            owner: None,
        },
    )];
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::ANKH_OF_MISHRA, PlayerId::One),
        creature(10_001, cards::SU_CHI, PlayerId::Two),
    ]);
    game.capture_trigger(TriggerCapture {
        source: AbilitySourceRef {
            object: CardInstanceId(10_000),
            ability: primary_ability(cards::ANKH_OF_MISHRA),
        },
        definition: cards::ANKH_OF_MISHRA,
        owner: PlayerId::One,
        controller: PlayerId::One,
        text: "Deal 2 damage to target creature an opponent controls.",
        target_defs: &TARGETS,
        effect: EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetSlotId(7)),
            amount: ValueDef::Constant(2),
        },
        resolver: StackAbilityResolver::Declarative(EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetSlotId(7)),
            amount: ValueDef::Constant(2),
        }),
        context: TriggerContext {
            object: None,
            object_controller: None,
            event_player: None,
            amount: None,
        },
    });
    game.finish_rules_procedure();

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.kind, DecisionKind::TriggerPlacement);
    assert_eq!(decision.visibility, DecisionVisibility::Public);
    assert_eq!(decision.minimum, 1);
    assert_eq!(decision.maximum, 1);
    assert_eq!(decision.options.len(), 1);
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decision.options[0].id],
        },
    )
    .unwrap();

    assert_eq!(
        game.stack[0].targets(),
        vec![Target::Permanent(CardInstanceId(10_001))]
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.battlefield[1].damage, 2);
}

#[test]
fn nonbattlefield_card_targets_are_zone_incarnations() {
    static INSTANT_OR_SORCERY: [ObjectPredicateDef; 2] = [
        ObjectPredicateDef::CardKind(CardKind::Instant),
        ObjectPredicateDef::CardKind(CardKind::Sorcery),
    ];
    let predicate = AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&INSTANT_OR_SORCERY),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    };
    let mut game = ready_game();
    let bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::One);
    let stone_rain = card(10_001, cards::STONE_RAIN, PlayerId::One);
    let mountain = card(10_002, cards::MOUNTAIN, PlayerId::One);
    let opposing_bolt = card(10_003, cards::LIGHTNING_BOLT, PlayerId::Two);
    game.players[0]
        .graveyard
        .extend([bolt.clone(), stone_rain.clone(), mountain]);
    game.players[1].graveyard.push(opposing_bolt);

    let targets = game.ability_targets_matching(
        predicate,
        PlayerId::One,
        GameObjectId(99_999),
        TriggerContext::empty(),
    );
    assert_eq!(
        targets,
        vec![Target::Card(bolt.id), Target::Card(stone_rain.id)]
    );

    let old_bolt = game.players[0].graveyard.remove(0);
    let (new_bolt, zone_change) = game.zone_change_card(old_bolt);
    game.players[0].hand.push(new_bolt);
    assert_eq!(zone_change.previous, bolt.id);
    assert_eq!(
        game.ability_targets_matching(
            predicate,
            PlayerId::One,
            GameObjectId(99_999),
            TriggerContext::empty(),
        ),
        vec![Target::Card(stone_rain.id)],
        "a target does not follow the physical card to its new zone object",
    );
}

#[test]
fn su_chi_mana_and_source_power_use_ordinary_stack_and_lki() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SU_CHI, PlayerId::One));
    game.destroy_permanent(CardInstanceId(10_000));
    assert_eq!(game.players[0].mana_pool.colorless, 0);
    game.finish_rules_procedure();
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].source, Some(CardInstanceId(10_000)));
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].mana_pool.colorless, 4);

    let mut game = ready_game();
    let mut source = creature(10_010, cards::SAVANNAH_LIONS, PlayerId::One);
    source.power_bonus = 3;
    game.battlefield.push(source);
    game.capture_trigger(TriggerCapture {
        source: AbilitySourceRef {
            object: CardInstanceId(10_010),
            ability: primary_ability(cards::SAVANNAH_LIONS),
        },
        definition: cards::SAVANNAH_LIONS,
        owner: PlayerId::One,
        controller: PlayerId::One,
        text: "Deal damage equal to this creature's power.",
        target_defs: &[],
        effect: EffectDef::DealDamage {
            recipient: EffectRecipientDef::Opponent,
            amount: ValueDef::SourcePower,
        },
        resolver: StackAbilityResolver::Declarative(EffectDef::DealDamage {
            recipient: EffectRecipientDef::Opponent,
            amount: ValueDef::SourcePower,
        }),
        context: TriggerContext {
            object: Some(CardInstanceId(10_010)),
            object_controller: Some(PlayerId::One),
            event_player: Some(PlayerId::One),
            amount: None,
        },
    });
    game.destroy_permanent(CardInstanceId(10_010));
    game.finish_rules_procedure();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 15, "last known power was five");
}

#[test]
fn workshop_mana_is_three_individual_values_without_an_unimplemented_restriction() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::MISHRA_S_WORKSHOP, PlayerId::One));
    let ability = mana_ability_for(&game, CardInstanceId(10_000), ManaColor::Colorless);
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: CardInstanceId(10_000),
            ability,
            color: ManaColor::Colorless,
        },
    )
    .unwrap();

    assert_eq!(game.players[0].mana_pool.colorless, 3);
    assert_eq!(game.players[0].mana.len(), 3);
    assert!(game.players[0].mana.iter().all(|mana| {
        mana.color == ManaColor::Colorless
            && mana.source
                == Some(ManaSource {
                    object: CardInstanceId(10_000),
                    ability,
                })
            && mana.restrictions.is_empty()
    }));
}

#[test]
fn explicitly_tagged_triggered_mana_ability_resolves_without_the_stack() {
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered_mana(
        crate::AbilityId::PRIMARY,
        "Whenever this becomes tapped, add {C}.",
        TriggerEventDef::BecomesTapped(ObjectPredicateDef::Source),
        EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Colorless)),
    )];
    let definition_id = CardDefinitionId(10_050);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test triggered mana source",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new(
        CardKind::Artifact,
        ManaCost::new(0, 0),
        "Whenever this becomes tapped, add {C}.",
    )
    .with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    game.battlefield
        .push(creature(10_050, definition_id, PlayerId::One));

    let _ = game.tap_permanent(CardInstanceId(10_050));

    assert_eq!(game.players[0].mana_pool.colorless, 1);
    assert_eq!(game.players[0].mana.len(), 1);
    assert!(game.pending_triggers.is_empty());
    assert!(game.stack.is_empty());
}

#[test]
fn a_mana_spend_rider_attaches_to_the_paid_spell_with_its_source() {
    static RIDERS: [ManaSpendEffectDef; 1] = [ManaSpendEffectDef::ApplyToPaidSpell(
        crate::AppliedEffectDef::CannotBeCountered,
    )];
    let mut object = spell(77, cards::SAVANNAH_LIONS, PlayerId::One, 0);
    let mana = Mana::from_ability(
        ManaColor::White,
        ManaSource {
            object: CardInstanceId(10_000),
            ability: AbilityOrigin::Printed {
                definition: cards::SAVANNAH_LIONS,
                part: CardPartId::PRIMARY,
                ability: crate::AbilityId(1),
            },
        },
        &[],
        &RIDERS,
    );

    Game::apply_spent_mana_to_spell(&mut object, &[mana]);

    assert_eq!(object.applied_effects.len(), 1);
    assert_eq!(object.applied_effects[0].source, mana.source);
    assert_eq!(
        object.applied_effects[0].effect,
        crate::AppliedEffectDef::CannotBeCountered
    );
}

#[test]
fn crusade_buffs_white_creatures() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::CRUSADE, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));

    assert_eq!(game.power(&game.battlefield[1]), Some(3));
    assert_eq!(game.toughness(&game.battlefield[1]), Some(2));
}

#[test]
fn demonic_tutor_exposes_a_library_choice_then_shuffles() {
    let mut game = ready_game();
    game.players[0]
        .library
        .push(card(10_001, cards::JUZAM_DJINN, PlayerId::One));
    let tutor = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);

    game.resolve_spell_effect(&tutor, CardBehavior::DemonicTutor);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    let option = decision
        .options
        .iter()
        .find(|option| option.card == Some((CardInstanceId(10_001), cards::JUZAM_DJINN)))
        .unwrap();
    let choice = Action::ChooseDecision {
        decision: decision.id,
        options: vec![option.id],
    };
    game.apply(PlayerId::One, choice).unwrap();

    assert_eq!(game.players[0].hand[0].definition, cards::JUZAM_DJINN);
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn a_search_may_fail_to_find_even_with_a_full_library() {
    // CR 701.19c: searching a hidden zone never obliges the searcher to find.
    // This is not cancelling the spell -- Demonic Tutor resolved, the search
    // happened, and it turned up nothing on purpose.
    let mut game = ready_game();
    for (index, definition) in [cards::JUZAM_DJINN, cards::BLACK_LOTUS]
        .into_iter()
        .enumerate()
    {
        let id = 10_001 + u32::try_from(index).unwrap();
        game.players[0]
            .library
            .push(card(id, definition, PlayerId::One));
    }
    let tutor = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);
    game.resolve_spell_effect(&tutor, CardBehavior::DemonicTutor);

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.minimum, 0, "a search is never compulsory");
    assert_eq!(decision.maximum, 1);
    assert!(
        !decision.cancellable,
        "failing to find is a resolution, not a way out of the spell"
    );

    let library_before = game.players[0].library.len();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .expect("failing to find is a legal resolution");

    assert!(game.players[0].hand.is_empty(), "nothing was found");
    assert_eq!(
        game.players[0].library.len(),
        library_before,
        "and nothing left the library"
    );
    assert!(game.pending_decisions.is_empty(), "the search is over");
}

#[test]
fn the_handcrafted_policy_still_finds_when_it_may_decline() {
    // Failing to find became legal, and the policy takes `minimum` options by
    // default -- which is now zero. Left alone it would tutor for nothing
    // every single time, quietly turning Demonic Tutor into a blank.
    use crate::{HandcraftedPolicy, Policy};

    let mut game = ready_game();
    game.players[0]
        .library
        .push(card(10_001, cards::BLACK_LOTUS, PlayerId::One));
    let tutor = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);
    game.resolve_spell_effect(&tutor, CardBehavior::DemonicTutor);

    let mut policy = HandcraftedPolicy::new(poc::catalog().unwrap());
    let action = policy
        .choose_action(&game.observe(PlayerId::One))
        .expect("the policy answers the search");
    let Action::ChooseDecision { options, .. } = &action else {
        panic!("expected a decision, got {action:?}");
    };
    assert_eq!(options.len(), 1, "the policy searched and found a card");

    game.apply(PlayerId::One, action.clone()).expect("legal");
    assert_eq!(game.players[0].hand.len(), 1, "the card reached hand");
}

#[test]
fn a_search_shuffles_even_when_it_finds_nothing() {
    // Otherwise a player learns their own library order for free: tutor, fail
    // to find, and the top of the deck is whatever it already was.
    let mut game = ready_game();
    let before: Vec<_> = game.players[0].library.iter().map(|card| card.id).collect();
    assert!(
        before.len() > 10,
        "the deck's library is long enough for a shuffle to be observable"
    );

    let tutor = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);
    game.resolve_spell_effect(&tutor, CardBehavior::DemonicTutor);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .expect("failing to find is legal");

    let after: Vec<_> = game.players[0].library.iter().map(|card| card.id).collect();
    assert_eq!(
        before.len(),
        after.len(),
        "a failed search moves no cards, it only shuffles"
    );
    assert_ne!(
        before, after,
        "the library was shuffled despite finding nothing"
    );
}

#[test]
fn a_tutor_with_nothing_to_find_leaves_a_legal_action() {
    // An empty library used to produce a decision asking for exactly one of
    // zero options, and not cancellable. `is_legal` rejects a ChooseDecision
    // carrying fewer than `minimum` options, so no legal action existed and
    // the game deadlocked -- every policy stalls, having nothing to return.
    let mut game = ready_game();
    game.players[0].library.clear();
    let tutor = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);

    game.resolve_spell_effect(&tutor, CardBehavior::DemonicTutor);

    let observation = game.observe(PlayerId::One);
    if let Some(decision) = observation.decision.as_ref() {
        assert!(
            decision.minimum <= decision.options.len(),
            "a decision must never ask for more than it offers: \
             minimum={} options={}",
            decision.minimum,
            decision.options.len(),
        );
    }
    assert!(
        !observation.legal_actions.is_empty(),
        "an empty library must still leave the player something to do"
    );

    // The player resolves it by finding nothing, and the game moves on.
    let decision = observation.decision.expect("the tutor still asks");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .expect("choosing nothing from nothing is legal");

    assert!(game.pending_decisions.is_empty());
    assert!(game.players[0].hand.is_empty(), "nothing was found");
}

#[test]
fn armageddon_destroys_every_land_but_not_creatures() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::CITY_OF_BRASS, PlayerId::One),
        creature(10_001, cards::SWAMP, PlayerId::Two),
        creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One),
    ]);
    let armageddon = spell(10_003, cards::ARMAGEDDON, PlayerId::One, 0);

    let effect = game
        .catalog
        .get(cards::ARMAGEDDON)
        .expect("Armageddon is in the catalog")
        .rules
        .ability_clauses()[0]
        .effect;
    game.resolve_effect_def(effect, &armageddon, TriggerContext::empty());

    assert_eq!(game.battlefield.len(), 1);
    assert_eq!(game.battlefield[0].card.definition, cards::SAVANNAH_LIONS);
}

#[test]
fn recall_uses_cancellable_cost_and_return_decisions() {
    let mut game = ready_game();
    game.players[0].hand.extend([
        card(10_000, cards::RECALL, PlayerId::One),
        card(10_001, cards::LIGHTNING_BOLT, PlayerId::One),
        card(10_002, cards::BALANCE, PlayerId::One),
    ]);
    game.players[0].mana_pool = ManaPool {
        blue: 1,
        colorless: 4,
        ..ManaPool::default()
    };

    game.cast_spell(
        PlayerId::One,
        CardInstanceId(10_000),
        cast_choices(Vec::new(), 2),
        &[],
    );
    let cost_decision = game.observe(PlayerId::One).decision.unwrap();
    assert!(cost_decision.cancellable);
    assert_eq!(cost_decision.minimum, 2);
    let cost_action = Action::ChooseDecision {
        decision: cost_decision.id,
        options: cost_decision
            .options
            .iter()
            .take(cost_decision.minimum)
            .map(|option| option.id)
            .collect(),
    };
    game.apply(PlayerId::One, cost_action).unwrap();
    assert_eq!(game.players[0].graveyard.len(), 2);

    pass_priority_pair(&mut game);
    let return_decision = game.observe(PlayerId::One).decision.unwrap();
    assert!(!return_decision.cancellable);
    assert_eq!(return_decision.minimum, 2);
    let return_action = Action::ChooseDecision {
        decision: return_decision.id,
        options: return_decision
            .options
            .iter()
            .take(return_decision.minimum)
            .map(|option| option.id)
            .collect(),
    };
    game.apply(PlayerId::One, return_action).unwrap();

    assert_eq!(game.players[0].hand.len(), 2);
    assert_eq!(game.players[0].exile[0].definition, cards::RECALL);
}

#[test]
fn balance_requests_public_sacrifices_and_private_discards() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::PLAINS, PlayerId::One),
        creature(10_001, cards::CITY_OF_BRASS, PlayerId::One),
        creature(10_002, cards::SWAMP, PlayerId::Two),
    ]);
    game.players[0].hand.extend([
        card(10_003, cards::LIGHTNING_BOLT, PlayerId::One),
        card(10_004, cards::BALANCE, PlayerId::One),
    ]);
    game.players[1]
        .hand
        .push(card(10_005, cards::TERROR, PlayerId::Two));

    game.resolve_balance();
    assert_eq!(
        game.observe(PlayerId::Two).decision.unwrap().visibility,
        DecisionVisibility::Public
    );
    let decision_player = game.decision_player().unwrap();
    let pending_actions = game.legal_actions(decision_player);
    assert_eq!(pending_actions.len(), 2);
    assert!(matches!(
        &pending_actions[1],
        Action::ChooseDecision {
            decision: _,
            options
        } if options.is_empty()
    ));
    while let Some(player) = game.decision_player() {
        let Some(decision) = game.observe(player).decision else {
            break;
        };
        let action = Action::ChooseDecision {
            decision: decision.id,
            options: decision
                .options
                .iter()
                .take(decision.minimum)
                .map(|option| option.id)
                .collect(),
        };
        game.apply(player, action).unwrap();
    }

    let land_counts = [PlayerId::One, PlayerId::Two].map(|player| {
        game.battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && game.permanent_kind(permanent) == Some(CardKind::Land)
            })
            .count()
    });
    assert_eq!(land_counts, [1, 1]);
    assert_eq!(game.players[0].hand.len(), game.players[1].hand.len());
}

#[test]
fn balance_defers_one_apnap_trigger_batch_until_its_decisions_finish() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::SU_CHI, PlayerId::One),
        creature(10_001, cards::SU_CHI, PlayerId::One),
        creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One),
        creature(10_003, cards::SAVANNAH_LIONS, PlayerId::Two),
    ]);
    game.players[0].hand.extend([
        card(10_004, cards::LIGHTNING_BOLT, PlayerId::One),
        card(10_005, cards::MOUNTAIN, PlayerId::One),
    ]);

    game.resolve_balance();
    let sacrifice = game.observe(PlayerId::One).decision.unwrap();
    let su_chi = sacrifice
        .options
        .iter()
        .filter(|option| {
            option
                .card
                .is_some_and(|(_, definition)| definition == cards::SU_CHI)
        })
        .map(|option| option.id)
        .collect::<Vec<_>>();
    assert_eq!(su_chi.len(), 2);
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: sacrifice.id,
            options: su_chi,
        },
    )
    .unwrap();

    let discard = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(discard.kind, DecisionKind::Choice);
    assert!(discard.prompt.contains("discard"));
    assert!(game.stack.is_empty());
    assert_eq!(game.pending_triggers.len(), 2);

    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: discard.id,
            options: discard.options.iter().map(|option| option.id).collect(),
        },
    )
    .unwrap();

    let order = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(order.kind, DecisionKind::TriggerOrder);
    assert_eq!(order.options.len(), 2);
    assert!(game.stack.is_empty());
    assert!(game.pending_triggers.is_empty());

    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: order.id,
            options: order.options.iter().map(|option| option.id).collect(),
        },
    )
    .unwrap();
    assert_eq!(game.stack.len(), 2);
    assert!(
        game.stack
            .iter()
            .all(|object| object.kind == StackObjectKind::TriggeredAbility)
    );
}

#[test]
fn time_vault_can_untap_by_skipping_the_controllers_next_turn() {
    let mut game = ready_game();
    let mut vault = creature(10_000, cards::TIME_VAULT, PlayerId::Two);
    vault.tapped = true;
    game.battlefield.push(vault);

    game.start_next_turn();
    let decision = game.observe(PlayerId::Two).decision.unwrap();
    let untap = Action::ChooseDecision {
        decision: decision.id,
        options: vec![1],
    };
    game.apply(PlayerId::Two, untap).unwrap();
    assert!(!game.battlefield[0].tapped);

    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::One);
    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::One);
}

#[test]
fn sylvan_library_tracks_drawn_cards_and_resolves_each_choice() {
    let mut game = ready_game();
    game.turn = 2;
    game.step = Step::Upkeep;
    game.battlefield
        .push(creature(10_000, cards::SYLVAN_LIBRARY, PlayerId::One));
    game.players[0].library = vec![
        card(10_001, cards::PLAINS, PlayerId::One),
        card(10_002, cards::SAVANNAH_LIONS, PlayerId::One),
        card(10_003, cards::SWORDS_TO_PLOWSHARES, PlayerId::One),
    ];

    game.advance_step();
    assert_eq!(game.players[0].hand.len(), 3);
    for mode in [1, 0] {
        let selection = game.observe(PlayerId::One).decision.unwrap();
        let select = Action::ChooseDecision {
            decision: selection.id,
            options: vec![selection.options[0].id],
        };
        game.apply(PlayerId::One, select).unwrap();
        let decision = game.observe(PlayerId::One).decision.unwrap();
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![mode],
            },
        )
        .unwrap();
    }

    assert_eq!(game.players[0].life, 16);
    assert_eq!(game.players[0].hand.len(), 2);
    assert_eq!(game.players[0].library.len(), 1);
}

#[test]
fn mana_vault_stays_tapped_and_can_be_paid_to_untap_at_upkeep() {
    let mut game = ready_game();
    let mut vault = creature(10_000, cards::MANA_VAULT, PlayerId::One);
    vault.tapped = true;
    game.battlefield.push(vault);
    for id in 10_001..10_005 {
        game.battlefield
            .push(creature(id, cards::MOUNTAIN, PlayerId::One));
    }
    game.step = Step::Upkeep;

    game.handle_upkeep_triggers();
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.prompt, "Mana Vault would remain tapped");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![1],
        },
    )
    .unwrap();

    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::MANA_VAULT)
            .unwrap()
            .tapped
    );
}

#[test]
fn multiple_mana_vault_upkeep_choices_do_not_reuse_stale_mana() {
    let mut game = ready_game();
    for id in 10_000..10_002 {
        let mut vault = creature(id, cards::MANA_VAULT, PlayerId::One);
        vault.tapped = true;
        game.battlefield.push(vault);
    }
    for id in 10_002..10_006 {
        game.battlefield
            .push(creature(id, cards::MOUNTAIN, PlayerId::One));
    }
    game.step = Step::Upkeep;

    game.handle_upkeep_triggers();
    let first = game.observe(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: first.id,
            options: vec![1],
        },
    )
    .unwrap();

    let second = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(second.prompt, "Mana Vault would remain tapped");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: second.id,
            options: vec![1],
        },
    )
    .unwrap();

    let vaults: Vec<_> = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == cards::MANA_VAULT)
        .map(|permanent| permanent.tapped)
        .collect();
    assert_eq!(vaults, vec![false, true]);
}

#[test]
fn tapped_mana_vault_deals_one_at_the_draw_step() {
    let mut game = ready_game();
    let mut vault = creature(10_000, cards::MANA_VAULT, PlayerId::One);
    vault.tapped = true;
    game.battlefield.push(vault);
    game.step = Step::Upkeep;

    game.advance_step();

    assert_eq!(game.players[0].life, 19);
    assert_eq!(game.step, Step::Draw);
}

#[test]
fn juggernaut_must_attack_if_able() {
    let mut game = ready_game();
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    let juggernaut = creature(10_000, cards::JUGGERNAUT, PlayerId::One);
    let juggernaut_id = juggernaut.card.id;
    game.battlefield.push(juggernaut);

    let actions = game.legal_actions(PlayerId::One);
    assert!(!actions.contains(&Action::FinishDeclaringAttackers));
    assert!(actions.contains(&Action::DeclareAttacker {
        attacker: juggernaut_id,
    }));

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: juggernaut_id,
        },
    )
    .unwrap();
    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&Action::FinishDeclaringAttackers)
    );
}

#[test]
fn triskelion_enters_with_counters_and_spends_one_to_deal_damage() {
    let mut game = ready_game();
    let triskelion = card(10_000, cards::TRISKELION, PlayerId::One);
    let triskelion_id = triskelion.id;
    game.players[0].hand.push(triskelion);
    game.players[0].mana_pool.colorless = 6;

    game.apply(
        PlayerId::One,
        cast_action(triskelion_id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::TRISKELION)
        .unwrap();
    let permanent_id = permanent.card.id;
    assert_eq!(game.power(permanent), Some(4));
    assert_eq!(game.toughness(permanent), Some(4));

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: permanent_id,
            ability: activated_ability_for(&game, permanent_id, 0),
            targets: activated_targets(Target::Player(PlayerId::Two)),
            sacrifice: None,
        },
    )
    .unwrap();
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == permanent_id)
        .unwrap();
    assert_eq!(game.power(permanent), Some(3));
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 19);
}

#[test]
fn tundras_pay_counterspells_double_blue_cost() {
    let mut game = ready_game();
    let bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::Two);
    let counterspell = card(10_001, cards::COUNTERSPELL, PlayerId::One);
    game.players[1].hand.push(bolt.clone());
    game.players[1].mana_pool.red = 1;
    game.players[0].hand.push(counterspell.clone());
    game.battlefield
        .push(creature(10_002, cards::TUNDRA, PlayerId::One));
    game.battlefield
        .push(creature(10_003, cards::TUNDRA, PlayerId::One));
    game.priority = PlayerId::Two;

    game.apply(
        PlayerId::Two,
        cast_action(bolt.id, vec![Target::Player(PlayerId::One)], Vec::new(), 0),
    )
    .unwrap();
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();
    let bolt_on_stack = game.stack[0].id;
    game.apply(
        PlayerId::One,
        cast_action(
            counterspell.id,
            vec![Target::Spell(bolt_on_stack)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.stack.is_empty());
    assert_eq!(game.players[0].life, 20);
    assert_eq!(game.players[0].graveyard[0].definition, cards::COUNTERSPELL);
    assert_eq!(
        game.players[1].graveyard[0].definition,
        cards::LIGHTNING_BOLT
    );
}

#[test]
fn counterspell_removes_an_older_spell_without_disturbing_an_intervening_spell() {
    let mut game = ready_game();
    let older_bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::Two);
    let intervening_bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::Two);
    let counterspell = card(10_002, cards::COUNTERSPELL, PlayerId::One);
    game.players[1]
        .hand
        .extend([older_bolt.clone(), intervening_bolt.clone()]);
    game.players[1].mana_pool.red = 2;
    game.players[0].hand.push(counterspell.clone());
    game.players[0].mana_pool.blue = 2;
    game.priority = PlayerId::Two;

    game.apply(
        PlayerId::Two,
        cast_action(
            older_bolt.id,
            vec![Target::Player(PlayerId::One)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    let older_stack_id = game.stack[0].id;
    game.apply(
        PlayerId::Two,
        cast_action(
            intervening_bolt.id,
            vec![Target::Player(PlayerId::One)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    let intervening_stack_id = game.stack[1].id;
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();
    game.apply(
        PlayerId::One,
        cast_action(
            counterspell.id,
            vec![Target::Spell(older_stack_id)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();

    pass_priority_pair(&mut game);

    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].id, intervening_stack_id);
    assert_eq!(game.players[0].life, 20);
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "the targeted older spell was countered",
    );

    pass_priority_pair(&mut game);
    assert!(game.stack.is_empty());
    assert_eq!(game.players[0].life, 17);
}

#[test]
fn swords_exiles_a_creature_and_grants_life_equal_to_power() {
    let mut game = ready_game();
    let serra = creature(10_000, cards::SERRA_ANGEL, PlayerId::Two);
    let serra_id = serra.card.id;
    game.battlefield.push(serra);
    let swords = card(10_001, cards::SWORDS_TO_PLOWSHARES, PlayerId::One);
    game.players[0].hand.push(swords.clone());
    game.players[0].mana_pool.white = 1;

    game.apply(
        PlayerId::One,
        cast_action(swords.id, vec![Target::Permanent(serra_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.battlefield.is_empty());
    assert_eq!(game.players[1].life, 24);
    assert_eq!(game.players[1].exile[0].definition, cards::SERRA_ANGEL);
}

#[test]
fn swords_cannot_target_order_of_the_ebon_hand() {
    let mut game = ready_game();
    let order = creature(10_000, cards::ORDER_OF_THE_EBON_HAND, PlayerId::Two);
    let order_id = order.card.id;
    game.battlefield.push(order);
    let swords = card(10_001, cards::SWORDS_TO_PLOWSHARES, PlayerId::One);
    game.players[0].hand.push(swords.clone());
    game.players[0].mana_pool.white = 1;

    let swords_action = cast_action(swords.id, vec![Target::Permanent(order_id)], Vec::new(), 0);
    assert!(!game.legal_actions(PlayerId::One).contains(&swords_action));
}

#[test]
fn protection_from_white_prevents_white_blockers() {
    let mut game = ready_game();
    let mut order = creature(10_000, cards::ORDER_OF_THE_EBON_HAND, PlayerId::One);
    order.attacking = true;
    let lion = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    game.battlefield = vec![order, lion];
    game.step = Step::DeclareBlockers;
    game.active_player = PlayerId::One;
    game.attackers_declared = true;
    game.blockers_declared = false;

    assert!(
        !game
            .legal_actions(PlayerId::Two)
            .contains(&Action::DeclareBlocker {
                blocker: CardInstanceId(10_001),
                attacker: CardInstanceId(10_000),
            })
    );
}

#[test]
fn protection_does_not_prevent_a_protected_creature_from_blocking() {
    let mut game = ready_game();
    let mut lion = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    lion.attacking = true;
    let knight = creature(10_001, cards::BLACK_KNIGHT, PlayerId::Two);
    game.battlefield = vec![lion, knight];
    game.step = Step::DeclareBlockers;
    game.active_player = PlayerId::One;
    game.attackers_declared = true;
    game.blockers_declared = false;

    assert!(
        game.legal_actions(PlayerId::Two)
            .contains(&Action::DeclareBlocker {
                blocker: CardInstanceId(10_001),
                attacker: CardInstanceId(10_000),
            })
    );
}

#[test]
fn protection_prevents_damage_from_a_source_of_the_protected_color() {
    let mut game = ready_game();
    let lion = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let knight = creature(10_001, cards::BLACK_KNIGHT, PlayerId::Two);
    let lion_id = lion.card.id;
    let knight_id = knight.card.id;
    game.battlefield = vec![lion, knight];

    game.damage_target_from(Some(lion_id), Some(Target::Permanent(knight_id)), 2);

    let knight = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == knight_id)
        .expect("protection keeps Black Knight on the battlefield");
    assert_eq!(knight.damage, 0);
}

#[test]
fn vampire_nighthawk_deathtouch_and_lifelink_are_executable_evergreen_abilities() {
    let mut game = ready_game();
    game.players[0].life = 10;
    let nighthawk = creature(10_000, cards::VAMPIRE_NIGHTHAWK, PlayerId::One);
    let nighthawk_id = nighthawk.card.id;
    let angel = creature(10_001, cards::SERRA_ANGEL, PlayerId::Two);
    let angel_id = angel.card.id;
    game.battlefield = vec![nighthawk, angel];

    game.damage_target_from(Some(nighthawk_id), Some(Target::Permanent(angel_id)), 1);
    game.check_state_based_actions();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != angel_id),
        "one point from a source with deathtouch is lethal",
    );
    assert_eq!(game.players[0].life, 11);

    game.damage_target_from(Some(nighthawk_id), Some(Target::Player(PlayerId::Two)), 2);
    assert_eq!(game.players[0].life, 13);
    assert_eq!(game.players[1].life, 18);
}

#[test]
fn ancestral_recall_draws_three_and_time_walk_queues_an_extra_turn() {
    let mut game = ready_game();
    let ancestral = card(10_000, cards::ANCESTRAL_RECALL, PlayerId::One);
    game.players[0].hand.push(ancestral.clone());
    game.players[0].mana_pool.blue = 1;
    let hand_before = game.players[0].hand.len();
    game.apply(
        PlayerId::One,
        cast_action(
            ancestral.id,
            vec![Target::Player(PlayerId::One)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].hand.len(), hand_before - 1 + 3);

    let time_walk = card(10_001, cards::TIME_WALK, PlayerId::One);
    game.players[0].hand.push(time_walk.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    game.apply(
        PlayerId::One,
        cast_action(time_walk.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::One);
    assert_eq!(game.observe(PlayerId::One).active_turn, 2);
}

#[test]
fn serra_angel_attacks_without_tapping() {
    let mut game = ready_game();
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    let serra = creature(10_000, cards::SERRA_ANGEL, PlayerId::One);
    let serra_id = serra.card.id;
    game.battlefield.push(serra);

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker { attacker: serra_id },
    )
    .unwrap();

    let serra = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == serra_id)
        .unwrap();
    assert!(serra.attacking);
    assert!(!serra.tapped);
}

#[test]
fn ivory_tower_and_jayemdae_tome_provide_control_card_advantage() {
    let mut game = ready_game();
    game.players[0].life = 10;
    for id in 10_000..10_006 {
        game.players[0]
            .hand
            .push(card(id, cards::MOUNTAIN, PlayerId::One));
    }
    game.battlefield
        .push(creature(10_010, cards::IVORY_TOWER, PlayerId::One));
    let tome = creature(10_011, cards::JAYEMDAE_TOME, PlayerId::One);
    let tome_id = tome.card.id;
    game.battlefield.push(tome);
    game.players[0].mana_pool.colorless = 4;

    game.handle_upkeep_triggers();
    game.finish_rules_procedure();
    assert_eq!(game.players[0].life, 10);
    assert_eq!(game.stack.len(), 1);
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].life, 12);
    let hand_before = game.players[0].hand.len();
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: tome_id,
            ability: activated_ability_for(&game, tome_id, 0),
            targets: Vec::new(),
            sacrifice: None,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].hand.len(), hand_before + 1);
}

#[test]
fn library_of_alexandria_draw_activation_keeps_its_printed_ability_id() {
    let mut game = ready_game();
    for id in 10_000..10_007 {
        game.players[0]
            .hand
            .push(card(id, cards::MOUNTAIN, PlayerId::One));
    }
    let library = creature(10_010, cards::LIBRARY_OF_ALEXANDRIA, PlayerId::One);
    let library_id = library.card.id;
    game.battlefield.push(library);

    let expected_origin = AbilityOrigin::Printed {
        definition: cards::LIBRARY_OF_ALEXANDRIA,
        part: CardPartId::PRIMARY,
        ability: AbilityId(1),
    };
    let activation = Action::ActivateAbility {
        source: library_id,
        ability: expected_origin,
        targets: Vec::new(),
        sacrifice: None,
    };

    assert_eq!(activated_ability_for(&game, library_id, 0), expected_origin);
    assert!(game.legal_actions(PlayerId::One).contains(&activation));
    game.apply(PlayerId::One, activation).unwrap();
    assert_eq!(game.stack[0].ability_origin(), Some(expected_origin));

    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].hand.len(), 8);
}

#[test]
fn fireball_pays_for_multiple_targets_and_divides_x_evenly() {
    let mut game = ready_game();
    let fireball = card(10_000, cards::FIREBALL, PlayerId::One);
    let creature = creature(10_001, cards::SU_CHI, PlayerId::Two);
    let creature_id = creature.card.id;
    game.players[0].hand.push(fireball.clone());
    game.players[0].mana_pool.red = 6;
    game.battlefield.push(creature);

    let action = cast_action(
        fireball.id,
        vec![
            Target::Player(PlayerId::Two),
            Target::Permanent(creature_id),
        ],
        Vec::new(),
        4,
    );
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].mana_pool.total(), 0);
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].life, 18);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == creature_id)
            .unwrap()
            .damage,
        2
    );
}

#[test]
fn fireball_x_three_can_hit_three_targets_for_six_mana() {
    let mut game = ready_game();
    let fireball = card(10_000, cards::FIREBALL, PlayerId::One);
    let first_creature = creature(10_001, cards::SU_CHI, PlayerId::Two);
    let first_creature_id = first_creature.card.id;
    let second_creature = creature(10_002, cards::JUGGERNAUT, PlayerId::Two);
    let second_creature_id = second_creature.card.id;
    game.players[0].hand.push(fireball.clone());
    game.players[0].mana_pool.red = 6;
    game.battlefield.push(first_creature);
    game.battlefield.push(second_creature);

    let action = cast_action(
        fireball.id,
        vec![
            Target::Player(PlayerId::Two),
            Target::Permanent(first_creature_id),
            Target::Permanent(second_creature_id),
        ],
        Vec::new(),
        3,
    );
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].mana_pool.total(), 0);
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].life, 19);
    for creature_id in [first_creature_id, second_creature_id] {
        assert_eq!(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == creature_id)
                .unwrap()
                .damage,
            1
        );
    }
}

#[test]
fn fork_controller_can_retarget_the_copied_spell() {
    let mut game = ready_game();
    let bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::Two);
    let fork = card(10_001, cards::FORK, PlayerId::One);
    game.players[1].hand.push(bolt.clone());
    game.players[1].mana_pool.red = 1;
    game.players[0].hand.push(fork.clone());
    game.players[0].mana_pool.red = 2;
    game.priority = PlayerId::Two;
    game.apply(
        PlayerId::Two,
        cast_action(bolt.id, vec![Target::Player(PlayerId::One)], Vec::new(), 0),
    )
    .unwrap();
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();
    let original = game.stack[0].id;

    game.apply(
        PlayerId::One,
        cast_action(fork.id, vec![Target::Spell(original)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let decision = game.observe(PlayerId::One).decision.unwrap();
    let retarget = decision
        .options
        .iter()
        .find(|option| option.label.contains("your opponent"))
        .map(|option| option.id)
        .unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![retarget],
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].life, 20);
    assert_eq!(game.players[1].life, 17);
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].targets(), vec![Target::Player(PlayerId::One)]);
}

#[test]
fn copied_spell_freezes_retargeted_ability_payload() {
    let mut game = ready_game();
    let shatter = card(10_000, cards::SHATTER, PlayerId::Two);
    let original_target = creature(10_001, cards::SOL_RING, PlayerId::One);
    let replacement_target = creature(10_002, cards::ANKH_OF_MISHRA, PlayerId::One);
    let original_target_id = original_target.card.id;
    let replacement_target_id = replacement_target.card.id;
    game.players[1].hand.push(shatter.clone());
    game.players[1].mana_pool.colorless = 1;
    game.players[1].mana_pool.red = 1;
    game.battlefield
        .extend([original_target, replacement_target]);
    game.priority = PlayerId::Two;

    game.apply(
        PlayerId::Two,
        cast_action(
            shatter.id,
            vec![Target::Permanent(original_target_id)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    let original = game.stack[0].clone();
    let replacement_targets = vec![TargetSelection::single(
        TargetSlotId(0),
        Target::Permanent(replacement_target_id),
    )];

    game.push_copy(original, PlayerId::One, replacement_targets.clone());

    let copy = game.stack.last().expect("the copied spell is on the stack");
    assert_eq!(
        copy.signature.as_ref().map(CastSignature::targets),
        Some(replacement_targets.as_slice()),
    );
    assert_eq!(
        copy.ability
            .as_ref()
            .map(|ability| ability.targets.as_slice()),
        Some(replacement_targets.as_slice()),
        "the executable payload must use the copy's replacement targets",
    );

    game.destroy_permanent(original_target_id);
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != replacement_target_id),
        "the copy must not fizzle because its original target became illegal",
    );
}

#[test]
fn fork_copies_a_targetless_spell_immediately_and_preserves_its_signature() {
    let mut game = ready_game();
    let original = spell(77, cards::DARK_RITUAL, PlayerId::Two, 0);
    let signature = original.signature.clone().unwrap();

    game.queue_fork_decision(PlayerId::One, original);

    assert!(game.pending_decisions.is_empty());
    let copied = game.stack.last().expect("the targetless copy is immediate");
    assert!(copied.is_copy);
    assert_eq!(copied.controller, PlayerId::One);
    assert_eq!(copied.card.backing, ObjectBacking::None);
    assert_eq!(copied.signature.as_ref(), Some(&signature));
}

#[test]
fn fork_can_keep_an_original_target_that_has_become_illegal() {
    let mut game = ready_game();
    let stale_target = Target::Permanent(CardInstanceId(99_999));
    game.queue_fork_decision(
        PlayerId::One,
        spell_with_targets(77, cards::SHATTER, PlayerId::Two, vec![stale_target], 0),
    );
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert!(
        decision
            .options
            .iter()
            .any(|option| option.label == "Keep original targets")
    );
}

#[test]
fn structured_target_predicates_are_rechecked_when_the_spell_resolves() {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    let mut factory = creature(10_000, cards::MISHRA_S_FACTORY, PlayerId::Two);
    factory.factory_animated = true;
    let factory_id = factory.card.id;
    game.battlefield.push(factory);
    let mut turn = spell(77, crate::card::cards::TURN_BURN, PlayerId::One, 0);
    turn.signature = Some(CastSignature::from_validated_choices(
        SpellForm::Part(CardPartId::PRIMARY),
        CastChoices::new(PlayOptionId::DEFAULT).with_targets(vec![TargetSelection::single(
            TargetSlotId(0),
            Target::Permanent(factory_id),
        )]),
    ));

    assert!(!game.spell_fizzles(&turn));
    game.battlefield[0].factory_animated = false;
    assert!(game.spell_fizzles(&turn));
}

#[test]
fn black_lotus_sacrifices_for_three_red_mana() {
    let mut game = ready_game();
    let lotus = creature(10_000, cards::BLACK_LOTUS, PlayerId::One);
    let lotus_id = lotus.card.id;
    game.battlefield.push(lotus);
    let action = Action::ActivateManaAbility {
        source: lotus_id,
        ability: mana_ability_for(&game, lotus_id, ManaColor::Red),
        color: ManaColor::Red,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action).unwrap();

    assert_eq!(game.players[0].mana_pool.red, 3);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != lotus_id)
    );
    let graveyard_lotus = game.players[0].graveyard.last().unwrap();
    assert_ne!(graveyard_lotus.id, lotus_id);
    assert_eq!(
        backing_cards(&graveyard_lotus.backing),
        vec![PhysicalCardId(10_000)]
    );
}

#[test]
fn the_legend_rule_keeps_one_pendelhaven_per_player() {
    let mut game = ready_game();
    let mut old_haven = creature(10_000, cards::PENDELHAVEN, PlayerId::One);
    old_haven.tapped = true;
    game.battlefield.push(old_haven);
    game.players[0]
        .hand
        .push(card(10_001, cards::PENDELHAVEN, PlayerId::One));
    // The opponent's own Pendelhaven is unaffected: the rule is per player.
    game.battlefield
        .push(creature(10_002, cards::PENDELHAVEN, PlayerId::Two));

    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: CardInstanceId(10_001),
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();

    let mine: Vec<_> = game
        .battlefield
        .iter()
        .filter(|permanent| {
            permanent.controller == PlayerId::One && permanent.card.definition == cards::PENDELHAVEN
        })
        .collect();
    assert_eq!(mine.len(), 1, "only one Pendelhaven survives");
    assert_eq!(
        backing_cards(&mine[0].card.backing),
        vec![PhysicalCardId(10_001)],
        "the untapped newcomer is kept over the tapped original",
    );
    assert!(!mine[0].tapped, "the survivor is the untapped one");
    assert_eq!(
        game.players[0].graveyard.len(),
        1,
        "the extra copy went to the graveyard",
    );
    assert!(
        game.battlefield.iter().any(|permanent| {
            permanent.controller == PlayerId::Two && permanent.card.definition == cards::PENDELHAVEN
        }),
        "the opponent keeps theirs",
    );
}

#[test]
fn black_vise_needs_no_target_and_still_squeezes_the_opponent() {
    let mut game = ready_game();
    let vise = card(10_000, cards::BLACK_VISE, PlayerId::One);
    game.players[0].hand.push(vise.clone());
    game.players[0].mana_pool.colorless = 1;

    // With two players "choose an opponent" has one answer, so the cast
    // carries no target and offers the player nothing to pick.
    let cast = cast_action(vise.id, Vec::new(), Vec::new(), 0);
    let casts: Vec<_> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == vise.id))
        .collect();
    assert_eq!(casts, vec![cast.clone()], "exactly one way to cast it");

    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);
    let resolved = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::BLACK_VISE)
        .expect("Black Vise resolved onto the battlefield");
    assert_eq!(
        resolved.chosen_player,
        Some(PlayerId::Two),
        "the opponent is implied rather than chosen",
    );

    // Six cards in hand is two beyond four, so their upkeep costs 2 life.
    for index in 0..6 {
        game.players[1]
            .hand
            .push(card(20_000 + index, cards::MOUNTAIN, PlayerId::Two));
    }
    let before = game.players[1].life;
    game.turn = 2;
    game.active_player = PlayerId::Two;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    assert_eq!(game.players[1].life, before - 2);
}

#[test]
fn mox_ruby_can_pay_black_vises_generic_cost() {
    let mut game = ready_game();
    let mox = creature(10_000, cards::MOX_RUBY, PlayerId::One);
    let vise = card(10_001, cards::BLACK_VISE, PlayerId::One);
    let mox_id = mox.card.id;
    game.battlefield.push(mox);
    game.players[0].hand.push(vise.clone());

    let cast_vise = cast_action(vise.id, Vec::new(), Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast_vise));
    game.apply(PlayerId::One, cast_vise).unwrap();
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == mox_id)
            .is_some_and(|permanent| permanent.tapped)
    );
}

#[test]
fn mana_preview_uses_existing_pool_before_tapping_sources() {
    let mut game = ready_game();
    let mountain = creature(10_000, cards::MOUNTAIN, PlayerId::One);
    let mox = creature(10_001, cards::MOX_RUBY, PlayerId::One);
    let vise = card(10_002, cards::BLACK_VISE, PlayerId::One);
    let mountain_id = mountain.card.id;
    let mox_id = mox.card.id;
    game.battlefield.extend([mox, mountain]);
    game.players[0].mana_pool.colorless = 1;
    game.players[0].hand.push(vise.clone());

    let action = cast_action(vise.id, Vec::new(), Vec::new(), 0);
    assert_eq!(
        game.mana_sources_for_action(PlayerId::One, &action),
        Vec::<CardInstanceId>::new(),
        "the floating mana already pays Black Vise's generic cost"
    );

    game.players[0].mana_pool = ManaPool::default();
    assert_eq!(
        game.mana_sources_for_action(PlayerId::One, &action),
        vec![mox_id],
        "the preview chooses a single flexible source without mutating the game"
    );
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == mountain_id)
            .expect("mountain remains on the battlefield")
            .tapped
    );
}

#[test]
fn mana_preview_uses_the_selected_declarative_activated_ability_cost() {
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::activated_mana(
            crate::AbilityId::PRIMARY,
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Colorless)),
        ),
        AbilityDef::activated(
            crate::AbilityId(1),
            "{1}, {T}: Draw a card.",
            &[
                AbilityCostDef::Mana(ManaCost::new(1, 0)),
                AbilityCostDef::TapSource,
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ];

    let mut game = ready_game();
    let tome = creature(10_000, cards::JAYEMDAE_TOME, PlayerId::One);
    let first_ring = creature(10_001, cards::SOL_RING, PlayerId::One);
    let second_ring = creature(10_002, cards::SOL_RING, PlayerId::One);
    let tome_id = tome.card.id;
    let first_ring_id = first_ring.card.id;
    let second_ring_id = second_ring.card.id;
    game.battlefield.extend([tome, first_ring, second_ring]);
    let action = Action::ActivateAbility {
        source: tome_id,
        ability: activated_ability_for(&game, tome_id, 0),
        targets: Vec::new(),
        sacrifice: None,
    };

    assert!(game.legal_actions(PlayerId::One).contains(&action));
    assert_eq!(
        game.mana_sources_for_action(PlayerId::One, &action),
        vec![first_ring_id, second_ring_id],
        "the behavior-free Tome activation previews its printed four-mana cost",
    );
    assert!(game.battlefield.iter().all(|permanent| !permanent.tapped));

    let definition_id = CardDefinitionId(10_065);
    let mut definition = CardDefinition::new(
        definition_id,
        "Mana preview tap-source test card",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules =
        CardRules::new(CardKind::Artifact, ManaCost::new(0, 0), "").with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let source = CardInstanceId(10_010);
    let mountain = CardInstanceId(10_011);
    game.battlefield.extend([
        creature(source.0, definition_id, PlayerId::One),
        creature(mountain.0, cards::MOUNTAIN, PlayerId::One),
    ]);
    let action = Action::ActivateAbility {
        source,
        ability: activated_ability_for(&game, source, 0),
        targets: Vec::new(),
        sacrifice: None,
    };

    assert_eq!(
        game.mana_sources_for_action(PlayerId::One, &action),
        vec![mountain],
        "a source needed for the activation's tap cost is avoided when another source can pay",
    );
}

#[test]
fn orcish_mechanics_can_sacrifice_an_artifact_to_damage_a_creature() {
    let mut game = ready_game();
    let mechanics = creature(10_000, cards::ORCISH_MECHANICS, PlayerId::One);
    let artifact = creature(10_001, cards::MOX_RUBY, PlayerId::One);
    let target = creature(10_002, cards::SU_CHI, PlayerId::Two);
    let mechanics_id = mechanics.card.id;
    let artifact_id = artifact.card.id;
    let target_id = target.card.id;
    game.battlefield = vec![mechanics, artifact, target];

    let action = Action::ActivateAbility {
        source: mechanics_id,
        ability: activated_ability_for(&game, mechanics_id, 0),
        targets: activated_targets(Target::Permanent(target_id)),
        sacrifice: Some(artifact_id),
    };
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action).unwrap();
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == mechanics_id)
            .is_some_and(|permanent| permanent.tapped)
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != artifact_id)
    );
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].targets(), vec![Target::Permanent(target_id)]);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == target_id)
            .unwrap()
            .damage,
        0
    );

    pass_priority_pair(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == target_id)
            .unwrap()
            .damage,
        2
    );
}

#[test]
fn iron_star_payment_can_use_untapped_mana_sources() {
    let mut game = ready_game();
    let first_mountain = creature(10_000, cards::MOUNTAIN, PlayerId::One);
    let second_mountain = creature(10_001, cards::MOUNTAIN, PlayerId::One);
    let second_mountain_id = second_mountain.card.id;
    game.battlefield.extend([
        first_mountain,
        second_mountain,
        creature(10_002, cards::IRON_STAR, PlayerId::One),
    ]);
    let bolt = card(10_003, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[0].hand.push(bolt);
    game.apply(
        PlayerId::One,
        cast_action(bolt_id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    assert_eq!(game.stack.len(), 2, "Iron Star's trigger is above the Bolt");
    pass_priority_pair(&mut game);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![1],
        },
    )
    .unwrap();

    assert_eq!(game.players[0].life, 21);
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == second_mountain_id)
            .is_some_and(|permanent| permanent.tapped)
    );
}

#[test]
fn chain_lightning_copy_payment_can_use_untapped_mountains() {
    let mut game = ready_game();
    let first = creature(10_000, cards::MOUNTAIN, PlayerId::Two);
    let second = creature(10_001, cards::MOUNTAIN, PlayerId::Two);
    let first_id = first.card.id;
    let second_id = second.card.id;
    game.battlefield = vec![first, second];
    game.queue_chain_lightning_decision(
        PlayerId::Two,
        spell_with_targets(
            77,
            cards::CHAIN_LIGHTNING,
            PlayerId::One,
            vec![Target::Player(PlayerId::Two)],
            0,
        ),
    );
    let decision = game.observe(PlayerId::Two).decision.unwrap();
    let copy = decision
        .options
        .iter()
        .find(|option| option.label.contains("your opponent"))
        .map(|option| option.id)
        .unwrap();
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![copy],
        },
    )
    .unwrap();

    assert_eq!(game.players[1].mana_pool, ManaPool::default());
    assert!(
        game.battlefield
            .iter()
            .filter(|permanent| [first_id, second_id].contains(&permanent.card.id))
            .all(|permanent| permanent.tapped)
    );
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].targets(), vec![Target::Player(PlayerId::One)]);
    assert!(game.stack[0].is_copy);
}

#[test]
fn chain_lightning_copy_payment_can_use_a_creature_dealt_lethal_damage() {
    let mut game = ready_game();
    game.turns_started[PlayerId::Two.index()] = 1;
    let birds = creature(10_000, cards::BIRDS_OF_PARADISE, PlayerId::Two);
    let mountain = creature(10_001, cards::MOUNTAIN, PlayerId::Two);
    let birds_id = birds.card.id;
    let mountain_id = mountain.card.id;
    game.battlefield = vec![birds, mountain];
    let chain = card(10_002, cards::CHAIN_LIGHTNING, PlayerId::One);
    let chain_id = chain.id;
    game.players[0].hand.push(chain);
    game.players[0].mana_pool.red = 1;

    game.apply(
        PlayerId::One,
        cast_action(chain_id, vec![Target::Permanent(birds_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == birds_id)
            .is_some_and(|permanent| permanent.damage == 3),
        "state-based actions wait while Chain Lightning asks whether to pay",
    );
    let decision = game.observe(PlayerId::Two).decision.unwrap();
    let copy = decision
        .options
        .iter()
        .find(|option| option.label.contains("your opponent"))
        .map(|option| option.id)
        .unwrap();
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![copy],
        },
    )
    .unwrap();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != birds_id),
        "Birds dies only after its mana ability pays for the copy",
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == mountain_id)
            .is_some_and(|permanent| permanent.tapped),
    );
    assert_eq!(game.players[1].mana_pool, ManaPool::default());
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].targets(), vec![Target::Player(PlayerId::One)]);
    assert!(game.stack[0].is_copy);
}

#[test]
fn goblin_grenade_requires_and_sacrifices_a_goblin() {
    let mut game = ready_game();
    let grenade = card(10_000, cards::GOBLIN_GRENADE, PlayerId::One);
    let goblin = creature(10_001, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::One);
    let goblin_id = goblin.card.id;
    game.players[0].hand.push(grenade.clone());
    game.players[0].mana_pool.red = 1;
    game.battlefield.push(goblin);
    let action = cast_action(
        grenade.id,
        vec![Target::Player(PlayerId::Two)],
        vec![goblin_id],
        0,
    );
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action).unwrap();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != goblin_id)
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 15);
}

#[test]
fn goblin_grenade_eats_exactly_one_of_two_identical_goblins() {
    let mut game = ready_game();
    let grenade = card(10_000, cards::GOBLIN_GRENADE, PlayerId::One);
    let first = creature(10_001, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::One);
    let second = creature(10_002, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::One);
    let first_id = first.card.id;
    let second_id = second.card.id;
    game.players[0].hand.push(grenade.clone());
    game.players[0].mana_pool.red = 1;
    game.battlefield.push(first);
    game.battlefield.push(second);

    // Each identical Goblin is its own separate cost, not one lumped choice.
    let casts: Vec<_> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == grenade.id))
        .collect();
    assert!(
        casts.iter().all(|action| matches!(
            action,
            Action::CastSpell { sacrifices, .. } if sacrifices.len() == 1
        )),
        "every Grenade cast sacrifices exactly one Goblin",
    );

    game.apply(
        PlayerId::One,
        cast_action(
            grenade.id,
            vec![Target::Player(PlayerId::Two)],
            vec![first_id],
            0,
        ),
    )
    .unwrap();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != first_id),
        "the chosen Goblin is gone",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == second_id),
        "its twin stays on the battlefield",
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 15);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == second_id),
        "resolving the Grenade does not take the twin either",
    );
}

#[test]
fn hypnotic_specter_discards_after_dealing_combat_damage() {
    let mut game = ready_game();
    let mut specter = creature(10_000, cards::HYPNOTIC_SPECTER, PlayerId::One);
    specter.attacking = true;
    game.battlefield.push(specter);
    game.players[1]
        .hand
        .push(card(10_001, cards::MOUNTAIN, PlayerId::Two));

    game.deal_combat_damage();

    assert_eq!(game.players[1].life, 18);
    assert!(game.players[1].hand.is_empty());
    assert_eq!(game.players[1].graveyard.len(), 1);
    assert!(game.events().iter().any(|event| {
        matches!(
            event,
            GameEvent::CardsDiscarded { player: PlayerId::Two, cards }
                if cards.len() == 1 && cards[0].1 == cards::MOUNTAIN
        )
    }));
}

#[test]
#[allow(clippy::too_many_lines)]
fn factory_animates_and_strip_mine_destroys_lands() {
    let mut game = ready_game();
    let factory = creature(10_000, cards::MISHRA_S_FACTORY, PlayerId::One);
    let strip = creature(10_001, cards::STRIP_MINE, PlayerId::One);
    let opposing_factory = creature(10_002, cards::MISHRA_S_FACTORY, PlayerId::Two);
    let factory_id = factory.card.id;
    let strip_id = strip.card.id;
    let opposing_id = opposing_factory.card.id;
    game.battlefield = vec![factory, strip, opposing_factory];
    game.players[0].mana_pool.colorless = 1;

    assert_eq!(
        activated_ability_for(&game, factory_id, 0),
        AbilityOrigin::Printed {
            definition: cards::MISHRA_S_FACTORY,
            part: CardPartId::PRIMARY,
            ability: crate::AbilityId(1),
        }
    );
    assert_eq!(
        activated_ability_for(&game, factory_id, 1),
        AbilityOrigin::Printed {
            definition: cards::MISHRA_S_FACTORY,
            part: CardPartId::PRIMARY,
            ability: crate::AbilityId(2),
        }
    );
    assert_eq!(
        activated_ability_for(&game, strip_id, 0),
        AbilityOrigin::Printed {
            definition: cards::STRIP_MINE,
            part: CardPartId::PRIMARY,
            ability: crate::AbilityId(1),
        }
    );

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: factory_id,
            ability: activated_ability_for(&game, factory_id, 0),
            targets: Vec::new(),
            sacrifice: None,
        },
    )
    .unwrap();
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == factory_id)
            .and_then(|permanent| game.power(permanent)),
        Some(2)
    );
    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&Action::ActivateAbility {
                source: factory_id,
                ability: AbilityOrigin::Printed {
                    definition: cards::MISHRA_S_FACTORY,
                    part: CardPartId::PRIMARY,
                    ability: crate::AbilityId(2),
                },
                targets: activated_targets(Target::Permanent(factory_id)),
                sacrifice: None,
            })
    );

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: strip_id,
            ability: activated_ability_for(&game, strip_id, 0),
            targets: activated_targets(Target::Permanent(opposing_id)),
            sacrifice: None,
        },
    )
    .unwrap();
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::ActivatedAbility);
    assert_eq!(
        game.stack[0].ability_origin(),
        Some(AbilityOrigin::Printed {
            definition: cards::STRIP_MINE,
            part: CardPartId::PRIMARY,
            ability: crate::AbilityId(1),
        })
    );
    assert_eq!(
        game.stack[0].targets(),
        vec![Target::Permanent(opposing_id)]
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != strip_id)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == opposing_id)
    );

    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != opposing_id)
    );
}

#[test]
fn mishras_factory_can_use_its_own_mana_to_animate() {
    let mut game = ready_game();
    let factory = creature(10_000, cards::MISHRA_S_FACTORY, PlayerId::One);
    let factory_id = factory.card.id;
    game.battlefield = vec![factory];
    let animate = Action::ActivateAbility {
        source: factory_id,
        ability: activated_ability_for(&game, factory_id, 0),
        targets: Vec::new(),
        sacrifice: None,
    };

    assert!(game.legal_actions(PlayerId::One).contains(&animate));
    game.apply(PlayerId::One, animate).unwrap();

    let factory = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == factory_id)
        .unwrap();
    assert!(factory.tapped);
    assert_eq!(game.power(factory), Some(2));
    assert_eq!(game.players[0].mana_pool.total(), 0);

    let shatter = card(10_001, cards::SHATTER, PlayerId::Two);
    game.players[1].hand.push(shatter.clone());
    game.players[1].mana_pool.red = 2;
    game.priority = PlayerId::Two;
    assert!(game.legal_actions(PlayerId::Two).contains(&cast_action(
        shatter.id,
        vec![Target::Permanent(factory_id)],
        Vec::new(),
        0,
    )));
}

#[test]
fn an_animated_untapped_mishras_factory_can_block() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::GOBLINS_OF_THE_FLARG, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    let mut factory = creature(10_001, cards::MISHRA_S_FACTORY, PlayerId::Two);
    factory.factory_animated = true;
    let factory_id = factory.card.id;
    game.battlefield = vec![attacker, factory];
    game.active_player = PlayerId::One;
    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;

    assert!(
        game.legal_actions(PlayerId::Two)
            .contains(&Action::DeclareBlocker {
                blocker: factory_id,
                attacker: attacker_id,
            })
    );
}

#[test]
fn strip_mine_can_be_activated_in_response_to_strip_mine() {
    let mut game = ready_game();
    let first_strip = creature(10_000, cards::STRIP_MINE, PlayerId::One);
    let second_strip = creature(10_001, cards::STRIP_MINE, PlayerId::Two);
    let other_land = creature(10_002, cards::MOUNTAIN, PlayerId::Two);
    let first_strip_id = first_strip.card.id;
    let second_strip_id = second_strip.card.id;
    let other_land_id = other_land.card.id;
    game.battlefield = vec![first_strip, second_strip, other_land];
    game.priority = PlayerId::Two;

    game.apply(
        PlayerId::Two,
        Action::ActivateAbility {
            source: second_strip_id,
            ability: activated_ability_for(&game, second_strip_id, 0),
            targets: activated_targets(Target::Permanent(first_strip_id)),
            sacrifice: None,
        },
    )
    .unwrap();
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();

    let response = Action::ActivateAbility {
        source: first_strip_id,
        ability: activated_ability_for(&game, first_strip_id, 0),
        targets: activated_targets(Target::Permanent(other_land_id)),
        sacrifice: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&response));
    game.apply(PlayerId::One, response).unwrap();
    assert_eq!(game.stack.len(), 2);

    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != other_land_id)
    );
    assert_eq!(game.stack.len(), 1);

    pass_priority_pair(&mut game);
    assert!(game.stack.is_empty());
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| ![first_strip_id, second_strip_id].contains(&permanent.card.id))
    );
}

#[test]
fn chaos_orb_uses_the_documented_deterministic_success_rule() {
    let mut game = ready_game();
    let orb = creature(10_000, cards::CHAOS_ORB, PlayerId::One);
    let target = creature(10_001, cards::BLACK_VISE, PlayerId::Two);
    let orb_id = orb.card.id;
    let target_id = target.card.id;
    game.battlefield = vec![orb, target];
    game.players[0].mana_pool.colorless = 1;
    let action = Action::ActivateAbility {
        source: orb_id,
        ability: activated_ability_for(&game, orb_id, 0),
        targets: activated_targets(Target::Permanent(target_id)),
        sacrifice: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action).unwrap();

    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::ActivatedAbility);
    assert_eq!(game.stack[0].chosen_permanents, vec![target_id]);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == orb_id)
            .is_some_and(|permanent| permanent.tapped)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == target_id)
    );
    pass_priority_pair(&mut game);
    assert!(game.battlefield.is_empty());
    assert_eq!(game.players[0].mana_pool.total(), 0);
}

#[test]
fn chaos_orb_can_be_activated_the_turn_it_enters_using_untapped_mana() {
    let mut game = ready_game();
    let mut orb = creature(10_000, cards::CHAOS_ORB, PlayerId::One);
    let mut mountain = creature(10_001, cards::MOUNTAIN, PlayerId::One);
    let target = creature(10_002, cards::BLACK_VISE, PlayerId::Two);
    orb.entered_controller_turn = game.turns_started[PlayerId::One.index()];
    mountain.entered_controller_turn = game.turns_started[PlayerId::One.index()];
    let orb_id = orb.card.id;
    let mountain_id = mountain.card.id;
    let target_id = target.card.id;
    game.battlefield = vec![orb, mountain, target];
    let action = Action::ActivateAbility {
        source: orb_id,
        ability: activated_ability_for(&game, orb_id, 0),
        targets: activated_targets(Target::Permanent(target_id)),
        sacrifice: None,
    };

    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == orb_id)
            .is_some_and(|permanent| permanent.tapped)
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == mountain_id)
            .is_some_and(|permanent| permanent.tapped)
    );
    assert_eq!(game.players[0].mana_pool.total(), 0);
    assert_eq!(game.stack.len(), 1);
}

#[test]
fn icatian_javelineers_cannot_activate_until_their_controller_turn() {
    let mut game = ready_game();
    let mut javeliners = creature(10_000, cards::ICATIAN_JAVELINEERS, PlayerId::One);
    javeliners.javelin_counters = 1;
    javeliners.entered_controller_turn = game.turns_started[PlayerId::One.index()];
    let source = javeliners.card.id;
    game.battlefield = vec![javeliners];
    let action = Action::ActivateAbility {
        source,
        ability: activated_ability_for(&game, source, 0),
        targets: activated_targets(Target::Player(PlayerId::Two)),
        sacrifice: None,
    };
    assert_eq!(game.power(&game.battlefield[0]), Some(1));
    assert_eq!(game.toughness(&game.battlefield[0]), Some(1));

    assert!(!game.legal_actions(PlayerId::One).contains(&action));

    game.start_next_turn();
    game.priority = PlayerId::One;
    assert_eq!(game.active_player, PlayerId::Two);
    assert!(!game.legal_actions(PlayerId::One).contains(&action));

    game.start_next_turn();
    game.priority = PlayerId::One;
    assert_eq!(game.active_player, PlayerId::One);
    assert!(game.legal_actions(PlayerId::One).contains(&action));
}

#[test]
fn removing_chaos_orb_in_response_nullifies_its_flip() {
    let mut game = ready_game();
    let orb = creature(10_000, cards::CHAOS_ORB, PlayerId::One);
    let target = creature(10_001, cards::BLACK_VISE, PlayerId::Two);
    let shatter = card(10_002, cards::SHATTER, PlayerId::Two);
    let orb_id = orb.card.id;
    let target_id = target.card.id;
    game.battlefield = vec![orb, target];
    game.players[0].mana_pool.colorless = 1;
    game.players[1].hand.push(shatter.clone());
    game.players[1].mana_pool.red = 2;

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: orb_id,
            ability: activated_ability_for(&game, orb_id, 0),
            targets: activated_targets(Target::Permanent(target_id)),
            sacrifice: None,
        },
    )
    .unwrap();
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    game.apply(
        PlayerId::Two,
        cast_action(shatter.id, vec![Target::Permanent(orb_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.stack.len(), 1);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != orb_id)
    );

    pass_priority_pair(&mut game);

    assert!(game.stack.is_empty());
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == target_id)
    );
}

#[test]
fn goblin_king_buffs_other_goblins_and_grants_mountainwalk() {
    let mut game = ready_game();
    let king = creature(10_000, cards::GOBLIN_KING, PlayerId::One);
    let mut flarg = creature(10_001, cards::GOBLINS_OF_THE_FLARG, PlayerId::One);
    flarg.attacking = true;
    let mountain = creature(10_002, cards::MOUNTAIN, PlayerId::Two);
    let blocker = creature(10_003, cards::IRONCLAW_ORCS, PlayerId::Two);
    let flarg_id = flarg.card.id;
    game.battlefield = vec![king, flarg, mountain, blocker];
    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;

    let flarg = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == flarg_id)
        .unwrap();
    assert_eq!(game.power(flarg), Some(2));
    assert!(
        game.legal_actions(PlayerId::Two)
            .iter()
            .all(|action| !matches!(
                action,
                Action::DeclareBlocker { attacker, .. } if *attacker == flarg_id
            ))
    );
}

#[test]
fn erhnam_djinn_upkeep_targets_a_creature_for_forestwalk() {
    let mut game = ready_game();
    let erhnam = creature(10_000, cards::ERHNAM_DJINN, PlayerId::One);
    let target = creature(10_001, cards::JUZAM_DJINN, PlayerId::Two);
    let target_id = target.card.id;
    game.battlefield = vec![erhnam, target];
    game.turn = 2;
    game.step = Step::Upkeep;

    game.handle_upkeep_triggers();

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(
        decision.prompt,
        "Erhnam Djinn: choose a creature for forestwalk"
    );
    assert_eq!(decision.options.len(), 1);
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![target_id.0],
        },
    )
    .unwrap();

    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == target_id)
            .unwrap()
            .forestwalk_until_upkeep_of,
        Some(PlayerId::One)
    );
}

#[test]
fn wheel_discards_both_hands_and_draws_seven() {
    let mut game = ready_game();
    let wheel = card(10_000, cards::WHEEL_OF_FORTUNE, PlayerId::One);
    game.players[0].hand.push(wheel.clone());
    game.players[0]
        .hand
        .push(card(10_001, cards::MOUNTAIN, PlayerId::One));
    game.players[1]
        .hand
        .push(card(10_002, cards::MOUNTAIN, PlayerId::Two));
    game.players[0].mana_pool.red = 3;

    game.apply(
        PlayerId::One,
        cast_action(wheel.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].hand.len(), 7);
    assert_eq!(game.players[1].hand.len(), 7);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| backing_cards(&card.backing) == vec![PhysicalCardId(10_001)])
    );
}

#[test]
fn cleanup_without_a_discard_advances_without_priority() {
    let mut game = ready_game();
    game.step = Step::End;
    let first_turn = game.turn;

    pass_priority_pair(&mut game);

    assert_eq!(game.turn, first_turn + 1);
    assert_eq!(game.step, Step::Upkeep);
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.observe(PlayerId::One).active_turn, 1);
    assert_eq!(game.decision_player(), Some(PlayerId::Two));
}

#[test]
fn cleanup_discard_advances_directly_to_the_next_upkeep() {
    let mut game = ready_game();
    game.step = Step::End;
    for id in 10_000..10_008 {
        game.players[0]
            .hand
            .push(card(id, cards::MOUNTAIN, PlayerId::One));
    }

    pass_priority_pair(&mut game);
    assert_eq!(game.step, Step::Cleanup);
    let discard = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::DiscardCards { .. }))
        .unwrap();
    game.apply(PlayerId::One, discard).unwrap();

    assert_eq!(game.turn, 2);
    assert_eq!(game.step, Step::Upkeep);
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.decision_player(), Some(PlayerId::Two));
}

#[test]
fn attacker_controller_assigns_damage_freely_across_multiple_blockers() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::SU_CHI, PlayerId::One);
    attacker.attacking = true;
    let mut first_blocker = creature(10_001, cards::ATOG, PlayerId::Two);
    first_blocker.blocking = Some(attacker.card.id);
    let mut second_blocker = creature(10_002, cards::ATOG, PlayerId::Two);
    second_blocker.blocking = Some(attacker.card.id);
    let attacker_id = attacker.card.id;
    let first_id = first_blocker.card.id;
    let second_id = second_blocker.card.id;
    game.battlefield = vec![attacker, first_blocker, second_blocker];
    game.begin_combat_damage_assignment();

    let assignment = Action::AssignCombatDamage {
        attacker: attacker_id,
        assignments: vec![
            CombatDamageAssignment {
                recipient: Target::Permanent(first_id),
                amount: 1,
            },
            CombatDamageAssignment {
                recipient: Target::Permanent(second_id),
                amount: 3,
            },
        ],
    };
    assert!(game.legal_actions(PlayerId::One).contains(&assignment));
    game.apply(PlayerId::One, assignment).unwrap();

    let first = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == first_id)
        .unwrap();
    assert_eq!(first.damage, 1);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != second_id)
    );
    let attacker = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == attacker_id)
        .unwrap();
    assert_eq!(attacker.damage, 2);
}

#[test]
fn a_single_blocker_needs_no_damage_assignment() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::BALL_LIGHTNING, PlayerId::One);
    attacker.attacking = true;
    let mut blocker = creature(10_001, cards::ATOG, PlayerId::Two);
    blocker.blocking = Some(attacker.card.id);
    let blocker_id = blocker.card.id;
    game.battlefield = vec![attacker, blocker];
    let life_before = game.players[1].life;
    game.begin_combat_damage_assignment();

    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::AssignCombatDamage { .. })),
        "one blocker leaves nothing worth deciding",
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != blocker_id),
        "the blocker still takes lethal damage",
    );
    assert_eq!(
        game.players[1].life,
        life_before - 4,
        "a 6/1 trampler over a 1/2 blocker spills the remaining 4",
    );
}

#[test]
fn trample_requires_lethal_assignment_before_player_damage() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::BALL_LIGHTNING, PlayerId::One);
    attacker.attacking = true;
    let mut first = creature(10_001, cards::ATOG, PlayerId::Two);
    first.blocking = Some(attacker.card.id);
    let mut second = creature(10_002, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::Two);
    second.blocking = Some(attacker.card.id);
    let attacker_id = attacker.card.id;
    let (first_id, second_id) = (first.card.id, second.card.id);
    game.battlefield = vec![attacker, first, second];
    game.begin_combat_damage_assignment();

    let mut recipients = [Target::Permanent(first_id), Target::Permanent(second_id)];
    recipients.sort_unstable();
    let assignment = |to_first: u16, to_second: u16, to_player: u16| {
        let mut assignments: Vec<_> = recipients
            .iter()
            .copied()
            .zip([to_first, to_second])
            .map(|(recipient, amount)| CombatDamageAssignment { recipient, amount })
            .collect();
        assignments.push(CombatDamageAssignment {
            recipient: Target::Player(PlayerId::Two),
            amount: to_player,
        });
        Action::AssignCombatDamage {
            attacker: attacker_id,
            assignments,
        }
    };
    let actions = game.legal_actions(PlayerId::One);
    let lethal: Vec<u16> = recipients
        .iter()
        .map(|target| match target {
            Target::Permanent(id) => game.lethal_damage(*id),
            _ => 0,
        })
        .collect();
    let spare = 6 - lethal[0] - lethal[1];

    assert!(
        actions.contains(&assignment(lethal[0], lethal[1], spare)),
        "lethal to both blockers then trample over is legal",
    );
    assert!(
        !actions.contains(&assignment(lethal[0] - 1, lethal[1], spare + 1)),
        "trample cannot spill while a blocker is short of lethal",
    );
}

#[test]
fn damage_cannot_be_dribbled_across_several_blockers_at_once() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::SU_CHI, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield = vec![attacker];
    let mut ids = Vec::new();
    for index in 0..3 {
        let mut blocker = creature(10_001 + index, cards::ATOG, PlayerId::Two);
        blocker.blocking = Some(attacker_id);
        ids.push(blocker.card.id);
        game.battlefield.push(blocker);
    }
    ids.sort_unstable();
    game.begin_combat_damage_assignment();

    let assignment = |amounts: [u16; 3]| Action::AssignCombatDamage {
        attacker: attacker_id,
        assignments: ids
            .iter()
            .copied()
            .zip(amounts)
            .map(|(id, amount)| CombatDamageAssignment {
                recipient: Target::Permanent(id),
                amount,
            })
            .collect(),
    };
    let actions = game.legal_actions(PlayerId::One);

    // Su-Chi is 4/4 into three 1/2 blockers, so it can kill two of them.
    assert!(
        actions.contains(&assignment([2, 2, 0])),
        "killing two blockers outright is legal",
    );
    assert!(
        !actions.contains(&assignment([1, 1, 2])),
        "only the blocker at the front of the order may be left short of lethal",
    );
}

#[test]
fn green_creatures_get_their_land_bonuses_and_llanowar_elves_make_green() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::TAIGA, PlayerId::One),
        creature(10_001, cards::KIRD_APE, PlayerId::One),
        creature(10_002, cards::LLANOWAR_ELVES, PlayerId::One),
    ]);
    assert_eq!(game.power(&game.battlefield[1]), Some(2));
    assert_eq!(game.toughness(&game.battlefield[1]), Some(3));
    assert_eq!(
        game.mana_ability_activations(&game.battlefield[2])
            .into_iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Green]
    );
}

#[test]
fn lands_expose_their_distinct_printed_mana_abilities() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::FOREST, PlayerId::One),
        creature(10_001, cards::TAIGA, PlayerId::One),
    ]);

    let forest = game.mana_ability_activations(&game.battlefield[0]);
    assert_eq!(forest.len(), 1);
    assert_eq!(forest[0].color, ManaColor::Green);
    assert_eq!(
        forest[0].ability,
        AbilityOrigin::Printed {
            definition: cards::FOREST,
            part: CardPartId::PRIMARY,
            ability: crate::AbilityId::PRIMARY,
        }
    );

    let taiga = game.mana_ability_activations(&game.battlefield[1]);
    assert_eq!(
        taiga
            .iter()
            .map(|activation| (activation.ability, activation.color))
            .collect::<Vec<_>>(),
        vec![
            (
                AbilityOrigin::Printed {
                    definition: cards::TAIGA,
                    part: CardPartId::PRIMARY,
                    ability: crate::AbilityId::PRIMARY,
                },
                ManaColor::Green,
            ),
            (
                AbilityOrigin::Printed {
                    definition: cards::TAIGA,
                    part: CardPartId::PRIMARY,
                    ability: crate::AbilityId(1),
                },
                ManaColor::Red,
            ),
        ]
    );
}

#[test]
fn blood_moon_replaces_nonbasic_land_abilities_with_intrinsic_red_mana() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::BLOOD_MOON, PlayerId::One),
        creature(10_001, cards::CITY_OF_BRASS, PlayerId::One),
        creature(10_002, cards::MISHRA_S_WORKSHOP, PlayerId::One),
        creature(10_003, cards::TAIGA, PlayerId::One),
    ]);

    for permanent in &game.battlefield[1..] {
        assert_eq!(
            game.effective_land_types(permanent),
            [false, false, false, true, false]
        );
        let activations = game.mana_ability_activations(permanent);
        assert_eq!(activations.len(), 1);
        assert_eq!(activations[0].color, ManaColor::Red);
        assert_eq!(
            activations[0].ability,
            AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain)
        );
        assert!(activations[0].effect.restrictions.is_empty());
        assert!(
            game.effective_behavior(permanent).is_none(),
            "Blood Moon grants intrinsic rules, not a special-behavior hook"
        );
    }
}

#[test]
fn copy_artifact_copies_an_artifact_creature() {
    let mut game = ready_game();
    let source = creature(10_000, cards::TETRAVUS, PlayerId::Two);
    game.battlefield.push(source);
    let copy = card(10_001, cards::COPY_ARTIFACT, PlayerId::One);
    game.players[0].hand.push(copy.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    let action = cast_action(
        copy.id,
        vec![Target::Permanent(CardInstanceId(10_000))],
        Vec::new(),
        0,
    );
    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);
    let copied = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::COPY_ARTIFACT)
        .unwrap();
    assert_eq!(
        game.effective_behavior(copied),
        Some(CardBehavior::Tetravus)
    );
    assert_eq!(copied.presented, CardPartId::PRIMARY);
    assert_eq!(
        game.effective_rules(copied),
        Some(CardBehavior::Tetravus.rules())
    );
    assert_eq!(game.power(copied), Some(4));
    assert!(game.has_flying(copied));
}

#[test]
fn copy_artifact_resolves_a_copied_icy_manipulator_ability_from_its_frozen_origin() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::ICY_MANIPULATOR, PlayerId::Two),
        creature(10_001, cards::MOUNTAIN, PlayerId::Two),
    ]);
    let copy = card(10_002, cards::COPY_ARTIFACT, PlayerId::One);
    game.players[0].hand.push(copy.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    game.apply(
        PlayerId::One,
        cast_action(
            copy.id,
            vec![Target::Permanent(CardInstanceId(10_000))],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let copied_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::COPY_ARTIFACT)
        .expect("Copy Artifact resolved")
        .card
        .id;
    let target_id = CardInstanceId(10_001);
    let ability = activated_ability_for(&game, copied_id, 0);
    assert_eq!(ability, primary_ability(cards::ICY_MANIPULATOR));

    game.players[0].mana_pool.colorless = 1;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: copied_id,
            ability,
            targets: activated_targets(Target::Permanent(target_id)),
            sacrifice: None,
        },
    )
    .unwrap();
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].card.definition, cards::ICY_MANIPULATOR);
    assert_eq!(
        game.stack[0].ability_origin(),
        Some(primary_ability(cards::ICY_MANIPULATOR))
    );
    assert_eq!(
        game.observe(PlayerId::One).stack[0].definition,
        cards::ICY_MANIPULATOR,
        "stack presentation follows the frozen copied ability definition",
    );

    game.destroy_permanent(copied_id);
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == target_id)
            .is_some_and(|permanent| permanent.tapped),
        "the copied Icy ability resolves after its physical source leaves play",
    );
}

#[test]
fn granted_activation_freezes_payload_before_sacrificing_grant_source() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        TargetSlotId(0),
        "any target",
        AbilityTargetPredicate::AnyTarget,
    )];
    static GRANTED_ABILITY: AbilityDef = AbilityDef::activated(
        crate::AbilityId(1),
        "Sacrifice an artifact: This creature deals 2 damage to any target.",
        &[AbilityCostDef::SacrificePermanent {
            object: ObjectPredicateDef::Artifact,
            controller: PlayerRelation::You,
        }],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetSlotId(0)),
            amount: ValueDef::Constant(2),
        },
    )
    .with_targets(&TARGETS);
    static GRANTOR_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
        crate::AbilityId::PRIMARY,
        "Creatures you control have the test ability.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::Creature,
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            },
            effect: AppliedEffectDef::GrantAbility(&GRANTED_ABILITY),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )];
    let grantor_definition_id = CardDefinitionId(10_062);
    let mut grantor_definition = CardDefinition::new(
        grantor_definition_id,
        "Activated snapshot test grantor",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    grantor_definition.rules = CardRules::new(CardKind::Artifact, ManaCost::new(0, 0), "")
        .with_abilities(&GRANTOR_ABILITIES);
    synchronize_single_part_definition(&mut grantor_definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(grantor_definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let grantor = CardInstanceId(10_000);
    let receiver = CardInstanceId(10_001);
    game.battlefield.extend([
        creature(grantor.0, grantor_definition_id, PlayerId::One),
        creature(receiver.0, cards::ATOG, PlayerId::One),
    ]);
    let origin = AbilityOrigin::Granted {
        source: grantor,
        ability: crate::AbilityId(1),
    };
    let activation = Action::ActivateAbility {
        source: receiver,
        ability: origin,
        targets: activated_targets(Target::Player(PlayerId::Two)),
        sacrifice: Some(grantor),
    };
    assert!(game.legal_actions(PlayerId::One).contains(&activation));

    game.apply(PlayerId::One, activation).unwrap();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != grantor),
        "the continuous-effect source was sacrificed as the activation cost",
    );
    let payload = game.stack[0]
        .ability
        .as_ref()
        .expect("the activated ability has a frozen stack payload");
    assert_eq!(payload.origin, origin);
    assert_eq!(payload.target_defs, &TARGETS);
    assert_eq!(
        payload.targets,
        vec![TargetSelection::single(
            TargetSlotId(0),
            Target::Player(PlayerId::Two),
        )],
    );
    assert!(matches!(
        payload.resolver,
        StackAbilityResolver::Declarative(EffectDef::DealDamage { .. })
    ));

    pass_priority_pair(&mut game);
    assert_eq!(
        game.players[1].life, 18,
        "resolution must use the definition frozen before the grant disappeared",
    );
}

static MULTI_SLOT_ACTIVATION_TARGETS: [AbilityTargetDef; 2] = [
    AbilityTargetDef::exactly_one(
        TargetSlotId(3),
        "opponent",
        AbilityTargetPredicate::Player(PlayerRelation::Opponent),
    ),
    AbilityTargetDef::exactly_one(
        TargetSlotId(7),
        "creature an opponent controls",
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Creature,
            zones: &[ZoneKind::Battlefield],
            controller: Some(PlayerRelation::Opponent),
            owner: None,
        },
    ),
];
static MULTI_SLOT_ACTIVATION_EFFECTS: [EffectDef; 2] = [
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetSlotId(3)),
        amount: ValueDef::Constant(1),
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetSlotId(7)),
        amount: ValueDef::Constant(1),
    },
];
static MULTI_SLOT_ACTIVATION_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
    crate::AbilityId::PRIMARY,
    "Sacrifice this artifact: It deals 1 damage to target opponent and 1 damage to target creature that player controls.",
    &[AbilityCostDef::SacrificeSource],
    EffectDef::Sequence(&MULTI_SLOT_ACTIVATION_EFFECTS),
)
.with_targets(&MULTI_SLOT_ACTIVATION_TARGETS)];

#[test]
fn declarative_activation_preserves_multiple_slots_before_sacrificing_its_source() {
    let definition_id = CardDefinitionId(10_063);
    let mut definition = CardDefinition::new(
        definition_id,
        "Multi-slot activation test card",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new(CardKind::Artifact, ManaCost::new(0, 0), "")
        .with_abilities(&MULTI_SLOT_ACTIVATION_ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let source = CardInstanceId(10_000);
    let creature_target = CardInstanceId(10_001);
    game.battlefield.extend([
        creature(source.0, definition_id, PlayerId::One),
        creature(creature_target.0, cards::SERRA_ANGEL, PlayerId::Two),
    ]);
    let targets = vec![
        TargetSelection::single(TargetSlotId(3), Target::Player(PlayerId::Two)),
        TargetSelection::single(TargetSlotId(7), Target::Permanent(creature_target)),
    ];
    let activation = Action::ActivateAbility {
        source,
        ability: primary_ability(definition_id),
        targets: targets.clone(),
        sacrifice: None,
    };

    let invalid_slots = Action::ActivateAbility {
        source,
        ability: primary_ability(definition_id),
        targets: vec![
            TargetSelection::single(TargetSlotId(7), Target::Player(PlayerId::Two)),
            TargetSelection::single(TargetSlotId(3), Target::Permanent(creature_target)),
        ],
        sacrifice: None,
    };
    assert!(game.apply(PlayerId::One, invalid_slots).is_err());
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == source),
        "slot validation must happen before sacrificing the source",
    );
    assert!(game.stack.is_empty());

    assert!(
        game.legal_actions(PlayerId::One).contains(&activation),
        "declarative action generation must retain abilities with multiple target slots",
    );
    game.apply(PlayerId::One, activation).unwrap();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != source),
        "the source was sacrificed as an activation cost",
    );
    let payload = game.stack[0]
        .ability
        .as_ref()
        .expect("the activated ability has a frozen payload");
    assert_eq!(payload.target_defs, &MULTI_SLOT_ACTIVATION_TARGETS);
    assert_eq!(payload.targets, targets);

    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 19);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == creature_target)
            .is_some_and(|permanent| permanent.damage == 1),
    );
}

#[test]
fn one_ability_target_slot_resolves_for_every_selected_legal_target() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef {
        id: TargetSlotId(5),
        label: "up to two creatures an opponent controls",
        predicate: AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Creature,
            zones: &[ZoneKind::Battlefield],
            controller: Some(PlayerRelation::Opponent),
            owner: None,
        },
        minimum: 1,
        maximum: 2,
    }];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
        crate::AbilityId::PRIMARY,
        "Deal 1 damage to up to two target creatures an opponent controls.",
        &[],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetSlotId(5)),
            amount: ValueDef::Constant(1),
        },
    )
    .with_targets(&TARGETS)];

    let definition_id = CardDefinitionId(10_064);
    let mut definition = CardDefinition::new(
        definition_id,
        "Multi-target slot test card",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules =
        CardRules::new(CardKind::Artifact, ManaCost::new(0, 0), "").with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let source = CardInstanceId(10_000);
    let first_target = CardInstanceId(10_001);
    let second_target = CardInstanceId(10_002);
    game.battlefield.extend([
        creature(source.0, definition_id, PlayerId::One),
        creature(first_target.0, cards::SERRA_ANGEL, PlayerId::Two),
        creature(second_target.0, cards::SERRA_ANGEL, PlayerId::Two),
    ]);
    let action = Action::ActivateAbility {
        source,
        ability: primary_ability(definition_id),
        targets: vec![TargetSelection::new(
            TargetSlotId(5),
            vec![
                Target::Permanent(first_target),
                Target::Permanent(second_target),
            ],
        )],
        sacrifice: None,
    };

    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);

    for target in [first_target, second_target] {
        assert!(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == target)
                .is_some_and(|permanent| permanent.damage == 1),
            "every legal target selected in the slot receives the effect",
        );
    }
}

#[test]
fn granted_ability_keeps_its_frozen_resolver_when_the_source_changes() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        TargetSlotId(0),
        "permanent",
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Any,
            zones: &[ZoneKind::Battlefield],
            controller: None,
            owner: None,
        },
    )];
    static GRANTED_ABILITY: AbilityDef = AbilityDef::activated(
        crate::AbilityId(1),
        "{T}: Tap target permanent.",
        &[AbilityCostDef::TapSource],
        EffectDef::Tap {
            object: EffectRecipientDef::Target(TargetSlotId(0)),
        },
    )
    .with_targets(&TARGETS)
    .with_implementation(AbilityImplementationDef::CustomFull {
        behavior: Some(CardBehavior::IcyManipulator),
        explanation: "The test intentionally grants a custom resolver.",
    });
    static SOURCE_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
        crate::AbilityId::PRIMARY,
        "This permanent has the test ability.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::GrantAbility(&GRANTED_ABILITY),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )];
    let definition_id = CardDefinitionId(10_061);
    let mut definition = CardDefinition::new(
        definition_id,
        "Granted resolver test card",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new(CardKind::Artifact, ManaCost::new(0, 0), "")
        .with_abilities(&SOURCE_ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    game.battlefield.extend([
        creature(10_000, definition_id, PlayerId::One),
        creature(10_001, cards::MOUNTAIN, PlayerId::Two),
    ]);
    let source = CardInstanceId(10_000);
    let target = CardInstanceId(10_001);
    let source_card = game.battlefield[0].card.clone();
    let origin = AbilityOrigin::Granted {
        source,
        ability: crate::AbilityId(1),
    };
    let frozen = game.freeze_activated_ability(&game.battlefield[0], origin);

    game.push_activated_ability(
        source,
        &source_card,
        PlayerId::One,
        frozen,
        activated_targets(Target::Permanent(target)),
        Vec::new(),
    );
    assert_eq!(game.stack[0].ability_origin(), Some(origin));
    assert!(matches!(
        game.stack[0]
            .ability
            .as_ref()
            .map(|ability| ability.resolver),
        Some(StackAbilityResolver::Custom(CardBehavior::IcyManipulator))
    ));

    // This models a continuous/copy effect changing the effective rules of a
    // source after activation. The origin remains provenance, while the stack
    // object's executable payload must remain the Icy Manipulator procedure.
    game.battlefield[0].copied_from = Some((cards::JAYEMDAE_TOME, CardPartId::PRIMARY));
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == target)
            .is_some_and(|permanent| permanent.tapped),
        "resolution must not rediscover a different handler from the changed source",
    );
}

#[test]
fn declarative_clause_uses_its_own_resolver_on_a_card_with_custom_behavior() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        TargetSlotId(0),
        "any target",
        AbilityTargetPredicate::AnyTarget,
    )];
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::activated(
            crate::AbilityId::PRIMARY,
            "Deal 1 damage to any target.",
            &[],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetSlotId(0)),
                amount: ValueDef::Constant(1),
            },
        )
        .with_targets(&TARGETS),
        AbilityDef::custom_full(
            crate::AbilityId(1),
            "A separate custom clause.",
            CardBehavior::IcyManipulator,
            "The test keeps one explicitly custom clause beside the declarative clause.",
        ),
    ];
    let definition_id = CardDefinitionId(10_060);
    let mut definition = CardDefinition::new(
        definition_id,
        "Mixed resolver test card",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules =
        CardRules::new(CardKind::Artifact, ManaCost::new(0, 0), "").with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    game.battlefield
        .push(creature(10_060, definition_id, PlayerId::One));
    let source = CardInstanceId(10_060);
    let source_card = game.battlefield[0].card.clone();
    let origin = primary_ability(definition_id);
    let frozen = game.freeze_activated_ability(&game.battlefield[0], origin);

    game.push_activated_ability(
        source,
        &source_card,
        PlayerId::One,
        frozen,
        activated_targets(Target::Player(PlayerId::Two)),
        Vec::new(),
    );
    assert!(matches!(
        game.stack[0]
            .ability
            .as_ref()
            .map(|ability| ability.resolver),
        Some(StackAbilityResolver::Declarative(
            EffectDef::DealDamage { .. }
        ))
    ));

    pass_priority_pair(&mut game);
    assert_eq!(
        game.players[1].life, 19,
        "the selected definition must not dispatch through Icy's unrelated hook",
    );
}

#[test]
fn resolving_ability_masks_an_illegal_target_in_each_frozen_slot() {
    static TARGETS: [AbilityTargetDef; 2] = [
        AbilityTargetDef::exactly_one(
            TargetSlotId(0),
            "first creature you control",
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Creature,
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        ),
        AbilityTargetDef::exactly_one(
            TargetSlotId(1),
            "second creature you control",
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Creature,
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        ),
    ];
    static EFFECTS: [EffectDef; 2] = [
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetSlotId(0)),
            amount: ValueDef::Constant(1),
        },
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetSlotId(1)),
            amount: ValueDef::Constant(1),
        },
    ];

    let mut game = ready_game();
    let source = CardInstanceId(10_000);
    let first = CardInstanceId(10_001);
    let second = CardInstanceId(10_002);
    game.battlefield.extend([
        creature(source.0, cards::ANKH_OF_MISHRA, PlayerId::One),
        creature(first.0, cards::SERRA_ANGEL, PlayerId::One),
        creature(second.0, cards::SERRA_ANGEL, PlayerId::One),
    ]);
    game.stack.push(StackObject {
        id: StackObjectId(20_000),
        kind: StackObjectKind::TriggeredAbility,
        card: card(20_000, cards::ANKH_OF_MISHRA, PlayerId::One),
        source: Some(source),
        ability: Some(StackAbilityPayload {
            origin: primary_ability(cards::ANKH_OF_MISHRA),
            presentation_definition: cards::ANKH_OF_MISHRA,
            text: Some("Test two-slot trigger"),
            target_defs: &TARGETS,
            targets: vec![
                TargetSelection::single(TargetSlotId(0), Target::Permanent(first)),
                TargetSelection::single(TargetSlotId(1), Target::Permanent(second)),
            ],
            context: TriggerContext {
                object: None,
                object_controller: None,
                event_player: None,
                amount: None,
            },
            resolver: StackAbilityResolver::Declarative(EffectDef::Sequence(&EFFECTS)),
        }),
        controller: PlayerId::One,
        signature: None,
        chosen_permanents: Vec::new(),
        applied_effects: Vec::new(),
        is_copy: false,
    });

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == first)
        .unwrap()
        .controller = PlayerId::Two;
    pass_priority_pair(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == first)
            .unwrap()
            .damage,
        0,
        "an illegal target in one slot is ignored",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == second)
            .unwrap()
            .damage,
        1,
        "the legal target in the other slot still receives its effect",
    );
}

#[test]
fn copy_artifact_copies_declarative_mana_abilities_without_a_behavior_hook() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SOL_RING, PlayerId::Two));
    let copy = card(10_001, cards::COPY_ARTIFACT, PlayerId::One);
    game.players[0].hand.push(copy.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    let action = cast_action(
        copy.id,
        vec![Target::Permanent(CardInstanceId(10_000))],
        Vec::new(),
        0,
    );
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);

    let copied_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::COPY_ARTIFACT)
        .expect("Copy Artifact resolved")
        .card
        .id;
    let ability = mana_ability_for(&game, copied_id, ManaColor::Colorless);
    assert_eq!(ability, primary_ability(cards::SOL_RING));
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: copied_id,
            ability,
            color: ManaColor::Colorless,
        },
    )
    .unwrap();

    assert_eq!(game.players[0].mana_pool.colorless, 2);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == copied_id)
            .is_some_and(|permanent| permanent.tapped)
    );
}

#[test]
fn dust_to_dust_exiles_two_artifacts_and_hurkyls_recall_returns_them() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::SOL_RING, PlayerId::Two),
        creature(10_001, cards::BLACK_VISE, PlayerId::Two),
    ]);
    let dust = spell(10_002, cards::DUST_TO_DUST, PlayerId::One, 0);
    dust_to_dust_targets(&mut game, dust);
    assert_eq!(game.players[0].exile.len(), 0);
    assert_eq!(game.players[1].exile.len(), 2);

    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::SOL_RING, PlayerId::Two),
        creature(10_001, cards::BLACK_VISE, PlayerId::Two),
    ]);
    let recall = spell_with_targets(
        10_002,
        cards::HURKYLS_RECALL,
        PlayerId::One,
        vec![Target::Player(PlayerId::Two)],
        0,
    );
    game.resolve_spell_effect(&recall, CardBehavior::HurkylsRecall);
    assert_eq!(game.players[1].hand.len(), 2);
    assert!(game.battlefield.is_empty());
}

fn dust_to_dust_targets(game: &mut Game, mut spell: StackObject) {
    spell.signature = Some(CastSignature::from_validated_choices(
        SpellForm::Part(CardPartId::PRIMARY),
        cast_choices(
            vec![
                Target::Permanent(CardInstanceId(10_000)),
                Target::Permanent(CardInstanceId(10_001)),
            ],
            0,
        ),
    ));
    game.resolve_spell_effect(&spell, CardBehavior::DustToDust);
}

#[test]
fn regeneration_shields_stop_destroy_but_not_wrath() {
    let mut game = ready_game();
    let mut troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    troll.regeneration_shields = 1;
    game.battlefield.push(troll);
    game.destroy_permanent(CardInstanceId(10_000));
    assert_eq!(game.battlefield.len(), 1);
    assert!(game.battlefield[0].tapped);
    assert_eq!(game.battlefield[0].regeneration_shields, 0);

    let wrath = spell(10_001, cards::WRATH_OF_GOD, PlayerId::Two, 0);
    let effect = game
        .catalog
        .get(cards::WRATH_OF_GOD)
        .expect("Wrath of God is in the catalog")
        .rules
        .ability_clauses()[0]
        .effect;
    game.resolve_effect_def(effect, &wrath, TriggerContext::empty());
    assert!(game.battlefield.is_empty());
}

#[test]
fn moat_prevents_nonfliers_and_argothian_pixies_dodge_artifact_blockers() {
    let mut game = ready_game();
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.battlefield
        .push(creature(10_000, cards::MOAT, PlayerId::Two));
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::SERENDIB_EFREET, PlayerId::One));
    let actions = game.legal_actions(PlayerId::One);
    assert!(!actions.contains(&Action::DeclareAttacker {
        attacker: CardInstanceId(10_001)
    }));
    assert!(actions.contains(&Action::DeclareAttacker {
        attacker: CardInstanceId(10_002)
    }));

    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;
    let mut pixies = creature(10_003, cards::ARGOTHIAN_PIXIES, PlayerId::One);
    pixies.attacking = true;
    game.battlefield.push(pixies);
    game.battlefield
        .push(creature(10_004, cards::SU_CHI, PlayerId::Two));
    assert!(
        !game
            .legal_actions(PlayerId::Two)
            .contains(&Action::DeclareBlocker {
                blocker: CardInstanceId(10_004),
                attacker: CardInstanceId(10_003),
            })
    );
}

#[test]
fn firebreathing_is_offered_while_the_mana_is_still_in_the_land() {
    for definition in [
        cards::DRAGON_WHELP,
        cards::GOBLIN_BALLOON_BRIGADE,
        cards::GRANITE_GARGOYLE,
    ] {
        let mut game = ready_game();
        let source = creature(10_000, definition, PlayerId::One);
        let source_id = source.card.id;
        game.battlefield.push(source);
        game.battlefield
            .push(creature(10_001, cards::MOUNTAIN, PlayerId::One));
        assert_eq!(game.players[0].mana_pool.red, 0);

        let activation = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source, targets, .. }
                    if targets.is_empty() && *source == source_id)
            })
            .expect("the ability is offered with an untapped Mountain and an empty pool");

        let before = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source_id)
            .map(|permanent| {
                (
                    game.power(permanent),
                    game.toughness(permanent),
                    game.has_flying(permanent),
                )
            })
            .unwrap();
        game.apply(PlayerId::One, activation).unwrap();
        while !game.stack.is_empty() {
            game.apply(PlayerId::One, Action::PassPriority).unwrap();
            game.apply(PlayerId::Two, Action::PassPriority).unwrap();
        }

        assert!(
            game.battlefield
                .iter()
                .any(|permanent| permanent.card.definition == cards::MOUNTAIN && permanent.tapped),
            "activating tapped the land for you",
        );
        assert_eq!(
            game.players[0].mana_pool.red, 0,
            "and spent exactly the red it produced",
        );
        let after = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source_id)
            .map(|permanent| {
                (
                    game.power(permanent),
                    game.toughness(permanent),
                    game.has_flying(permanent),
                )
            })
            .unwrap();
        match definition {
            cards::DRAGON_WHELP => {
                assert_eq!(
                    after.0,
                    before.0.map(|value| value + 1),
                    "Dragon Whelp grew"
                );
            }
            cards::GOBLIN_BALLOON_BRIGADE => {
                assert!(!before.2);
                assert!(after.2, "Goblin Balloon Brigade gained flying");
            }
            cards::GRANITE_GARGOYLE => {
                assert_eq!(
                    after.1,
                    before.1.map(|value| value + 1),
                    "Granite Gargoyle gained toughness",
                );
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn declarative_ritual_psionic_blast_and_sign_in_blood_resolve() {
    let mut game = ready_game();
    let ritual = card(10_000, cards::DARK_RITUAL, PlayerId::One);
    game.players[0].hand.push(ritual.clone());
    game.players[0].mana_pool.black = 1;
    game.apply(
        PlayerId::One,
        cast_action(ritual.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].mana_pool.black, 3);

    let mut game = ready_game();
    let blast = card(10_000, cards::PSIONIC_BLAST, PlayerId::One);
    game.players[0].hand.push(blast.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        cast_action(blast.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].life, 18);
    assert_eq!(game.players[1].life, 16);

    let mut game = ready_game();
    let sign = card(10_000, cards::SIGN_IN_BLOOD, PlayerId::One);
    game.players[0].hand.push(sign.clone());
    game.players[0].mana_pool.black = 2;
    game.apply(
        PlayerId::One,
        cast_action(sign.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].hand.len(), 2);
    assert_eq!(game.players[1].life, 18);
}

#[test]
fn any_target_damage_can_remove_a_planeswalker() {
    let definition_id = CardDefinitionId(10_075);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test Planeswalker",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new(CardKind::Planeswalker, ManaCost::default(), "")
        .with_supertype(CardSupertype::Legendary)
        .with_subtypes(&["Test"])
        .planeswalker(3);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let mut planeswalker = creature(10_000, definition_id, PlayerId::Two);
    planeswalker.loyalty = Some(3);
    let planeswalker_id = planeswalker.card.id;
    game.battlefield.push(planeswalker);
    let bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::One);
    game.players[0].hand.push(bolt.clone());
    game.players[0].mana_pool.red = 1;

    let action = cast_action(
        bolt.id,
        vec![Target::Permanent(planeswalker_id)],
        Vec::new(),
        0,
    );
    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != planeswalker_id)
    );
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == definition_id)
    );
}

#[test]
fn declarative_destroy_spells_enforce_their_target_types_and_resolve() {
    for (spell_definition, target_definition, colored_mana) in [
        (cards::SHATTER, cards::BLACK_VISE, ManaColor::Red),
        (cards::DISENCHANT, cards::ENERGY_FLUX, ManaColor::White),
        (cards::SINKHOLE, cards::MOUNTAIN, ManaColor::Black),
        (cards::STONE_RAIN, cards::MOUNTAIN, ManaColor::Red),
    ] {
        let mut game = ready_game();
        let target = creature(10_000, target_definition, PlayerId::Two);
        let target_id = target.card.id;
        let spell = card(10_001, spell_definition, PlayerId::One);
        game.battlefield.push(target);
        game.players[0].hand.push(spell.clone());
        game.players[0].mana_pool.colorless = 3;
        match colored_mana {
            ManaColor::White => game.players[0].mana_pool.white = 2,
            ManaColor::Blue => game.players[0].mana_pool.blue = 2,
            ManaColor::Black => game.players[0].mana_pool.black = 2,
            ManaColor::Red => game.players[0].mana_pool.red = 2,
            ManaColor::Green => game.players[0].mana_pool.green = 2,
            ManaColor::Colorless => game.players[0].mana_pool.colorless = 5,
        }
        let action = cast_action(spell.id, vec![Target::Permanent(target_id)], Vec::new(), 0);
        assert!(
            game.legal_actions(PlayerId::One).contains(&action),
            "{spell_definition:?} accepts its declared target type",
        );
        game.apply(PlayerId::One, action).unwrap();
        pass_priority_pair(&mut game);
        assert!(
            game.battlefield
                .iter()
                .all(|permanent| permanent.card.id != target_id),
            "{spell_definition:?} destroys its target on resolution",
        );
    }
}

#[test]
fn sage_and_relic_barrier_use_the_shared_activated_ability_stack() {
    let mut game = ready_game();
    let sage = creature(10_000, cards::SAGE_OF_LAT_NAM, PlayerId::One);
    let sage_id = sage.card.id;
    let ring = creature(10_001, cards::SOL_RING, PlayerId::One);
    let ring_id = ring.card.id;
    game.battlefield = vec![sage, ring];
    let hand_before = game.players[0].hand.len();
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: sage_id,
            ability: activated_ability_for(&game, sage_id, 0),
            targets: Vec::new(),
            sacrifice: Some(ring_id),
        },
    )
    .unwrap();
    assert!(game.battlefield[0].tapped);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != ring_id)
    );
    assert_eq!(game.stack.len(), 1);
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].hand.len(), hand_before + 1);

    let mut game = ready_game();
    let barrier = creature(10_000, cards::RELIC_BARRIER, PlayerId::One);
    let barrier_id = barrier.card.id;
    let ring = creature(10_001, cards::SOL_RING, PlayerId::Two);
    let ring_id = ring.card.id;
    game.battlefield = vec![barrier, ring];
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: barrier_id,
            ability: activated_ability_for(&game, barrier_id, 0),
            targets: activated_targets(Target::Permanent(ring_id)),
            sacrifice: None,
        },
    )
    .unwrap();
    assert!(game.battlefield[0].tapped);
    assert!(!game.battlefield[1].tapped);
    pass_priority_pair(&mut game);
    assert!(game.battlefield[1].tapped);
}

#[test]
fn migrated_upkeep_and_death_triggers_resolve_from_the_stack() {
    for (definition, active_player, damaged_player) in [
        (cards::COPPER_TABLET, PlayerId::Two, PlayerId::Two),
        (cards::JUZAM_DJINN, PlayerId::One, PlayerId::One),
        (cards::SERENDIB_EFREET, PlayerId::One, PlayerId::One),
    ] {
        let mut game = ready_game();
        game.active_player = active_player;
        game.priority = active_player;
        game.step = Step::Upkeep;
        game.battlefield
            .push(creature(10_000, definition, PlayerId::One));
        game.handle_upkeep_triggers();
        game.finish_rules_procedure();
        assert_eq!(game.stack.len(), 1);
        pass_priority_pair(&mut game);
        assert_eq!(game.players[damaged_player.index()].life, 19);
        assert_eq!(game.players[damaged_player.opponent().index()].life, 20);
    }

    let mut game = ready_game();
    let vampire = creature(10_000, cards::SENGIR_VAMPIRE, PlayerId::One);
    let vampire_id = vampire.card.id;
    let lion = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    let lion_id = lion.card.id;
    game.battlefield = vec![vampire, lion];
    game.damage_target_from(Some(vampire_id), Some(Target::Permanent(lion_id)), 1);
    game.check_state_based_actions();
    game.finish_rules_procedure();
    assert_eq!(game.stack.len(), 1);
    pass_priority_pair(&mut game);
    let vampire = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == vampire_id)
        .unwrap();
    assert_eq!(vampire.plus_one_counters, 1);
    assert_eq!(game.power(vampire), Some(5));
}

#[test]
fn state_based_actions_repeat_after_static_toughness_bonuses_disappear() {
    let mut game = ready_game();
    let mut first_king = creature(10_000, cards::GOBLIN_KING, PlayerId::One);
    first_king.damage = 3;
    let mut second_king = creature(10_001, cards::GOBLIN_KING, PlayerId::One);
    second_king.damage = 2;
    let mut balloon = creature(10_002, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::One);
    balloon.damage = 1;
    game.battlefield = vec![first_king, second_king, balloon];

    game.check_state_based_actions();

    assert!(game.battlefield.is_empty());
    assert_eq!(game.players[0].graveyard.len(), 3);
}

#[test]
fn simultaneous_deaths_use_the_pre_exit_trigger_listener_snapshot() {
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered(
        AbilityId::PRIMARY,
        "Whenever a creature dies, you gain 1 life.",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::Creature,
            from: Some(ZoneKind::Battlefield),
            to: Some(ZoneKind::Graveyard),
        },
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )];
    let definition_id = CardDefinitionId(10_080);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test death listener",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new(CardKind::Creature, ManaCost::default(), "")
        .creature(1, 1)
        .with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let mut first = creature(10_000, definition_id, PlayerId::One);
    first.damage = 1;
    let mut second = creature(10_001, definition_id, PlayerId::One);
    second.damage = 1;
    game.battlefield = vec![first, second];

    game.check_state_based_actions();

    assert!(game.battlefield.is_empty());
    assert_eq!(game.pending_triggers.len(), 4);
}

#[test]
fn simultaneous_exits_keep_pre_exit_characteristics_for_trigger_matching() {
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered(
        AbilityId::PRIMARY,
        "Whenever a Mountain leaves the battlefield, you gain 1 life.",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::Subtype("Mountain"),
            from: Some(ZoneKind::Battlefield),
            to: Some(ZoneKind::Graveyard),
        },
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )];
    let definition_id = CardDefinitionId(10_081);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test Mountain exit listener",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new(CardKind::Creature, ManaCost::default(), "")
        .creature(1, 1)
        .with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let moon = creature(10_000, cards::BLOOD_MOON, PlayerId::One);
    let moon_id = moon.card.id;
    let taiga = creature(10_001, cards::TAIGA, PlayerId::One);
    let taiga_id = taiga.card.id;
    game.battlefield = vec![creature(10_002, definition_id, PlayerId::One), moon, taiga];

    game.move_permanents_to_graveyard(&[moon_id, taiga_id]);

    assert_eq!(game.pending_triggers.len(), 1);
    assert_eq!(game.pending_triggers[0].context.object, Some(taiga_id));
}
