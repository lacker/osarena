use penta::card;
use penta::game::{DecisionKind, DecisionOrderSemantics};
use penta::{
    AbilityOrigin, Action, ActivatedAbilityText, BattlefieldExit, CardCatalog, CardDefinitionId,
    CardInstanceId, Format, Game, GameEvent, GameResult, HandcraftedPolicy, ModeId, PlayOptionId,
    PlayerId, PlayerObservation, Policy, RandomPolicy, Step, Target,
};
use serde_json::{Value, json};
use std::fmt::Write as _;
use wasm_bindgen::prelude::*;

const BOT_ACTION_LIMIT: usize = 50_000;

enum BotPolicy {
    Random(RandomPolicy),
    Handcrafted(HandcraftedPolicy),
}

impl BotPolicy {
    fn choose_action(&mut self, observation: &PlayerObservation) -> Option<Action> {
        match self {
            Self::Random(policy) => policy.choose_action(observation),
            Self::Handcrafted(policy) => policy.choose_action(observation),
        }
    }
}

/// Browser-owned game facade. JavaScript only selects legal action indexes;
/// rules and bot decisions remain inside the Rust engine.
#[wasm_bindgen]
pub struct WebGame {
    game: Game,
    catalog: CardCatalog,
    human: PlayerId,
    bot: BotPolicy,
    opponent_actions: Vec<Value>,
    pending_opponent_mana: Vec<String>,
    mana_undo_history: Vec<Game>,
    phase_stops: Vec<String>,
    autopass_enabled: bool,
    attack_undo: Option<Game>,
    /// The turn the presentation has already announced, so a turn nobody acts
    /// on still gets its banner instead of being skipped over in silence.
    announced_turn: Option<u32>,
    /// The board the moment your own action landed, before the game answered.
    human_action_state: Option<Value>,
}

#[wasm_bindgen]
impl WebGame {
    /// Creates a mirror-format game and advances until the human must decide.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error when a deck or policy name is unknown, game
    /// construction fails, or the bot cannot reach a human decision.
    #[allow(clippy::needless_pass_by_value)] // wasm-bindgen owns optional strings at the ABI.
    #[wasm_bindgen(constructor)]
    pub fn new(
        human_deck: &str,
        bot_deck: &str,
        bot_policy: &str,
        human_first: bool,
        seed: u32,
        format: Option<String>,
    ) -> Result<WebGame, JsValue> {
        let format = penta::protocol::parse_format_slug(
            format.as_deref().unwrap_or(Format::OldSchool9394.slug()),
        )
        .map_err(js_error)?;
        let catalog = card::catalog().map_err(js_error)?;
        let human_deck = deck_by_name(format, human_deck)?;
        let bot_deck = deck_by_name(format, bot_deck)?;
        let human = if human_first {
            PlayerId::One
        } else {
            PlayerId::Two
        };
        let decks = match human {
            PlayerId::One => [human_deck, bot_deck],
            PlayerId::Two => [bot_deck, human_deck],
        };
        let game = Game::new_with_format(format, catalog.clone(), decks, u64::from(seed))
            .map_err(js_error)?;
        let bot = match bot_policy.to_ascii_lowercase().as_str() {
            "random" => BotPolicy::Random(RandomPolicy::new(u64::from(seed) ^ 0x00b0_7b07)),
            "handcrafted" => BotPolicy::Handcrafted(HandcraftedPolicy::new(catalog.clone())),
            _ => return Err(JsValue::from_str("unknown bot policy")),
        };
        let mut web_game = Self {
            game,
            catalog,
            human,
            bot,
            opponent_actions: Vec::new(),
            pending_opponent_mana: Vec::new(),
            mana_undo_history: Vec::new(),
            phase_stops: Vec::new(),
            autopass_enabled: true,
            attack_undo: None,
            // The opening turn arrives with the board, not as a change to it.
            announced_turn: Some(1),
            human_action_state: None,
        };
        web_game.advance_until_human_choice()?;
        Ok(web_game)
    }

    /// Applies one action from the current state's action list.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error when the game is not waiting for the human,
    /// the index is stale, the action is rejected, or the bot cannot finish.
    pub fn act(&mut self, action_index: usize) -> Result<(), JsValue> {
        if self.game.decision_player() != Some(self.human) {
            return Err(JsValue::from_str("the game is not waiting for the human"));
        }
        let observation = self.game.observe(self.human);
        let action = observation
            .legal_actions
            .get(action_index)
            .cloned()
            .ok_or_else(|| JsValue::from_str("unknown legal action"))?;
        self.apply_human_action(action)
    }

    /// Submits the selected option IDs for the current generic decision.
    ///
    /// The selection is validated by the engine, so the browser does not need
    /// to receive an eagerly-expanded action for every possible combination.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error when the game is not waiting for the human,
    /// the JSON is malformed, or the engine rejects the selection.
    pub fn choose_decision(&mut self, decision: u32, options_json: &str) -> Result<(), JsValue> {
        if self.game.decision_player() != Some(self.human) {
            return Err(JsValue::from_str("the game is not waiting for the human"));
        }
        let options: Vec<u32> = serde_json::from_str(options_json).map_err(js_error)?;
        self.apply_human_action(Action::ChooseDecision { decision, options })
    }

    fn apply_human_action(&mut self, action: Action) -> Result<(), JsValue> {
        let mana_checkpoint =
            matches!(action, Action::ActivateManaAbility { .. }).then(|| self.game.clone());
        if mana_checkpoint.is_none() {
            self.mana_undo_history.clear();
        }
        // The first declaration of the combat is the point a cancel returns to.
        if matches!(action, Action::DeclareAttacker { .. }) && self.attack_undo.is_none() {
            self.attack_undo = Some(self.game.clone());
        }
        self.opponent_actions.clear();
        self.pending_opponent_mana.clear();
        let event_start = self.game.events().len();
        self.game.apply(self.human, action).map_err(js_error)?;
        // What you just did, before anything the game does in response. The
        // replay is told from here, so a land you played is on the board
        // before the turn it ended is announced.
        self.human_action_state = Some(self.snapshot_value(false));
        // Yielding is how combat damage happens, and ending your own turn hands
        // one to the opponent. Both need showing every bit as much as the
        // actions the bot takes on its own.
        self.record_combat_damage(event_start);
        self.record_draw_step(event_start);
        self.record_turn_change(event_start);
        // Committing the attack invalidates the checkpoint immediately, so no
        // snapshot taken while the turn plays out still offers the cancel.
        self.forget_attack_undo_unless_still_declaring();
        self.advance_until_human_choice()?;
        self.forget_attack_undo_unless_still_declaring();
        if let Some(checkpoint) = mana_checkpoint {
            let before = checkpoint.observe(self.human);
            let after = self.game.observe(self.human);
            if self.game.decision_player() == Some(self.human)
                && before.turn == after.turn
                && before.step == after.step
                && before.active_player == after.active_player
                && before.stack == after.stack
            {
                self.mana_undo_history.push(checkpoint);
            } else {
                self.mana_undo_history.clear();
            }
        }
        Ok(())
    }

    /// Declares every currently legal attacker, then finishes attacker declaration.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error unless the human is declaring attackers or
    /// the engine rejects one of the otherwise-legal actions.
    pub fn attack_all(&mut self) -> Result<(), JsValue> {
        if self.game.decision_player() != Some(self.human)
            || self.game.observe(self.human).step != Step::DeclareAttackers
        {
            return Err(JsValue::from_str("the human is not declaring attackers"));
        }
        self.mana_undo_history.clear();
        self.opponent_actions.clear();
        self.pending_opponent_mana.clear();
        if self.attack_undo.is_none() {
            self.attack_undo = Some(self.game.clone());
        }
        loop {
            let action = self
                .game
                .observe(self.human)
                .legal_actions
                .into_iter()
                .find(|action| matches!(action, Action::DeclareAttacker { .. }));
            let Some(action) = action else {
                break;
            };
            self.game.apply(self.human, action).map_err(js_error)?;
        }
        if let Some(finish) = self
            .game
            .observe(self.human)
            .legal_actions
            .into_iter()
            .find(|action| matches!(action, Action::FinishDeclaringAttackers))
        {
            self.game.apply(self.human, finish).map_err(js_error)?;
        }
        self.forget_attack_undo_unless_still_declaring();
        self.advance_until_human_choice()?;
        self.forget_attack_undo_unless_still_declaring();
        Ok(())
    }

    /// Takes back every attacker declared so far this combat.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error when the attack has already been committed.
    pub fn cancel_attackers(&mut self) -> Result<(), JsValue> {
        let previous = self
            .attack_undo
            .take()
            .ok_or_else(|| JsValue::from_str("there are no declared attackers to take back"))?;
        self.game = previous;
        self.mana_undo_history.clear();
        self.opponent_actions.clear();
        self.pending_opponent_mana.clear();
        Ok(())
    }

    /// A cancel is only offered while the attack is still being assembled;
    /// once it is committed the declaration is part of the game.
    fn forget_attack_undo_unless_still_declaring(&mut self) {
        if self.attack_undo.is_none() {
            return;
        }
        let still_declaring = self.game.decision_player() == Some(self.human)
            && self
                .game
                .observe(self.human)
                .legal_actions
                .iter()
                .any(|action| matches!(action, Action::FinishDeclaringAttackers));
        if !still_declaring {
            self.attack_undo = None;
        }
    }

    /// Commits a complete set of blocker assignments selected by the browser UI.
    ///
    /// Assignments are encoded as JSON pairs of `[blocker_id, attacker_id]` so
    /// the UI can rearrange arrows freely before making one atomic submission.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error unless the human is declaring blockers or an
    /// assignment is duplicated, malformed, or no longer legal.
    pub fn finalize_blocks(&mut self, assignments_json: &str) -> Result<(), JsValue> {
        if self.game.decision_player() != Some(self.human)
            || self.game.observe(self.human).step != Step::DeclareBlockers
        {
            return Err(JsValue::from_str("the human is not declaring blockers"));
        }
        let assignments: Vec<[u32; 2]> =
            serde_json::from_str(assignments_json).map_err(js_error)?;
        let mut used_blockers = Vec::with_capacity(assignments.len());
        let legal_actions = self.game.observe(self.human).legal_actions;
        let mut block_actions = Vec::with_capacity(assignments.len());
        for [blocker, attacker] in assignments {
            let blocker = CardInstanceId(blocker);
            if used_blockers.contains(&blocker) {
                return Err(JsValue::from_str("a blocker can only block one attacker"));
            }
            used_blockers.push(blocker);
            let action = Action::DeclareBlocker {
                blocker,
                attacker: CardInstanceId(attacker),
            };
            if !legal_actions.contains(&action) {
                return Err(JsValue::from_str("a blocker assignment is no longer legal"));
            }
            block_actions.push(action);
        }
        self.mana_undo_history.clear();
        self.opponent_actions.clear();
        self.pending_opponent_mana.clear();
        for action in block_actions {
            self.game.apply(self.human, action).map_err(js_error)?;
        }
        self.game
            .apply(self.human, Action::FinishDeclaringBlockers)
            .map_err(js_error)?;
        self.advance_until_human_choice()
    }

    /// Rewinds the most recent manual mana ability while it is still safe to do so.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error when there is no reversible mana activation.
    pub fn undo_mana(&mut self) -> Result<(), JsValue> {
        let previous = self
            .mana_undo_history
            .pop()
            .ok_or_else(|| JsValue::from_str("there is no mana ability to undo"))?;
        self.game = previous;
        self.opponent_actions.clear();
        self.pending_opponent_mana.clear();
        Ok(())
    }

    /// Enables or disables a human-interface stop for one displayed phase.
    /// The rules engine still exposes every individual step.
    /// Sets or clears a UI phase stop.
    ///
    /// # Errors
    ///
    /// Returns an error if advancing the facade encounters an invalid engine action.
    pub fn set_phase_stop(&mut self, phase: &str, enabled: bool) -> Result<(), JsValue> {
        if !matches!(
            phase,
            "Beginning" | "Main 1" | "Combat" | "Main 2" | "Ending"
        ) {
            return Err(JsValue::from_str("unknown displayed phase"));
        }
        self.phase_stops.retain(|candidate| candidate != phase);
        if enabled {
            self.phase_stops.push(phase.into());
        }
        self.opponent_actions.clear();
        self.pending_opponent_mana.clear();
        Ok(())
    }

    /// Enables or disables the browser's smooth automatic priority yields.
    /// Enables or disables routine UI priority passing.
    ///
    /// # Errors
    ///
    /// Returns an error if advancing the facade encounters an invalid engine action.
    pub fn set_autopass(&mut self, enabled: bool) -> Result<(), JsValue> {
        self.autopass_enabled = enabled;
        self.opponent_actions.clear();
        self.pending_opponent_mana.clear();
        if enabled {
            self.advance_until_human_choice()?;
        }
        Ok(())
    }

