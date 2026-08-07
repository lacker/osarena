import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

import init, { WebGame } from "../app/wasm/penta_wasm.js";

test("The Deck exposes colored costs and control rules to the browser", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  const game = new WebGame("The Deck", "The Deck", "Handcrafted", true, 3);
  const opening = JSON.parse(game.state_json());
  const swords = opening.human.hand.find(
    (card) => card.name === "Swords to Plowshares",
  );
  const serra = opening.human.hand.find((card) => card.name === "Serra Angel");
  assert.ok(swords);
  assert.equal(swords.manaCost.white, 1);
  assert.match(swords.rulesText, /exile target creature/i);
  assert.ok(serra);
  assert.equal(serra.manaCost.white, 2);
  assert.equal(serra.power, 4);
  assert.equal(serra.toughness, 4);

  game.free();
});

test("staged engine decisions are serialized as generic private choices", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  const game = new WebGame("The Deck", "Goblins", "Random", true, 214);
  let state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Keep this hand").index);
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Cast Black Lotus").index);
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Cast Demonic Tutor").index);
  state = JSON.parse(game.state_json());

  assert.equal(state.decision.visibility, "Private");
  assert.equal(state.decision.minimum, 1);
  assert.equal(state.decision.maximum, 1);
  assert.ok(state.decision.options.length > 40);
  const choice = state.decision.options[0];
  game.choose_decision(state.decision.id, JSON.stringify([choice.id]));
  assert.equal(JSON.parse(game.state_json()).decision, null);

  game.free();
});

test("opponent pregame choices do not block the game with animations", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  const game = new WebGame("The Deck", "Sligh", "Handcrafted", true, 0);
  const opening = JSON.parse(game.state_json());
  const keep = opening.actions.find((action) => action.label === "Keep this hand");
  game.act(keep.index);

  const afterKeep = JSON.parse(game.state_json());
  assert.ok(
    afterKeep.opponentActions.every(
      (action) =>
        action.label !== "Keep this hand" &&
        action.label !== "Take a mulligan" &&
        !action.label.startsWith("Bottom "),
    ),
    "keep, mulligan, and bottom choices stay out of the opponent animation queue",
  );

  game.free();
});

test("the Robots deck and its new card rules are packaged for the browser", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  const game = new WebGame(
    "Robots",
    "Robots",
    "Handcrafted",
    true,
    823380616,
  );
  const opening = JSON.parse(game.state_json());
  const juggernaut = opening.human.hand.find(
    (card) => card.name === "Juggernaut",
  );
  assert.ok(juggernaut, "the deterministic Robots hand includes Juggernaut");
  assert.equal(juggernaut.power, 5);
  assert.equal(juggernaut.toughness, 3);
  assert.match(juggernaut.rulesText, /attacks each combat if able/i);

  game.free();
});

test("the packaged Rust engine plays through browser actions", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  const game = new WebGame("Goblins", "Sligh", "Handcrafted", true, 9394);
  const opening = JSON.parse(game.state_json());
  assert.equal(opening.human.hand.length, 7);
  assert.equal(opening.opponent.handSize, 7);
  assert.ok(
    opening.human.hand.every((card) => card.manaCost !== undefined),
    "cards expose their casting costs to the interface",
  );
  assert.ok(
    opening.human.hand.every(
      (card) => typeof card.rulesText === "string" && card.rulesText.length > 0,
    ),
    "cards expose their rules text to the interface",
  );
  const openingCreature = opening.human.hand.find((card) =>
    card.kind.includes("creature"),
  );
  assert.ok(openingCreature, "the deterministic opening hand includes a creature");
  assert.equal(typeof openingCreature.power, "number");
  assert.equal(typeof openingCreature.toughness, "number");
  assert.ok(opening.actions.some((action) => action.label === "Keep this hand"));

  const keep = opening.actions.find((action) => action.label === "Keep this hand");
  game.act(keep.index);
  const afterKeep = JSON.parse(game.state_json());
  assert.equal(afterKeep.turn, 1);
  assert.equal(
    afterKeep.step,
    "Precombat Main",
    "the web facade passes through an uneventful opening upkeep",
  );
  assert.ok(Array.isArray(afterKeep.opponentActions));
  assert.ok(
    afterKeep.opponentActions.every((action) => action.label !== "Pass priority"),
    "routine opponent priority passes stay out of the animation queue",
  );
  assert.ok(
    afterKeep.opponentActions.every((action) => action.kind !== "mana"),
    "mana taps stay out of the standalone animation queue",
  );
  assert.ok(
    afterKeep.opponentActions.every(
      (action) =>
        action.state &&
        Array.isArray(action.state.battlefield) &&
        action.state.opponentActions.length === 0,
    ),
    "each opponent animation carries a non-recursive board snapshot",
  );
  assert.ok(
    afterKeep.actions.some(
      (action) => action.kind === "primary" || action.kind === "combat",
    ),
    "choice-free priority windows are passed automatically",
  );
  assert.ok(
    !afterKeep.actions.some((action) => action.label === "Keep this hand"),
  );
  assert.ok(
    afterKeep.events.every(
      (event) =>
        !event.includes("CardInstanceId") &&
        !event.includes("active_player") &&
        !event.includes("card #"),
    ),
    "the game log contains player-facing descriptions rather than engine diagnostics",
  );

  game.free();
});

