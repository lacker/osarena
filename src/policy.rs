//! Bot policies and deterministic game-running utilities.

use std::error::Error;
use std::fmt;

use crate::card::{CardBehavior, CardCatalog};
use crate::game::{
    DecisionObservation, DecisionPreference, Game, GameResult, PlayerObservation, Step,
};
use crate::{Action, ActionError, CardDefinitionId, CastChoices, GameObjectId, PlayerId, Target};

/// Chooses one of the actions in a player's current observation.
pub trait Policy {
    fn choose_action(&mut self, observation: &PlayerObservation) -> Option<Action>;
}

/// Selects uniformly from the non-concession legal actions using a seeded PRNG.
#[derive(Clone, Debug)]
pub struct RandomPolicy {
    state: u64,
}

impl RandomPolicy {
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }
}

impl Policy for RandomPolicy {
    fn choose_action(&mut self, observation: &PlayerObservation) -> Option<Action> {
        if let Some(decision) = observation.decision.as_ref() {
            let mut options = decision
                .options
                .iter()
                .map(|option| option.id)
                .collect::<Vec<_>>();
            if options.len() < decision.minimum {
                return None;
            }
            for index in (1..options.len()).rev() {
                let index_u64 = u64::try_from(index + 1).unwrap_or(u64::MAX);
                let offset = usize::try_from(self.next_u64() % index_u64).unwrap_or(0);
                options.swap(index, offset);
            }
            let count = if decision.minimum == decision.maximum {
                decision.minimum
            } else {
                let span = decision.maximum - decision.minimum + 1;
                let offset =
                    usize::try_from(self.next_u64() % u64::try_from(span).unwrap_or(u64::MAX))
                        .unwrap_or(0);
                decision.minimum + offset
            };
            return Some(Action::ChooseDecision {
                decision: decision.id,
                options: options.into_iter().take(count).collect(),
            });
        }
        let choices: Vec<_> = observation
            .legal_actions
            .iter()
            .filter(|action| !matches!(action, Action::Concede))
            .collect();
        if choices.is_empty() {
            return observation.legal_actions.first().cloned();
        }
        let choice_count = u64::try_from(choices.len()).unwrap_or(u64::MAX);
        let unbiased_range = u64::MAX - u64::MAX % choice_count;
        loop {
            let value = self.next_u64();
            if value < unbiased_range {
                let index = usize::try_from(value % choice_count).unwrap_or(0);
                return Some(choices[index].clone());
            }
        }
    }
}

/// A deterministic baseline that applies simple card- and combat-aware rules.
#[derive(Clone, Debug)]
pub struct HandcraftedPolicy {
    catalog: CardCatalog,
    mulligans_taken: u8,
}

impl HandcraftedPolicy {
    #[must_use]
    pub fn new(catalog: CardCatalog) -> Self {
        Self {
            catalog,
            mulligans_taken: 0,
        }
    }

    fn behavior(&self, definition: CardDefinitionId) -> Option<CardBehavior> {
        self.catalog.get(definition).map(|card| card.behavior)
    }

    fn hand_definition(
        observation: &PlayerObservation,
        id: GameObjectId,
    ) -> Option<CardDefinitionId> {
        observation
            .hand
            .iter()
            .find_map(|(candidate, definition)| (*candidate == id).then_some(*definition))
    }

    fn permanent_definition(
        observation: &PlayerObservation,
        id: GameObjectId,
    ) -> Option<CardDefinitionId> {
        observation
            .battlefield
            .iter()
            .find_map(|permanent| (permanent.id == id).then_some(permanent.definition))
    }

    /// Spells whose whole purpose is to remove something the opponent
    /// controls. Pump and recursion also take a permanent target, so they are
    /// deliberately excluded — those want a friendly target.
    fn is_already_a_creature(observation: &PlayerObservation, id: GameObjectId) -> bool {
        observation
            .battlefield
            .iter()
            .any(|permanent| permanent.id == id && permanent.power.is_some())
    }

