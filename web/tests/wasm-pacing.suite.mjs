import assert from "node:assert/strict";
import test from "node:test";

import { initializeWasm, WebGame } from "./wasm-test-support.mjs";

test("opponent mana taps are grouped with the spell they pay for", async () => {
  await initializeWasm();

  const game = new WebGame("Goblins", "Sligh", "Handcrafted", false, 9394);
  const opening = JSON.parse(game.state_json());
  const keep = opening.actions.find((action) => action.label === "Keep this hand");
  game.act(keep.index);

  const afterKeep = JSON.parse(game.state_json());
  const paidAction = afterKeep.opponentActions.find(
    (action) => action.manaSources?.length > 0,
  );
  assert.ok(paidAction, "a paid spell or ability includes its tapped mana sources");
  assert.match(paidAction.label, /^(Cast|Activate) /);
  assert.ok(
    afterKeep.opponentActions.every((action) => action.kind !== "mana"),
    "there is no separate mana animation",
  );
  assert.ok(
    afterKeep.opponentActions.length > 1,
    "the deterministic turn provides a multi-action animation sequence",
  );
  assert.notDeepEqual(
    afterKeep.opponentActions[0].state.battlefield,
    afterKeep.battlefield,
    "the first animation does not expose the final battlefield",
  );
  for (const source of paidAction.manaSources) {
    assert.equal(
      paidAction.state.battlefield.find(
        (card) => card.owner === "opponent" && card.name === source,
      )?.tapped,
      true,
      `${source} taps in the same snapshot as the paid action`,
    );
  }

  game.free();
});
test("phase stops override smooth UI auto-passing without changing engine steps", async () => {
  await initializeWasm();

  const game = new WebGame("Goblins", "Sligh", "Handcrafted", true, 5);
  game.set_phase_stop("Beginning", true);
  let state = JSON.parse(game.state_json());
  assert.deepEqual(state.phaseStops, ["Beginning"]);
  game.act(state.actions.find((action) => action.label === "Keep this hand").index);
  state = JSON.parse(game.state_json());
  assert.equal(state.step, "Upkeep");
  assert.ok(state.actions.some((action) => action.label === "Pass priority"));

  game.set_phase_stop("Beginning", false);
  state = JSON.parse(game.state_json());
  assert.deepEqual(state.phaseStops, []);
  game.free();
});

test("the pass button label reports the engine's real auto-pass destination", async () => {
  await initializeWasm();

  const game = new WebGame("Goblins", "Sligh", "Handcrafted", true, 9394);
  const currentState = () => JSON.parse(game.state_json());
  const pass = (state) =>
    game.act(state.actions.find((action) => action.label === "Pass priority").index);

  let state = currentState();
  game.act(state.actions.find((action) => action.label === "Keep this hand").index);
  state = currentState();
  assert.equal(state.step, "Precombat Main");
  assert.equal(
    state.passLabel,
    "End turn",
    "an empty board has no combat and nothing to do after it, so the pass is the whole turn",
  );

  game.set_phase_stop("Ending", true);
  state = currentState();
  assert.equal(state.passLabel, "Go to end step", "a stop puts the end step back");
  pass(state);
  state = currentState();
  assert.equal(state.step, "End");

  game.set_phase_stop("Ending", false);
  state = currentState();
  assert.equal(state.passLabel, "End turn");
  pass(state);
  state = currentState();
  assert.equal(state.step, "Precombat Main");
  assert.equal(state.active, "You");
  assert.equal(state.turn, 2, "the promised pass really ends the turn");

  game.set_autopass(false);
  state = currentState();
  assert.equal(
    state.passLabel,
    "Go to attacks",
    "with auto-pass off the label only promises the next window",
  );
  pass(state);
  state = currentState();
  assert.equal(state.step, "Beginning Of Combat");

  game.free();
});

test("mulligans are not a turn, and the draw happens in the beginning phase", async () => {
  await initializeWasm();

  const game = new WebGame("Sligh", "The Deck", "Handcrafted", true, 4242);
  const opening = JSON.parse(game.state_json());
  assert.equal(opening.pregame, true, "choosing an opening hand is not turn one");
  assert.ok(
    opening.actions.some((action) => action.label === "Keep this hand"),
    "the opening decision is the mulligan",
  );

  const keep = opening.actions.find((action) => action.label === "Keep this hand");
  game.act(keep.index);
  const started = JSON.parse(game.state_json());
  assert.equal(started.pregame, false, "keeping starts the game");

  // Every draw beat has to be labelled with the step the phase strip shows,
  // or the card animates into a hand the board says is already in main one.
  let drawBeats = 0;
  for (let step = 0; step < 200; step += 1) {
    const state = JSON.parse(game.state_json());
    if (state.result) break;
    for (const beat of state.opponentActions ?? []) {
      if (beat.kind !== "draw") continue;
      drawBeats += 1;
      assert.equal(beat.state.step, "Draw", "a draw beat is held in the draw step");
      assert.equal(beat.state.pregame, false);
    }
    if (state.decision) {
      const wanted = Math.max(state.decision.minimum, 1);
      game.choose_decision(
        state.decision.id,
        JSON.stringify(state.decision.options.slice(0, wanted).map((option) => option.id)),
      );
      continue;
    }
    const actions = state.actions.filter((action) => action.kind !== "danger");
    const next =
      actions.find((action) => action.label.startsWith("Play ")) ??
      actions.find((action) => action.kind === "pass") ??
      actions[0];
    if (!next) break;
    game.act(next.index);
  }
  assert.ok(drawBeats >= 4, `every turn's draw gets a beat, saw ${drawBeats}`);

  game.free();
});