test("auto-pass declines an unavailable Chain Lightning copy", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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

test("the web facade skips combat when no attackers exist", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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

test("opponent mana taps are grouped with the spell they pay for", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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

test("casting a spell automatically taps available mana sources", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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

test("phase stops override smooth UI auto-passing without changing engine steps", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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

test("Orcish Mechanics exposes creature targets and distinct artifact costs", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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

test("the pass button label reports the engine's real auto-pass destination", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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

test("the game-over message names whoever actually lost", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  const playOut = (game) => {
    for (let turn = 0; turn < 2000; turn++) {
      const state = JSON.parse(game.state_json());
      if (state.result) return state.result;
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
        actions.find((action) => action.label.startsWith("Bottom ")) ??
        actions.find((action) => action.label.startsWith("Discard ")) ??
        actions.find((action) => action.kind === "pass") ??
        actions[0];
      if (!next) return null;
      game.act(next.index);
    }
    return null;
  };

  const lost = new WebGame("The Deck", "Sligh", "Handcrafted", false, 8);
  const loss = playOut(lost);
  assert.equal(loss.outcome, "loss");
  assert.equal(
    loss.message,
    "You lose — you lost all life",
    "the reason is phrased from the browser player's seat, not the winner's",
  );
  lost.free();
});

test("the game log names cards that have left every visible zone", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  // Swords exiles its target and Counterspell empties the stack, so both leave
  // the log holding references the observation can no longer resolve.
  const game = new WebGame("White Weenie", "GR Aggro", "Handcrafted", true, 3041712688);
  let sawExileReference = false;
  for (let turn = 0; turn < 1200; turn++) {
    const state = JSON.parse(game.state_json());
    if (state.result) break;
    for (const line of state.events) {
      assert.ok(
        !/card #\d+|spell #\d+|Unknown card/.test(line),
        `game log leaked a raw instance id: "${line}"`,
      );
      if (/Swords to Plowshares/.test(line)) sawExileReference = true;
    }
    for (const action of state.actions) {
      assert.ok(
        !/card #\d+|spell #\d+/.test(action.label),
        `action label leaked a raw instance id: "${action.label}"`,
      );
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
      actions.find((action) => action.label === "Keep this hand") ??
      actions.find((action) => action.label.startsWith("Bottom ")) ??
      actions.find((action) => action.label.startsWith("Play ")) ??
      actions.find((action) => action.label.startsWith("Attack with ")) ??
      actions.find((action) => action.label.startsWith("Block ")) ??
      actions.find((action) => action.label.startsWith("Assign ")) ??
      actions.find((action) => action.label.startsWith("Discard ")) ??
      actions.find((action) => action.label.startsWith("Cast ")) ??
      actions.find((action) => action.kind === "pass") ??
      actions[0];
    if (!next) break;
    game.act(next.index);
  }
  assert.ok(sawExileReference, "the chosen seed still exercises Swords to Plowshares");

  game.free();
});

test("every deck the picker offers is one the engine can build", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  const decksByFormat = {
    "old-school-93-94": [
      "Goblins", "Sligh", "Artifacts", "Robots", "The Deck", "Mono Black",
      "White Weenie", "Erhnamgeddon", "Counterburn", "Lions DIB",
      "Lion Dib Bolt", "BWR Aggro", "GR Aggro", "Troll Disk", "Jeskai Aggro",
    ],
    "isd-rtr-standard": [
      "Briksza Naya Midrange", "Greer G/R Aggro", "Fyrberg B/G Midrange",
      "Smith Naya Midrange", "McDuffie U/W/R Flash", "Lorren U/W Flash",
      "Arch U/W Flash", "Kuenzinger Junk Reanimator",
    ],
  };

  for (const [format, names] of Object.entries(decksByFormat)) {
    for (const name of names) {
      const game = new WebGame(name, name, "Handcrafted", true, 1, format);
      const state = JSON.parse(game.state_json());
      assert.equal(state.format, format, `${name} uses the selected format`);
      assert.equal(state.human.hand.length, 7, `${name} deals an opening hand`);
      game.free();
    }
  }

  assert.throws(
    () => new WebGame("Goblins", "Goblins", "Handcrafted", true, 1, "isd-rtr-standard"),
    /unknown deck for format/,
    "a deck from another format cannot leak into Standard",
  );
  assert.throws(
    () => new WebGame("Briksza Naya Midrange", "Briksza Naya Midrange", "Handcrafted", true, 1),
    /unknown deck for format/,
    "the compatibility default remains Old School",
  );
  assert.throws(
    () => new WebGame("Goblins", "Goblins", "Handcrafted", true, 1, "not-a-format"),
    /unknown format/,
  );
});

test("the game log reports permanents leaving the battlefield", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  const game = new WebGame("White Weenie", "GR Aggro", "Handcrafted", true, 3041712688);
  const reported = new Set();
  for (let turn = 0; turn < 1200; turn++) {
    const state = JSON.parse(game.state_json());
    if (state.result) break;
    for (const line of state.events) {
      if (/ was destroyed$| was exiled$| returned to hand$/.test(line)) reported.add(line);
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
      actions.find((action) => action.label === "Keep this hand") ??
      actions.find((action) => action.label.startsWith("Bottom ")) ??
      actions.find((action) => action.label.startsWith("Play ")) ??
      actions.find((action) => action.label.startsWith("Attack with ")) ??
      actions.find((action) => action.label.startsWith("Block ")) ??
      actions.find((action) => action.label.startsWith("Assign ")) ??
      actions.find((action) => action.label.startsWith("Discard ")) ??
      actions.find((action) => action.label.startsWith("Cast ")) ??
      actions.find((action) => action.kind === "pass") ??
      actions[0];
    if (!next) break;
    game.act(next.index);
  }

  assert.ok(
    [...reported].some((line) => line.endsWith("was destroyed")),
    "creatures dying in combat reach the log",
  );
  assert.ok(
    [...reported].some((line) => line.endsWith("was exiled")),
    "Swords to Plowshares exiling a creature reaches the log",
  );
  assert.ok(
    [...reported].every((line) => /^(Your|Opponent’s) /.test(line)),
    `every line names whose permanent it was: ${[...reported].join(" | ")}`,
  );

  game.free();
});

test("the attacker button counts the attack instead of naming the step", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  const game = new WebGame("Goblins", "The Deck", "Handcrafted", true, 9394);
  const seen = new Set();
  for (let turn = 0; turn < 500; turn++) {
    const state = JSON.parse(game.state_json());
    if (state.result) break;
    for (const action of state.actions) {
      assert.notEqual(action.label, "Finish attacking", "the step name is gone");
      if (/^(No attacks|Attack with )/.test(action.label)) seen.add(action.label);
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

test("actions that eat a permanent report what they would take", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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

test("combat damage is only asked about when it is a real choice", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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
  const split = new WebGame("GR Aggro", "Robots", "Handcrafted", true, 40990);
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

test("the Random setup choice is a placeholder, never a deck name", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  const source = await readFile(new URL("../app/game-config.ts", import.meta.url), "utf8");
  const sentinel = /export const randomDeck = "([^"]+)"/.exec(source)?.[1];
  assert.equal(sentinel, "Random");
  assert.equal(
    /export const defaultHumanDeck = (\w+)/.exec(source)?.[1],
    "randomDeck",
    "both seats default to Random",
  );
  assert.equal(/export const defaultBotDeck = (\w+)/.exec(source)?.[1], "randomDeck");

  assert.ok(
    !/^\s*Random:\s*"/m.test(source),
    "the sentinel is not one of the real decks, so it must be resolved before it reaches the engine",
  );
  assert.throws(
    () => new WebGame(sentinel, "Goblins", "Handcrafted", true, 1),
    /unknown deck/,
    "the engine rejects it, which is why the browser resolves it first",
  );

  const game = new WebGame("Goblins", "Goblins", "Handcrafted", true, 1);
  game.free();
});

test("declaring attackers always offers a confirm and a way back", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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

test("the pass button label matches where the click actually lands", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  // gameTurn is the global counter. `turn` is per-player and changes meaning
  // when the active player flips, so boundaries must be read from gameTurn.
  const sameTurnAt = (steps) => (before, after) =>
    after.gameTurn === before.gameTurn && steps.includes(after.step);
  const arrivals = {
    "Your turn": (b, a) => a.gameTurn > b.gameTurn && a.active === "You",
    "End turn": (b, a) => a.gameTurn > b.gameTurn && b.active === "You",
    "Draw a card": sameTurnAt(["Draw"]),
    "Go to upkeep": sameTurnAt(["Upkeep"]),
    "Go to main phase": sameTurnAt(["Precombat Main"]),
    "Go to attacks": sameTurnAt(["Beginning Of Combat", "Declare Attackers"]),
    "Go to blocks": sameTurnAt(["Declare Blockers"]),
    // Damage names the button whenever the pass causes it, not only when the
    // yield happens to stop on the step.
    "Go to damage": (before, after) =>
      before.battlefield.some((card) => card.attacking) &&
      (after.gameTurn > before.gameTurn ||
        ["Combat Damage", "End Of Combat", "Postcombat Main", "End", "Cleanup"].includes(after.step)),
    // On defense the button names the commitment: nothing of yours blocks, and
    // the click carries the attack all the way past the block step.
    "No blocks": (before, after) =>
      before.battlefield.some((card) => card.attacking) &&
      !after.battlefield.some((card) => card.blocking != null) &&
      (after.gameTurn > before.gameTurn ||
        ["Combat Damage", "End Of Combat", "Postcombat Main", "End", "Cleanup"].includes(after.step)),
    "Go to end of combat": sameTurnAt(["End Of Combat"]),
    "Go to second main": sameTurnAt(["Postcombat Main"]),
    "Go to end step": sameTurnAt(["End"]),
    "Discard down to seven": sameTurnAt(["Cleanup"]),
    "Go to their upkeep": sameTurnAt(["Upkeep"]),
    "Go to their draw": sameTurnAt(["Draw"]),
    "Go to their main phase": sameTurnAt(["Precombat Main"]),
    "Go to their attack": sameTurnAt(["Beginning Of Combat", "Declare Attackers"]),
    "Go to their second main": sameTurnAt(["Postcombat Main"]),
    "Go to their end step": sameTurnAt(["End"]),
    "Go to cleanup": sameTurnAt(["Cleanup"]),
  };

  const decks = ["Goblins", "Sligh", "White Weenie", "Erhnamgeddon", "GR Aggro", "The Deck"];
  const tally = new Map();
  const record = (label, hit, quiet) => {
    const row = tally.get(label) ?? { used: 0, hit: 0, quiet: 0, quietHit: 0 };
    row.used += 1;
    if (hit) row.hit += 1;
    if (quiet) {
      row.quiet += 1;
      if (hit) row.quietHit += 1;
    }
    tally.set(label, row);
  };
  const misses = [];

  for (let game = 0; game < 40; game += 1) {
    const match = new WebGame(
      decks[game % decks.length],
      decks[(game * 5 + 2) % decks.length],
      "Handcrafted",
      game % 2 === 0,
      game * 7919 + 13,
    );
    for (let turn = 0; turn < 600; turn += 1) {
      const before = JSON.parse(match.state_json());
      if (before.result) break;
      if (before.decision) {
        const wanted = Math.max(before.decision.minimum, 1);
        try {
          match.choose_decision(
            before.decision.id,
            JSON.stringify(before.decision.options.slice(0, wanted).map((option) => option.id)),
          );
        } catch { break; }
        continue;
      }
      const actions = before.actions.filter((action) => action.kind !== "danger");
      const pass = actions.find((action) => action.label === "Pass priority");
      const usePass = pass && turn % 2 === 0;
      const next =
        (usePass ? pass : null) ??
        actions.find((action) => action.label === "Keep this hand") ??
        actions.find((action) => action.label.startsWith("Play ")) ??
        actions.find((action) => /^Attack with \D/.test(action.label)) ??
        actions.find((action) => action.label.startsWith("Block ")) ??
        actions.find((action) => action.label.startsWith("Cast ")) ??
        actions.find((action) => action.label.startsWith("Discard ")) ??
        actions.find((action) => action.kind === "pass") ??
        actions[0];
      if (!next) break;
      const promised = before.passLabel;
      try { match.act(next.index); } catch { break; }
      if (!usePass) continue;

      const after = JSON.parse(match.state_json());
      if (after.result) continue;
      const quiet = (after.opponentActions ?? []).length === 0;

      if (promised?.startsWith("Resolve ")) {
        record("Resolve", after.stack.length < before.stack.length, quiet);
        continue;
      }
      const arrived = arrivals[promised];
      assert.ok(arrived, `unmapped pass label "${promised}"`);
      const hit = arrived(before, after);
      record(promised, hit, quiet);
      if (!hit && quiet) {
        misses.push(
          `"${promised}" from turn ${before.gameTurn} ${before.step} (${before.active}) landed on turn ${after.gameTurn} ${after.step} (${after.active})`,
        );
      }
    }
    match.free();
  }

  const total = [...tally.values()].reduce((sum, row) => sum + row.used, 0);
  assert.ok(total > 300, `exercised enough passes, got ${total}`);
  // "Go to damage" needs a block to have been declared, which this sweep only
  // reaches by luck; the defender test below covers it deliberately.
  for (const required of ["Your turn", "End turn", "Go to attacks", "No blocks", "Go to their end step"]) {
    assert.ok(tally.has(required), `saw "${required}"; got ${[...tally.keys()].join(", ")}`);
  }

  // Only the opponent taking a turn of their own can invalidate a prediction,
  // and that is exactly when the game should stop to show you what they did.
  // Their attack is the one call the preview guesses at from public board
  // state, so it is the one label allowed to be conservative.
  const guessed = new Set(["Go to their attack", "Resolve"]);
  const quietMisses = misses.filter((line) => !line.startsWith('"Go to their attack"'));
  assert.deepEqual(quietMisses, [], "a quiet opponent never invalidates a prediction");

  for (const [label, row] of tally) {
    if (guessed.has(label) || row.used < 20) continue;
    const rate = row.hit / row.used;
    assert.ok(rate >= 0.95, `"${label}" landed where promised ${row.hit}/${row.used} times`);
  }
  const attack = tally.get("Go to their attack");
  if (attack && attack.used >= 20) {
    assert.ok(
      attack.hit / attack.used >= 0.9,
      `"Go to their attack" landed in their combat ${attack.hit}/${attack.used} times`,
    );
  }
});

test("a board with no creatures skips its own second main", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  const decks = ["Goblins", "Sligh", "White Weenie", "GR Aggro", "The Deck"];
  let idledWithoutCreatures = 0;
  let firstMainWithoutCreatures = 0;
  let saidSecondMain = 0;
  let dealtDamageLabel = 0;
  let blockedBeforeDamage = 0;

  for (let game = 0; game < 40; game += 1) {
    const match = new WebGame(
      decks[game % decks.length],
      decks[(game * 3 + 1) % decks.length],
      "Handcrafted",
      game % 2 === 0,
      game * 31337 + 5,
    );
    for (let turn = 0; turn < 600; turn += 1) {
      const state = JSON.parse(match.state_json());
      if (state.result) break;

      const myCreatures = state.battlefield.filter(
        (card) => card.owner === "human" && card.power != null,
      ).length;
      if (state.active === "You" && myCreatures === 0) {
        if (state.step === "Postcombat Main") idledWithoutCreatures += 1;
        if (state.step === "Precombat Main" && state.passLabel) {
          firstMainWithoutCreatures += 1;
          if (state.passLabel === "Go to second main") saidSecondMain += 1;
        }
      }
      // Attacking into declared blockers: the pass is about to deal damage.
      if (
        state.active === "You" &&
        state.step === "Declare Blockers" &&
        state.battlefield.some((card) => card.owner === "human" && card.attacking) &&
        state.battlefield.some((card) => card.owner === "opponent" && card.blocking != null) &&
        state.passLabel
      ) {
        blockedBeforeDamage += 1;
        if (state.passLabel === "Go to damage") dealtDamageLabel += 1;
      }

      if (state.decision) {
        const wanted = Math.max(state.decision.minimum, 1);
        try {
          match.choose_decision(
            state.decision.id,
            JSON.stringify(state.decision.options.slice(0, wanted).map((option) => option.id)),
          );
        } catch { break; }
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
      if (!next) break;
      try { match.act(next.index); } catch { break; }
    }
    match.free();
  }

  assert.ok(firstMainWithoutCreatures > 50, `exercised the empty board, got ${firstMainWithoutCreatures}`);
  assert.equal(idledWithoutCreatures, 0, "an empty board never waits in its own second main");
  assert.equal(saidSecondMain, 0, "and never promises to go there");
  assert.ok(blockedBeforeDamage > 10, `exercised blocked combat, got ${blockedBeforeDamage}`);
  assert.equal(
    dealtDamageLabel,
    blockedBeforeDamage,
    "passing into declared blockers always names the damage it causes",
  );
});

test("opponent-action snapshots never contain your next draw", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  // Each animation frame carries the state right after its own action, so the
  // story can be told in order: the card you draw for your next turn must not
  // sit in your hand while the opponent's turn is still being replayed.
  const game = new WebGame("Goblins", "The Deck", "Handcrafted", true, 9394);
  let turnsChecked = 0;
  for (let turn = 0; turn < 400; turn += 1) {
    const state = JSON.parse(game.state_json());
    if (state.result) break;
    const handBefore = new Set(state.human.hand.map((card) => card.id));
    const actions = state.actions.filter((action) => action.kind !== "danger");
    if (state.decision) {
      const wanted = Math.max(state.decision.minimum, 1);
      game.choose_decision(
        state.decision.id,
        JSON.stringify(state.decision.options.slice(0, wanted).map((option) => option.id)),
      );
      continue;
    }
    const next =
      actions.find((action) => action.label === "Keep this hand") ??
      actions.find((action) => action.label.startsWith("Play ")) ??
      actions.find((action) => action.label.startsWith("Cast Goblin")) ??
      actions.find((action) => action.label.startsWith("Block ")) ??
      actions.find((action) => action.label.startsWith("Discard ")) ??
      actions.find((action) => action.kind === "pass") ??
      actions[0];
    if (!next) break;
    game.act(next.index);

    const after = JSON.parse(game.state_json());
    const animations = after.opponentActions ?? [];
    if (animations.length === 0) continue;
    const drawn = after.human.hand.filter((card) => !handBefore.has(card.id));
    if (drawn.length === 0) continue;
    turnsChecked += 1;
    for (const card of drawn) {
      // A card may enter the hand mid-replay (Timetwister resolving refills
      // the hand at its own beat) — but once it appears it must stay, and it
      // must never show up in frames before the beat that produced it.
      const appears = animations.map((frame) =>
        frame.state.human.hand.some((held) => held.id === card.id),
      );
      assert.ok(
        !appears[0] || animations.length === 1,
        `"${animations[0].label}" already shows ${card.name} in hand`,
      );
      for (let i = 1; i < appears.length; i += 1) {
        assert.ok(
          !(appears[i - 1] && !appears[i]),
          `${card.name} flickers out of hand at "${animations[i].label}"`,
        );
      }
    }
    for (const frame of animations) {
      assert.ok(
        !frame.state.canCancelAttackers,
        "no replayed frame still offers taking the attack back",
      );
    }
  }
  assert.ok(turnsChecked >= 3, `checked ${turnsChecked} turns with draws`);

  game.free();
});

test("the bot animates its Factory to attack instead of tapping it to pump itself", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  // The pump costs the Factory its tap, so aiming it at itself trades the
  // attack for +1/+1 on a creature that can no longer attack. The bot used to
  // do this every turn it could.
  let selfPumps = 0;
  let attacks = 0;
  let animations = 0;
  for (const deck of ["Sligh", "Artifacts", "Robots", "The Deck", "Lions DIB"]) {
    for (const seed of [37, 74, 148, 296]) {
      const game = new WebGame("Mono Black", deck, "Handcrafted", true, seed);
      for (let step = 0; step < 400; step += 1) {
        const state = JSON.parse(game.state_json());
        if (state.result) break;
        for (const beat of state.opponentActions ?? []) {
          if (/Give Mishra's Factory \+1\/\+1 with Mishra's Factory/.test(beat.label)) {
            selfPumps += 1;
          }
          if (/Make Mishra's Factory a 2\/2/.test(beat.label)) animations += 1;
          if (/^Attack with Mishra's Factory/.test(beat.label)) attacks += 1;
        }
        if (state.decision) {
          const wanted = Math.max(state.decision.minimum, 1);
          try {
            game.choose_decision(
              state.decision.id,
              JSON.stringify(state.decision.options.slice(0, wanted).map((o) => o.id)),
            );
          } catch { break; }
          continue;
        }
        const actions = state.actions.filter((action) => action.kind !== "danger");
        const next =
          actions.find((action) => action.label === "Keep this hand") ??
          actions.find((action) => action.label.startsWith("Play ")) ??
          actions.find((action) => action.kind === "pass") ??
          actions[0];
        if (!next) break;
        try { game.act(next.index); } catch { break; }
      }
      game.free();
    }
  }
  assert.equal(selfPumps, 0, "a Factory never taps itself to pump itself");
  assert.ok(animations > 0, `the Factory still becomes a creature, saw ${animations}`);
  assert.ok(attacks > 0, `and attacks with it, saw ${attacks}`);
});

test("your own spell resolving is not a beat you have to sit through", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  // The yield that resolves your own spell is automatic, so replaying it puts
  // the board in "opponent acting" — every button disabled — for a beat you
  // did not need to watch. A fizzle still gets one: it is the only
  // explanation for a spell that did nothing.
  let casts = 0;
  let theirResolutions = 0;
  for (const deck of ["Sligh", "Artifacts", "White Weenie", "The Deck"]) {
    for (const seed of [97, 291, 485]) {
      const game = new WebGame(deck, "The Deck", "Handcrafted", true, seed);
      for (let step = 0; step < 250; step += 1) {
        const state = JSON.parse(game.state_json());
        if (state.result) break;
        if (state.decision) {
          const wanted = Math.max(state.decision.minimum, 1);
          try {
            game.choose_decision(
              state.decision.id,
              JSON.stringify(state.decision.options.slice(0, wanted).map((o) => o.id)),
            );
          } catch { break; }
          continue;
        }
        const actions = state.actions.filter((action) => action.kind !== "danger");
        const next =
          actions.find((action) => action.label === "Keep this hand") ??
          actions.find((action) => action.label.startsWith("Play ")) ??
          actions.find((action) => action.label.startsWith("Cast ")) ??
          actions.find((action) => action.kind === "pass") ??
          actions[0];
        if (!next) break;
        const cast = /^Cast ([^→(]+)/.exec(next.label)?.[1]?.trim();
        try { game.act(next.index); } catch { break; }

        const beats = JSON.parse(game.state_json()).opponentActions ?? [];
        if (cast) {
          casts += 1;
          assert.ok(
            !beats.some((beat) => beat.label === `${cast} resolves`),
            `"${next.label}" replays its own resolution: ${beats.map((b) => b.label).join(", ")}`,
          );
        }
        theirResolutions += beats.filter((beat) => / resolves$/.test(beat.label)).length;
      }
      game.free();
    }
  }
  assert.ok(casts >= 100, `exercised enough casts, got ${casts}`);
  assert.ok(theirResolutions > 0, "their spells still resolve on their own beat");
});

test("your own play is on the board before the turn it ended is announced", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  // The client replays a turn from the board your click left behind, not from
  // the board before it, so a land played in your second main is down before
  // the "Opponent's turn" banner is held over it. This mirrors that rule.
  const turnChanged = (from, to) =>
    from
      ? from.pregame !== to.pregame ||
        (!to.pregame && (from.gameTurn !== to.gameTurn || from.active !== to.active))
      : true;

  let banners = 0;
  let handovers = 0;
  for (const deck of ["Sligh", "White Weenie", "GR Aggro", "Lions DIB", "Robots", "Artifacts"]) {
    for (const seed of [31, 62, 155, 217, 318, 424, 530]) {
      const game = new WebGame(deck, "The Deck", "Handcrafted", true, seed);
      let displayed = JSON.parse(game.state_json());
      for (let step = 0; step < 250; step += 1) {
        const state = JSON.parse(game.state_json());
        if (state.result) break;
        if (state.decision) {
          const wanted = Math.max(state.decision.minimum, 1);
          try {
            game.choose_decision(
              state.decision.id,
              JSON.stringify(state.decision.options.slice(0, wanted).map((o) => o.id)),
            );
          } catch { break; }
          displayed = JSON.parse(game.state_json());
          continue;
        }
        const actions = state.actions.filter((action) => action.kind !== "danger");
        // Lands always, spells in main one only every other turn: that leaves
        // a board to hold the second main open and something in hand to spend
        // there, which is the click that resolves and hands the turn over in
        // one go.
        const next =
          actions.find((action) => action.label === "Keep this hand") ??
          (state.step === "Precombat Main"
            ? actions.find((action) => action.label.startsWith("Play "))
            : null) ??
          (state.step === "Precombat Main" && state.gameTurn % 2 === 1
            ? actions.find((action) => action.label.startsWith("Cast "))
            : null) ??
          (state.step === "Postcombat Main"
            ? actions.find((action) => action.label.startsWith("Cast "))
            : null) ??
          actions.find((action) => /^Attack with \d/.test(action.label)) ??
          actions.find((action) => action.kind === "pass") ??
          actions[0];
        if (!next) break;
        const before = new Set(
          state.battlefield.filter((card) => card.owner === "human").map((card) => card.id),
        );
        try { game.act(next.index); } catch { break; }

        const after = JSON.parse(game.state_json());
        const beats = after.opponentActions ?? [];
        let cursor = displayed;
        const acted = after.afterYourAction;
        if (
          acted &&
          cursor &&
          acted.gameTurn === cursor.gameTurn &&
          acted.active === cursor.active &&
          acted.pregame === cursor.pregame
        ) {
          cursor = acted;
        }
        if (beats.length && turnChanged(cursor, beats[0].state)) {
          const played = (acted ?? after).battlefield.filter(
            (card) => card.owner === "human" && !before.has(card.id),
          );
          handovers += 1;
          if (played.length) banners += 1;
          for (const card of played) {
            assert.ok(
              cursor.battlefield.some((held) => held.id === card.id),
              `${card.name} is on the board the banner is held over, after "${next.label}"`,
            );
          }
          // A spell you cast resolves on an automatic yield, so by the time
          // their turn is announced it belongs on the board, not the stack.
          assert.deepEqual(
            cursor.stack.filter((object) => object.owner === "human").map((o) => o.name),
            [],
            `nothing of yours is still on the stack after "${next.label}"`,
          );
        }
        displayed = beats.length ? beats[beats.length - 1].state : after;
      }
      game.free();
    }
  }
  assert.ok(banners >= 10, `saw your play land before the banner ${banners} times`);
  assert.ok(handovers >= 100, `exercised enough handovers, saw ${handovers}`);
});

test("mulligans are not a turn, and the draw happens in the beginning phase", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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

test("declining a block runs to their end step; blocking keeps the damage stop", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

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

test("combat runs out to a decision, not through empty windows", async () => {
  const bytes = await readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  );
  await init({ module_or_path: bytes });

  const decks = ["Goblins", "Sligh", "White Weenie", "GR Aggro", "Erhnamgeddon", "Robots"];
  let endOfCombatStops = 0;
  let secondMainIdle = 0;
  let secondMainHoldingSpell = 0;

  for (let game = 0; game < 24; game += 1) {
    // Develops a board but holds every non-creature spell, so the second main
    // always has something worth stopping for.
    const match = new WebGame(
      decks[game % decks.length],
      decks[(game * 3 + 1) % decks.length],
      "Handcrafted",
      game % 2 === 0,
      game * 7919 + 41,
    );
    for (let turn = 0; turn < 700; turn += 1) {
      const state = JSON.parse(match.state_json());
      if (state.result) break;
      if (state.step === "End Of Combat") endOfCombatStops += 1;
      if (state.active === "You" && state.step === "Postcombat Main") {
        if (state.actions.some((action) => /^(Cast |Play )/.test(action.label))) {
          secondMainHoldingSpell += 1;
        } else {
          secondMainIdle += 1;
        }
      }
      if (state.decision) {
        const wanted = Math.max(state.decision.minimum, 1);
        try {
          match.choose_decision(
            state.decision.id,
            JSON.stringify(state.decision.options.slice(0, wanted).map((option) => option.id)),
          );
        } catch { break; }
        continue;
      }
      const inFirstMain = state.active === "You" && state.step === "Precombat Main";
      const actions = state.actions.filter((action) => action.kind !== "danger");
      const next =
        actions.find((action) => action.label === "Keep this hand") ??
        (inFirstMain ? actions.find((action) => action.label.startsWith("Play ")) : null) ??
        (inFirstMain
          ? actions.find(
              (action) =>
                action.label.startsWith("Cast ") &&
                /Goblin|Knight|Lion|Elves|Ape|Djinn|Atog|Juggernaut|Brigade|Orcs|Troll/.test(
                  action.label,
                ),
            )
          : null) ??
        actions.find((action) => /^Attack with \D/.test(action.label)) ??
        actions.find((action) => /^Attack with \d/.test(action.label)) ??
        actions.find((action) => action.label.startsWith("Block ")) ??
        actions.find((action) => action.label.startsWith("Assign ")) ??
        actions.find((action) => action.label.startsWith("Discard ")) ??
        actions.find((action) => action.kind === "pass") ??
        actions[0];
      if (!next) break;
      try { match.act(next.index); } catch { break; }
    }
    match.free();
  }

  assert.equal(
    endOfCombatStops,
    0,
    "damage is already dealt by end of combat, so the window is never held",
  );
  assert.equal(
    secondMainIdle,
    0,
    "a second main with nothing to commit from hand is passed through",
  );
  assert.ok(
    secondMainHoldingSpell > 10,
    `but a castable card still holds it, got ${secondMainHoldingSpell}`,
  );
});