    /// Returns the human-visible game state as JSON.
    #[must_use]
    pub fn state_json(&self) -> String {
        self.snapshot().to_string()
    }

    fn advance_until_human_choice(&mut self) -> Result<(), JsValue> {
        for _ in 0..BOT_ACTION_LIMIT {
            let Some(player) = self.game.decision_player() else {
                return Ok(());
            };
            let observation = self.game.observe(player);
            let action = if player == self.human {
                let automatic_action = self.automatic_human_action_for(&observation);
                let Some(action) = automatic_action else {
                    return Ok(());
                };
                action
            } else {
                self.bot
                    .choose_action(&observation)
                    .ok_or_else(|| JsValue::from_str("bot returned no action"))?
            };
            let mut pending_animation = None;
            if player != self.human {
                if let Action::ActivateManaAbility { source, .. } = &action {
                    self.pending_opponent_mana
                        .push(self.instance_name(&observation, *source));
                } else if should_animate_action(&action) {
                    let mana_sources = if matches!(
                        action,
                        Action::CastSpell { .. } | Action::ActivateAbility { .. }
                    ) {
                        std::mem::take(&mut self.pending_opponent_mana)
                    } else {
                        Vec::new()
                    };
                    let label = self.opponent_action_label(&observation, &action);
                    let kind = animated_action_kind(&action);
                    let card_id = action_card(&action);
                    let card = card_id.map(|id| self.instance_name(&observation, id));
                    pending_animation = Some(json!({
                        "label": label,
                        "kind": kind,
                        "card": card,
                        "cardId": card_id.map(|id| id.0),
                        "manaSources": mana_sources,
                    }));
                } else {
                    self.pending_opponent_mana.clear();
                }
            }
            // Who owns each object on the stack, read before it leaves: a
            // resolution event names the card but the object is gone by then.
            let stack_owners: Vec<(CardInstanceId, PlayerId)> = observation
                .stack
                .iter()
                .map(|object| (object.id, object.controller))
                .collect();
            let event_start = self.game.events().len();
            self.game.apply(player, action).map_err(js_error)?;
            if pending_animation.is_none() {
                self.record_resolutions(event_start, &stack_owners);
            }
            self.record_combat_damage(event_start);
            self.record_draw_step(event_start);
            if let Some(mut animation) = pending_animation.take() {
                let mana_sources = self.game.events()[event_start..]
                    .iter()
                    .filter_map(|event| match event {
                        GameEvent::ManaAdded {
                            player: producer,
                            source,
                        } if *producer == player => Some(*source),
                        _ => None,
                    })
                    .map(|source| json!(self.instance_name(&observation, source)))
                    .collect::<Vec<_>>();
                if let Some(existing) = animation["manaSources"].as_array_mut() {
                    existing.extend(mana_sources);
                }
                animation["state"] = self.snapshot_value(false);
                self.opponent_actions.push(animation);
            }
            // Last, so the beat that ended the turn is still watched before the
            // next turn is announced.
            self.record_turn_change(event_start);
            // Your click is not finished until your own spell has left the
            // stack: the yields that resolve it are automatic and produce no
            // beat, so they belong to what you did rather than to the replay.
            // The moment anything worth watching happens, this stops moving
            // and the replay starts from there.
            if player == self.human && self.opponent_actions.is_empty() {
                self.human_action_state = Some(self.snapshot_value(false));
            }
        }
        Err(JsValue::from_str(
            "game exceeded its automatic action limit",
        ))
    }

    /// Gives anything that resolved off the stack its own beat.
    ///
    /// A pass that completes a round resolves the top of the stack, an event
    /// no one clicked. Without a beat the object would blink out between
    /// frames — and a turn banner could show while a spell everyone watched
    /// resolve still sat on the stack.
    ///
    /// Your own spell is the exception: its resolution is the rest of the
    /// click you just made, and replaying it locks the board for a beat you
    /// did not need to watch. A fizzle is always shown, whoever cast it, since
    /// it is the only explanation for a spell that did nothing.
    fn record_resolutions(
        &mut self,
        event_start: usize,
        stack_owners: &[(CardInstanceId, PlayerId)],
    ) {
        let resolved: Vec<_> = self.game.events()[event_start..]
            .iter()
            .filter_map(|event| match event {
                GameEvent::SpellResolved { card, definition } => Some((*card, *definition, false)),
                GameEvent::AbilityResolved {
                    object, definition, ..
                }
                | GameEvent::TriggeredAbilityResolved {
                    object, definition, ..
                } => Some((*object, *definition, false)),
                GameEvent::SpellFizzled { card, definition } => Some((*card, *definition, true)),
                _ => None,
            })
            .collect();
        for (card, definition, fizzled) in resolved {
            let yours = stack_owners
                .iter()
                .any(|(object, controller)| *object == card && *controller == self.human);
            if yours && !fizzled {
                continue;
            }
            let name = self.card_name(definition);
            self.opponent_actions.push(json!({
                "label": if fizzled {
                    format!("{name} fizzles")
                } else {
                    format!("{name} resolves")
                },
                "kind": "spell",
                "card": name,
                "cardId": card.0,
                "manaSources": Vec::<String>::new(),
                "state": self.snapshot_value(false),
            }));
        }
    }

    /// Gives the turn's draw its own beat.
    ///
    /// The draw step is over in the same yield that entered it, so without a
    /// beat the card arrives in a frame the board already labels "first main".
    /// Holding it here draws the card where the phase strip says it happens.
    fn record_draw_step(&mut self, event_start: usize) {
        let events = &self.game.events()[event_start..];
        let drew = events
            .iter()
            .any(|event| matches!(event, GameEvent::CardDrawn { .. }));
        let in_draw_step = events.iter().any(|event| {
            matches!(
                event,
                GameEvent::StepChanged {
                    step: Step::Draw,
                    ..
                }
            )
        });
        if !drew || !in_draw_step {
            return;
        }
        self.opponent_actions.push(json!({
            "label": "Draw",
            "kind": "draw",
            "card": Value::Null,
            "cardId": Value::Null,
            "manaSources": Vec::<String>::new(),
            "state": self.snapshot_value(false),
        }));
    }

    /// Gives combat damage its own beat.
    ///
    /// Nobody clicks damage into happening, and yielding through the step is
    /// now the normal way an unblocked attack ends. Without a beat the life
    /// totals and the dead creatures would change between frames.
    fn record_combat_damage(&mut self, event_start: usize) {
        let events = &self.game.events()[event_start..];
        let entered_damage = events.iter().any(|event| {
            matches!(
                event,
                GameEvent::StepChanged {
                    step: Step::CombatDamage,
                    ..
                }
            )
        });
        if !entered_damage {
            return;
        }
        let landed = events.iter().any(|event| {
            matches!(
                event,
                GameEvent::DamageDealt { .. } | GameEvent::PermanentLeftBattlefield { .. }
            )
        });
        if !landed {
            return;
        }
        self.opponent_actions.push(json!({
            "label": "Combat damage",
            "kind": "combat",
            "card": Value::Null,
            "cardId": Value::Null,
            "manaSources": Vec::<String>::new(),
            "state": self.snapshot_value(false),
        }));
    }

    /// Gives a turn that just began its own presentation beat.
    ///
    /// Turn banners are otherwise inferred from the beats around them, so an
    /// opponent who draws and passes would slide by without ever being
    /// announced. This beat carries no action of its own — the client shows
    /// the banner and moves on.
    fn record_turn_change(&mut self, event_start: usize) {
        let Some(turn) = self.game.events()[event_start..]
            .iter()
            .filter_map(|event| match event {
                GameEvent::StepChanged { turn, .. } => Some(*turn),
                _ => None,
            })
            .next_back()
        else {
            return;
        };
        if self.announced_turn == Some(turn) {
            return;
        }
        self.announced_turn = Some(turn);
        self.opponent_actions.push(json!({
            "label": "New turn",
            "kind": "turn",
            "card": Value::Null,
            "cardId": Value::Null,
            "manaSources": Vec::<String>::new(),
            "state": self.snapshot_value(false),
        }));
    }

    fn automatic_human_action_for(&self, observation: &PlayerObservation) -> Option<Action> {
        if self.autopass_enabled
            && let Some(decision) = observation.decision.as_ref()
            && decision.kind == DecisionKind::Choice
            && decision.minimum == 1
            && decision.maximum == 1
            && decision.options.len() == 1
            && !decision.prompt.starts_with("Erhnam Djinn")
        {
            return Some(Action::ChooseDecision {
                decision: decision.id,
                options: vec![decision.options[0].id],
            });
        }
        automatic_human_action_for_context(
            AutoPassContext {
                step: observation.step,
                human_is_active: observation.active_player == self.human,
                stack_is_empty: observation.stack.is_empty(),
                has_attacker: observation
                    .battlefield
                    .iter()
                    .any(|permanent| permanent.attacking),
                has_blocker: observation
                    .battlefield
                    .iter()
                    .any(|permanent| permanent.blocking.is_some()),
                stop_here: self.should_stop(observation.step),
                autopass_enabled: self.autopass_enabled,
                only_human_objects_on_stack: !observation.stack.is_empty()
                    && observation
                        .stack
                        .iter()
                        .all(|object| object.controller == self.human),
                human_has_floating_mana: observation.mana_pools[self.human.index()].total() > 0,
                human_controls_creature: observation.battlefield.iter().any(|permanent| {
                    permanent.controller == self.human && permanent.power.is_some()
                }),
            },
            &observation.legal_actions,
        )
    }

    /// Predicts where a pass would land by replaying the real auto-pass
    /// policy on a cloned game, assuming the opponent declines to act. The
    /// result is the label for the UI's pass button, so the button always
    /// names the destination the engine will actually reach.
    fn pass_preview_label(&self) -> Option<String> {
        if self.game.decision_player() != Some(self.human) {
            return None;
        }
        let observation = self.game.observe(self.human);
        if !observation
            .legal_actions
            .iter()
            .any(|action| matches!(action, Action::PassPriority))
        {
            return None;
        }
        if let Some(top) = observation.stack.last() {
            return Some(format!("Resolve {}", self.card_name(top.definition)));
        }
        let start_turn = observation.turn;
        let start_active_is_human = observation.active_player == self.human;
        let mut sim = self.game.clone();
        sim.apply(self.human, Action::PassPriority).ok()?;
        // Combat damage is the loudest thing a pass can cause, so it names the
        // button even when the yield carries on past it. Watching for attackers
        // standing in a pre-damage step and then for the step advancing past it
        // catches the combats where every attacker dies on the way through.
        let mut attack_pending = Self::attack_awaiting_damage(&observation);
        let mut deals_combat_damage = false;
        // Every exit below labels the step the simulation reached, so a pass
        // that runs the game out (lethal damage) or that the preview cannot
        // carry further still names a destination instead of falling back to
        // the bare "Pass priority".
        for _ in 0..BOT_ACTION_LIMIT {
            let Some(player) = sim.decision_player() else {
                break;
            };
            let sim_observation = sim.observe(player);
            // Only this turn's combat is the pass's doing; an attack a whole
            // turn away is not what the button is about to cause.
            attack_pending |= sim_observation.turn == start_turn
                && Self::attack_awaiting_damage(&sim_observation);
            if attack_pending
                && (sim_observation.turn != start_turn
                    || matches!(
                        sim_observation.step,
                        Step::CombatDamage
                            | Step::EndOfCombat
                            | Step::PostcombatMain
                            | Step::End
                            | Step::Cleanup
                    ))
            {
                deals_combat_damage = true;
            }
            let action = if player == self.human {
                match self.automatic_human_action_for(&sim_observation) {
                    Some(action) => action,
                    None => break,
                }
            } else if let Some(action) = neutral_opponent_action(&sim_observation) {
                action
            } else {
                // The opponent holds a real choice here, so this is where the
                // human ends up waiting.
                break;
            };
            if sim.apply(player, action).is_err() {
                break;
            }
        }
        let ending = sim.observe(self.human);
        // Taking an attack unblocked is a commitment, not a destination, so on
        // defense the button names the decision the same way "No attacks" does.
        if !start_active_is_human && Self::declines_all_blocks(&observation, &ending, start_turn) {
            return Some("No blocks".into());
        }
        if deals_combat_damage {
            return Some("Go to damage".into());
        }
        Some(Self::pass_destination_label(
            &ending,
            start_turn,
            start_active_is_human,
        ))
    }

