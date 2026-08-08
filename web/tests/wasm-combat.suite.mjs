import assert from "node:assert/strict";
import test from "node:test";

import { initializeWasm, WebGame } from "./wasm-test-support.mjs";

test("the web facade skips combat when no attackers exist", async () => {
  await initializeWasm();

  const game = new WebGame("Goblins", "Sligh", "Handcrafted", true, 5);
  let state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Keep this hand").index);
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Play Mountain").index);
  state = JSON.parse(game.state_json());
  game.set_phase_stop("Combat", true);
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.kind === "pass").index);
  state = JSON.parse(game.state_json());

  assert.equal(state.step, "Beginning Of Combat");
  const beforeCombat = state;
  game.act(state.actions.find((action) => action.kind === "pass").index);
  state = JSON.parse(game.state_json());

  // With no creatures there is no combat to react to, so the second main has
  // nothing to offer either and the pass carries the turn out.
  assert.ok(
    state.gameTurn > beforeCombat.gameTurn,
    `the turn ended instead of idling in ${state.step}`,
  );

  game.free();
});

test("attack all declares every currently legal attacker", async () => {
  await initializeWasm();

  const game = new WebGame("Goblins", "Goblins", "Random", true, 5);
  let state;
  for (let step = 0; step < 20; step += 1) {
    state = JSON.parse(game.state_json());
    if (
      state.step === "Declare Attackers" &&
      state.actions.some((action) => action.label.startsWith("Attack with "))
    ) {
      break;
    }
    const next =
      state.actions.find((action) => action.label === "Keep this hand") ??
      state.actions.find((action) => action.label === "Play Mountain") ??
      state.actions.find((action) => action.label.startsWith("Cast Goblins of the Flarg")) ??
      state.actions.find((action) => action.kind === "pass") ??
      state.actions.find((action) => /^(Don't|Leave) /.test(action.label));
    assert.ok(next, `the attack-all fixture can advance from ${state.step}`);
    game.act(next.index);
  }

  const attackOptions = state.actions.filter((action) =>
    action.label.startsWith("Attack with "),
  );
  assert.ok(attackOptions.length > 0);
  game.set_phase_stop("Combat", true);
  game.attack_all();
  state = JSON.parse(game.state_json());
  assert.equal(
    state.battlefield.filter((card) => card.owner === "human" && card.attacking).length,
    attackOptions.length,
  );
  assert.ok(
    !state.actions.some((action) => action.label.startsWith("Attack with ")),
    "attacker declaration is finished by the bulk action",
  );
  game.free();
});

test("the attacker button counts the attack instead of naming the step", async () => {
  await initializeWasm();

  const game = new WebGame("Goblins", "The Deck", "Handcrafted", true, 1);
  const seen = new Set();
  for (let turn = 0; turn < 500; turn++) {
    const state = JSON.parse(game.state_json());
    if (state.result) break;
    for (const action of state.actions) {
      assert.notEqual(action.label, "Finish attacking", "the step name is gone");
      if (/^(No attacks|Attack with )/.test(action.label)) seen.add(action.label);
    }
    if (state.decision) {
      game.choose_decision(
        state.decision.id,
        JSON.stringify(
          state.decision.options
            .slice(0, state.decision.minimum)
            .map((option) => option.id),
        ),
      );
      continue;
    }
    const actions = state.actions.filter((action) => action.kind !== "danger");
    const next =
      actions.find((action) => action.label === "Keep this hand") ??
      actions.find((action) => action.label.startsWith("Play ")) ??
      actions.find((action) => action.label.startsWith("Cast Goblin")) ??
      actions.find((action) => /^Attack with \D/.test(action.label)) ??
      actions.find((action) => action.label.startsWith("Block ")) ??
      actions.find((action) => action.label.startsWith("Assign ")) ??
      actions.find((action) => action.label.startsWith("Discard ")) ??
      actions.find((action) => action.kind === "pass") ??
      actions[0];
    if (!next) break;
    game.act(next.index);
  }

  assert.ok(seen.has("No attacks"), `saw: ${[...seen].join(", ")}`);
  assert.ok(seen.has("Attack with 1 creature"), `saw: ${[...seen].join(", ")}`);
  assert.ok(
    [...seen].some((label) => /^Attack with [2-9] creatures$/.test(label)),
    `plural form appears: ${[...seen].join(", ")}`,
  );

  game.free();
});

test("combat damage is only asked about when it is a real choice", async () => {
  await initializeWasm();

  const advance = (game, stopWhen) => {
    for (let turn = 0; turn < 700; turn++) {
      const state = JSON.parse(game.state_json());
      if (state.result) return null;
      const found = stopWhen(state);
      if (found) return found;
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
        actions.find((action) => action.label === "Keep this hand") ??
        actions.find((action) => action.label.startsWith("Play ")) ??
        actions.find((action) => /^Attack with \D/.test(action.label)) ??
        actions.find((action) => action.label.startsWith("Block ")) ??
        actions.find((action) => action.label.startsWith("Cast ")) ??
        actions.find((action) => action.label.startsWith("Discard ")) ??
        actions.find((action) => action.kind === "pass") ??
        actions[0];
      if (!next) return null;
      game.act(next.index);
    }
    return null;
  };

  // A lone blocker is never a question, so a trampler facing one resolves on
  // its own instead of listing every way to waste damage on the blocker.
  const solo = new WebGame("Goblins", "The Deck", "Handcrafted", true, 9394);
  const prompted = advance(solo, (state) => {
    const asks = state.actions.filter((action) => action.combatDamageAttacker != null);
    if (!asks.length) return null;
    const attacker = state.battlefield.find((card) => card.id === asks[0].combatDamageAttacker);
    const blockers = state.battlefield.filter((card) => card.blocking === attacker?.id);
    return blockers.length > 1 ? null : { attacker: attacker?.name, blockers: blockers.length };
  });
  assert.equal(prompted, null, `a single blocker still prompted: ${JSON.stringify(prompted)}`);
  solo.free();

  // Splitting between several blockers is a real decision and stays asked.
  const split = new WebGame("GR Aggro", "Robots", "Handcrafted", true, 1);
  const ask = advance(split, (state) => {
    const asks = state.actions.filter((action) => action.combatDamageAttacker != null);
    return asks.length ? { asks, state } : null;
  });
  assert.ok(ask, "the seeded game reaches a multi-blocker assignment");
  const attacker = ask.state.battlefield.find(
    (card) => card.id === ask.asks[0].combatDamageAttacker,
  );
  assert.ok(attacker, "the browser can name the attacker being assigned");
  assert.ok(
    ask.state.battlefield.filter((card) => card.blocking === attacker.id).length > 1,
    "it is only asked when several blockers share the damage",
  );
  for (const action of ask.asks) {
    assert.ok(
      /^\d+ to /.test(action.label),
      `the option says where damage lands: "${action.label}"`,
    );
    assert.ok(!/ 0 to /.test(action.label), `recipients taking nothing are left out: "${action.label}"`);
  }
  split.free();
});

test("declaring attackers always offers a confirm and a way back", async () => {
  await initializeWasm();

  const game = new WebGame("Goblins", "The Deck", "Handcrafted", true, 9394);
  const state = () => JSON.parse(game.state_json());
  const play = (predicate) => {
    for (let turn = 0; turn < 500; turn++) {
      const current = state();
      if (current.result) return null;
      if (predicate(current)) return current;
      const actions = current.actions.filter((action) => action.kind !== "danger");
      const next =
        actions.find((action) => action.label === "Keep this hand") ??
        actions.find((action) => action.label.startsWith("Play ")) ??
        actions.find((action) => action.label.startsWith("Cast Goblin")) ??
        actions.find((action) => action.label.startsWith("Discard ")) ??
        actions.find((action) => action.kind === "pass") ??
        actions[0];
      if (!next) return null;
      game.act(next.index);
    }
    return null;
  };

  const declaring = play(
    (current) =>
      current.step === "Declare Attackers" &&
      current.actions.some((action) => /^Attack with \D/.test(action.label)),
  );
  assert.ok(declaring, "the seeded game reaches attacker declaration");
  assert.equal(declaring.canCancelAttackers, false, "nothing to take back yet");
  assert.ok(
    declaring.actions.some((action) => action.label === "No attacks"),
    "with nothing declared the commit reads as declining",
  );

  // Declare every attacker on offer; the last one must not commit the attack.
  let declared = 0;
  for (;;) {
    const current = state();
    const attack = current.actions.find((action) => /^Attack with \D/.test(action.label));
    if (!attack) break;
    game.act(attack.index);
    declared += 1;
    assert.equal(state().step, "Declare Attackers", "declaring never leaves the step on its own");
  }
  assert.ok(declared > 0);

  const committed = state();
  assert.equal(committed.canCancelAttackers, true, "the attack can still be taken back");
  assert.equal(
    committed.battlefield.filter((card) => card.owner === "human" && card.attacking).length,
    declared,
  );
  assert.ok(
    committed.actions.some((action) => action.label === `Attack with ${declared} creature${declared === 1 ? "" : "s"}`),
    `the confirm counts the attack: ${committed.actions.map((a) => a.label).join(", ")}`,
  );

  // Cancelling restores the board exactly as it was before the first declaration.
  game.cancel_attackers();
  const reverted = state();
  assert.equal(reverted.canCancelAttackers, false);
  assert.equal(
    reverted.battlefield.filter((card) => card.attacking).length,
    0,
    "every attacker is taken back",
  );
  assert.equal(
    reverted.actions.filter((action) => /^Attack with \D/.test(action.label)).length,
    declared,
    "and every creature can be declared again",
  );
  assert.throws(() => game.cancel_attackers(), /no declared attackers/);

  game.free();
});

test("declining a block runs to their end step; blocking keeps the damage stop", async () => {
  await initializeWasm();

  // Defending is meant to be one decision, not four: block or don't, and if
  // you don't, the next thing worth stopping for is their end step.
  const defend = (block, deck = "White Weenie", seed = 12) => {
    const game = new WebGame(deck, "Goblins", "Handcrafted", false, seed);
    const stops = [];
    for (let step = 0; step < 400; step += 1) {
      const state = JSON.parse(game.state_json());
      if (state.result) break;
      if (state.decision) {
        const wanted = Math.max(state.decision.minimum, 1);
        game.choose_decision(
          state.decision.id,
          JSON.stringify(state.decision.options.slice(0, wanted).map((option) => option.id)),
        );
        continue;
      }
      const actions = state.actions.filter((action) => action.kind !== "danger");
      const blocks = actions.filter((action) => action.label.startsWith("Block "));
      if (state.active !== "You" && state.battlefield.some((card) => card.attacking)) {
        stops.push({
          step: state.step,
          pass: state.passLabel,
          canBlock: blocks.length > 0,
        });
      }
      const next =
        (block && blocks.length ? blocks[0] : null) ??
        actions.find((action) => action.label === "Keep this hand") ??
        actions.find((action) => action.label.startsWith("Play ")) ??
        actions.find((action) => action.label.startsWith("Cast ")) ??
        actions.find((action) => action.kind === "pass") ??
        actions[0];
      if (!next) break;
      game.act(next.index);
    }
    game.free();
    return stops;
  };

  const declined = defend(false);
  assert.ok(
    declined.some((stop) => stop.canBlock),
    "the block decision itself still stops",
  );
  assert.ok(
    !declined.some((stop) => stop.pass === "Go to damage"),
    `no damage stop once nothing is blocking; got ${JSON.stringify(declined)}`,
  );
  assert.ok(
    declined.every((stop) => stop.pass !== "Go to their end step"),
    `their end step is where the yield lands, not a second button; got ${JSON.stringify(declined)}`,
  );

  const blocked = defend(true);
  assert.ok(
    blocked.some((stop) => stop.pass === "Go to damage"),
    `a declared block keeps its pre-damage window; got ${JSON.stringify(blocked)}`,
  );
  assert.ok(
    blocked.every((stop) => stop.step !== "Combat Damage" && stop.step !== "End Of Combat"),
    `damage is history by the time priority returns; got ${JSON.stringify(blocked)}`,
  );

  // With no creature able to block, the pass is the decision, so it says so
  // instead of promising a block step that will not happen.
  const creatureless = defend(false, "The Deck", 77);
  assert.ok(
    creatureless.some((stop) => stop.pass === "No blocks"),
    `taking an attack unblocked is named as such; got ${JSON.stringify(creatureless)}`,
  );
  assert.ok(
    creatureless.every((stop) => stop.pass !== "Go to damage"),
    `nothing to block with means no damage stop; got ${JSON.stringify(creatureless)}`,
  );
});