    /// Whether an ability's target is attacking or blocking right now, which
    /// is the only time a until-end-of-turn pump changes anything.
    fn target_is_fighting(observation: &PlayerObservation, target: Option<Target>) -> bool {
        let Some(Target::Permanent(id)) = target else {
            return false;
        };
        observation.battlefield.iter().any(|permanent| {
            permanent.id == id && (permanent.attacking || permanent.blocking.is_some())
        })
    }

    fn counter_target_score(observation: &PlayerObservation, target: Target) -> i32 {
        match target {
            Target::Spell(id) => observation
                .stack
                .iter()
                .find(|object| object.id == id)
                .map_or(-10_000, |object| {
                    if object.controller == observation.viewer {
                        -10_000
                    } else if observation.stack.iter().any(|counter| {
                        counter.controller == observation.viewer
                            && counter.targets.contains(&Target::Spell(id))
                    }) {
                        // Already answered: a second counter aimed at the same
                        // spell fizzles for nothing.
                        -10_000
                    } else {
                        2_000
                    }
                }),
            Target::Permanent(id) => observation
                .battlefield
                .iter()
                .find(|permanent| permanent.id == id)
                .map_or(-10_000, |permanent| {
                    if permanent.controller == observation.viewer {
                        -10_000
                    } else {
                        1_000
                    }
                }),
            Target::Player(_) => -10_000,
        }
    }

    fn is_hostile_removal(behavior: Option<CardBehavior>) -> bool {
        matches!(
            behavior,
            Some(
                CardBehavior::SwordsToPlowshares
                    | CardBehavior::Disenchant
                    | CardBehavior::DivineOffering
                    | CardBehavior::Shatter
                    | CardBehavior::Terror
                    | CardBehavior::DustToDust
                    | CardBehavior::Detonate
            )
        )
    }

    /// Removal aimed at your own board is never worth its base score, so the
    /// penalty has to cancel that base outright rather than merely rank below
    /// a hostile target that may not exist.
    fn removal_target_score(observation: &PlayerObservation, target: Target) -> i32 {
        match target {
            Target::Permanent(id) => observation
                .battlefield
                .iter()
                .find(|permanent| permanent.id == id)
                .map_or(-10_000, |permanent| {
                    if permanent.controller == observation.viewer {
                        -10_000
                    } else {
                        250 + i32::from(permanent.power.unwrap_or(0).max(0)) * 25
                    }
                }),
            Target::Player(_) | Target::Spell(_) => -10_000,
        }
    }

    fn target_score(observation: &PlayerObservation, target: Target) -> i32 {
        match target {
            Target::Player(player) if player == observation.viewer.opponent() => 500,
            Target::Player(_) => -10_000,
            Target::Permanent(id) => observation
                .battlefield
                .iter()
                .find(|permanent| permanent.id == id)
                .map_or(-500, |permanent| {
                    if permanent.controller == observation.viewer {
                        -500
                    } else {
                        250
                    }
                }),
            Target::Spell(_) => 100,
        }
    }

    fn damage_target_score(observation: &PlayerObservation, target: Target, amount: u16) -> i32 {
        match target {
            Target::Player(player) if player == observation.viewer.opponent() => {
                if observation.life_totals[player.index()]
                    <= i16::try_from(amount).unwrap_or(i16::MAX)
                {
                    10_000
                } else {
                    -2_000
                }
            }
            Target::Player(_) => -10_000,
            Target::Permanent(id) => observation
                .battlefield
                .iter()
                .find(|permanent| permanent.id == id)
                .map_or(-500, |permanent| {
                    if permanent.controller == observation.viewer {
                        return -10_000;
                    }
                    let remaining = permanent
                        .toughness
                        .unwrap_or(0)
                        .saturating_sub(i16::try_from(permanent.damage).unwrap_or(i16::MAX));
                    if i16::try_from(amount).unwrap_or(i16::MAX) >= remaining {
                        700 + i32::from(permanent.power.unwrap_or(0).max(0)) * 25
                    } else {
                        100
                    }
                }),
            Target::Spell(_) => -500,
        }
    }