    /// Whether this pass carries the defender through the block step without
    /// blocking anything: attackers are in, nothing of yours is blocking, and
    /// the simulation runs out the other side of blockers still that way.
    fn declines_all_blocks(
        before: &PlayerObservation,
        after: &PlayerObservation,
        start_turn: u32,
    ) -> bool {
        let blocking = |observation: &PlayerObservation| {
            observation
                .battlefield
                .iter()
                .any(|permanent| permanent.blocking.is_some())
        };
        matches!(
            before.step,
            Step::BeginningOfCombat | Step::DeclareAttackers | Step::DeclareBlockers
        ) && before
            .battlefield
            .iter()
            .any(|permanent| permanent.attacking)
            && !blocking(before)
            && !blocking(after)
            && (after.turn != start_turn
                || matches!(
                    after.step,
                    Step::CombatDamage | Step::EndOfCombat | Step::PostcombatMain | Step::End
                ))
    }

    /// Attackers are declared and the damage step has not happened yet.
    fn attack_awaiting_damage(observation: &PlayerObservation) -> bool {
        matches!(
            observation.step,
            Step::DeclareAttackers | Step::DeclareBlockers
        ) && observation
            .battlefield
            .iter()
            .any(|permanent| permanent.attacking)
    }

    fn pass_destination_label(
        observation: &PlayerObservation,
        start_turn: u32,
        start_active_is_human: bool,
    ) -> String {
        // Name what happens next rather than the step the rules call it. The
        // simulation above already settled where the pass lands, so these
        // read as a promise the click has to keep.
        if observation.turn != start_turn {
            return if start_active_is_human {
                "End turn".into()
            } else {
                "Your turn".into()
            };
        }
        // The same step means different things depending on whose turn it is:
        // "Go to attacks" is a promise to attack, not a warning to block.
        if start_active_is_human {
            match observation.step {
                Step::Upkeep => "Go to upkeep",
                Step::Draw => "Draw a card",
                Step::PrecombatMain => "Go to main phase",
                Step::BeginningOfCombat | Step::DeclareAttackers => "Go to attacks",
                Step::DeclareBlockers => "Go to blocks",
                Step::CombatDamage => "Go to damage",
                Step::EndOfCombat => "Go to end of combat",
                Step::PostcombatMain => "Go to second main",
                Step::End => "Go to end step",
                // The only reason to hold priority in cleanup is a full hand.
                Step::Cleanup => "Discard down to seven",
            }
        } else {
            match observation.step {
                Step::Upkeep => "Go to their upkeep",
                Step::Draw => "Go to their draw",
                Step::PrecombatMain => "Go to their main phase",
                Step::BeginningOfCombat | Step::DeclareAttackers => "Go to their attack",
                Step::DeclareBlockers => "Go to blocks",
                Step::CombatDamage => "Go to damage",
                Step::EndOfCombat => "Go to end of combat",
                Step::PostcombatMain => "Go to their second main",
                Step::End => "Go to their end step",
                Step::Cleanup => "Go to cleanup",
            }
        }
        .into()
    }

    #[allow(clippy::too_many_lines)]
    fn snapshot(&self) -> Value {
        self.snapshot_value(true)
    }

    fn should_stop(&self, step: Step) -> bool {
        let phase = match step {
            Step::Upkeep | Step::Draw => "Beginning",
            Step::PrecombatMain => "Main 1",
            Step::BeginningOfCombat
            | Step::DeclareAttackers
            | Step::DeclareBlockers
            | Step::CombatDamage
            | Step::EndOfCombat => "Combat",
            Step::PostcombatMain => "Main 2",
            Step::End | Step::Cleanup => "Ending",
        };
        self.phase_stops.iter().any(|candidate| candidate == phase)
    }

    fn automatic_mana_sources(&self, action: &Action) -> Vec<u32> {
        self.game
            .mana_sources_for_action(self.human, action)
            .into_iter()
            .map(|source| source.0)
            .collect()
    }

    #[allow(clippy::too_many_lines)]
    fn snapshot_value(&self, include_opponent_actions: bool) -> Value {
        let observation = self.game.observe(self.human);
        let opponent = self.human.opponent();
        let actions = observation
            .legal_actions
            .iter()
            .enumerate()
            .map(|(index, action)| {
                json!({
                    "index": index,
                    "label": self.action_label(&observation, action),
                    "kind": action_kind(action),
                    "cardId": action_card(action).map(|id| id.0),
                    "targetCardId": action_target_card(action).map(|id| id.0),
                    "targetPlayer": action_target_player(action, self.human),
                    "targetStackId": action_target_stack(action),
                    "targetCardIds": action_target_cards(action),
                    "targetPlayers": action_target_players(action, self.human),
                    "targetStackIds": action_target_stacks(action),
                    "targetCount": action_targets(action).len(),
                    // What the ability does, with no target picked yet, so the
                    // card's menu can offer the effect by name.
                    "abilitySummary": match action {
                        Action::ActivateAbility { source, ability, targets, .. }
                            if targets
                                .iter()
                                .any(|selection| !selection.targets().is_empty()) =>
                            self.ability_text(&observation, *source, *ability)
                                .map(|text| text.summary),
                        _ => None,
                    },
                    "ability": action_ability_origin(action),
                    "manaAbility": matches!(action, Action::ActivateManaAbility { .. }),
                    "spellAction": matches!(action, Action::CastSpell { .. }),
                    "sacrificeCardIds": action_sacrifices(action),
                    "combatDamageAttacker": match action {
                        Action::AssignCombatDamage { attacker, .. } => Some(attacker.0),
                        _ => None,
                    },
                    "x": match action {
                        Action::CastSpell { choices, .. } => Some(choices.x()),
                        _ => None,
                    },
                    "playOptionId": match action {
                        Action::PlayLand { option, .. } => Some(option.0),
                        Action::CastSpell { choices, .. } => Some(choices.play_option().0),
                        _ => None,
                    },
                    "modeIds": match action {
                        Action::CastSpell { choices, .. } => Some(
                            choices.modes().iter().map(|mode| mode.0).collect::<Vec<_>>(),
                        ),
                        _ => None,
                    },
                    "paymentAction": matches!(action, Action::CastSpell { .. } | Action::ActivateAbility { .. }),
                    "manaSourceIds": self.automatic_mana_sources(action),
                    "decisionId": match action {
                        Action::ChooseDecision { decision, .. }
                        | Action::CancelDecision { decision } => Some(*decision),
                        _ => None,
                    },
                    "decisionOptionIds": match action {
                        Action::ChooseDecision { options, .. } => options.clone(),
                        _ => Vec::new(),
                    },
                })
            })
            .collect::<Vec<_>>();
        let battlefield = observation
            .battlefield
            .iter()
            .map(|permanent| {
                let card = self.catalog.get(permanent.definition);
                let art = card.and_then(|card| card.art.as_ref());
                let part = card.and_then(|card| card.part(permanent.presented));
                let rules = part
                    .map(|part| &part.rules)
                    .or_else(|| card.map(|card| &card.rules));
                let mana_cost = part.map_or_else(
                    || card.map(|card| card.rules.mana_cost),
                    |part| part.mana_cost,
                );
                let current_kind = rules.map_or("unknown".into(), |rules| {
                    if card.is_some_and(|card| {
                        card.rules.special_behavior() == Some(penta::CardBehavior::MishrasFactory)
                    }) && permanent.power.is_some()
                    {
                        "artifactcreature".into()
                    } else {
                        format!("{:?}", rules.kind).to_ascii_lowercase()
                    }
                });
                json!({
                    "id": permanent.id.0,
                    "partId": permanent.presented.0,
                    "name": part.map_or_else(
                        || self.card_name(permanent.definition),
                        |part| part.name.clone(),
                    ),
                    "art": card_art_value(art),
                    "kind": current_kind,
                    "typeLine": rules.map_or_else(String::new, penta::CardRules::type_line),
                    "implementationStatus": rules.map_or("complete", |rules| {
                        implementation_status_name(rules.implementation_status())
                    }),
                    "isLand": rules.is_some_and(|rules| rules.kind == penta::CardKind::Land),
                    "manaCost": mana_cost.map(|cost| json!({
                        "generic": cost.generic,
                        "white": cost.white,
                        "blue": cost.blue,
                        "black": cost.black,
                        "red": cost.red,
                        "green": cost.green,
                        "whiteRedHybrid": cost.white_red_hybrid,
                        "x": cost.variable_x,
                    })),
                    "rulesText": rules.map_or_else(String::new, |rules| {
                        rules.rules_text().into_owned()
                    }),
                    "owner": if permanent.controller == self.human { "human" } else { "opponent" },
                    "tapped": permanent.tapped,
                    "power": permanent.power,
                    "toughness": permanent.toughness,
                    "damage": permanent.damage,
                    "attacking": permanent.attacking,
                    "blocking": permanent.blocking.map(|id| id.0),
                    "flying": permanent.flying,
                    "canAttack": permanent.can_attack,
                    "enteredThisTurn": permanent.entered_this_turn,
                })
            })
            .collect::<Vec<_>>();
        let hand = observation
            .hand
            .iter()
            .map(|(id, definition)| {
                let card = self.catalog.get(*definition);
                let art = card.and_then(|card| card.art.as_ref());
                let creature_stats = card.and_then(|card| card.rules.creature_stats);
                json!({
                    "id": id.0,
                    "name": self.card_name(*definition),
                    "art": card_art_value(art),
                    "kind": card.map_or("unknown".into(), |card| {
                        format!("{:?}", card.rules.kind).to_ascii_lowercase()
                    }),
                    "typeLine": card.map_or_else(String::new, |card| card.rules.type_line()),
                    "implementationStatus": card.map_or("complete", |card| {
                        implementation_status_name(card.implementation_status())
                    }),
                    "isLand": card.is_some_and(|card| card.rules.kind == penta::CardKind::Land),
                    "manaCost": hand_mana_cost_value(card),
                    "rulesText": card.map_or_else(String::new, |card| {
                        card.rules.rules_text().into_owned()
                    }),
                    "power": creature_stats.map(|stats| stats.power),
                    "toughness": creature_stats.map(|stats| stats.toughness),
                })
            })
            .collect::<Vec<_>>();
        let stack = observation
            .stack
            .iter()
            .rev()
            .map(|object| {
                let ability_id = object.ability.and_then(|origin| match origin {
                    AbilityOrigin::Printed { ability, .. }
                    | AbilityOrigin::Granted { ability, .. } => Some(ability.0),
                    AbilityOrigin::IntrinsicBasicLand(_) => None,
                });
                // Enough card detail for the browser to draw a real card on
                // the stack rather than a name tag.
                let card = self.catalog.get(object.definition);
                let art = card.and_then(|card| card.art.as_ref());
                let signature = object.signature.as_ref();
                let presentation = stack_card_presentation(card, signature);
                let targets = signature.map_or_else(
                    || object.targets.clone(),
                    |signature| signature.iter_targets().copied().collect(),
                );
                json!({
                    "id": object.id.0,
                    // Kept as a JSON compatibility field for the browser;
                    // this is the spell/ability object, not physical lineage.
                    "cardId": object.id.0,
                    "sourceId": object.source.map(|source| source.0),
                    "ability": object.ability.map(ability_origin_value),
                    // Compatibility projection for clients that only know clause IDs.
                    "abilityId": ability_id,
                    "abilityText": object.ability_text,
                    "name": presentation.name,
                    "art": card_art_value(art),
                    "owner": if object.controller == self.human { "human" } else { "opponent" },
                    "kind": format!("{:?}", object.kind),
                    "x": signature.map_or(0, penta::CastSignature::x),
                    "playOptionId": signature.map(|signature| signature.play_option().0),
                    "modeIds": signature.map(|signature| {
                        signature.modes().iter().map(|mode| mode.0).collect::<Vec<_>>()
                    }),
                    "signature": signature.map(|signature| {
                        cast_signature_value(signature, self.human)
                    }),
                    "targetCardIds": targets
                        .iter()
                        .filter_map(|target| match target {
                            Target::Card(id) | Target::Permanent(id) => Some(id.0),
                            _ => None,
                        })
                        .collect::<Vec<_>>(),
                    "targetPlayers": targets
                        .iter()
                        .filter_map(|target| match target {
                            Target::Player(player) if *player == self.human => Some("human"),
                            Target::Player(_) => Some("opponent"),
                            _ => None,
                        })
                        .collect::<Vec<_>>(),
                    "targetStackIds": targets
                        .iter()
                        .filter_map(|target| match target {
                            Target::Spell(id) => Some(id.0),
                            _ => None,
                        })
                        .collect::<Vec<_>>(),
                    "cardKind": presentation.kind,
                    "typeLine": presentation.type_line,
                    "implementationStatus": implementation_status_name(
                        presentation.implementation_status,
                    ),
                    "isLand": presentation.is_land,
                    "manaCost": presentation.mana_cost.map(|cost| json!({
                        "generic": cost.generic,
                        "white": cost.white,
                        "blue": cost.blue,
                        "black": cost.black,
                        "red": cost.red,
                        "green": cost.green,
                        "whiteRedHybrid": cost.white_red_hybrid,
                        "x": cost.variable_x,
                    })),
                    "rulesText": presentation.rules_text,
                    "power": presentation.power,
                    "toughness": presentation.toughness,
                })
            })
            .collect::<Vec<_>>();
        let graveyard = |player: PlayerId| {
            observation.graveyards[player.index()]
                .iter()
                .rev()
                .map(|(_, definition)| self.card_name(*definition))
                .collect::<Vec<_>>()
        };
        let result = self.game.result().map(|result| match result {
            GameResult::Winner { winner, reason } => json!({
                "outcome": if winner == self.human { "win" } else { "loss" },
                "message": format!(
                    "{} — {}",
                    if winner == self.human { "You win" } else { "You lose" },
                    // WinReason names the loser as "the opponent" from the
                    // winner's seat. The browser only ever has the human's
                    // seat, so say who actually did the losing.
                    win_reason_text(reason, winner != self.human)
                ),
            }),
            GameResult::Draw => json!({"outcome": "draw", "message": "Draw"}),
        });
        let events = self
            .game
            .events()
            .iter()
            .rev()
            .filter_map(|event| self.event_label(&observation, event))
            .take(16)
            .collect::<Vec<_>>();
        let opponent_actions = if include_opponent_actions {
            self.opponent_actions.clone()
        } else {
            Vec::new()
        };
        // Only worth sending alongside a replay: with nothing to replay the
        // client shows this state directly and your action is already in it.
        let human_action_state = if include_opponent_actions && !self.opponent_actions.is_empty() {
            self.human_action_state.clone()
        } else {
            None
        };
        let decision = observation.decision.as_ref().map(|decision| {
            let mut value = json!({
                "id": decision.id,
                "kind": match decision.kind {
                    DecisionKind::Choice => "Choice",
                    DecisionKind::TriggerOrder => "TriggerOrder",
                    DecisionKind::TriggerPlacement => "TriggerPlacement",
                },
                "prompt": decision.prompt,
                "minimum": decision.minimum,
                "maximum": decision.maximum,
                "cancellable": decision.cancellable,
                "visibility": readable_debug(decision.visibility),
                "options": decision.options.iter().map(|option| json!({
                    "id": option.id,
                    "triggerId": matches!(decision.kind, DecisionKind::TriggerOrder).then_some(option.id),
                    "label": option.label,
                    "cardId": option.card.map(|(card, _)| card.0),
                    "cardName": option.card.map(|(_, definition)| self.card_name(definition)),
                    "abilityText": option.ability_text,
                    "zone": readable_debug(option.zone),
                })).collect::<Vec<_>>(),
            });
            if let Some(order_semantics) = decision.order_semantics {
                value["orderSemantics"] = Value::from(match order_semantics {
                    DecisionOrderSemantics::Resolution => "resolution",
                });
            }
            value
        });

        json!({
            "format": self.game.format().slug(),
            "turn": observation.active_turn,
            "gameTurn": observation.turn,
            "step": readable_debug(observation.step),
            // Turn one has not started yet, so the board should not be
            // claiming an upkeep is happening.
            "pregame": self.game.in_pregame(),
            "active": if observation.active_player == self.human { "You" } else { "Opponent" },
            "priority": if observation.priority == self.human { "You" } else { "Opponent" },
            "human": {
                "life": observation.life_totals[self.human.index()],
                "library": observation.library_sizes[self.human.index()],
                "mana": {
                    "white": observation.mana_pools[self.human.index()].white,
                    "blue": observation.mana_pools[self.human.index()].blue,
                    "black": observation.mana_pools[self.human.index()].black,
                    "red": observation.mana_pools[self.human.index()].red,
                    "green": observation.mana_pools[self.human.index()].green,
                    "colorless": observation.mana_pools[self.human.index()].colorless,
                },
                "hand": hand,
                "graveyard": graveyard(self.human),
            },
            "opponent": {
                "life": observation.life_totals[opponent.index()],
                "library": observation.library_sizes[opponent.index()],
                "handSize": observation.opponent_hand_size,
                "mana": {
                    "white": observation.mana_pools[opponent.index()].white,
                    "blue": observation.mana_pools[opponent.index()].blue,
                    "black": observation.mana_pools[opponent.index()].black,
                    "red": observation.mana_pools[opponent.index()].red,
                    "green": observation.mana_pools[opponent.index()].green,
                    "colorless": observation.mana_pools[opponent.index()].colorless,
                },
                "graveyard": graveyard(opponent),
            },
            "battlefield": battlefield,
            "stack": stack,
            "actions": actions,
            "passLabel": self.pass_preview_label(),
            "decision": decision,
            "canUndoMana": !self.mana_undo_history.is_empty(),
            "canCancelAttackers": self.attack_undo.is_some(),
            "phaseStops": self.phase_stops,
            "autopassEnabled": self.autopass_enabled,
            "opponentActions": opponent_actions,
            "afterYourAction": human_action_state,
            "result": result,
            "events": events,
        })
    }

