import assert from "node:assert/strict";
import test from "node:test";

import { initializeWasm, WebGame } from "./wasm-test-support.mjs";

test("auto-pass declines an unavailable Chain Lightning copy", async () => {
  await initializeWasm();

  const game = new WebGame("Goblins", "Goblins", "Handcrafted", true, 5);
  let state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Keep this hand").index);
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Play Mountain").index);
  state = JSON.parse(game.state_json());
  game.act(
    state.actions.find(
      (action) => action.label === "Cast Goblins of the Flarg",
    ).index,
  );
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Pass priority").index);
  state = JSON.parse(game.state_json());

  assert.equal(state.turn, 2);
  assert.equal(state.step, "Precombat Main");
  assert.ok(
    !state.actions.some((action) => action.label === "Don't copy Chain Lightning"),
    "an impossible copy choice does not interrupt the player",
  );
  assert.ok(
    state.events.some((event) => event.includes("Opponent cast Chain Lightning")),
  );
  assert.ok(state.events.some((event) => event === "Turn 2 · your turn"));

  game.free();
});
test("player-targeted spells identify a clickable player target", async () => {
  await initializeWasm();

  const game = new WebGame("Goblins", "Sligh", "Handcrafted", true, 5);
  let state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Keep this hand").index);
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Play Mountain").index);
  state = JSON.parse(game.state_json());

  const bolt = state.actions.find(
    (action) =>
      action.label.startsWith("Cast Lightning Bolt") &&
      action.targetPlayer === "opponent",
  );
  assert.ok(bolt, "Lightning Bolt exposes the opponent as its board target");
  assert.equal(bolt.targetCardId, null);
  assert.equal(bolt.targetStackId, null);

  game.free();
});

test("casting a spell automatically taps available mana sources", async () => {
  await initializeWasm();

  const game = new WebGame("Artifacts", "Sligh", "Handcrafted", true, 16);
  let state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Keep this hand").index);

  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Cast Mox Ruby").index);

  state = JSON.parse(game.state_json());
  const castVise = state.actions.find((action) =>
    action.label.startsWith("Cast Black Vise"),
  );
  assert.ok(castVise, "Black Vise is castable before manually tapping Mox Ruby");
  assert.equal(castVise.paymentAction, true);
  assert.deepEqual(
    castVise.manaSourceIds,
    [state.battlefield.find((card) => card.name === "Mox Ruby").id],
    "the browser can preview the exact automatic mana tap before committing",
  );
  game.act(castVise.index);

  state = JSON.parse(game.state_json());
  const mox = state.battlefield.find(
    (card) => card.owner === "human" && card.name === "Mox Ruby",
  );
  assert.equal(mox?.tapped, true);
  assert.equal(state.human.mana.red, 0);
  assert.equal(state.autopassEnabled, true);
  assert.equal(state.stack.length, 0, "your spell resolves without another UI priority prompt");

  game.free();
});

test("turning auto-pass off exposes priority over your own spell", async () => {
  await initializeWasm();

  const game = new WebGame("Artifacts", "Goblins", "Handcrafted", true, 16);
  let state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Keep this hand").index);
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Cast Mox Ruby").index);
  state = JSON.parse(game.state_json());
  game.set_autopass(false);
  state = JSON.parse(game.state_json());
  game.act(
    state.actions.find((action) => action.label.startsWith("Cast Black Vise")).index,
  );

  state = JSON.parse(game.state_json());
  assert.equal(state.autopassEnabled, false);
  assert.equal(state.stack[0]?.name, "Black Vise");
  assert.ok(state.actions.some((action) => action.label === "Pass priority"));

  game.set_autopass(true);
  state = JSON.parse(game.state_json());
  assert.equal(state.autopassEnabled, true);
  assert.equal(state.stack.length, 0);
  assert.ok(state.battlefield.some((card) => card.name === "Black Vise"));
  game.free();
});