    fn card_value(&self, definition: CardDefinitionId) -> i32 {
        match self.behavior(definition) {
            Some(CardBehavior::BlackLotus) => 100,
            Some(
                CardBehavior::MoxRuby
                | CardBehavior::MoxEmerald
                | CardBehavior::MoxJet
                | CardBehavior::MoxPearl
                | CardBehavior::MoxSapphire
                | CardBehavior::ManaVault
                | CardBehavior::SolRing,
            ) => 90,
            Some(
                CardBehavior::Island
                | CardBehavior::Forest
                | CardBehavior::Pendelhaven
                | CardBehavior::Mountain
                | CardBehavior::MishrasFactory
                | CardBehavior::Badlands
                | CardBehavior::Bayou
                | CardBehavior::CityOfBrass
                | CardBehavior::Plateau
                | CardBehavior::Plains
                | CardBehavior::Savannah
                | CardBehavior::Scrubland
                | CardBehavior::StripMine
                | CardBehavior::Taiga
                | CardBehavior::TropicalIsland
                | CardBehavior::Tundra
                | CardBehavior::UndergroundSea
                | CardBehavior::VolcanicIsland,
            ) => 80,
            Some(CardBehavior::LightningBolt | CardBehavior::GoblinGrenade) => 75,
            Some(behavior) if behavior.kind().is_creature() => 65,
            Some(_) => 55,
            None => 0,
        }
    }

    fn score_cast(
        &self,
        observation: &PlayerObservation,
        card: GameObjectId,
        choices: &CastChoices,
    ) -> i32 {
        let behavior = Self::hand_definition(observation, card).and_then(|id| self.behavior(id));
        let x = choices.x();
        let damage = match behavior {
            Some(CardBehavior::LightningBolt | CardBehavior::ChainLightning) => Some(3),
            Some(CardBehavior::GoblinGrenade) => Some(5),
            Some(CardBehavior::Fireball) => Some(
                x.checked_div(u16::try_from(choices.iter_targets().count()).unwrap_or(u16::MAX))
                    .unwrap_or(0),
            ),
            _ => None,
        };
        let target_score: i32 = choices
            .iter_targets()
            .map(|target| {
                if matches!(
                    behavior,
                    Some(CardBehavior::AncestralRecall | CardBehavior::Braingeyser)
                ) {
                    let cards_drawn = if behavior == Some(CardBehavior::AncestralRecall) {
                        3
                    } else {
                        x
                    };
                    match target {
                        Target::Player(player) if *player == observation.viewer => {
                            if usize::from(cards_drawn) > observation.library_sizes[player.index()]
                            {
                                -20_000
                            } else {
                                1_000 + i32::from(cards_drawn) * 100
                            }
                        }
                        Target::Player(player) => {
                            if usize::from(cards_drawn) > observation.library_sizes[player.index()]
                            {
                                20_000
                            } else {
                                -10_000
                            }
                        }
                        Target::Permanent(_) | Target::Spell(_) => -10_000,
                    }
                } else if matches!(
                    behavior,
                    Some(CardBehavior::Counterspell | CardBehavior::RedElementalBlast)
                ) {
                    Self::counter_target_score(observation, *target)
                } else if Self::is_hostile_removal(behavior) {
                    Self::removal_target_score(observation, *target)
                } else {
                    damage.map_or_else(
                        || Self::target_score(observation, *target),
                        |amount| Self::damage_target_score(observation, *target, amount),
                    )
                }
            })
            .sum();
        let base = match behavior {
            Some(CardBehavior::AncestralRecall) => 9_200,
            Some(CardBehavior::Counterspell | CardBehavior::RedElementalBlast) => 8_900,
            Some(CardBehavior::SwordsToPlowshares | CardBehavior::Disenchant) => 8_400,
            Some(CardBehavior::TimeWalk) => 8_300,
            Some(CardBehavior::Braingeyser) => 7_500 + i32::from(x) * 30,
            Some(CardBehavior::GoblinGrenade) => 8_500,
            Some(CardBehavior::LightningBolt | CardBehavior::ChainLightning) => 8_000,
            Some(CardBehavior::Fireball) => 7_900 + i32::from(x) * 20,
            Some(CardBehavior::Shatter | CardBehavior::Detonate | CardBehavior::ChaosOrb) => 7_400,
            Some(CardBehavior::Fork) => 7_300,
            Some(CardBehavior::WheelOfFortune) => 6_600,
            Some(behavior) if behavior.kind().is_permanent() => 6_800,
            Some(_) => 6_200,
            None => -10_000,
        };
        base + target_score
    }

