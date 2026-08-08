import assert from "node:assert/strict";
import test from "node:test";

import { initializeWasm, WebGame } from "./wasm-test-support.mjs";

test("[slow] the bot animates its Factory to attack instead of tapping it to pump itself", async () => {
  await initializeWasm();

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

test("[slow] combat runs out to a decision, not through empty windows", async () => {
  await initializeWasm();

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