test("targeted permanent actions identify their clickable battlefield target", async () => {
  await initializeWasm();

  const game = new WebGame("Goblins", "Sligh", "Handcrafted", true, 1138831559);
  let state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Keep this hand").index);
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Play Mountain").index);

  for (let step = 0; step < 30; step += 1) {
    state = JSON.parse(game.state_json());
    if (state.actions.some((action) => action.label === "Play Strip Mine")) {
      break;
    }
    const pass =
      state.actions.find((action) => action.kind === "pass") ??
      state.actions.find((action) =>
        /^(Don't|Leave) /.test(action.label),
      );
    assert.ok(
      pass,
      `the human can yield each intervening priority window: ${JSON.stringify({
        turn: state.turn,
        step: state.step,
        actions: state.actions,
      })}`,
    );
    game.act(pass.index);
  }

  state = JSON.parse(game.state_json());
  const playStrip = state.actions.find((action) => action.label === "Play Strip Mine");
  assert.ok(playStrip, "the deterministic hand can play Strip Mine on turn two");
  game.act(playStrip.index);

  state = JSON.parse(game.state_json());
  const stripMana = state.actions.find(
    (action) => action.label === "Tap Strip Mine for Colorless mana",
  );
  assert.ok(stripMana, "Strip Mine remains available as a colorless mana source");
  assert.equal(stripMana.manaAbility, true);
  const stripAction = state.actions.find((action) => {
    // The label describes the effect, not the card: "Destroy Plains with Strip Mine".
    if (!/^Destroy .* with Strip Mine$/.test(action.label)) return false;
    assert.equal(action.abilitySummary, "Destroy a land");
    return state.battlefield.some(
      (card) => card.id === action.targetCardId && card.owner === "opponent",
    );
  });
  assert.ok(stripAction, "Strip Mine exposes a targeted activation");
  const target = state.battlefield.find(
    (card) => card.id === stripAction.targetCardId,
  );
  assert.equal(target?.owner, "opponent");
  assert.equal(target?.kind, "land");

  game.free();
});

test("Mishra's Factory offers both modes and manual mana can be undone", async () => {
  await initializeWasm();

  const game = new WebGame("Artifacts", "Sligh", "Handcrafted", true, 0);
  let state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Keep this hand").index);
  state = JSON.parse(game.state_json());
  game.act(
    state.actions.find((action) => action.label === "Cast Mox Sapphire").index,
  );
  state = JSON.parse(game.state_json());
  game.act(
    state.actions.find((action) => action.label.startsWith("Play Mishra's Factory"))
      .index,
  );
  state = JSON.parse(game.state_json());

  const factory = state.battlefield.find(
    (card) => card.owner === "human" && card.name === "Mishra's Factory",
  );
  const factoryActions = state.actions.filter(
    (action) => action.cardId === factory.id,
  );
  const mox = state.battlefield.find(
    (card) => card.owner === "human" && card.name === "Mox Sapphire",
  );
  assert.deepEqual(
    factoryActions.map((action) => action.label),
    [
      "Tap Mishra's Factory for Colorless mana",
      "Make Mishra's Factory a 2/2 creature",
    ],
  );
  assert.deepEqual(
    factoryActions.find((action) => !action.manaAbility).manaSourceIds,
    [mox.id],
    "auto-pay preserves the Factory when another source can animate it",
  );

  game.act(factoryActions.find((action) => action.manaAbility).index);
  state = JSON.parse(game.state_json());
  assert.equal(state.canUndoMana, true);
  assert.equal(
    state.battlefield.find((card) => card.id === factory.id).tapped,
    true,
  );
  assert.equal(state.human.mana.colorless, 1);

  game.undo_mana();
  state = JSON.parse(game.state_json());
  assert.equal(state.canUndoMana, false);
  assert.equal(
    state.battlefield.find((card) => card.id === factory.id).tapped,
    false,
  );
  assert.equal(state.human.mana.colorless, 0);

  const animate = state.actions.find(
    (action) => action.label === "Make Mishra's Factory a 2/2 creature",
  );
  game.set_phase_stop("Main 1", true);
  game.act(animate.index);
  state = JSON.parse(game.state_json());
  const animatedFactory = state.battlefield.find(
    (card) => card.id === factory.id,
  );
  assert.equal(animatedFactory.kind, "artifactcreature");
  assert.equal(animatedFactory.isLand, true);
  assert.equal(animatedFactory.power, 2);
  assert.equal(animatedFactory.toughness, 2);

  game.free();
});

test("X spells expose explicit affordable values to the browser", async () => {
  await initializeWasm();

  const game = new WebGame("The Deck", "Goblins", "Random", true, 654);
  let state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Keep this hand").index);
  state = JSON.parse(game.state_json());
  game.act(
    state.actions.find((action) => action.label === "Play Mishra's Factory").index,
  );
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Cast Black Lotus").index);
  state = JSON.parse(game.state_json());

  const fireballs = state.actions.filter(
    (action) => action.spellAction && action.label.startsWith("Cast Fireball"),
  );
  assert.deepEqual(
    [...new Set(fireballs.map((action) => action.x))],
    [0, 1, 2, 3],
    "the UI can present every affordable value of X",
  );
  const twoTargetFireball = fireballs.find(
    (action) =>
      action.x === 2 &&
      action.targetCount === 2 &&
      action.targetPlayers.includes("human") &&
      action.targetPlayers.includes("opponent"),
  );
  assert.ok(twoTargetFireball, "the UI receives complete multi-target Fireball actions");
  assert.deepEqual(twoTargetFireball.targetCardIds, []);
  const fireballForThree = fireballs.find(
    (action) => action.x === 3 && action.targetPlayer === "opponent",
  );
  assert.ok(fireballForThree);
  game.act(fireballForThree.index);
  state = JSON.parse(game.state_json());
  assert.equal(state.opponent.life, 17);

  game.free();
});

test("Orcish Mechanics exposes creature targets and distinct artifact costs", async () => {
  await initializeWasm();

  const game = new WebGame("Artifacts", "Sligh", "Handcrafted", true, 7);
  let state;
  let mechanics;
  let creatureTargets;
  for (let step = 0; step < 160; step += 1) {
    state = JSON.parse(game.state_json());
    mechanics = state.battlefield.find(
      (card) =>
        card.owner === "human" &&
        card.name === "Orcish Mechanics" &&
        !card.tapped,
    );
    creatureTargets = mechanics
      ? state.actions.filter(
          (action) =>
            action.cardId === mechanics.id &&
            action.targetCardId != null &&
            state.battlefield.some(
              (card) =>
                card.id === action.targetCardId &&
                card.owner === "opponent" &&
                card.kind.includes("creature"),
            ),
        )
      : [];
    if (creatureTargets.length >= 2) break;

    const actions = state.actions;
    const next =
      actions.find((action) => action.label === "Keep this hand") ??
      actions.find((action) => action.label.startsWith("Cast Mox ")) ??
      actions.find((action) => action.label === "Cast Black Lotus") ??
      actions.find((action) => action.label === "Play Mountain") ??
      actions.find((action) => action.label.startsWith("Play Mishra")) ??
      actions.find((action) => action.label.startsWith("Play Strip")) ??
      actions.find((action) => action.label.startsWith("Cast Orcish Mechanics")) ??
      actions.find((action) => action.label.startsWith("Cast Sol Ring")) ??
      actions.find((action) => action.label.startsWith("Cast Black Vise")) ??
      actions.find((action) => action.label.startsWith("Cast Copper Tablet")) ??
      actions.find((action) => action.label.startsWith("Cast Ankh")) ??
      actions.find((action) => /^(Don't|Leave) /.test(action.label)) ??
      actions.find((action) => action.kind === "pass");
    assert.ok(next, `seed 7 can advance from turn ${state.turn} ${state.step}`);
    game.act(next.index);
  }

  assert.ok(mechanics, "Orcish Mechanics reaches the battlefield");
  assert.equal(
    new Set(creatureTargets.map((action) => action.targetCardId)).size,
    1,
    "the opposing creature is a legal target",
  );
  assert.ok(
    new Set(creatureTargets.map((action) => action.label)).size >= 2,
    "each sacrifice choice has a distinct action label",
  );
  assert.ok(
    creatureTargets.every((action) => action.label.includes("sacrifice")),
    "the interface can name the artifact paid for each action",
  );

  game.free();
});

test("actions that eat a permanent report what they would take", async () => {
  await initializeWasm();

  // Seed 4 puts Atog on the board with exactly one artifact to eat, which is
  // the case the browser must never resolve on the player's behalf.
  const game = new WebGame("Artifacts", "Robots", "Handcrafted", true, 4);
  const play = (label) => {
    const state = JSON.parse(game.state_json());
    const action = state.actions.find((candidate) => candidate.label.startsWith(label));
    assert.ok(action, `${label} is available; have ${state.actions.map((a) => a.label).join(", ")}`);
    game.act(action.index);
  };
  play("Keep this hand");
  play("Play Mountain");
  play("Cast Mox Emerald");
  play("Cast Atog");

  const state = JSON.parse(game.state_json());
  const eats = state.actions.filter((action) => (action.sacrificeCardIds ?? []).length > 0);
  assert.equal(eats.length, 1, "exactly one artifact is available to eat");
  assert.match(eats[0].label, /sacrifice Mox Emerald/);
  const mox = state.battlefield.find((card) => card.name === "Mox Emerald");
  assert.ok(mox, "the Mox is still on the battlefield until the player commits");
  assert.deepEqual(eats[0].sacrificeCardIds, [mox.id], "the cost names the exact permanent");

  game.free();
});