    fn score_ability(
        &self,
        observation: &PlayerObservation,
        source: GameObjectId,
        target: Option<Target>,
        sacrifice: Option<GameObjectId>,
    ) -> i32 {
        let behavior =
            Self::permanent_definition(observation, source).and_then(|id| self.behavior(id));
        let target_score = target.map_or(0, |value| {
            if behavior == Some(CardBehavior::OrcishMechanics) {
                Self::damage_target_score(observation, value, 2)
            } else if behavior == Some(CardBehavior::Triskelion) {
                Self::damage_target_score(observation, value, 1)
            } else {
                Self::target_score(observation, value)
            }
        });
        let sacrifice_cost = sacrifice
            .filter(|card| *card != source)
            .and_then(|card| Self::permanent_definition(observation, card))
            .map_or(0, |definition| self.card_value(definition));
        let score = match behavior {
            Some(
                CardBehavior::ChaosOrb | CardBehavior::StripMine | CardBehavior::OrcishMechanics,
            ) => 7_200 + target_score,
            // Animating a Factory that is already a creature does nothing but
            // spend mana, so only the +1/+1 mode stays repeatable.
            Some(CardBehavior::MishrasFactory)
                if target.is_none() && Self::is_already_a_creature(observation, source) =>
            {
                -100
            }
            // The +1/+1 costs the Factory its tap, so it only pays for itself
            // once the creature it feeds is in combat. Pumping earlier — most
            // often itself — spends the attack it was about to make.
            Some(CardBehavior::MishrasFactory)
                if target.is_some() && !Self::target_is_fighting(observation, target) =>
            {
                -100
            }
            Some(CardBehavior::MishrasFactory) => 5_800 + target_score,
            Some(
                CardBehavior::GoblinBalloonBrigade
                | CardBehavior::GraniteGargoyle
                | CardBehavior::DragonWhelp,
            ) => 5_200,
            Some(CardBehavior::Atog) if self.atog_can_attack_for_lethal(observation, source) => {
                10_000
            }
            Some(CardBehavior::Atog) => -100,
            Some(_) => 4_500 + target_score,
            None => -10_000,
        };
        if behavior == Some(CardBehavior::OrcishMechanics)
            && matches!(target, Some(Target::Player(player)) if player == observation.viewer.opponent())
            && observation.life_totals[observation.viewer.opponent().index()] > 2
        {
            return -1_000;
        }
        score - sacrifice_cost
    }

