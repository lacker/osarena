import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

import { initializeWasm, WebGame } from "./wasm-test-support.mjs";

test("The Deck exposes colored costs and control rules to the browser", async () => {
  await initializeWasm();

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
  await initializeWasm();

  const game = new WebGame("The Deck", "Goblins", "Random", true, 214);
  let state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Keep this hand").index);
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Cast Black Lotus").index);
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Cast Demonic Tutor").index);
  state = JSON.parse(game.state_json());

  assert.equal(state.decision.kind, "Choice");
  assert.equal(state.decision.orderSemantics, undefined);
  assert.equal(state.decision.visibility, "Private");
  // A search never obliges the searcher to find, so the minimum is zero even
  // with a full library. The client relies on this to offer "Choose none".
  assert.equal(state.decision.minimum, 0);
  assert.equal(state.decision.maximum, 1);
  assert.equal(
    state.decision.cancellable,
    false,
    "failing to find resolves the spell rather than backing out of it",
  );
  assert.ok(state.decision.options.length > 40);
  const choice = state.decision.options[0];
  game.choose_decision(state.decision.id, JSON.stringify([choice.id]));
  assert.equal(JSON.parse(game.state_json()).decision, null);

  game.free();
});

test("concurrent triggers expose resolution ordering and frozen stack ability metadata", async () => {
  await initializeWasm();

  const game = new WebGame("Artifacts", "Goblins", "Random", true, 183);
  let state = JSON.parse(game.state_json());
  const act = (label) => {
    const action = state.actions.find((candidate) => candidate.label.includes(label));
    assert.ok(action, `${label} is available`);
    game.act(action.index);
    state = JSON.parse(game.state_json());
  };

  act("Keep this hand");
  act("Cast Black Lotus");
  act("Cast Mox Sapphire");
  act("Black Lotus for Red");
  act("Mox Sapphire for Blue");
  act("Cast Ankh of Mishra");
  act("Cast Ankh of Mishra");
  act("Play Mountain");

  assert.equal(state.decision.kind, "TriggerOrder");
  assert.equal(state.decision.orderSemantics, "resolution");
  assert.equal(state.decision.minimum, 2);
  assert.equal(state.decision.maximum, 2);
  assert.equal(state.decision.options.length, 2);
  assert.equal(state.decision.options[0].triggerId, state.decision.options[0].id);
  assert.match(state.decision.options[0].abilityText, /Whenever a land enters/);

  const desiredResolutionOrder = state.decision.options.map((option) => option.id).reverse();
  const firstSource = state.decision.options[1].cardId;
  game.set_autopass(false);
  game.choose_decision(state.decision.id, JSON.stringify(desiredResolutionOrder));
  state = JSON.parse(game.state_json());

  assert.equal(state.stack.length, 2);
  assert.equal(state.stack[0].kind, "TriggeredAbility");
  assert.equal(state.stack[0].sourceId, firstSource);
  assert.equal(state.stack[0].abilityId, 0);
  assert.match(state.stack[0].abilityText, /Whenever a land enters/);
  assert.ok(
    state.stack.every((object) => object.kind === "TriggeredAbility"),
    "both objects remain visibly distinct triggered abilities",
  );

  game.free();
});

test("a search can be resolved by failing to find", async () => {
  await initializeWasm();

  const game = new WebGame("The Deck", "Goblins", "Random", true, 214);
  let state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Keep this hand").index);
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Cast Black Lotus").index);
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Cast Demonic Tutor").index);
  state = JSON.parse(game.state_json());

  const handBeforeSearch = state.human.hand.length;
  const libraryBeforeSearch = state.human.library;

  // An empty selection is the wire form of failing to find.
  game.choose_decision(state.decision.id, JSON.stringify([]));
  state = JSON.parse(game.state_json());

  assert.equal(state.decision, null, "the search resolved");
  assert.equal(state.human.hand.length, handBeforeSearch, "no card was found");
  assert.equal(state.human.library, libraryBeforeSearch, "and none left the library");

  game.free();
});

test("opponent pregame choices do not block the game with animations", async () => {
  await initializeWasm();

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
  await initializeWasm();

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
  await initializeWasm();

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

test("every deck the picker offers is one the engine can build", async () => {
  await initializeWasm();

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

test("the Random setup choice is a placeholder, never a deck name", async () => {
  await initializeWasm();

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
