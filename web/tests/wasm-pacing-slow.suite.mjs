import assert from "node:assert/strict";
import test from "node:test";

import { initializeWasm, WebGame } from "./wasm-test-support.mjs";

test("[slow] the pass button label matches where the click actually lands", async () => {
  await initializeWasm();

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

test("[slow] a board with no creatures skips its own second main", async () => {
  await initializeWasm();

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

test("[slow] opponent-action snapshots never contain your next draw", async () => {
  await initializeWasm();

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

test("[slow] your own spell resolving is not a beat you have to sit through", async () => {
  await initializeWasm();

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

test("[slow] your own play is on the board before the turn it ended is announced", async () => {
  await initializeWasm();

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