    fn atog_can_attack_for_lethal(
        &self,
        observation: &PlayerObservation,
        source: GameObjectId,
    ) -> bool {
        let Some(atog) = observation
            .battlefield
            .iter()
            .find(|permanent| permanent.id == source)
        else {
            return false;
        };
        if !atog.attacking
            || observation
                .battlefield
                .iter()
                .any(|permanent| permanent.blocking == Some(source))
        {
            return false;
        }
        let artifacts = observation
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == observation.viewer
                    && self
                        .behavior(permanent.definition)
                        .is_some_and(|behavior| behavior.kind().is_artifact())
            })
            .count();
        let potential_power = atog
            .power
            .unwrap_or(0)
            .saturating_add(i16::try_from(artifacts.saturating_mul(2)).unwrap_or(i16::MAX));
        potential_power >= observation.life_totals[observation.viewer.opponent().index()]
    }

    fn score_attack(&self, observation: &PlayerObservation, attacker: GameObjectId) -> i32 {
        let Some(attacker) = observation
            .battlefield
            .iter()
            .find(|permanent| permanent.id == attacker)
        else {
            return -1_000;
        };
        let attacker_power = attacker.power.unwrap_or(0).max(0);
        let attacker_toughness = attacker
            .toughness
            .unwrap_or(0)
            .saturating_sub(i16::try_from(attacker.damage).unwrap_or(i16::MAX));
        let already_attacking = observation
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == observation.viewer && permanent.attacking)
            .count();
        let blockers: Vec<_> = observation
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == observation.viewer.opponent()
                    && !permanent.tapped
                    && permanent.power.is_some()
                    && (!attacker.flying || permanent.flying)
                    && !(self.behavior(permanent.definition) == Some(CardBehavior::IronclawOrcs)
                        && attacker_power >= 2)
            })
            .collect();
        if already_attacking >= blockers.len() {
            return 7_000;
        }
        let gets_eaten = blockers.iter().any(|blocker| {
            let blocker_power = blocker.power.unwrap_or(0).max(0);
            let blocker_toughness = blocker
                .toughness
                .unwrap_or(0)
                .saturating_sub(i16::try_from(blocker.damage).unwrap_or(i16::MAX));
            blocker_power >= attacker_toughness && blocker_toughness > attacker_power
        });
        if gets_eaten { 500 } else { 7_000 }
    }

    fn score_block(
        observation: &PlayerObservation,
        blocker: GameObjectId,
        attacker: GameObjectId,
    ) -> i32 {
        let blocker = observation
            .battlefield
            .iter()
            .find(|permanent| permanent.id == blocker);
        let attacker = observation
            .battlefield
            .iter()
            .find(|permanent| permanent.id == attacker);
        let (Some(blocker), Some(attacker)) = (blocker, attacker) else {
            return 0;
        };
        let blocker_power = blocker.power.unwrap_or(0);
        let blocker_toughness =
            blocker.toughness.unwrap_or(0) - i16::try_from(blocker.damage).unwrap_or(i16::MAX);
        let attacker_power = attacker.power.unwrap_or(0);
        let attacker_toughness =
            attacker.toughness.unwrap_or(0) - i16::try_from(attacker.damage).unwrap_or(i16::MAX);
        let existing_power: i16 = observation
            .battlefield
            .iter()
            .filter(|permanent| permanent.blocking == Some(attacker.id))
            .filter_map(|permanent| permanent.power)
            .fold(0, i16::saturating_add);
        if existing_power >= attacker_toughness {
            return 0;
        }
        let kills = existing_power.saturating_add(blocker_power) >= attacker_toughness;
        let survives = blocker_toughness > attacker_power;
        match (kills, survives) {
            (true, true) => 7_000,
            (true, false) => 6_000,
            (false, true) => 4_000,
            (false, false) if attacker_power >= 4 => 2_000,
            (false, false) => 500,
        }
    }

    fn score_assignment(
        observation: &PlayerObservation,
        assignments: &[crate::CombatDamageAssignment],
    ) -> i32 {
        assignments
            .iter()
            .map(|assignment| match assignment.recipient {
                Target::Player(player) if player == observation.viewer.opponent() => {
                    i32::from(assignment.amount) * 200
                }
                Target::Permanent(id) => observation
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.id == id)
                    .map_or(0, |permanent| {
                        let remaining = permanent
                            .toughness
                            .unwrap_or(0)
                            .saturating_sub(i16::try_from(permanent.damage).unwrap_or(i16::MAX));
                        if i16::try_from(assignment.amount).unwrap_or(i16::MAX) >= remaining {
                            500
                        } else {
                            i32::from(assignment.amount)
                        }
                    }),
                Target::Player(_) | Target::Spell(_) => 0,
            })
            .sum()
    }

    fn should_mulligan(&self, observation: &PlayerObservation) -> bool {
        if self.mulligans_taken >= 2 {
            return false;
        }
        let mana_sources = observation
            .hand
            .iter()
            .filter(|(_, definition)| {
                matches!(
                    self.behavior(*definition),
                    Some(
                        CardBehavior::Mountain
                            | CardBehavior::MishrasFactory
                            | CardBehavior::MoxEmerald
                            | CardBehavior::MoxJet
                            | CardBehavior::MoxPearl
                            | CardBehavior::MoxRuby
                            | CardBehavior::MoxSapphire
                            | CardBehavior::ManaVault
                            | CardBehavior::BlackLotus
                            | CardBehavior::Island
                            | CardBehavior::Forest
                            | CardBehavior::Pendelhaven
                            | CardBehavior::Badlands
                            | CardBehavior::Bayou
                            | CardBehavior::CityOfBrass
                            | CardBehavior::Plateau
                            | CardBehavior::Plains
                            | CardBehavior::Savannah
                            | CardBehavior::Scrubland
                            | CardBehavior::Taiga
                            | CardBehavior::Tundra
                            | CardBehavior::TropicalIsland
                            | CardBehavior::UndergroundSea
                            | CardBehavior::VolcanicIsland
                            | CardBehavior::FellwarStone
                            | CardBehavior::LlanowarElves
                    )
                )
            })
            .count();
        !(2..=5).contains(&mana_sources)
    }

    fn mana_action_score(&self, observation: &PlayerObservation, source: GameObjectId) -> i32 {
        let needs_factory_mana = observation.active_player == observation.viewer
            && matches!(
                observation.step,
                Step::BeginningOfCombat | Step::DeclareAttackers
            )
            && observation.mana_pools[observation.viewer.index()].total() == 0
            && observation.battlefield.iter().any(|permanent| {
                permanent.controller == observation.viewer
                    && !permanent.tapped
                    && permanent.power.is_none()
                    && self.behavior(permanent.definition) == Some(CardBehavior::MishrasFactory)
            });
        if needs_factory_mana
            && Self::permanent_definition(observation, source)
                .and_then(|definition| self.behavior(definition))
                != Some(CardBehavior::MishrasFactory)
        {
            8_800
        } else {
            -100
        }
    }

    fn score_land(&self, observation: &PlayerObservation, card: GameObjectId) -> i32 {
        let definition = Self::hand_definition(observation, card);
        let behavior = definition.and_then(|id| self.behavior(id));
        // The legend rule bins a duplicate on arrival. Replacing a tapped copy
        // with a fresh one is fine; duplicating an untapped one wastes both
        // the card and the land drop.
        if behavior.is_some_and(CardBehavior::is_legendary)
            && observation.battlefield.iter().any(|permanent| {
                permanent.controller == observation.viewer
                    && Some(permanent.definition) == definition
                    && !permanent.tapped
            })
        {
            return 40;
        }
        match behavior {
            Some(CardBehavior::Tundra | CardBehavior::VolcanicIsland) => 9_400,
            Some(CardBehavior::Island | CardBehavior::Mountain | CardBehavior::Plains) => 9_300,
            Some(CardBehavior::MishrasFactory) => 9_200,
            Some(CardBehavior::StripMine) => 9_100,
            Some(_) | None => 9_000,
        }
    }

    fn score_untap(&self, observation: &PlayerObservation, permanents: &[GameObjectId]) -> i32 {
        8_000
            + permanents
                .iter()
                .filter_map(|id| {
                    observation
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.id == *id)
                })
                .map(|permanent| {
                    let card = self.card_value(permanent.definition);
                    let power = i32::from(permanent.power.unwrap_or(0).max(0));
                    card + power * 10
                })
                .sum::<i32>()
    }

    fn score_action(&self, observation: &PlayerObservation, action: &Action) -> i32 {
        match action {
            Action::KeepHand => 10_000,
            Action::TakeMulligan if self.should_mulligan(observation) => 11_000,
            Action::TakeMulligan => -5_000,
            Action::BottomCards { cards } | Action::DiscardCards { cards } => {
                9_000
                    - cards
                        .iter()
                        .filter_map(|card| Self::hand_definition(observation, *card))
                        .map(|definition| self.card_value(definition))
                        .sum::<i32>()
            }
            Action::ChooseDecision { options, .. } => {
                let selected_value = observation.decision.as_ref().map_or(0, |decision| {
                    decision
                        .options
                        .iter()
                        .filter(|option| options.contains(&option.id))
                        .filter_map(|option| option.card)
                        .map(|(_, definition)| self.card_value(definition))
                        .sum::<i32>()
                });
                match observation
                    .decision
                    .as_ref()
                    .map(|decision| decision.preference)
                {
                    Some(crate::DecisionPreference::HigherCardValue) => 8_000 + selected_value,
                    Some(crate::DecisionPreference::LowerCardValue) => 8_000 - selected_value,
                    Some(crate::DecisionPreference::Neutral) | None => 8_000,
                }
            }
            Action::CancelDecision { .. } => -1_000,
            Action::ChooseUntap { permanents } => self.score_untap(observation, permanents),
            Action::PlayLand { card, .. } => self.score_land(observation, *card),
            Action::ActivateManaAbility { source, .. } => {
                self.mana_action_score(observation, *source)
            }
            Action::PayLifeForMana => 5,
            Action::CastSpell { card, choices, .. } => self.score_cast(observation, *card, choices),
            Action::ActivateAbility {
                source,
                target,
                sacrifice,
            } => self.score_ability(observation, *source, *target, *sacrifice),
            Action::DeclareAttacker { attacker } => self.score_attack(observation, *attacker),
            Action::DeclareBlocker { blocker, attacker } => {
                Self::score_block(observation, *blocker, *attacker)
            }
            Action::FinishDeclaringAttackers | Action::FinishDeclaringBlockers => 1_000,
            Action::AssignCombatDamage { assignments, .. } => {
                6_000 + Self::score_assignment(observation, assignments)
            }
            Action::PassPriority => 0,
            Action::Concede => i32::MIN,
        }
    }

    fn choose_decision(&self, decision: &DecisionObservation) -> Option<Action> {
        if decision.options.len() < decision.minimum {
            return None;
        }
        let mut options = decision.options.iter().collect::<Vec<_>>();
        options.sort_by_key(|option| {
            let value = option
                .card
                .map_or(0, |(_, definition)| self.card_value(definition));
            match decision.preference {
                DecisionPreference::HigherCardValue => -value,
                DecisionPreference::LowerCardValue => value,
                DecisionPreference::Neutral => 0,
            }
        });
        Some(Action::ChooseDecision {
            decision: decision.id,
            options: options
                .into_iter()
                .take(decision.minimum)
                .map(|option| option.id)
                .collect(),
        })
    }
}