    /// Plain-language description of a permanent's targeted ability, so menus
    /// can name the effect rather than the card that carries it.
    fn ability_text(
        &self,
        observation: &PlayerObservation,
        source: CardInstanceId,
        origin: AbilityOrigin,
    ) -> Option<ActivatedAbilityText> {
        observation
            .battlefield
            .iter()
            .find(|permanent| permanent.id == source)?;
        let AbilityOrigin::Printed {
            definition,
            part,
            ability,
        } = origin
        else {
            // Intrinsic and granted abilities do not have card-local action
            // presentation metadata. Their generic source/target label remains
            // accurate until granted ability snapshots expose that metadata.
            return None;
        };
        let card = self.catalog.get(definition)?;
        card.part(part)?
            .rules
            .ability_clauses()
            .iter()
            .find(|candidate| candidate.id == ability)
            .and_then(|candidate| candidate.activation_text)
    }

    fn card_name(&self, definition: CardDefinitionId) -> String {
        self.catalog
            .get(definition)
            .map_or_else(|| "Unknown card".into(), |card| card.name.clone())
    }

    fn instance_definition(
        observation: &PlayerObservation,
        id: CardInstanceId,
    ) -> Option<CardDefinitionId> {
        observation
            .hand
            .iter()
            .find_map(|(candidate, definition)| (*candidate == id).then_some(*definition))
            .or_else(|| {
                observation
                    .battlefield
                    .iter()
                    .find_map(|permanent| (permanent.id == id).then_some(permanent.definition))
            })
            .or_else(|| {
                observation
                    .graveyards
                    .iter()
                    .flatten()
                    .find_map(|(candidate, definition)| (*candidate == id).then_some(*definition))
            })
            .or_else(|| {
                // A spell the opponent just cast is still on the stack, and it
                // is public there even though it never passed through a zone
                // this observation can see.
                observation
                    .stack
                    .iter()
                    .find_map(|object| (object.id == id).then_some(object.definition))
            })
            .or_else(|| {
                // Exiled cards stay public, and the log keeps referring to them
                // long after Swords to Plowshares removed them from the board.
                observation
                    .exiles
                    .iter()
                    .flatten()
                    .find_map(|(candidate, definition)| (*candidate == id).then_some(*definition))
            })
    }

    fn instance_name(&self, observation: &PlayerObservation, id: CardInstanceId) -> String {
        Self::instance_definition(observation, id).map_or_else(
            // A card that has since moved somewhere this observation cannot
            // read — shuffled back into a library, say — is still described
            // in words rather than as a raw instance id.
            || "a card".into(),
            |definition| self.card_name(definition),
        )
    }

    fn play_option_label(
        &self,
        observation: &PlayerObservation,
        card: CardInstanceId,
        option: PlayOptionId,
    ) -> Option<String> {
        Self::instance_definition(observation, card)
            .and_then(|definition| self.catalog.get(definition))
            .and_then(|definition| definition.play_option(option))
            .map(|option| option.label.clone())
    }

    fn mode_labels(
        &self,
        observation: &PlayerObservation,
        card: CardInstanceId,
        option: PlayOptionId,
        modes: &[ModeId],
    ) -> Vec<String> {
        let mode_definitions = Self::instance_definition(observation, card)
            .and_then(|definition| self.catalog.get(definition))
            .and_then(|definition| definition.play_option(option))
            .and_then(|option| option.modes.as_ref());
        modes
            .iter()
            .map(|id| {
                mode_definitions
                    .and_then(|definitions| definitions.modes.iter().find(|mode| mode.id == *id))
                    .map_or_else(|| format!("Mode {}", id.0), |mode| mode.label.clone())
            })
            .collect()
    }

    fn target_name(&self, observation: &PlayerObservation, target: Target) -> String {
        match target {
            Target::Player(player) if player == self.human => "you".into(),
            Target::Player(_) => "opponent".into(),
            Target::Card(id) | Target::Permanent(id) => self.instance_name(observation, id),
            // A countered or resolved spell leaves no trace the observation can
            // name, and stack object ids are not card ids, so the log says what
            // it honestly knows rather than printing a raw id.
            Target::Spell(id) => observation
                .stack
                .iter()
                .find(|object| object.id == id)
                .map_or_else(
                    || "a spell".into(),
                    |object| self.card_name(object.definition),
                ),
        }
    }