impl Policy for HandcraftedPolicy {
    fn choose_action(&mut self, observation: &PlayerObservation) -> Option<Action> {
        if let Some(decision) = observation.decision.as_ref() {
            return self.choose_decision(decision);
        }
        let action = observation
            .legal_actions
            .iter()
            .max_by_key(|action| self.score_action(observation, action))
            .cloned();
        if matches!(action, Some(Action::TakeMulligan)) {
            self.mulligans_taken += 1;
        } else if matches!(action, Some(Action::KeepHand)) {
            self.mulligans_taken = 0;
        }
        action
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlayError {
    PolicyReturnedNoAction(PlayerId),
    IllegalAction(Box<ActionError>),
    ActionLimitExceeded(usize),
}

impl fmt::Display for PlayError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PolicyReturnedNoAction(player) => {
                write!(formatter, "policy for {player} returned no action")
            }
            Self::IllegalAction(error) => {
                write!(formatter, "policy returned an illegal action: {error}")
            }
            Self::ActionLimitExceeded(limit) => {
                write!(formatter, "game exceeded its action limit of {limit}")
            }
        }
    }
}

impl Error for PlayError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IllegalAction(error) => Some(error.as_ref()),
            Self::PolicyReturnedNoAction(_) | Self::ActionLimitExceeded(_) => None,
        }
    }
}

/// Plays a game to completion using one policy for each player.
///
/// # Errors
///
/// Returns [`PlayError`] if a policy fails to choose an action, chooses an
/// illegal action, or the game exceeds `action_limit`.
pub fn play_game(
    game: &mut Game,
    player_one: &mut dyn Policy,
    player_two: &mut dyn Policy,
    action_limit: usize,
) -> Result<GameResult, PlayError> {
    for _ in 0..action_limit {
        if let Some(result) = game.result() {
            return Ok(result);
        }
        let Some(player) = game.decision_player() else {
            return game
                .result()
                .ok_or(PlayError::ActionLimitExceeded(action_limit));
        };
        let observation = game.observe(player);
        let action = match player {
            PlayerId::One => player_one.choose_action(&observation),
            PlayerId::Two => player_two.choose_action(&observation),
        }
        .ok_or(PlayError::PolicyReturnedNoAction(player))?;
        game.apply(player, action)
            .map_err(|error| PlayError::IllegalAction(Box::new(error)))?;
    }
    game.result()
        .ok_or(PlayError::ActionLimitExceeded(action_limit))
}