    fn player_name(&self, player: PlayerId) -> &'static str {
        if player == self.human {
            "You"
        } else {
            "Opponent"
        }
    }

    #[allow(clippy::too_many_lines)]
    fn event_label(&self, observation: &PlayerObservation, event: &GameEvent) -> Option<String> {
        match event {
            GameEvent::GameStarted { seed } => Some(format!("Game started · seed {seed}")),
            GameEvent::CardDrawn { player, card } if *player == self.human => Some(format!(
                "You drew {}",
                self.instance_name(observation, *card)
            )),
            GameEvent::CardDrawn { .. } => Some("Opponent drew a card".into()),
            GameEvent::CardsDiscarded { player, cards } => Some(format!(
                "{} discarded {} at random",
                self.player_name(*player),
                cards
                    .iter()
                    .map(|(_, definition)| self.card_name(*definition))
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            GameEvent::LandPlayed {
                player, definition, ..
            } => Some(format!(
                "{} played {}",
                self.player_name(*player),
                self.card_name(*definition)
            )),
            GameEvent::SpellCast {
                player,
                definition,
                targets,
                ..
            } => {
                let mut label = format!(
                    "{} cast {}",
                    self.player_name(*player),
                    self.card_name(*definition)
                );
                if !targets.is_empty() {
                    let target_names = targets
                        .iter()
                        .map(|target| self.target_name(observation, *target))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let _ = write!(label, " → {target_names}");
                }
                Some(label)
            }
            GameEvent::AbilityActivated {
                player, definition, ..
            } => Some(format!(
                "{} activated {}",
                self.player_name(*player),
                self.card_name(*definition)
            )),
            GameEvent::AbilityTriggered {
                player, definition, ..
            } => Some(format!(
                "{} {} triggered",
                if *player == self.human {
                    "Your"
                } else {
                    "Opponent’s"
                },
                self.card_name(*definition)
            )),
            GameEvent::AttackDeclared { player, attackers } => Some(format!(
                "{} attacked with {}",
                self.player_name(*player),
                attackers
                    .iter()
                    .map(|attacker| self.instance_name(observation, *attacker))
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            GameEvent::BlockDeclared {
                player,
                assignments,
            } => Some(format!(
                "{} blocked {}",
                self.player_name(*player),
                assignments
                    .iter()
                    .map(|(blocker, attacker)| {
                        format!(
                            "{} with {}",
                            self.instance_name(observation, *attacker),
                            self.instance_name(observation, *blocker)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            GameEvent::ErhnamForestwalkGranted {
                player,
                source,
                target,
            } => Some(format!(
                "{} used {} to give {} forestwalk",
                self.player_name(*player),
                self.instance_name(observation, *source),
                self.instance_name(observation, *target)
            )),
            GameEvent::DamageDealt { player, amount } => Some(format!(
                "{} took {amount} damage",
                self.player_name(*player)
            )),
            GameEvent::ManaBurn { player, amount } => Some(format!(
                "{} took {amount} mana burn",
                self.player_name(*player)
            )),
            GameEvent::StepChanged {
                turn,
                active_player,
                step: Step::PrecombatMain,
            } => Some(format!(
                "Turn {} · {} turn",
                turn.div_ceil(2),
                if *active_player == self.human {
                    "your"
                } else {
                    "opponent’s"
                }
            )),
            GameEvent::SpellFizzled { definition, .. } => Some(format!(
                "{} fizzled — its target was gone",
                self.card_name(*definition)
            )),
            GameEvent::PermanentLeftBattlefield {
                controller,
                definition,
                destination,
                ..
            } => Some(format!(
                "{} {} {}",
                if *controller == self.human {
                    "Your"
                } else {
                    "Opponent’s"
                },
                self.card_name(*definition),
                match destination {
                    BattlefieldExit::Graveyard => "was destroyed",
                    BattlefieldExit::Exile => "was exiled",
                    BattlefieldExit::Hand => "returned to hand",
                }
            )),
            GameEvent::GameEnded { result } => Some(match result {
                GameResult::Winner { winner, .. } if *winner == self.human => "You won".into(),
                GameResult::Winner { .. } => "Opponent won".into(),
                GameResult::Draw => "Game ended in a draw".into(),
            }),
            GameEvent::ManaAdded { .. }
            | GameEvent::SpellResolved { .. }
            | GameEvent::AbilityResolved { .. }
            | GameEvent::TriggeredAbilityPutOnStack { .. }
            | GameEvent::TriggeredAbilityResolved { .. }
            | GameEvent::StepChanged { .. } => None,
        }
    }

    #[allow(clippy::too_many_lines)]
    fn action_label(&self, observation: &PlayerObservation, action: &Action) -> String {
        let targets = |values: &[Target]| {
            values
                .iter()
                .map(|target| self.target_name(observation, *target))
                .collect::<Vec<_>>()
                .join(", ")
        };
        match action {
            Action::KeepHand => "Keep this hand".into(),
            Action::TakeMulligan => "Take a mulligan".into(),
            Action::BottomCards { cards } => format!(
                "Bottom {}",
                cards
                    .iter()
                    .map(|id| self.instance_name(observation, *id))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Action::DiscardCards { cards } => format!(
                "Discard {}",
                cards
                    .iter()
                    .map(|id| self.instance_name(observation, *id))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Action::ChooseDecision { options, .. } => {
                let labels = observation
                    .decision
                    .as_ref()
                    .map_or_else(Vec::new, |decision| {
                        decision
                            .options
                            .iter()
                            .filter(|option| options.contains(&option.id))
                            .map(|option| option.label.clone())
                            .collect::<Vec<_>>()
                    });
                if labels.is_empty() {
                    // The engine also enumerates a bare schema placeholder for
                    // the pending decision; never hand the browser a blank
                    // label it could render as an unlabelled control.
                    "Choose an option".into()
                } else {
                    labels.join(", ")
                }
            }
            Action::CancelDecision { .. } => "Cancel".into(),
            Action::ChooseUntap { permanents } => format!(
                "Untap {}",
                permanents
                    .iter()
                    .map(|id| self.instance_name(observation, *id))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Action::PassPriority => "Pass priority".into(),
            Action::PlayLand { card, option } => {
                let option = self
                    .play_option_label(observation, *card, *option)
                    .unwrap_or_else(|| self.instance_name(observation, *card));
                format!("Play {option}")
            }
            Action::ActivateManaAbility { source, color, .. } => {
                format!(
                    "Tap {} for {} mana",
                    self.instance_name(observation, *source),
                    readable_debug(*color)
                )
            }
            Action::PayLifeForMana => "Pay 1 life for 1 colorless mana".into(),
            Action::CastSpell {
                card,
                choices,
                sacrifices,
            } => {
                let option = self
                    .play_option_label(observation, *card, choices.play_option())
                    .unwrap_or_else(|| self.instance_name(observation, *card));
                let mut label = format!("Cast {option}");
                let modes =
                    self.mode_labels(observation, *card, choices.play_option(), choices.modes());
                if !modes.is_empty() {
                    let _ = write!(label, " — {}", modes.join(" + "));
                }
                if choices.x() > 0 {
                    let _ = write!(label, " (X={})", choices.x());
                }
                if !sacrifices.is_empty() {
                    let _ = write!(
                        label,
                        " (sacrifice {})",
                        sacrifices
                            .iter()
                            .map(|id| self.instance_name(observation, *id))
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }
                let values = choices.iter_targets().copied().collect::<Vec<_>>();
                if !values.is_empty() {
                    let _ = write!(label, " → {}", targets(&values));
                }
                label
            }
            Action::ActivateAbility {
                source,
                ability,
                targets,
                sacrifice,
            } => {
                let source_name = self.instance_name(observation, *source);
                let target = targets
                    .iter()
                    .flat_map(penta::TargetSelection::targets)
                    .next()
                    .copied();
                if source_name == "Mishra's Factory" && target.is_none() {
                    return "Make Mishra's Factory a 2/2 creature".into();
                }
                // "Activate Strip Mine" says nothing about what the click does,
                // so a described ability names its own effect instead.
                let described = target
                    .zip(self.ability_text(observation, *source, *ability))
                    .map(|(target, text)| {
                        text.targeted
                            .replace("{}", &self.target_name(observation, target))
                    });
                let mut label = described
                    .clone()
                    .unwrap_or_else(|| format!("Activate {source_name}"));
                if let Some(sacrifice) = sacrifice
                    && sacrifice != source
                {
                    let _ = write!(
                        label,
                        " (sacrifice {})",
                        self.instance_name(observation, *sacrifice)
                    );
                }
                if let Some(target) = target
                    && described.is_none()
                {
                    let _ = write!(label, " → {}", self.target_name(observation, target));
                }
                label
            }
            Action::DeclareAttacker { attacker } => {
                format!("Attack with {}", self.instance_name(observation, *attacker))
            }
            // Naming the commitment reads better than naming the step: the
            // button is the last chance to see how big the attack is.
            Action::FinishDeclaringAttackers => {
                let declared = observation
                    .battlefield
                    .iter()
                    .filter(|permanent| {
                        permanent.controller == observation.viewer && permanent.attacking
                    })
                    .count();
                match declared {
                    0 => "No attacks".into(),
                    1 => "Attack with 1 creature".into(),
                    count => format!("Attack with {count} creatures"),
                }
            }
            Action::DeclareBlocker { blocker, attacker } => format!(
                "Block {} with {}",
                self.instance_name(observation, *attacker),
                self.instance_name(observation, *blocker)
            ),
            Action::FinishDeclaringBlockers => "Finish blocking".into(),
            // The attacker is already named in the prompt above these buttons,
            // so each option only has to say where the damage lands. Recipients
            // taking nothing are noise and stay out of the label.
            Action::AssignCombatDamage { assignments, .. } => {
                let landed = assignments
                    .iter()
                    .filter(|assignment| assignment.amount > 0)
                    .map(|assignment| {
                        format!(
                            "{} to {}",
                            assignment.amount,
                            self.target_name(observation, assignment.recipient)
                        )
                    })
                    .collect::<Vec<_>>();
                if landed.is_empty() {
                    "Deal no damage".into()
                } else {
                    landed.join(", ")
                }
            }
            Action::Concede => "Concede game".into(),
        }
    }

    fn opponent_action_label(&self, observation: &PlayerObservation, action: &Action) -> String {
        match action {
            Action::BottomCards { cards } => format!(
                "Bottom {} {}",
                cards.len(),
                if cards.len() == 1 { "card" } else { "cards" }
            ),
            // Only the human's own pending decision has option labels this
            // observation can read. Anything else the opponent chose stays
            // private, including when the human is mid-decision themselves.
            Action::ChooseDecision { decision, .. }
                if observation
                    .decision
                    .as_ref()
                    .is_none_or(|visible| visible.id != *decision) =>
            {
                "Opponent made a private choice".into()
            }
            _ => self.action_label(observation, action),
        }
    }
}

fn card_art_value(art: Option<&penta::CardArt>) -> Value {
    art.map_or(Value::Null, |art| {
        json!({
            "scryfallId": art.scryfall_id,
            "artist": art.artist,
        })
    })
}

fn hand_mana_cost_value(card: Option<&penta::CardDefinition>) -> Value {
    card.and_then(penta::CardDefinition::primary_part)
        .and_then(|part| part.mana_cost)
        .map_or(Value::Null, |cost| {
            json!({
                "generic": cost.generic,
                "white": cost.white,
                "blue": cost.blue,
                "black": cost.black,
                "red": cost.red,
                "green": cost.green,
                "whiteRedHybrid": cost.white_red_hybrid,
                "x": cost.variable_x,
            })
        })
}

fn cast_signature_value(signature: &penta::CastSignature, human: PlayerId) -> Value {
    let form = match signature.form() {
        penta::SpellForm::Part(part) => json!({
            "kind": "part",
            "partId": part.0,
        }),
        penta::SpellForm::Combined(parts) => json!({
            "kind": "combined",
            "partIds": parts.iter().map(|part| part.0).collect::<Vec<_>>(),
        }),
    };
    let target_selections = signature
        .targets()
        .iter()
        .map(|selection| {
            json!({
                "slotId": selection.slot().0,
                "targetCardIds": selection.targets().iter().filter_map(|target| match target {
                    Target::Card(id) | Target::Permanent(id) => Some(id.0),
                    Target::Player(_) | Target::Spell(_) => None,
                }).collect::<Vec<_>>(),
                "targetPlayers": selection.targets().iter().filter_map(|target| match target {
                    Target::Player(player) => Some(if *player == human {
                        "human"
                    } else {
                        "opponent"
                    }),
                    Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                }).collect::<Vec<_>>(),
                "targetStackIds": selection.targets().iter().filter_map(|target| match target {
                    Target::Spell(id) => Some(id.0),
                    Target::Player(_) | Target::Card(_) | Target::Permanent(_) => None,
                }).collect::<Vec<_>>(),
            })
        })
        .collect::<Vec<_>>();
    json!({
        "playOptionId": signature.play_option().0,
        "form": form,
        "modeIds": signature.modes().iter().map(|mode| mode.0).collect::<Vec<_>>(),
        "alternativeCostId": signature.costs().alternative().map(|cost| cost.0),
        "additionalCostIds": signature
            .costs()
            .additional()
            .iter()
            .map(|cost| cost.0)
            .collect::<Vec<_>>(),
        "x": signature.x(),
        "targetSelections": target_selections,
    })
}

struct StackCardPresentation {
    name: String,
    kind: String,
    type_line: String,
    implementation_status: penta::ImplementationStatus,
    is_land: bool,
    mana_cost: Option<penta::ManaCost>,
    rules_text: String,
    power: Option<i16>,
    toughness: Option<i16>,
}

impl StackCardPresentation {
    fn unknown() -> Self {
        Self {
            name: "Unknown card".into(),
            kind: "unknown".into(),
            type_line: String::new(),
            implementation_status: penta::ImplementationStatus::Complete,
            is_land: false,
            mana_cost: None,
            rules_text: String::new(),
            power: None,
            toughness: None,
        }
    }

    fn from_rules(
        name: String,
        rules: &penta::CardRules,
        mana_cost: Option<penta::ManaCost>,
    ) -> Self {
        Self {
            name,
            kind: format!("{:?}", rules.kind).to_ascii_lowercase(),
            type_line: rules.type_line(),
            implementation_status: rules.implementation_status(),
            is_land: rules.kind == penta::CardKind::Land,
            mana_cost,
            rules_text: rules.rules_text().into_owned(),
            power: rules.creature_stats.map(|stats| stats.power),
            toughness: rules.creature_stats.map(|stats| stats.toughness),
        }
    }
}

fn stack_card_presentation(
    card: Option<&penta::CardDefinition>,
    signature: Option<&penta::CastSignature>,
) -> StackCardPresentation {
    let Some(card) = card else {
        return StackCardPresentation::unknown();
    };
    let canonical = || {
        StackCardPresentation::from_rules(
            card.name.clone(),
            &card.rules,
            Some(card.rules.mana_cost),
        )
    };
    let Some(signature) = signature else {
        return canonical();
    };

    match signature.form() {
        penta::SpellForm::Part(part_id) => card.part(*part_id).map_or_else(canonical, |part| {
            StackCardPresentation::from_rules(part.name.clone(), &part.rules, part.mana_cost)
        }),
        penta::SpellForm::Combined(part_ids) => {
            let Some(parts) = part_ids
                .iter()
                .map(|part_id| card.part(*part_id))
                .collect::<Option<Vec<_>>>()
            else {
                return canonical();
            };
            if parts.is_empty() {
                return canonical();
            }

            let name = parts
                .iter()
                .map(|part| part.name.as_str())
                .collect::<Vec<_>>()
                .join(" // ");
            let kind = join_distinct(
                parts
                    .iter()
                    .map(|part| format!("{:?}", part.rules.kind).to_ascii_lowercase()),
            );
            let type_line = join_distinct(parts.iter().map(|part| part.rules.type_line()));
            let rules_text = parts
                .iter()
                .map(|part| format!("{} — {}", part.name, part.rules.rules_text()))
                .collect::<Vec<_>>()
                .join("\n\n");
            let stats = parts
                .iter()
                .filter_map(|part| part.rules.creature_stats)
                .collect::<Vec<_>>();
            let shared_stats = stats
                .first()
                .copied()
                .filter(|first| stats.iter().all(|stats| stats == first));
            let mana_cost = card
                .play_option(signature.play_option())
                .filter(|option| &option.form == signature.form())
                .and_then(|option| option.mana_cost);

            StackCardPresentation {
                name,
                kind,
                type_line,
                implementation_status: parts
                    .iter()
                    .map(|part| part.rules.implementation_status())
                    .reduce(penta::ImplementationStatus::combine)
                    .unwrap_or_default(),
                is_land: parts
                    .iter()
                    .any(|part| part.rules.kind == penta::CardKind::Land),
                mana_cost,
                rules_text,
                power: shared_stats.map(|stats| stats.power),
                toughness: shared_stats.map(|stats| stats.toughness),
            }
        }
    }
}

const fn implementation_status_name(status: penta::ImplementationStatus) -> &'static str {
    match status {
        penta::ImplementationStatus::Complete => "complete",
        penta::ImplementationStatus::Partial => "partial",
        penta::ImplementationStatus::MetadataOnly => "metadataOnly",
    }
}

fn join_distinct(values: impl IntoIterator<Item = String>) -> String {
    let mut distinct = Vec::new();
    for value in values {
        if !distinct.contains(&value) {
            distinct.push(value);
        }
    }
    distinct.join(" // ")
}

fn deck_by_name(format: Format, name: &str) -> Result<penta::Deck, JsValue> {
    penta::protocol::deck_by_name_for_format(format, name)
        .ok_or_else(|| JsValue::from_str("unknown deck for format"))
}

/// Describes why the game ended from the browser player's seat.
/// `human_lost` selects the second-person phrasing.
fn win_reason_text(reason: penta::WinReason, human_lost: bool) -> &'static str {
    match (reason, human_lost) {
        (penta::WinReason::OpponentConceded, false) => "opponent conceded",
        (penta::WinReason::OpponentConceded, true) => "you conceded",
        (penta::WinReason::OpponentLostAllLife, false) => "opponent lost all life",
        (penta::WinReason::OpponentLostAllLife, true) => "you lost all life",
        (penta::WinReason::OpponentTriedToDrawFromEmptyLibrary, false) => {
            "opponent drew from an empty library"
        }
        (penta::WinReason::OpponentTriedToDrawFromEmptyLibrary, true) => {
            "you drew from an empty library"
        }
    }
}

fn action_kind(action: &Action) -> &'static str {
    match action {
        Action::Concede => "danger",
        Action::PassPriority
        | Action::FinishDeclaringAttackers
        | Action::FinishDeclaringBlockers => "pass",
        Action::DeclareAttacker { .. }
        | Action::DeclareBlocker { .. }
        | Action::AssignCombatDamage { .. } => "combat",
        _ => "primary",
    }
}

#[derive(Clone, Copy)]
// These flags are independent policy inputs, not a state machine; keeping
// them named makes the Arena-style priority rules auditable at the call site.
#[allow(clippy::struct_excessive_bools)]
struct AutoPassContext {
    step: Step,
    human_is_active: bool,
    stack_is_empty: bool,
    has_attacker: bool,
    has_blocker: bool,
    stop_here: bool,
    autopass_enabled: bool,
    only_human_objects_on_stack: bool,
    human_has_floating_mana: bool,
    human_controls_creature: bool,
}

/// The pass-preview stand-in for the opponent: let the pass through wherever
/// possible, and resolve forced decisions (such as a cleanup discard) with an
/// arbitrary minimal selection so the preview can keep moving. Returns `None`
/// when the opponent holds a choice the preview cannot neutrally guess at.
fn neutral_opponent_action(observation: &PlayerObservation) -> Option<Action> {
    if let Some(decision) = observation.decision.as_ref() {
        if decision.options.len() < decision.minimum {
            return None;
        }
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
    // An opponent holding an untapped creature attacks with it almost every
    // time, and the human needs to be told they are heading into blocks
    // rather than idling through to an end step that never arrives.
    if observation.step == Step::DeclareAttackers
        && let Some(attack) = observation
            .legal_actions
            .iter()
            .find(|action| matches!(action, Action::DeclareAttacker { .. }))
    {
        return Some(attack.clone());
    }
    observation
        .legal_actions
        .iter()
        .find(|action| {
            matches!(
                action,
                Action::PassPriority
                    | Action::FinishDeclaringAttackers
                    | Action::FinishDeclaringBlockers
            )
        })
        .or_else(|| {
            // A forced cleanup discard: any card works as a stand-in, since
            // the preview only reports where the human ends up waiting.
            observation
                .legal_actions
                .iter()
                .find(|action| matches!(action, Action::DiscardCards { .. }))
        })
        .cloned()
}

/// Whether this priority window is one the browser hands back without asking.
fn is_routine_window(context: &AutoPassContext, actions: &[Action]) -> bool {
    // Arena normally hides routine beginning-phase priority windows on either
    // turn. Stops restore them; floating mana in the end step is a
    // smart-priority case.
    let routine_beginning_step = matches!(context.step, Step::Upkeep | Step::Draw);
    // Activating a permanent you already control is available at instant speed
    // — on their turn, at your end step — so it is not a reason to hold a
    // routine window open. Committing from hand is once-a-turn and is.
    let can_commit_from_hand = actions
        .iter()
        .any(|action| matches!(action, Action::CastSpell { .. } | Action::PlayLand { .. }));
    // Damage lands on the way into the damage step, so by the time anyone
    // holds priority there it is history. Neither window can change the
    // combat, on either player's turn — they just cost a click each.
    let combat_is_settled = matches!(context.step, Step::CombatDamage | Step::EndOfCombat);
    let routine_own_turn_step = context.human_is_active
        && (context.step == Step::BeginningOfCombat
            || (context.step == Step::PostcombatMain
                && (!context.human_controls_creature || !can_commit_from_hand))
            || (context.step == Step::End && !context.human_has_floating_mana));
    // On the opponent's turn the interesting window is their end step, where
    // instants are cheapest. Combat they never committed to, and their second
    // main, are both worth skipping to get there.
    let routine_opponent_turn_step = !context.human_is_active
        && (context.step == Step::BeginningOfCombat
            || context.step == Step::PostcombatMain
            || (!context.has_attacker
                && matches!(context.step, Step::DeclareAttackers | Step::DeclareBlockers)));
    let smooth_unblocked_attack = context.human_is_active
        && context.has_attacker
        && !context.has_blocker
        && matches!(context.step, Step::DeclareAttackers | Step::DeclareBlockers);
    // Nothing is blocking, so the rest of their combat is just watching the
    // damage land. The interesting window is their end step, so head there
    // instead of stopping once per remaining combat step. A block still on
    // offer means the decision has not been made yet and this is not it.
    let block_still_available = actions
        .iter()
        .any(|action| matches!(action, Action::DeclareBlocker { .. }));
    let smooth_unblocked_defense = !context.human_is_active
        && context.has_attacker
        && !context.has_blocker
        && !block_still_available
        && context.step == Step::DeclareBlockers;
    routine_beginning_step
        || combat_is_settled
        || routine_own_turn_step
        || routine_opponent_turn_step
        || smooth_unblocked_attack
        || smooth_unblocked_defense
        || (!context.has_attacker
            && matches!(context.step, Step::DeclareAttackers | Step::DeclareBlockers))
}

fn automatic_human_action_for_context(
    context: AutoPassContext,
    actions: &[Action],
) -> Option<Action> {
    if !context.autopass_enabled {
        return None;
    }
    let has_attack_option = actions
        .iter()
        .any(|action| matches!(action, Action::DeclareAttacker { .. }));
    let empty_combat_after_attackers = !context.has_attacker
        && (matches!(
            context.step,
            Step::DeclareBlockers | Step::CombatDamage | Step::EndOfCombat
        ) || (context.step == Step::DeclareAttackers && !has_attack_option));
    if context.stop_here && !empty_combat_after_attackers {
        return None;
    }
    if context.only_human_objects_on_stack
        && let Some(pass) = actions
            .iter()
            .find(|action| matches!(action, Action::PassPriority))
    {
        return Some(pass.clone());
    }
    // Never fast-forward through the human's entire turn. Even with no card
    // actions available, Main 1 is the stable point where the player can see
    // the draw and deliberately advance into combat.
    if context.human_is_active && context.step == Step::PrecombatMain && context.stack_is_empty {
        return None;
    }
    let has_meaningful_choice = actions.iter().any(|action| {
        !matches!(
            action,
            Action::Concede
                | Action::PassPriority
                | Action::ActivateManaAbility { .. }
                | Action::FinishDeclaringAttackers
                | Action::FinishDeclaringBlockers
        )
    });
    let has_combat_ability = actions
        .iter()
        .any(|action| matches!(action, Action::ActivateAbility { .. }));
    let auto_yield_step = is_routine_window(&context, actions);
    // A combat ability is worth pausing for while it can still change the
    // outcome. Once damage is dealt, pumping a creature decides nothing.
    let combat_step = matches!(context.step, Step::DeclareAttackers | Step::DeclareBlockers);
    // Defending an attack nobody blocked is the exception: no creature of
    // yours is in the combat, so no ability of yours can change it.
    let ability_changes_combat = context.human_is_active || context.has_blocker;
    if auto_yield_step
        && (!combat_step || !context.has_attacker || !has_combat_ability || !ability_changes_combat)
        && context.stack_is_empty
        && let Some(pass) = actions
            .iter()
            .find(|action| matches!(action, Action::PassPriority))
    {
        return Some(pass.clone());
    }
    if has_meaningful_choice {
        return None;
    }
    // Committing an attack is the player's call. Running out of creatures to
    // declare is not the same as being finished, so once anything is attacking
    // the browser waits for them to confirm.
    if context.has_attacker && context.step == Step::DeclareAttackers {
        return None;
    }
    actions
        .iter()
        .find(|action| {
            matches!(
                action,
                Action::PassPriority
                    | Action::FinishDeclaringAttackers
                    | Action::FinishDeclaringBlockers
            )
        })
        .cloned()
}

#[cfg(test)]
#[allow(clippy::fn_params_excessive_bools, clippy::too_many_arguments)]
fn automatic_human_action(
    step: Step,
    human_is_active: bool,
    stack_is_empty: bool,
    has_attacker: bool,
    stop_here: bool,
    autopass_enabled: bool,
    only_human_objects_on_stack: bool,
    human_has_floating_mana: bool,
    actions: &[Action],
) -> Option<Action> {
    automatic_human_action_for_context(
        AutoPassContext {
            step,
            human_is_active,
            stack_is_empty,
            has_attacker,
            has_blocker: false,
            stop_here,
            autopass_enabled,
            only_human_objects_on_stack,
            human_has_floating_mana,
            human_controls_creature: true,
        },
        actions,
    )
}

#[cfg(test)]
#[allow(clippy::fn_params_excessive_bools, clippy::too_many_arguments)]
fn automatic_human_action_with_blockers(
    step: Step,
    human_is_active: bool,
    stack_is_empty: bool,
    has_attacker: bool,
    has_blocker: bool,
    stop_here: bool,
    autopass_enabled: bool,
    only_human_objects_on_stack: bool,
    human_has_floating_mana: bool,
    actions: &[Action],
) -> Option<Action> {
    automatic_human_action_for_context(
        AutoPassContext {
            step,
            human_is_active,
            stack_is_empty,
            has_attacker,
            has_blocker,
            stop_here,
            autopass_enabled,
            only_human_objects_on_stack,
            human_has_floating_mana,
            human_controls_creature: true,
        },
        actions,
    )
}

fn action_card(action: &Action) -> Option<CardInstanceId> {
    match action {
        Action::PlayLand { card, .. } | Action::CastSpell { card, .. } => Some(*card),
        Action::ActivateManaAbility { source, .. } | Action::ActivateAbility { source, .. } => {
            Some(*source)
        }
        Action::DeclareAttacker { attacker } | Action::AssignCombatDamage { attacker, .. } => {
            Some(*attacker)
        }
        Action::DeclareBlocker { blocker, .. } => Some(*blocker),
        _ => None,
    }
}

fn action_ability_origin(action: &Action) -> Option<Value> {
    let origin = match action {
        Action::ActivateManaAbility { ability, .. } | Action::ActivateAbility { ability, .. } => {
            *ability
        }
        _ => return None,
    };
    Some(ability_origin_value(origin))
}

fn ability_origin_value(origin: AbilityOrigin) -> Value {
    match origin {
        AbilityOrigin::Printed {
            definition,
            part,
            ability,
        } => json!({
            "kind": "printed",
            "definition": definition.0,
            "partId": part.0,
            "abilityId": ability.0,
        }),
        AbilityOrigin::IntrinsicBasicLand(land_type) => json!({
            "kind": "intrinsicBasicLand",
            "landType": format!("{land_type:?}").to_ascii_lowercase(),
        }),
        AbilityOrigin::Granted { source, ability } => json!({
            "kind": "granted",
            "source": source.0,
            "abilityId": ability.0,
        }),
    }
}

/// Permanents this action would destroy as part of its cost. The browser makes
/// the player pick these explicitly rather than spending whatever is to hand.
fn action_sacrifices(action: &Action) -> Vec<u32> {
    match action {
        Action::CastSpell { sacrifices, .. } => sacrifices.iter().map(|id| id.0).collect(),
        Action::ActivateAbility {
            source,
            sacrifice: Some(sacrifice),
            ..
        } if sacrifice != source => vec![sacrifice.0],
        _ => Vec::new(),
    }
}

fn action_target_card(action: &Action) -> Option<CardInstanceId> {
    if let Action::DeclareBlocker { attacker, .. } = action {
        return Some(*attacker);
    }
    action_targets(action)
        .iter()
        .find_map(|target| match target {
            Target::Card(id) | Target::Permanent(id) => Some(*id),
            Target::Player(_) | Target::Spell(_) => None,
        })
}

fn action_target_player(action: &Action, human: PlayerId) -> Option<&'static str> {
    action_targets(action)
        .iter()
        .find_map(|target| match target {
            Target::Player(player) => Some(if *player == human {
                "human"
            } else {
                "opponent"
            }),
            Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
        })
}

fn action_target_stack(action: &Action) -> Option<u32> {
    action_targets(action)
        .iter()
        .find_map(|target| match target {
            Target::Spell(id) => Some(id.0),
            Target::Player(_) | Target::Card(_) | Target::Permanent(_) => None,
        })
}

fn action_targets(action: &Action) -> Vec<Target> {
    match action {
        Action::CastSpell { choices, .. } => choices.iter_targets().copied().collect(),
        Action::ActivateAbility { targets, .. } => targets
            .iter()
            .flat_map(penta::TargetSelection::targets)
            .copied()
            .collect(),
        _ => Vec::new(),
    }
}

fn action_target_cards(action: &Action) -> Vec<u32> {
    action_targets(action)
        .iter()
        .filter_map(|target| match target {
            Target::Card(id) | Target::Permanent(id) => Some(id.0),
            Target::Player(_) | Target::Spell(_) => None,
        })
        .collect()
}

fn action_target_players(action: &Action, human: PlayerId) -> Vec<&'static str> {
    action_targets(action)
        .iter()
        .filter_map(|target| match target {
            Target::Player(player) => Some(if *player == human {
                "human"
            } else {
                "opponent"
            }),
            Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
        })
        .collect()
}

fn action_target_stacks(action: &Action) -> Vec<u32> {
    action_targets(action)
        .iter()
        .filter_map(|target| match target {
            Target::Spell(id) => Some(id.0),
            Target::Player(_) | Target::Card(_) | Target::Permanent(_) => None,
        })
        .collect()
}

fn should_animate_action(action: &Action) -> bool {
    !matches!(
        action,
        Action::KeepHand
            | Action::TakeMulligan
            | Action::BottomCards { .. }
            | Action::Concede
            | Action::PassPriority
            | Action::ActivateManaAbility { .. }
            | Action::FinishDeclaringAttackers
            | Action::FinishDeclaringBlockers
    )
}

fn animated_action_kind(action: &Action) -> &'static str {
    match action {
        Action::PlayLand { .. } => "land",
        Action::CastSpell { .. } => "spell",
        Action::ActivateAbility { .. } => "ability",
        Action::DeclareAttacker { .. }
        | Action::DeclareBlocker { .. }
        | Action::AssignCombatDamage { .. } => "combat",
        Action::KeepHand
        | Action::TakeMulligan
        | Action::BottomCards { .. }
        | Action::DiscardCards { .. }
        | Action::ChooseDecision { .. }
        | Action::CancelDecision { .. }
        | Action::ChooseUntap { .. } => "choice",
        Action::Concede
        | Action::PassPriority
        | Action::ActivateManaAbility { .. }
        | Action::PayLifeForMana
        | Action::FinishDeclaringAttackers
        | Action::FinishDeclaringBlockers => "quiet",
    }
}

fn readable_debug(value: impl std::fmt::Debug) -> String {
    let source = format!("{value:?}");
    let mut output = String::with_capacity(source.len() + 4);
    for (index, character) in source.chars().enumerate() {
        if index > 0 && character.is_ascii_uppercase() {
            output.push(' ');
        }
        output.push(character);
    }
    output
}

fn js_error(error: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_nested_card_art(card: &Value) {
        assert!(card.get("scryfallId").is_none());
        assert!(card.get("artist").is_none());

        let art = card["art"].as_object().expect("card art is an object");
        assert_eq!(art.len(), 2);
        assert!(art["scryfallId"].as_str().is_some_and(|id| id.len() == 36));
        assert!(
            art["artist"]
                .as_str()
                .is_some_and(|artist| !artist.is_empty())
        );
    }

    fn act_matching(game: &mut WebGame, predicate: impl Fn(&Action) -> bool) {
        let action_index = game
            .game
            .observe(game.human)
            .legal_actions
            .iter()
            .position(predicate)
            .expect("matching legal action");
        game.act(action_index).expect("legal action succeeds");
    }

    fn choices_targeting(target: Target) -> penta::CastChoices {
        penta::CastChoices::default().with_targets(vec![penta::TargetSelection::single(
            penta::TargetSlotId(0),
            target,
        )])
    }

    #[test]
    fn stack_signature_json_preserves_forms_modes_costs_and_target_slots() {
        let signature = penta::CastSignature::from_validated_choices(
            penta::SpellForm::Combined(vec![penta::CardPartId(0), penta::CardPartId(1)]),
            penta::CastChoices::new(penta::PlayOptionId(2))
                .with_modes(vec![penta::ModeId(3)])
                .with_costs(penta::CostConfiguration::new(
                    Some(penta::AlternativeCostId(4)),
                    vec![penta::AdditionalCostId(5)],
                ))
                .with_x(6)
                .with_targets(vec![penta::TargetSelection::new(
                    penta::TargetSlotId(7),
                    vec![
                        Target::Permanent(penta::GameObjectId(8)),
                        Target::Player(PlayerId::Two),
                        Target::Spell(penta::GameObjectId(9)),
                    ],
                )]),
        );

        assert_eq!(
            cast_signature_value(&signature, PlayerId::One),
            json!({
                "playOptionId": 2,
                "form": { "kind": "combined", "partIds": [0, 1] },
                "modeIds": [3],
                "alternativeCostId": 4,
                "additionalCostIds": [5],
                "x": 6,
                "targetSelections": [{
                    "slotId": 7,
                    "targetCardIds": [8],
                    "targetPlayers": ["opponent"],
                    "targetStackIds": [9],
                }],
            })
        );

        let part_signature = penta::CastSignature::from_validated_choices(
            penta::SpellForm::Part(penta::CardPartId::PRIMARY),
            penta::CastChoices::default(),
        );
        assert_eq!(
            cast_signature_value(&part_signature, PlayerId::One)["form"],
            json!({ "kind": "part", "partId": 0 })
        );
    }

    #[test]
    fn stack_presentation_uses_the_locked_split_card_form() {
        let catalog = card::catalog().expect("catalog builds");
        let turn_burn = catalog
            .get(penta::card::cards::TURN_BURN)
            .expect("Turn // Burn is cataloged");
        let burn_signature = penta::CastSignature::from_validated_choices(
            penta::SpellForm::Part(penta::CardPartId(1)),
            penta::CastChoices::new(penta::PlayOptionId(1)),
        );

        let burn = stack_card_presentation(Some(turn_burn), Some(&burn_signature));
        assert_eq!(burn.name, "Burn");
        assert_eq!(burn.kind, "instant");
        assert_eq!(burn.type_line, "Instant");
        assert_eq!(
            burn.implementation_status,
            penta::ImplementationStatus::MetadataOnly
        );
        assert_eq!(
            burn.mana_cost,
            Some(penta::ManaCost::colored(1, 0, 0, 0, 1, 0))
        );
        assert!(burn.rules_text.starts_with("Burn deals 2 damage"));
        assert_eq!((burn.power, burn.toughness), (None, None));

        let fused_signature = penta::CastSignature::from_validated_choices(
            penta::SpellForm::Combined(vec![penta::CardPartId::PRIMARY, penta::CardPartId(1)]),
            penta::CastChoices::new(penta::PlayOptionId(2)),
        );
        let fused = stack_card_presentation(Some(turn_burn), Some(&fused_signature));
        assert_eq!(fused.name, "Turn // Burn");
        assert_eq!(fused.kind, "instant");
        assert_eq!(fused.type_line, "Instant");
        assert_eq!(
            fused.mana_cost,
            Some(penta::ManaCost::colored(3, 0, 1, 0, 1, 0))
        );
        assert!(fused.rules_text.contains("Turn — Until end of turn"));
        assert!(fused.rules_text.contains("Burn — Burn deals 2 damage"));
        assert_eq!(
            fused.implementation_status,
            penta::ImplementationStatus::MetadataOnly
        );
    }

    #[test]
    fn visible_card_coverage_comes_from_ability_implementations() {
        let game = WebGame::new(
            "Briksza Naya Midrange",
            "Greer G/R Aggro",
            "Handcrafted",
            true,
            2,
            Some("isd-rtr-standard".into()),
        )
        .unwrap();
        let snapshot = game.snapshot_value(false);
        let hand = snapshot["human"]["hand"].as_array().expect("hand array");
        let find = |name: &str| {
            hand.iter()
                .find(|card| card["name"] == name)
                .unwrap_or_else(|| panic!("{name} is in the deterministic hand"))
        };

        assert_eq!(
            find("Avacyn's Pilgrim")["implementationStatus"],
            "complete",
            "the fully modeled creature and mana ability must not inherit its legacy play gate",
        );
        assert_eq!(
            find("Bonfire of the Damned")["implementationStatus"],
            "metadataOnly"
        );
        assert!(
            hand.iter().all(|card| card.get("metadataOnly").is_none()),
            "the WASM surface exposes the derived status, not its former boolean projection",
        );
    }

    #[test]
    fn missing_card_art_serializes_as_null() {
        assert_eq!(card_art_value(None), Value::Null);
    }

    #[test]
    fn hand_mana_cost_distinguishes_no_cost_from_printed_zero() {
        let catalog = card::catalog().expect("catalog builds");
        let mountain = catalog
            .get(penta::card::cards::MOUNTAIN)
            .expect("Mountain is cataloged");
        let mox = catalog
            .get(penta::card::cards::MOX_RUBY)
            .expect("Mox Ruby is cataloged");

        assert_eq!(hand_mana_cost_value(Some(mountain)), Value::Null);
        let zero = hand_mana_cost_value(Some(mox));
        assert!(zero.is_object());
        assert_eq!(zero["generic"], 0);
        assert_eq!(zero["red"], 0);
    }

    #[test]
    fn visible_cards_include_nested_scryfall_metadata() {
        let game = WebGame::new("Goblins", "Sligh", "Handcrafted", true, 9_394, None).unwrap();
        let snapshot = game.snapshot_value(false);
        let hand = snapshot["human"]["hand"].as_array().unwrap();

        assert_eq!(hand.len(), 7);
        hand.iter().for_each(assert_nested_card_art);
    }

    #[test]
    fn battlefield_and_stack_include_nested_scryfall_metadata() {
        let mut game =
            WebGame::new("Goblins", "Sligh", "Handcrafted", true, 3_756_436_840, None).unwrap();
        act_matching(&mut game, |action| matches!(action, Action::KeepHand));
        game.set_autopass(false).unwrap();
        act_matching(&mut game, |action| {
            matches!(action, Action::CastSpell { .. })
        });

        let stack_snapshot = game.snapshot_value(false);
        let stack = stack_snapshot["stack"].as_array().unwrap();
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0]["name"], "Black Lotus");
        assert_nested_card_art(&stack[0]);

        game.set_autopass(true).unwrap();
        let battlefield_snapshot = game.snapshot_value(false);
        let lotus = battlefield_snapshot["battlefield"]
            .as_array()
            .unwrap()
            .iter()
            .find(|card| card["name"] == "Black Lotus")
            .expect("Black Lotus resolved to the battlefield");
        assert_nested_card_art(lotus);
    }

    #[test]
    fn standard_visible_cards_include_nested_scryfall_metadata() {
        let game = WebGame::new(
            "Briksza Naya Midrange",
            "Greer G/R Aggro",
            "Handcrafted",
            true,
            2_013,
            Some("isd-rtr-standard".into()),
        )
        .unwrap();
        let snapshot = game.snapshot_value(false);
        let hand = snapshot["human"]["hand"].as_array().unwrap();
        let standard_cards = hand
            .iter()
            .filter(|card| {
                !matches!(
                    card["name"].as_str(),
                    Some("Plains" | "Island" | "Swamp" | "Mountain" | "Forest")
                )
            })
            .collect::<Vec<_>>();

        assert!(!standard_cards.is_empty());
        for card in standard_cards {
            assert_nested_card_art(card);
        }
    }

    #[test]
    fn blocker_actions_expose_the_attacker_as_their_board_target() {
        let attacker = CardInstanceId(7);
        let blocker = CardInstanceId(8);
        let action = Action::DeclareBlocker { blocker, attacker };
        assert_eq!(action_card(&action), Some(blocker));
        assert_eq!(action_target_card(&action), Some(attacker));
    }

    #[test]
    fn ability_actions_expose_their_stable_origins() {
        let action = Action::ActivateAbility {
            source: CardInstanceId(8),
            ability: penta::AbilityOrigin::Printed {
                definition: penta::poc::cards::MISHRA_S_FACTORY,
                part: penta::CardPartId::PRIMARY,
                ability: penta::AbilityId::PRIMARY,
            },
            targets: vec![
                penta::TargetSelection::single(
                    penta::TargetSlotId(3),
                    Target::Player(PlayerId::Two),
                ),
                penta::TargetSelection::new(
                    penta::TargetSlotId(7),
                    vec![
                        Target::Permanent(CardInstanceId(11)),
                        Target::Spell(CardInstanceId(12)),
                    ],
                ),
            ],
            sacrifice: None,
        };
        assert_eq!(
            action_ability_origin(&action),
            Some(json!({
                "kind": "printed",
                "definition": penta::poc::cards::MISHRA_S_FACTORY.0,
                "partId": 0,
                "abilityId": 0,
            }))
        );
        assert_eq!(action_target_card(&action), Some(CardInstanceId(11)));
        assert_eq!(
            action_target_player(&action, PlayerId::One),
            Some("opponent")
        );
        assert_eq!(action_target_stack(&action), Some(12));
        assert_eq!(action_target_cards(&action), vec![11]);
        assert_eq!(
            action_target_players(&action, PlayerId::One),
            vec!["opponent"]
        );
        assert_eq!(action_target_stacks(&action), vec![12]);

        let mana_action = Action::ActivateManaAbility {
            source: CardInstanceId(9),
            ability: penta::AbilityOrigin::IntrinsicBasicLand(penta::BasicLandType::Mountain),
            color: penta::ManaColor::Red,
        };
        assert_eq!(
            action_ability_origin(&mana_action),
            Some(json!({
                "kind": "intrinsicBasicLand",
                "landType": "mountain",
            }))
        );
    }

    #[test]
    fn human_main_one_stops_even_when_only_mana_actions_are_available() {
        let actions = [
            Action::Concede,
            Action::ActivateManaAbility {
                source: CardInstanceId(7),
                ability: penta::AbilityOrigin::IntrinsicBasicLand(penta::BasicLandType::Mountain),
                color: penta::ManaColor::Red,
            },
            Action::PassPriority,
        ];

        assert_eq!(
            automatic_human_action(
                Step::PrecombatMain,
                true,
                true,
                false,
                false,
                true,
                false,
                false,
                &actions,
            ),
            None
        );
        assert_eq!(
            automatic_human_action(
                Step::PrecombatMain,
                false,
                true,
                false,
                false,
                true,
                false,
                false,
                &actions,
            ),
            Some(Action::PassPriority),
            "an actionless opponent main phase can still auto-yield",
        );
    }

    #[test]
    fn a_real_game_action_still_stops_auto_pass() {
        let actions = [
            Action::Concede,
            Action::PlayLand {
                card: CardInstanceId(7),
                option: PlayOptionId::DEFAULT,
            },
            Action::PassPriority,
        ];

        assert_eq!(
            automatic_human_action(
                Step::PrecombatMain,
                true,
                true,
                false,
                false,
                true,
                false,
                false,
                &actions,
            ),
            None
        );
    }

    #[test]
    fn pregame_choices_do_not_enter_the_animation_queue() {
        assert!(!should_animate_action(&Action::KeepHand));
        assert!(!should_animate_action(&Action::TakeMulligan));
        assert!(!should_animate_action(&Action::BottomCards {
            cards: vec![CardInstanceId(4)],
        }));
        assert!(should_animate_action(&Action::PlayLand {
            card: CardInstanceId(4),
            option: PlayOptionId::DEFAULT,
        }));
    }

    #[test]
    fn routine_beginning_windows_auto_pass_on_either_turn() {
        let actions = [
            Action::Concede,
            Action::CastSpell {
                card: CardInstanceId(7),
                choices: choices_targeting(Target::Player(PlayerId::Two)),
                sacrifices: Vec::new(),
            },
            Action::PassPriority,
        ];

        assert_eq!(
            automatic_human_action(
                Step::Upkeep,
                true,
                true,
                false,
                false,
                true,
                false,
                false,
                &actions,
            ),
            Some(Action::PassPriority)
        );
        assert_eq!(
            automatic_human_action(
                Step::Draw,
                true,
                true,
                false,
                false,
                true,
                false,
                false,
                &actions,
            ),
            Some(Action::PassPriority)
        );
        assert_eq!(
            automatic_human_action(
                Step::End,
                true,
                true,
                false,
                false,
                true,
                false,
                false,
                &actions,
            ),
            Some(Action::PassPriority)
        );
        assert_eq!(
            automatic_human_action(
                Step::Upkeep,
                false,
                true,
                false,
                false,
                true,
                false,
                false,
                &actions,
            ),
            Some(Action::PassPriority),
            "routine opponent upkeep priority is hidden unless the player sets a stop",
        );
        assert_eq!(
            automatic_human_action(
                Step::Draw,
                false,
                true,
                false,
                false,
                true,
                false,
                false,
                &actions,
            ),
            Some(Action::PassPriority),
            "routine opponent draw-step priority is hidden unless the player sets a stop",
        );
        assert_eq!(
            automatic_human_action(
                Step::End,
                true,
                true,
                false,
                false,
                true,
                false,
                true,
                &actions,
            ),
            None,
            "smart priority preserves floating mana in the human's end step",
        );
    }

    #[test]
    fn empty_and_unblocked_combat_steps_auto_pass() {
        let actions = [
            Action::Concede,
            Action::CastSpell {
                card: CardInstanceId(7),
                choices: choices_targeting(Target::Player(PlayerId::Two)),
                sacrifices: Vec::new(),
            },
            Action::PassPriority,
        ];

        assert_eq!(
            automatic_human_action(
                Step::BeginningOfCombat,
                true,
                true,
                false,
                false,
                true,
                false,
                false,
                &actions,
            ),
            Some(Action::PassPriority)
        );
        assert_eq!(
            automatic_human_action(
                Step::CombatDamage,
                true,
                true,
                false,
                false,
                true,
                false,
                false,
                &actions,
            ),
            Some(Action::PassPriority)
        );
        assert_eq!(
            automatic_human_action(
                Step::CombatDamage,
                true,
                true,
                true,
                false,
                true,
                false,
                false,
                &actions,
            ),
            Some(Action::PassPriority),
            "an unblocked attack runs through combat damage without extra clicks",
        );

        assert_eq!(
            automatic_human_action_with_blockers(
                Step::DeclareBlockers,
                true,
                true,
                true,
                true,
                false,
                true,
                false,
                false,
                &actions,
            ),
            None,
            "a declared blocker interrupts smooth combat",
        );
    }

    #[test]
    fn a_pump_ability_holds_combat_open_only_while_it_matters() {
        let actions = [
            Action::Concede,
            Action::ActivateAbility {
                source: CardInstanceId(8),
                ability: penta::AbilityOrigin::Printed {
                    definition: penta::CardDefinitionId(0),
                    part: penta::CardPartId::PRIMARY,
                    ability: penta::AbilityId::PRIMARY,
                },
                targets: vec![penta::TargetSelection::single(
                    penta::TargetSlotId(0),
                    Target::Permanent(CardInstanceId(9)),
                )],
                sacrifice: None,
            },
            Action::PassPriority,
        ];
        assert_eq!(
            automatic_human_action(
                Step::DeclareAttackers,
                true,
                true,
                true,
                false,
                true,
                false,
                false,
                &actions,
            ),
            None,
            "a pump ability keeps priority while it can still change the attack",
        );
        assert_eq!(
            automatic_human_action(
                Step::CombatDamage,
                true,
                true,
                true,
                false,
                true,
                false,
                false,
                &actions,
            ),
            Some(Action::PassPriority),
            "but damage is already dealt by the time priority comes back",
        );
    }

    #[test]
    fn a_combat_stop_interrupts_an_unblocked_attack() {
        let actions = [Action::Concede, Action::PassPriority];
        assert_eq!(
            automatic_human_action_with_blockers(
                Step::CombatDamage,
                true,
                true,
                true,
                false,
                true,
                true,
                false,
                false,
                &actions,
            ),
            None
        );
    }

    #[test]
    fn the_opponents_combat_and_second_main_yield_but_their_end_step_does_not() {
        // A castable instant is exactly what makes these windows "meaningful",
        // and exactly why the end step has to stay.
        let actions = [
            Action::Concede,
            Action::CastSpell {
                card: CardInstanceId(7),
                choices: choices_targeting(Target::Player(PlayerId::Two)),
                sacrifices: Vec::new(),
            },
            Action::PassPriority,
        ];
        let on_their_turn = |step| {
            automatic_human_action(
                step, false, true, false, false, true, false, false, &actions,
            )
        };

        for step in [
            Step::BeginningOfCombat,
            Step::DeclareAttackers,
            Step::DeclareBlockers,
            Step::EndOfCombat,
            Step::PostcombatMain,
        ] {
            assert_eq!(
                on_their_turn(step),
                Some(Action::PassPriority),
                "an unattacked {step:?} on the opponent's turn should yield",
            );
        }
        assert_eq!(
            on_their_turn(Step::End),
            None,
            "the opponent's end step is where instants get cast",
        );
    }

    #[test]
    fn a_declared_attack_still_stops_on_the_opponents_turn() {
        let actions = [
            Action::Concede,
            Action::DeclareBlocker {
                blocker: CardInstanceId(7),
                attacker: CardInstanceId(8),
            },
            Action::PassPriority,
        ];
        assert_eq!(
            automatic_human_action(
                Step::DeclareBlockers,
                false,
                true,
                true,
                false,
                true,
                false,
                false,
                &actions,
            ),
            None,
            "blocks have to be declared against a real attack",
        );
    }

    #[test]
    fn declaring_the_last_attacker_does_not_commit_the_attack() {
        // Running out of creatures to declare is not the same as being done,
        // so the browser still gets to show the confirm and cancel pair.
        let actions = [Action::Concede, Action::FinishDeclaringAttackers];
        assert_eq!(
            automatic_human_action(
                Step::DeclareAttackers,
                true,
                true,
                true,
                false,
                true,
                false,
                false,
                &actions,
            ),
            None,
        );
        assert_eq!(
            automatic_human_action(
                Step::DeclareAttackers,
                true,
                true,
                false,
                false,
                true,
                false,
                false,
                &actions,
            ),
            Some(Action::FinishDeclaringAttackers),
            "with nothing declared there is no attack to confirm",
        );
    }

    #[test]
    fn a_phase_stop_blocks_the_ui_auto_pass() {
        let actions = [Action::Concede, Action::PassPriority];
        assert_eq!(
            automatic_human_action(
                Step::Upkeep,
                true,
                true,
                false,
                true,
                true,
                false,
                false,
                &actions,
            ),
            None
        );
    }

    #[test]
    fn no_attackers_skip_the_rest_of_combat_even_with_a_combat_stop() {
        let actions = [Action::Concede, Action::PassPriority];
        assert_eq!(
            automatic_human_action(
                Step::CombatDamage,
                true,
                true,
                false,
                true,
                true,
                false,
                false,
                &actions,
            ),
            Some(Action::PassPriority)
        );
        assert_eq!(
            automatic_human_action(
                Step::CombatDamage,
                true,
                true,
                false,
                true,
                false,
                false,
                false,
                &actions,
            ),
            None,
            "turning auto-pass off still exposes the empty combat window",
        );
    }

    #[test]
    fn autopass_yields_when_only_human_objects_are_on_the_stack() {
        let actions = [
            Action::Concede,
            Action::CastSpell {
                card: CardInstanceId(7),
                choices: choices_targeting(Target::Player(PlayerId::Two)),
                sacrifices: Vec::new(),
            },
            Action::PassPriority,
        ];
        assert_eq!(
            automatic_human_action(
                Step::PrecombatMain,
                true,
                false,
                false,
                false,
                true,
                true,
                false,
                &actions,
            ),
            Some(Action::PassPriority)
        );
        assert_eq!(
            automatic_human_action(
                Step::PrecombatMain,
                true,
                false,
                false,
                false,
                false,
                true,
                false,
                &actions,
            ),
            None
        );
    }
}
