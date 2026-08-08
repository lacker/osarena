"use client";

import { useCallback, useEffect, useLayoutEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { CardArt } from "./CardArt";
import { isScryfallId, type CardArtMode } from "./card-art-mode";
import {
  createEngineGame,
  initializeEngine,
  readEngineState,
  type EngineGame,
} from "./engine-client";
import type {
  Action,
  Card,
  DecisionState,
  GameState,
  OpponentAction,
  Owner,
  PlayerState,
} from "./game-types";
import {
  deckChoiceNote,
  deckChoicesForFormat,
  deckNamesForFormat,
  defaultBotDeck,
  defaultFormat,
  defaultHumanDeck,
  drawBeatDurationMs,
  formatConfigs,
  isFormatId,
  opponentActionDurationMs,
  placeholderDeckForFormat,
  turnBannerDurationMs,
  randomDeck,
  turnPhases,
  type FormatId,
} from "./game-config";

const randomSeed = () => crypto.getRandomValues(new Uint32Array(1))[0];

const initialSeed = () => {
  const requested = new URLSearchParams(window.location.search).get("seed");
  if (requested !== null && /^\d+$/.test(requested)) {
    const parsed = Number(requested);
    if (Number.isSafeInteger(parsed) && parsed <= 0xffff_ffff) return parsed;
  }
  return randomSeed();
};

/// Turns a setup choice into a deck the engine can build. Every choice but
/// `randomDeck` is already one.
const resolveDeck = (format: FormatId, choice: string) => {
  if (choice !== randomDeck) return choice;
  const deckNames = deckNamesForFormat(format);
  return deckNames[randomSeed() % deckNames.length];
};

const initialFormat = () => {
  const requested = new URLSearchParams(window.location.search).get("format");
  return isFormatId(requested) ? requested : defaultFormat;
};

const initialDeckPair = (format: FormatId) => {
  const requested = new URLSearchParams(window.location.search).get("deck");
  const deckNotes = formatConfigs[format].deckNotes;
  return {
    humanDeck: requested && Object.hasOwn(deckNotes, requested) ? requested : defaultHumanDeck,
    botDeck: defaultBotDeck,
  };
};

const initialHumanFirst = () =>
  new URLSearchParams(window.location.search).get("first") !== "false";

const cardName = (state: GameState | null, id: number) =>
  state?.battlefield.find((card) => card.id === id)?.name ?? "this attacker";

const cardTargetKey = (id: number) => `card:${id}`;
const playerTargetKey = (owner: Owner) => `player:${owner}`;
const actionTargetKeys = (action: Action) => Array.from(new Set([
  ...(action.targetCardId != null ? [cardTargetKey(action.targetCardId)] : []),
  ...(action.targetCardIds ?? []).map(cardTargetKey),
  ...(action.targetPlayer != null ? [playerTargetKey(action.targetPlayer)] : []),
  ...(action.targetPlayers ?? []).map(playerTargetKey),
  ...(action.targetStackId != null ? [`stack:${action.targetStackId}`] : []),
  ...(action.targetStackIds ?? []).map((id) => `stack:${id}`),
]));
const hasActionTargets = (action: Action) => actionTargetKeys(action).length > 0;
const singleTargetKey = (action: Action) => {
  const targets = actionTargetKeys(action);
  return targets.length === 1 ? targets[0] : null;
};
const sameTargets = (left: string[], right: string[]) =>
  left.length === right.length && left.every((target) => right.includes(target));

const isTriggerOrderDecision = (
  decision: DecisionState | null | undefined,
): decision is DecisionState & { kind: "TriggerOrder"; orderSemantics: "resolution" } =>
  decision?.kind === "TriggerOrder" && decision.orderSemantics === "resolution";

const isTriggerPlacementDecision = (
  decision: DecisionState | null | undefined,
): decision is DecisionState & { kind: "TriggerPlacement" } =>
  decision?.kind === "TriggerPlacement";

type CardPresentationRect = {
  rect: DOMRect;
  owner: Owner;
  name: string;
  zone: string;
};

const isPlausibleVisibleZoneTransition = (from: string, to: string) => {
  if (from === "hand") return to === "stack" || to === "battlefield";
  if (from === "stack") return to === "battlefield" || to === "hand";
  return from === "battlefield" && to === "hand";
};

const isPresentationPredecessor = (
  before: CardPresentationRect,
  after: CardPresentationRect,
) =>
  before.owner === after.owner &&
  before.name === after.name &&
  isPlausibleVisibleZoneTransition(before.zone, after.zone);

export function GameClient({
  defaultCardArtMode = "off",
}: {
  defaultCardArtMode?: CardArtMode;
} = {}) {
  const game = useRef<EngineGame | null>(null);
  const tableRef = useRef<HTMLElement | null>(null);
  const wasmReady = useRef(false);
  const finalStateAfterOpponentActions = useRef<GameState | null>(null);
  const [state, setState] = useState<GameState | null>(null);
  // What the player picked, which may be "Random", versus the deck that
  // pick actually became for the game now on the table.
  const [format, setFormat] = useState<FormatId>(defaultFormat);
  const [humanDeckChoice, setHumanDeckChoice] = useState(defaultHumanDeck);
  const [botDeckChoice, setBotDeckChoice] = useState(defaultBotDeck);
  const [humanDeck, setHumanDeck] = useState(placeholderDeckForFormat(defaultFormat));
  const [botDeck, setBotDeck] = useState(placeholderDeckForFormat(defaultFormat));
  const [policy, setPolicy] = useState("Handcrafted");
  const [humanFirst, setHumanFirst] = useState(true);
  // The engine prepares a table behind the initial setup dialog. Keep that
  // table image-free until Deal commits the player's draft choice, otherwise
  // choosing "Symbols only" would come after the first Scryfall requests.
  const [cardArtMode, setCardArtMode] = useState<CardArtMode>("off");
  const [draftFormat, setDraftFormat] = useState<FormatId>(defaultFormat);
  const [draftHumanDeck, setDraftHumanDeck] = useState(defaultHumanDeck);
  const [draftBotDeck, setDraftBotDeck] = useState(defaultBotDeck);
  const [draftPolicy, setDraftPolicy] = useState("Handcrafted");
  const [draftHumanFirst, setDraftHumanFirst] = useState(true);
  const [draftCardArtMode, setDraftCardArtMode] = useState<CardArtMode>(defaultCardArtMode);
  // `pregame` banners announce the opening hand rather than a turn, since
  // turn one has not started while anyone is still deciding to mulligan.
  type TurnBanner = { active: string; turn: number; pregame?: boolean };
  type PresentationStep =
    | { kind: "action"; action: OpponentAction; state: GameState }
    | { kind: "banner"; banner: TurnBanner; state: GameState };
  const [setupOpen, setSetupOpen] = useState(true);
  const [setupDismissible, setSetupDismissible] = useState(false);
  const [seed, setSeed] = useState(9394);
  const [selectedCard, setSelectedCard] = useState<number | null>(null);
  const [selectedTargetCard, setSelectedTargetCard] = useState<number | null>(null);
  const [selectedTargetPlayer, setSelectedTargetPlayer] = useState<Owner | null>(null);
  const [selectedTargetStackId, setSelectedTargetStackId] = useState<number | null>(null);
  const [selectedX, setSelectedX] = useState<number | null>(null);
  const [xPickerCard, setXPickerCard] = useState<number | null>(null);
  const [fireballTargets, setFireballTargets] = useState<string[]>([]);
  const [selectedBlocker, setSelectedBlocker] = useState<number | null>(null);
  const [blockAssignments, setBlockAssignments] = useState<Record<number, number>>({});
  const [cardActionMenu, setCardActionMenu] = useState<number | null>(null);
  const [gameLogOpen, setGameLogOpen] = useState(true);
  const [pendingAction, setPendingAction] = useState<Action | null>(null);
  const [draggingCardId, setDraggingCardId] = useState<number | null>(null);
  const [dragOverTarget, setDragOverTarget] = useState<string | null>(null);
  const [hoverPaymentAction, setHoverPaymentAction] = useState<Action | null>(null);
  const [decisionSelectionState, setDecisionSelectionState] = useState<{
    decisionId: number | null;
    options: number[];
  }>({ decisionId: null, options: [] });
  const pendingActionRef = useRef<Action | null>(null);
  const dragDropped = useRef(false);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [engineReady, setEngineReady] = useState(false);
  const [presentationQueue, setPresentationQueue] = useState<PresentationStep[]>([]);
  // What the player is currently looking at; the queue builder diffs against
  // this so every popup and animation runs from the state actually on screen.
  const displayedState = useRef<GameState | null>(null);
  // Card positions from the last presented state, keyed strictly by current
  // game-object id. Cross-zone continuity is inferred for one paint only.
  const flipRects = useRef<Map<number, CardPresentationRect>>(new Map());
  const suppressFlip = useRef(false);
  const currentStep = presentationQueue[0] ?? null;
  const currentOpponentAction = currentStep?.kind === "action" ? currentStep.action : null;
  const turnBanner = currentStep?.kind === "banner" ? currentStep.banner : null;
  const watchingOpponent = currentStep !== null;
  // Drawing for the turn is something the game does, not something the
  // opponent chose, so it stays out of the "N actions" count.
  const actionStepsRemaining = presentationQueue.filter(
    (step) => step.kind === "action" && step.action.kind !== "draw",
  ).length;
  const draftDeckChoices = deckChoicesForFormat(draftFormat);

  // What the phase strip describes. A turn banner is the gap between two
  // turns: the board still shows the one that ended, so the strip names the
  // one arriving and lights no step until it actually begins.
  const strip = turnBanner
    ? {
        active: turnBanner.active,
        turn: turnBanner.turn,
        pregame: turnBanner.pregame ?? false,
        step: null as string | null,
      }
    : {
        active: state?.active ?? "You",
        turn: state?.turn ?? 1,
        pregame: state?.pregame ?? false,
        step: state && !state.pregame ? state.step : null,
      };

  const decisionSelection =
    state?.decision?.id === decisionSelectionState.decisionId
      ? decisionSelectionState.options
      : [];
  const triggerResolutionOrder = isTriggerOrderDecision(state?.decision)
    ? decisionSelectionState.decisionId === state.decision.id &&
      decisionSelectionState.options.length === state.decision.options.length &&
      state.decision.options.every((option) =>
        decisionSelectionState.options.includes(option.id),
      )
      ? decisionSelectionState.options
      : state.decision.options.map((option) => option.id)
    : [];

  const applyState = useCallback((next: GameState) => {
    displayedState.current = next;
    setState(next);
  }, []);

  // Lays the engine's result out as a strictly ordered story: each opponent
  // play is its own beat with the state as it stood right after that play, and
  // a turn change gets its own beat before anything from the new turn — so the
  // card you draw next turn is never in your hand while the old turn is still
  // being told.
  const presentSnapshot = useCallback((snapshot: GameState) => {
    // Opening hands get announced too, and the turn they lead into gets its
    // own banner once the mulligans are settled.
    const turnChanged = (from: GameState | null, to: GameState) =>
      from
        ? from.pregame !== to.pregame ||
          (!to.pregame && (from.gameTurn !== to.gameTurn || from.active !== to.active))
        : true;
    const bannerFor = (state: GameState): TurnBanner => ({
      active: state.active,
      turn: state.turn,
      pregame: state.pregame,
    });
    // The untap step belongs to the turn being announced, so the banner's held
    // frame shows the incoming player's permanents straightening.
    const withUntap = (held: GameState, active: string): GameState => ({
      ...held,
      battlefield: held.battlefield.map((card) =>
        card.tapped && card.owner === (active === "You" ? "human" : "opponent")
          ? { ...card, tapped: false }
          : card,
      ),
    });
    const steps: PresentationStep[] = [];
    let cursor = displayedState.current;
    // Your own click is not a beat to watch — it already happened. Replaying
    // from the board it left behind keeps a land you just played out of the
    // frame that announces the turn it ended.
    const acted = snapshot.afterYourAction;
    if (
      acted &&
      cursor &&
      acted.gameTurn === cursor.gameTurn &&
      acted.active === cursor.active &&
      acted.pregame === cursor.pregame
    ) {
      cursor = acted;
    }
    for (const action of snapshot.opponentActions ?? []) {
      if (turnChanged(cursor, action.state)) {
        steps.push({
          kind: "banner",
          banner: bannerFor(action.state),
          state: cursor ? withUntap(cursor, action.state.active) : action.state,
        });
      }
      // A turn beat is only there to be announced; the banner above already
      // said everything it has to say.
      if (action.kind !== "turn") {
        steps.push({ kind: "action", action, state: action.state });
      }
      cursor = action.state;
    }
    if (turnChanged(cursor, snapshot)) {
      steps.push({
        kind: "banner",
        banner: bannerFor(snapshot),
        state: cursor ? withUntap(cursor, snapshot.active) : snapshot,
      });
    }
    if (steps.length > 0) {
      finalStateAfterOpponentActions.current = snapshot;
      setPresentationQueue(steps);
      applyState(steps[0].state);
    } else {
      finalStateAfterOpponentActions.current = null;
      setPresentationQueue([]);
      applyState(snapshot);
    }
  }, [applyState]);

  const refresh = useCallback(() => {
    if (!game.current) return;
    const snapshot = readEngineState(game.current);
    presentSnapshot(snapshot);
    // A rejected action leaves a banner behind; the next accepted one retires
    // it, so a transient failure cannot cover the table for the rest of the game.
    setError(null);
    setSelectedCard(null);
    setSelectedTargetCard(null);
    setSelectedTargetPlayer(null);
    setSelectedTargetStackId(null);
    setSelectedX(null);
    setXPickerCard(null);
    setFireballTargets([]);
    setSelectedBlocker(null);
    setBlockAssignments({});
    setCardActionMenu(null);
    setPendingAction(null);
    setDraggingCardId(null);
    setDragOverTarget(null);
    setHoverPaymentAction(null);
    pendingActionRef.current = null;
  }, [presentSnapshot]);

  const newGame = useCallback(
    (
      nextSeed = randomSeed(),
      // These take a setup choice, so "Random" here rolls a new deck for this
      // game. Pass the deck already in play to deal the same matchup again.
      nextHumanDeck = humanDeckChoice,
      nextBotDeck = botDeckChoice,
      nextPolicy = policy,
      nextHumanFirst = humanFirst,
      nextFormat = format,
    ) => {
      if (!wasmReady.current) return false;
      const dealtHumanDeck = resolveDeck(nextFormat, nextHumanDeck);
      const dealtBotDeck = resolveDeck(nextFormat, nextBotDeck);
      try {
        // Build before replacing the live game. A bad format/deck pairing can
        // then leave the current board intact and the setup dialog open.
        const replacement = createEngineGame({
          format: nextFormat,
          humanDeck: dealtHumanDeck,
          botDeck: dealtBotDeck,
          policy: nextPolicy,
          humanFirst: nextHumanFirst,
          seed: nextSeed,
        });
        // A fresh game replaces the whole board; nothing should glide between
        // unrelated games, and no stale beats should keep playing.
        suppressFlip.current = true;
        setPresentationQueue([]);
        setDecisionSelectionState({ decisionId: null, options: [] });
        displayedState.current = null;
        game.current?.free();
        game.current = replacement;
        setSeed(nextSeed);
        setFormat(nextFormat);
        setHumanDeck(dealtHumanDeck);
        setBotDeck(dealtBotDeck);
        finalStateAfterOpponentActions.current = null;
        setError(null);
        refresh();
        const matchUrl = new URL(window.location.href);
        matchUrl.searchParams.set("format", nextFormat);
        matchUrl.searchParams.set("deck", dealtHumanDeck);
        matchUrl.searchParams.set("seed", String(nextSeed));
        matchUrl.searchParams.set("first", String(nextHumanFirst));
        window.history.replaceState(null, "", matchUrl);
        return true;
      } catch (cause) {
        setError(String(cause));
        return false;
      }
    },
    [botDeckChoice, format, humanDeckChoice, humanFirst, policy, refresh],
  );

  useEffect(() => {
    let alive = true;
    const load = async () => {
      try {
        await initializeEngine();
        if (!alive) return;
        const startingSeed = initialSeed();
        const startingFormat = initialFormat();
        const startingChoices = initialDeckPair(startingFormat);
        const startingHumanDeck = resolveDeck(startingFormat, startingChoices.humanDeck);
        const startingBotDeck = resolveDeck(startingFormat, startingChoices.botDeck);
        const startingHumanFirst = initialHumanFirst();
        setSeed(startingSeed);
        setFormat(startingFormat);
        setDraftFormat(startingFormat);
        setHumanDeckChoice(startingChoices.humanDeck);
        setBotDeckChoice(startingChoices.botDeck);
        setDraftHumanDeck(startingChoices.humanDeck);
        setDraftBotDeck(startingChoices.botDeck);
        setHumanDeck(startingHumanDeck);
        setBotDeck(startingBotDeck);
        setHumanFirst(startingHumanFirst);
        setDraftHumanFirst(startingHumanFirst);
        wasmReady.current = true;
        setEngineReady(true);
        game.current = createEngineGame({
          format: startingFormat,
          humanDeck: startingHumanDeck,
          botDeck: startingBotDeck,
          policy: "Handcrafted",
          humanFirst: startingHumanFirst,
          seed: startingSeed,
        });
        const snapshot = readEngineState(game.current);
        presentSnapshot(snapshot);
      } catch (cause) {
        if (alive) setError(`Could not start the Rust engine: ${String(cause)}`);
      } finally {
        if (alive) setLoading(false);
      }
    };
    void load();
    return () => {
      alive = false;
      game.current?.free();
    };
  }, [presentSnapshot]);

  useEffect(() => {
    if (currentStep === null) return;
    const duration =
      currentStep.kind !== "action"
        ? turnBannerDurationMs
        : currentStep.action.kind === "draw"
          ? drawBeatDurationMs
          : opponentActionDurationMs;
    const timer = window.setTimeout(() => {
      const remaining = presentationQueue.slice(1);
      if (remaining.length > 0) {
        applyState(remaining[0].state);
      } else if (finalStateAfterOpponentActions.current) {
        applyState(finalStateAfterOpponentActions.current);
        finalStateAfterOpponentActions.current = null;
      }
      setPresentationQueue(remaining);
    }, duration);
    return () => window.clearTimeout(timer);
  }, [applyState, currentStep, presentationQueue]);

  const skipOpponentActions = () => {
    suppressFlip.current = true;
    if (finalStateAfterOpponentActions.current) {
      applyState(finalStateAfterOpponentActions.current);
      finalStateAfterOpponentActions.current = null;
    }
    setPresentationQueue([]);
  };

  // FLIP layer: whenever the presented state changes, diff every on-table
  // card's position against where it was. Cards that moved glide there, cards
  // that appeared fly in from the zone they logically came from (your library
  // for a draw, their hand for their play), and cards that vanished leave a
  // ghost that drifts to the owner's graveyard counter.
  useLayoutEffect(() => {
    const table = tableRef.current;
    if (!state || !table) return;
    const previous = flipRects.current;
    const entries = new Map<
      number,
      CardPresentationRect & { el: HTMLElement }
    >();
    table.querySelectorAll<HTMLElement>("[data-card-id]").forEach((el) => {
      const id = Number(el.dataset.cardId);
      if (!Number.isFinite(id)) return;
      entries.set(id, {
        el,
        rect: el.getBoundingClientRect(),
        owner: el.dataset.cardOwner === "opponent" ? "opponent" : "human",
        name: el.dataset.cardName ?? "",
        zone: el.dataset.cardZone ?? "",
      });
    });
    const store = new Map<number, CardPresentationRect>();
    entries.forEach((entry, id) =>
      store.set(id, { rect: entry.rect, owner: entry.owner, name: entry.name, zone: entry.zone }),
    );
    const reduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    const skip = reduced || suppressFlip.current || previous.size === 0;
    suppressFlip.current = false;
    flipRects.current = store;
    if (skip) return;

    const anchorRect = (selector: string) =>
      table.querySelector(selector)?.getBoundingClientRect() ??
      document.querySelector(selector)?.getBoundingClientRect() ??
      null;
    const flyFrom = (entry: { el: HTMLElement; rect: DOMRect }, from: DOMRect, entering: boolean) => {
      const dx = from.left + from.width / 2 - (entry.rect.left + entry.rect.width / 2);
      const dy = from.top + from.height / 2 - (entry.rect.top + entry.rect.height / 2);
      const scale = entering
        ? Math.max(0.2, Math.min(1, from.width / Math.max(entry.rect.width, 1)))
        : 1;
      entry.el.animate(
        [
          {
            transform: `translate(${dx}px, ${dy}px) scale(${scale})`,
            opacity: entering ? 0.35 : 1,
          },
          { transform: "none", opacity: 1 },
        ],
        {
          duration: 460,
          easing: "cubic-bezier(0.2, 0.8, 0.2, 1)",
          // Compose with the card's own transform instead of replacing it. A
          // tapped card carries rotate(7deg); replacing that would straighten
          // it for the length of the glide and snap it back at the end, which
          // reads exactly like untapping and re-tapping.
          composite: "add",
        },
      );
    };

    // A true zone change creates a fresh GameObjectId, so exact-id FLIP cannot
    // connect the two rendered nodes. Infer presentation continuity only from
    // visible information and only within this layout pass. Each disappearing
    // predecessor can be consumed once; the correlation is never stored or
    // sent back to the engine.
    const disappearing = new Map(
      Array.from(previous).filter(([id]) => !entries.has(id)),
    );
    const presentationPredecessors = new Map<
      number,
      { id: number; snapshot: CardPresentationRect }
    >();
    entries.forEach((entry, id) => {
      if (previous.has(id)) return;
      // Four identical Savannah Lions all match each other on owner and name,
      // so taking the first candidate would pick by map order and fly the
      // wrong one. Take the nearest instead: the shortest glide is both the
      // likeliest pairing and the least wrong one to look at when it isn't.
      let nearestId: number | null = null;
      let nearest: CardPresentationRect | null = null;
      let shortest = Infinity;
      for (const [previousId, before] of disappearing) {
        if (!isPresentationPredecessor(before, entry)) continue;
        const dx = before.rect.left - entry.rect.left;
        const dy = before.rect.top - entry.rect.top;
        const distance = dx * dx + dy * dy;
        if (distance >= shortest) continue;
        shortest = distance;
        nearestId = previousId;
        nearest = before;
      }
      if (nearestId === null || nearest === null) return;
      disappearing.delete(nearestId);
      presentationPredecessors.set(id, { id: nearestId, snapshot: nearest });
    });
    const matchedPredecessorIds = new Set(
      Array.from(presentationPredecessors.values(), (predecessor) => predecessor.id),
    );

    entries.forEach((entry, id) => {
      const before = previous.get(id) ?? presentationPredecessors.get(id)?.snapshot;
      if (before) {
        // The hand re-fans itself whenever a card leaves it, with its own
        // transition on the slot. Gliding the card inside that slot as well
        // sets the two against each other, which is what made the hand
        // jitter every time you played something.
        if (before.zone === "hand" && entry.zone === "hand") return;
        const dx = before.rect.left - entry.rect.left;
        const dy = before.rect.top - entry.rect.top;
        if (Math.abs(dx) > 6 || Math.abs(dy) > 6) flyFrom(entry, before.rect, false);
        return;
      }
      const origin =
        entry.owner === "opponent"
          ? anchorRect(".opponent-hand")
          : entry.zone === "hand"
            ? anchorRect('.player-bar:not(.player-opponent) .zone-counts span[title="Library"]')
            : anchorRect(".player-bar:not(.player-opponent) .zone-counts");
      if (origin) flyFrom(entry, origin, true);
    });

    previous.forEach((before, id) => {
      if (entries.has(id) || matchedPredecessorIds.has(id)) return;
      const grave = anchorRect(
        before.owner === "opponent"
          ? '.player-opponent .zone-counts span[title="Graveyard"]'
          : '.player-bar:not(.player-opponent) .zone-counts span[title="Graveyard"]',
      );
      if (!grave) return;
      const ghost = document.createElement("div");
      ghost.className = "card-ghost";
      ghost.textContent = before.name;
      ghost.style.left = `${before.rect.left}px`;
      ghost.style.top = `${before.rect.top}px`;
      ghost.style.width = `${before.rect.width}px`;
      ghost.style.height = `${before.rect.height}px`;
      document.body.appendChild(ghost);
      const dx = grave.left + grave.width / 2 - (before.rect.left + before.rect.width / 2);
      const dy = grave.top + grave.height / 2 - (before.rect.top + before.rect.height / 2);
      const animation = ghost.animate(
        [
          { transform: "none", opacity: 1 },
          { transform: `translate(${dx}px, ${dy}px) scale(0.12)`, opacity: 0.1 },
        ],
        { duration: 560, easing: "cubic-bezier(0.5, 0, 0.75, 0.6)" },
      );
      animation.onfinish = () => ghost.remove();
      animation.oncancel = () => ghost.remove();
    });
  }, [state]);

  const act = (action: Action) => {
    try {
      game.current?.act(action.index);
      refresh();
    } catch (cause) {
      setError(String(cause));
    }
  };

  const prepareAction = (action: Action) => {
    act(action);
  };

  const cancelPendingAction = () => {
    setPendingAction(null);
    pendingActionRef.current = null;
  };

  const confirmPendingAction = () => {
    const action = pendingActionRef.current;
    if (action) act(action);
  };

  const submitDecision = (options: number[]) => {
    if (!state?.decision) return;
    try {
      game.current?.choose_decision(
        state.decision.id,
        JSON.stringify(options),
      );
      setDecisionSelectionState({ decisionId: null, options: [] });
      refresh();
    } catch (cause) {
      setError(String(cause));
    }
  };

  const togglePhaseStop = (phase: string, enabled: boolean) => {
    try {
      game.current?.set_phase_stop(phase, enabled);
      // Re-read the engine snapshot so the pass label reflects the new stop.
      refresh();
    } catch (cause) {
      setError(String(cause));
    }
  };

  const toggleAutopass = (enabled: boolean) => {
    try {
      game.current?.set_autopass(enabled);
      refresh();
    } catch (cause) {
      setError(String(cause));
    }
  };

  const attackAll = () => {
    try {
      game.current?.attack_all();
      refresh();
    } catch (cause) {
      setError(String(cause));
    }
  };

  const finalizeBlocks = () => {
    try {
      const assignments = Object.entries(blockAssignments).map(([blocker, attacker]) => [
        Number(blocker),
        attacker,
      ]);
      game.current?.finalize_blocks(JSON.stringify(assignments));
      refresh();
    } catch (cause) {
      setError(String(cause));
    }
  };

  const beginCardDrag = (id: number) => {
    const matching = state?.actions.filter((action) => action.cardId === id) ?? [];
    const targeted = matching.filter(
      (action) =>
        (action.spellAction || (declaringBlockers && action.kind === "combat")) &&
        !action.manaAbility &&
        singleTargetKey(action) !== null,
    );
    const playable = matching.filter(
      (action) =>
        !action.manaAbility &&
        !hasActionTargets(action),
    );
    dragDropped.current = false;
    if (
      targeted.length > 0 &&
      playable.length === 0 &&
      new Set(targeted.map((action) => action.x ?? null)).size === 1
    ) {
      setDraggingCardId(id);
      setDragOverTarget(null);
    } else if (playable.length === 1) {
      setPendingAction(playable[0]);
      pendingActionRef.current = playable[0];
    } else if (playable.length > 1) {
      selectCard(id);
    }
  };

  const finishCardDrag = () => {
    if (!dragDropped.current) cancelPendingAction();
    setDraggingCardId(null);
    setDragOverTarget(null);
    dragDropped.current = false;
  };

  const previewPaymentForCard = (id: number) => {
    const payment = state?.actions.find(
      (action) => action.cardId === id && action.paymentAction,
    );
    setHoverPaymentAction(payment ?? null);
  };

  const clearPaymentPreview = () => setHoverPaymentAction(null);

  const declaringBlockers =
    state?.step === "Declare Blockers" &&
    state.actions.some((action) => action.label === "Finish blocking");
  // The engine only asks about damage when one attacker is split between
  // several blockers, so there is exactly one attacker to name.
  const assigningDamageFor = state?.actions.find(
    (action) => action.combatDamageAttacker != null,
  )?.combatDamageAttacker;
  const preparingBlockers =
    state?.step === "Declare Attackers" &&
    state.active === "Opponent" &&
    state.priority === "You" &&
    state.battlefield.some((card) => card.attacking);
  const actionMatchesSelectedX = useCallback(
    (action: Action) =>
      selectedX === null || action.cardId !== selectedCard || action.x === selectedX,
    [selectedCard, selectedX],
  );
  const selectedHandCard = state?.human.hand.find((card) => card.id === selectedCard);
  const choosingFireballTargets =
    selectedHandCard?.name === "Fireball" && selectedX !== null;
  const fireballActions =
    state?.actions.filter(
      (action) =>
        choosingFireballTargets &&
        action.cardId === selectedCard &&
        action.x === selectedX,
    ) ?? [];
  const fireballCastAction = fireballActions.find((action) =>
    sameTargets(actionTargetKeys(action), fireballTargets),
  );
  const canChooseFireballTarget = (target: string) => {
    if (!choosingFireballTargets) return false;
    if (fireballTargets.includes(target)) return true;
    const proposed = [...fireballTargets, target];
    return fireballActions.some((action) =>
      proposed.every((candidate) => actionTargetKeys(action).includes(candidate)),
    );
  };
  const toggleFireballTarget = (target: string) => {
    if (!canChooseFireballTarget(target)) return;
    setFireballTargets((current) =>
      current.includes(target)
        ? current.filter((candidate) => candidate !== target)
        : [...current, target],
    );
  };
  const panelActions = (() => {
    if (!state) return [];
    const sourceActions = state.actions.filter(
      (action) => action.cardId === selectedCard && actionMatchesSelectedX(action),
    );
    const sourceHasTargets = sourceActions.some(
      (action) =>
        action.targetCardId != null ||
        action.targetPlayer != null ||
        action.targetStackId != null,
    );
    return state.actions.filter((action) => {
      if (action.kind === "danger") return false;
      if (state.decision && action.decisionId === state.decision.id) return false;
      if (!actionMatchesSelectedX(action)) return false;
      // Splitting damage is the only thing the game is waiting for, so it is
      // listed outright rather than hidden behind selecting the attacker.
      if (action.combatDamageAttacker != null) return true;
      if (declaringBlockers && action.kind === "combat") return false;
      if (declaringBlockers && action.label === "Finish blocking") return false;
      if (action.kind === "pass" || action.cardId == null) return true;
      if (action.cardId !== selectedCard) return false;
      if (selectedTargetCard !== null) {
        return action.targetCardId === selectedTargetCard;
      }
      if (selectedTargetPlayer !== null) {
        return action.targetPlayer === selectedTargetPlayer;
      }
      if (selectedTargetStackId !== null) {
        return action.targetStackId === selectedTargetStackId;
      }
      return !sourceHasTargets && sourceActions.length > 1;
    });
  })();
  const dangerActions = state?.actions.filter((action) => action.kind === "danger") ?? [];
  const attackAllCount =
    state?.step === "Declare Attackers"
      ? state.actions.filter((action) => action.label.startsWith("Attack with ")).length
      : 0;
  const cancelDecisionAction = state?.decision
    ? state.actions.find(
        (action) =>
          action.decisionId === state.decision?.id && action.label === "Cancel",
      )
    : undefined;
  const chooseDecisionOption = (optionId: number) => {
    if (!state?.decision) return;
    if (state.decision.minimum === 1 && state.decision.maximum === 1) {
      submitDecision([optionId]);
      return;
    }
    setDecisionSelectionState((current) => {
      const selected =
        current.decisionId === state.decision!.id ? current.options : [];
      if (selected.includes(optionId)) {
        return {
          decisionId: state.decision!.id,
          options: selected.filter((candidate) => candidate !== optionId),
        };
      }
      if (selected.length >= state.decision!.maximum) return current;
      return {
        decisionId: state.decision!.id,
        options: [...selected, optionId],
      };
    });
  };

  const moveTriggerOrderOption = (optionId: number, offset: -1 | 1) => {
    if (!isTriggerOrderDecision(state?.decision)) return;
    setDecisionSelectionState((current) => {
      const optionIds = state.decision!.options.map((option) => option.id);
      const order =
        current.decisionId === state.decision!.id &&
        current.options.length === optionIds.length &&
        optionIds.every((option) => current.options.includes(option))
          ? [...current.options]
          : optionIds;
      const index = order.indexOf(optionId);
      const destination = index + offset;
      if (index < 0 || destination < 0 || destination >= order.length) return current;
      [order[index], order[destination]] = [order[destination], order[index]];
      return { decisionId: state.decision!.id, options: order };
    });
  };

  const opponentPermanents =
    state?.battlefield.filter((card) => card.owner === "opponent") ?? [];
  const humanPermanents =
    state?.battlefield.filter((card) => card.owner === "human") ?? [];

  const cardActions = (id: number) =>
    watchingOpponent
      ? 0
      : state?.actions.filter(
          (action) =>
            action.cardId === id ||
            (action.cardId === selectedCard &&
              ((action.targetCardIds ?? []).includes(id) || action.targetCardId === id) &&
              actionMatchesSelectedX(action)) ||
            (action.cardId === selectedBlocker && action.targetCardId === id),
        ).length ?? 0;
  const dragTargetActionsForCard = (id: number) =>
    state?.actions.filter(
      (action) =>
        action.cardId === id &&
        (action.spellAction || (declaringBlockers && action.kind === "combat")) &&
        !action.manaAbility &&
        // Dropping on a target would also settle the sacrifice, which the
        // player has to choose deliberately.
        (action.sacrificeCardIds?.length ?? 0) === 0 &&
        singleTargetKey(action) !== null,
    ) ?? [];
  const draggingTargetActions = draggingCardId === null
    ? []
    : dragTargetActionsForCard(draggingCardId);
  const canDragTarget = (target: string) =>
    draggingCardId !== null &&
    draggingTargetActions.some((action) => actionTargetKeys(action).includes(target));
  const handleTargetDragOver = (target: string) => {
    if (canDragTarget(target)) setDragOverTarget(target);
  };
  const handleTargetDragLeave = (target: string) => {
    setDragOverTarget((current) => (current === target ? null : current));
  };
  const handleTargetDrop = (target: string) => {
    if (!canDragTarget(target)) return;
    const action = draggingTargetActions.find((candidate) =>
      actionTargetKeys(candidate).includes(target),
    );
    if (!action) return;
    dragDropped.current = true;
    setDraggingCardId(null);
    setDragOverTarget(null);
    if (declaringBlockers && action.kind === "combat" && action.targetCardId != null) {
      setBlockAssignments((current) => ({
        ...current,
        [action.cardId as number]: action.targetCardId as number,
      }));
      setSelectedBlocker(null);
      return;
    }
    prepareAction(action);
  };
  const cardIsDraggable = (id: number) => {
    if (watchingOpponent) return false;
    if (
      declaringBlockers &&
      state?.battlefield.some((card) => card.owner === "human" && card.id === id) &&
      dragTargetActionsForCard(id).length > 0
    ) {
      return true;
    }
    if (!state?.human.hand.some((card) => card.id === id)) return false;
    const directActions = state.actions.filter(
      (action) =>
        action.cardId === id &&
        !action.manaAbility &&
        !hasActionTargets(action),
    );
    const targetedActions = dragTargetActionsForCard(id);
    const hasSingleXValue = new Set(targetedActions.map((action) => action.x ?? null)).size <= 1;
    return (directActions.length === 1 && targetedActions.length === 0) ||
      (directActions.length === 0 && targetedActions.length > 0 && hasSingleXValue);
  };

  const isTargetable = (id: number) =>
    !watchingOpponent &&
    (canDragTarget(cardTargetKey(id)) ||
      (choosingFireballTargets && canChooseFireballTarget(cardTargetKey(id))) ||
      (declaringBlockers &&
      selectedBlocker !== null &&
      (state?.actions.some(
        (action) =>
          action.cardId === selectedBlocker && action.targetCardId === id,
      ) ?? false)) ||
      (selectedCard !== null &&
        (state?.actions.some(
          (action) =>
            action.cardId === selectedCard &&
            action.targetCardId === id &&
            actionMatchesSelectedX(action),
        ) ?? false)));

  const isPlayerTargetable = (owner: Owner) =>
    !watchingOpponent &&
    (draggingCardId !== null || selectedCard !== null) &&
    (canDragTarget(playerTargetKey(owner)) ||
      (choosingFireballTargets && canChooseFireballTarget(playerTargetKey(owner))) ||
      (state?.actions.some(
        (action) =>
          action.cardId === selectedCard &&
          action.targetPlayer === owner &&
          actionMatchesSelectedX(action),
      ) ?? false));

  const isStackTargetable = (id: number) =>
    !watchingOpponent &&
    (draggingCardId !== null || selectedCard !== null) &&
    (canDragTarget(`stack:${id}`) ||
      (state?.actions.some(
        (action) =>
          action.cardId === selectedCard &&
          action.targetStackId === id &&
          actionMatchesSelectedX(action),
      ) ?? false));

  const selectedSource = state?.battlefield
    .concat(state.human.hand)
    .find((card) => card.id === selectedCard);
  const actionMenuSource = state?.battlefield
    .concat(state.human.hand)
    .find((card) => card.id === cardActionMenu);
  const xPickerSource = state?.human.hand.find((card) => card.id === xPickerCard);
  const xPickerValues = Array.from(
    new Set(
      state?.actions
        .filter((action) => action.cardId === xPickerCard && action.x != null)
        .map((action) => action.x as number) ?? [],
    ),
  ).sort((left, right) => left - right);
  const previewedPayment = fireballCastAction ?? pendingAction ?? hoverPaymentAction;
  const actionMenuActions =
    state?.actions.filter(
      (action) =>
        action.cardId === cardActionMenu &&
        action.targetCardId == null &&
        action.targetPlayer == null &&
        action.targetStackId == null,
    ) ?? [];
  const actionMenuTargetedActions =
    cardActionMenu === null
      ? []
      : (state?.actions.filter(
          (action) =>
            action.cardId === cardActionMenu &&
            (action.targetCardId != null ||
              action.targetPlayer != null ||
              action.targetStackId != null),
        ) ?? []);
  const actionMenuHasTargets = actionMenuTargetedActions.length > 0;
  // "Choose a target" says nothing about what the target is for, so the menu
  // reads back the ability whenever the engine can describe it.
  const actionMenuTargetLabel =
    actionMenuTargetedActions.find((action) => action.abilitySummary)?.abilitySummary ??
    "Choose a target on the battlefield";
  const choosingTarget =
    !choosingFireballTargets &&
    selectedCard !== null &&
    selectedTargetCard === null &&
    selectedTargetPlayer === null &&
    selectedTargetStackId === null &&
    (state?.actions.some(
      (action) =>
        action.cardId === selectedCard &&
        actionMatchesSelectedX(action) &&
        (action.targetCardId != null ||
          action.targetPlayer != null ||
          action.targetStackId != null),
    ) ?? false);
  const selectedAbilitySummary = state?.actions.find(
    (action) => action.cardId === selectedCard && action.abilitySummary,
  )?.abilitySummary;
  const selectedTarget = state?.battlefield.find(
    (card) => card.id === selectedTargetCard,
  );
  const choosingSacrifice = selectedCard !== null && selectedTargetCard !== null;

  const clearCardSelection = () => {
    setSelectedCard(null);
    setSelectedTargetCard(null);
    setSelectedTargetPlayer(null);
    setSelectedTargetStackId(null);
    setCardActionMenu(null);
    setSelectedX(null);
    setXPickerCard(null);
    setFireballTargets([]);
  };

  const selectPlayer = (owner: Owner) => {
    if (selectedCard === null) return;
    if (choosingFireballTargets) {
      toggleFireballTarget(playerTargetKey(owner));
      return;
    }
    const matching =
      state?.actions.filter(
        (action) =>
          action.cardId === selectedCard &&
          action.targetPlayer === owner &&
          actionMatchesSelectedX(action),
      ) ?? [];
    if (matching.length === 1) {
      prepareAction(matching[0]);
    } else if (matching.length > 1) {
      setSelectedTargetCard(null);
      setSelectedTargetPlayer(owner);
      setSelectedTargetStackId(null);
    }
  };

  const selectStackTarget = (id: number) => {
    if (selectedCard === null) return;
    const matching =
      state?.actions.filter(
        (action) =>
          action.cardId === selectedCard &&
          action.targetStackId === id &&
          actionMatchesSelectedX(action),
      ) ?? [];
    if (matching.length === 1) {
      prepareAction(matching[0]);
    } else if (matching.length > 1) {
      setSelectedTargetCard(null);
      setSelectedTargetPlayer(null);
      setSelectedTargetStackId(id);
    }
  };

  const selectCard = (id: number) => {
    if (declaringBlockers) {
      const blockerOptions =
        state?.actions.filter(
          (action) => action.cardId === id && action.targetCardId != null,
        ) ?? [];
      if (blockerOptions.length > 0) {
        if (selectedBlocker === id) {
          setSelectedBlocker(null);
          setBlockAssignments((current) => {
            const next = { ...current };
            delete next[id];
            return next;
          });
        } else {
          setSelectedBlocker(id);
        }
        return;
      }
      if (selectedBlocker !== null) {
        const assignment = state?.actions.find(
          (action) =>
            action.cardId === selectedBlocker && action.targetCardId === id,
        );
        if (assignment) {
          setBlockAssignments((current) => ({ ...current, [selectedBlocker]: id }));
          setSelectedBlocker(null);
          return;
        }
      }
    }
    if (choosingFireballTargets && canChooseFireballTarget(cardTargetKey(id))) {
      toggleFireballTarget(cardTargetKey(id));
      return;
    }
    if (selectedCard !== null) {
      const targeted =
        state?.actions.filter(
          (action) =>
            action.cardId === selectedCard &&
            action.targetCardId === id &&
            actionMatchesSelectedX(action),
        ) ?? [];
      if (targeted.length === 1) {
        prepareAction(targeted[0]);
        return;
      }
      if (targeted.length > 1) {
        setSelectedTargetCard(id);
        setSelectedTargetPlayer(null);
        setSelectedTargetStackId(null);
        return;
      }
      if (id === selectedCard) {
        clearCardSelection();
        return;
      }
    }

    const matching =
      state?.actions.filter((action) => action.cardId === id) ?? [];
    if (selectedCard !== null && selectedCard !== id) {
      setSelectedX(null);
      setXPickerCard(null);
      setFireballTargets([]);
    }
    const source = state?.human.hand.find((card) => card.id === id);
    const xValues = Array.from(
      new Set(
        matching
          .filter((action) => action.x != null)
          .map((action) => action.x as number),
      ),
    );
    if (source?.manaCost?.x && selectedCard !== id && xValues.length > 1) {
      setSelectedCard(id);
      setSelectedTargetCard(null);
      setSelectedTargetPlayer(null);
      setSelectedTargetStackId(null);
      setSelectedX(null);
      setXPickerCard(id);
      setFireballTargets([]);
      return;
    }
    const untargeted = matching.filter(
      (action) =>
        action.targetCardId == null &&
        action.targetPlayer == null &&
        action.targetStackId == null,
    );
    const hasTargetedAction = matching.some(
      (action) =>
        action.targetCardId != null ||
        action.targetPlayer != null ||
        action.targetStackId != null,
    );
    // Destroying one of your own permanents is never something a single click
    // should decide for you, even when there is only one thing it could take.
    const costsSacrifice = matching.some(
      (action) => (action.sacrificeCardIds?.length ?? 0) > 0,
    );
    if (
      untargeted.length > 1 ||
      (untargeted.length > 0 && hasTargetedAction) ||
      (costsSacrifice && !hasTargetedAction)
    ) {
      setSelectedCard(null);
      setSelectedTargetCard(null);
      setSelectedTargetPlayer(null);
      setSelectedTargetStackId(null);
      setCardActionMenu(id);
      return;
    }
    // A lone legal target still gets picked out on the board by hand, so the
    // player always sees what the spell is about to hit.
    if (hasTargetedAction) {
      setSelectedCard(id);
      setSelectedTargetCard(null);
      setSelectedTargetPlayer(null);
      setSelectedTargetStackId(null);
      return;
    }
    if (matching.length === 1) {
      prepareAction(matching[0]);
    } else if (matching.length > 1) {
      if (selectedCard === id) {
        clearCardSelection();
      } else {
        setSelectedCard(id);
        setSelectedTargetCard(null);
        setSelectedTargetPlayer(null);
        setSelectedTargetStackId(null);
      }
    }
  };

  const openSetup = () => {
    setDraftFormat(format);
    setDraftHumanDeck(humanDeckChoice);
    setDraftBotDeck(botDeckChoice);
    setDraftPolicy(policy);
    setDraftHumanFirst(humanFirst);
    setDraftCardArtMode(cardArtMode);
    setSetupOpen(true);
  };

  const startConfiguredGame = () => {
    if (!wasmReady.current) return;
    const started = newGame(
      setupDismissible ? randomSeed() : seed,
      draftHumanDeck,
      draftBotDeck,
      draftPolicy,
      draftHumanFirst,
      draftFormat,
    );
    if (!started) return;
    setHumanDeckChoice(draftHumanDeck);
    setBotDeckChoice(draftBotDeck);
    setPolicy(draftPolicy);
    setHumanFirst(draftHumanFirst);
    setCardArtMode(draftCardArtMode);
    setSetupDismissible(true);
    setSetupOpen(false);
  };

  const undoMana = () => {
    try {
      game.current?.undo_mana();
      refresh();
    } catch (cause) {
      setError(String(cause));
    }
  };

  const cancelAttackers = () => {
    try {
      game.current?.cancel_attackers();
      refresh();
    } catch (cause) {
      setError(String(cause));
    }
  };

  return (
    <main className="arena">
      {setupOpen && (
        <div className="setup-backdrop">
          <section
            className="setup-dialog"
            role="dialog"
            aria-modal="true"
            aria-labelledby="setup-title"
          >
            <span className="setup-kicker">NEW MATCH</span>
            <h1 id="setup-title">Choose your deck</h1>
            <p>Choose a format, then pick both sides of the matchup.</p>
            {error && <div className="setup-error" role="alert">{error}</div>}
            <label className="setup-format">
              <span>Format</span>
              <select
                value={draftFormat}
                onChange={(event) => {
                  const nextFormat = event.target.value as FormatId;
                  setDraftFormat(nextFormat);
                  setDraftHumanDeck(defaultHumanDeck);
                  setDraftBotDeck(defaultBotDeck);
                }}
              >
                {Object.entries(formatConfigs).map(([id, config]) => (
                  <option key={id} value={id}>{config.name}</option>
                ))}
              </select>
              <small>{formatConfigs[draftFormat].description}</small>
            </label>
            <label className="setup-format setup-art-mode">
              <span>Card images</span>
              <select
                value={draftCardArtMode}
                onChange={(event) => setDraftCardArtMode(event.target.value as CardArtMode)}
              >
                <option value="full">Full cards</option>
                <option value="cropped">Cropped artwork</option>
                <option value="off">Symbols only</option>
              </select>
              <small>
                {draftCardArtMode === "full"
                  ? "Show the complete printed card image."
                  : draftCardArtMode === "cropped"
                    ? "Place the illustration inside Penta’s card frame."
                    : "Do not request card images."}
              </small>
            </label>
            <div className="setup-fields">
              <div className="setup-primary-choice">
                <label>
                  <span>Your deck</span>
                  <select
                    value={draftHumanDeck}
                    onChange={(event) => setDraftHumanDeck(event.target.value)}
                  >
                    {draftDeckChoices.map((deck) => (
                      <option key={deck}>{deck}</option>
                    ))}
                  </select>
                  <small>{deckChoiceNote(draftFormat, draftHumanDeck)}</small>
                </label>
                <label className="setup-seat">
                  <input
                    type="checkbox"
                    checked={draftHumanFirst}
                    onChange={(event) => setDraftHumanFirst(event.target.checked)}
                  />
                  <span>You play first</span>
                </label>
              </div>
              <div className="setup-opponent-choices">
                <label>
                  <span>Opponent deck</span>
                  <select
                    value={draftBotDeck}
                    onChange={(event) => setDraftBotDeck(event.target.value)}
                  >
                    {draftDeckChoices.map((deck) => (
                      <option key={deck}>{deck}</option>
                    ))}
                  </select>
                  <small>{deckChoiceNote(draftFormat, draftBotDeck)}</small>
                </label>
                <label className="setup-policy">
                  <span>Opponent style</span>
                  <select
                    value={draftPolicy}
                    onChange={(event) => setDraftPolicy(event.target.value)}
                  >
                    <option>Handcrafted</option>
                    <option>Random</option>
                  </select>
                  <small>
                    {draftPolicy === "Handcrafted"
                      ? "Purposeful, card-aware play."
                      : "Chooses randomly from legal actions."}
                  </small>
                </label>
              </div>
            </div>
            <div className="setup-actions">
              {setupDismissible && (
                <button className="setup-cancel" onClick={() => setSetupOpen(false)}>
                  Back to game
                </button>
              )}
              <button
                className="setup-start"
                onClick={startConfiguredGame}
                disabled={!engineReady}
              >
                {loading ? "Loading engine…" : "Deal game"}
              </button>
            </div>
          </section>
        </div>
      )}

      {xPickerCard !== null && xPickerSource && (
        <div className="card-action-backdrop">
          <section
            className="card-action-dialog x-picker-dialog"
            role="dialog"
            aria-modal="true"
            aria-labelledby="x-picker-title"
          >
            <span>CHOOSE X</span>
            <h2 id="x-picker-title">Cast {xPickerSource.name}</h2>
            <p>How much mana should X be?</p>
            <div className="x-picker-options">
              {xPickerValues.map((value) => (
                <button
                  key={value}
                  onClick={() => {
                    setSelectedX(value);
                    setFireballTargets([]);
                    setXPickerCard(null);
                  }}
                >
                  <strong>X = {value}</strong>
                </button>
              ))}
            </div>
            <button className="card-action-cancel" onClick={clearCardSelection}>
              Cancel
            </button>
          </section>
        </div>
      )}

      {cardActionMenu !== null && actionMenuSource && (
        <div className="card-action-backdrop">
          <section
            className="card-action-dialog"
            role="dialog"
            aria-modal="true"
            aria-labelledby="card-action-title"
          >
            <span>CHOOSE AN ACTION</span>
            <h2 id="card-action-title">{actionMenuSource.name}</h2>
            <div>
              {actionMenuActions.map((action) => (
                <button key={action.index} onClick={() => prepareAction(action)}>
                  <strong>{action.label}</strong>
                  <i aria-hidden="true">→</i>
                </button>
              ))}
              {actionMenuHasTargets && (
                <button
                  onClick={() => {
                    setSelectedCard(cardActionMenu);
                    setCardActionMenu(null);
                  }}
                >
                  <strong>{actionMenuTargetLabel}</strong>
                  <i aria-hidden="true">→</i>
                </button>
              )}
            </div>
            <button className="card-action-cancel" onClick={clearCardSelection}>
              Cancel
            </button>
          </section>
        </div>
      )}

      {loading && (
        <section className="engine-loading" role="status">
          <span className="loader-rune">R</span>
          <p>Waking the Rust engine…</p>
        </section>
      )}

      {error && (
        <div className="error-banner" role="alert">
          {error}
        </div>
      )}

      {state && (
        <div className="game-layout">
          <section
            ref={tableRef}
            className={`table ${pendingAction ? "is-payment-drop-target" : ""}`}
            aria-label="Game table"
            onDragOver={(event) => {
              if (pendingActionRef.current) event.preventDefault();
            }}
            onDrop={(event) => {
              if (!pendingActionRef.current) return;
              event.preventDefault();
              dragDropped.current = true;
              confirmPendingAction();
            }}
          >
            <StackTargetArrows stack={state.stack} state={state} tableRef={tableRef} />
            {declaringBlockers && (
              <BlockArrows
                assignments={blockAssignments}
                tableRef={tableRef}
              />
            )}
            {/* The masthead shares its row with their hand and their panel, so
                the top of the table costs one row instead of three. */}
            <div className="table-header">
              <div className="brand">
                <span className="brand-mark" aria-hidden="true">P</span>
                <div>
                  <strong>PENTA</strong>
                  <small>{formatConfigs[format].shortName}</small>
                </div>
              </div>
              <div className="opponent-hand" aria-label={`${state.opponent.handSize} hidden cards`}>
                {Array.from({ length: state.opponent.handSize }, (_, index) => (
                  <span className="card-back" key={index}>
                    <i>{formatConfigs[format].cardBackMark}</i>
                  </span>
                ))}
              </div>
              <PlayerBar
                player={state.opponent}
                opponent
                targetable={isPlayerTargetable("opponent")}
                selected={
                  fireballTargets.includes(playerTargetKey("opponent")) ||
                  dragOverTarget === playerTargetKey("opponent")
                }
                onTarget={() => selectPlayer("opponent")}
                onDragOverTarget={() => handleTargetDragOver(playerTargetKey("opponent"))}
                onDragLeaveTarget={() => handleTargetDragLeave(playerTargetKey("opponent"))}
                onDropTarget={() => handleTargetDrop(playerTargetKey("opponent"))}
              />
            </div>

            <Zone
              cards={opponentPermanents}
              cardArtMode={cardArtMode}
              label="Opponent battlefield"
              actionCount={cardActions}
              isDraggable={cardIsDraggable}
              isTargetable={isTargetable}
              onSelect={selectCard}
              selectedCard={selectedTargetCard ?? selectedCard}
              selectedCardIds={fireballTargets
                .filter((target) => target.startsWith("card:"))
                .map((target) => Number(target.slice(5)))}
              animatedCardId={currentOpponentAction?.cardId ?? null}
              previewManaSourceIds={previewedPayment?.manaSourceIds ?? []}
              onDragStartCard={beginCardDrag}
              onDragEndCard={finishCardDrag}
              dragOverTarget={dragOverTarget}
              onDragOverTarget={handleTargetDragOver}
              onDragLeaveTarget={handleTargetDragLeave}
              onDropTarget={handleTargetDrop}
              opponent
            />

            <div
              className={`stack-zone ${state.stack.length === 0 ? "stack-zone-empty" : ""}`}
              aria-label="Stack"
            >
              {turnBanner && (
                <div
                  className={`turn-banner ${
                    turnBanner.active === "You" ? "turn-banner-yours" : ""
                  }`}
                  key={`${turnBanner.pregame ? "pregame" : turnBanner.active}-${turnBanner.turn}`}
                  role="status"
                  aria-live="polite"
                >
                  {/* The opening banner answers the two things you cannot
                      read off the board yet: who is on the play, and — when
                      you left the choice to chance — what you are playing. */}
                  <strong>
                    {turnBanner.pregame
                      ? turnBanner.active === "You"
                        ? "You go first"
                        : "Opponent goes first"
                      : turnBanner.active === "You"
                        ? "Your turn"
                        : "Opponent’s turn"}
                  </strong>
                  <small>{turnBanner.pregame ? humanDeck : `Turn ${turnBanner.turn}`}</small>
                </div>
              )}
              {/* The row's own frame says where the stack is; a card sitting
                  in it says what is on it. Neither needs labelling. */}
              {state.stack.map((item) => {
                const triggered = item.kind === "TriggeredAbility";
                return (
                  <div
                    className={`stack-card-slot ${triggered ? "is-triggered-ability" : ""}`}
                    data-stack-kind={item.kind}
                    key={item.id}
                  >
                    <GameCard
                      card={{
                        id: item.id,
                        name: item.name,
                        art: triggered ? null : item.art,
                        kind: triggered ? "triggeredability" : item.cardKind,
                        typeLine: triggered ? "Triggered Ability" : item.typeLine,
                        metadataOnly: item.metadataOnly,
                        isLand: triggered ? false : item.isLand,
                        manaCost: triggered ? null : item.manaCost,
                        rulesText: item.abilityText ?? item.rulesText,
                        power: triggered ? null : item.power,
                        toughness: triggered ? null : item.toughness,
                        owner: item.owner,
                        xValue: !triggered && item.manaCost?.x ? item.x : null,
                      }}
                      cardArtMode={cardArtMode}
                      zone="stack"
                      targetKey={`stack:${item.id}`}
                      actionable={isStackTargetable(item.id)}
                      targetable={isStackTargetable(item.id)}
                      selected={false}
                      dragOverTarget={dragOverTarget === `stack:${item.id}`}
                      onSelect={() => selectStackTarget(item.id)}
                      onDragOverTarget={handleTargetDragOver}
                      onDragLeaveTarget={handleTargetDragLeave}
                      onDropTarget={handleTargetDrop}
                      compact
                    />
                    {item.manaCost?.x ? (
                      <span className="stack-x-badge">X = {item.x}</span>
                    ) : null}
                    <small className="stack-card-owner">
                      {item.owner === "human" ? "YOU" : "OPPONENT"}
                      {triggered ? " · TRIGGERED" : ""}
                    </small>
                  </div>
                );
              })}
            </div>

            <Zone
              cards={humanPermanents}
              cardArtMode={cardArtMode}
              label="Your battlefield"
              actionCount={cardActions}
              isDraggable={cardIsDraggable}
              isTargetable={isTargetable}
              onSelect={selectCard}
              selectedCard={selectedBlocker ?? selectedTargetCard ?? selectedCard}
              selectedCardIds={fireballTargets
                .filter((target) => target.startsWith("card:"))
                .map((target) => Number(target.slice(5)))}
              animatedCardId={currentOpponentAction?.cardId ?? null}
              previewManaSourceIds={previewedPayment?.manaSourceIds ?? []}
              onDragStartCard={beginCardDrag}
              onDragEndCard={finishCardDrag}
              dragOverTarget={dragOverTarget}
              onDragOverTarget={handleTargetDragOver}
              onDragLeaveTarget={handleTargetDragLeave}
              onDropTarget={handleTargetDrop}
            />

            {/* Your panel rides the phase strip: it is the only other thing
                that belongs to you rather than to the board. */}
            <div className="center-line">
              {/* While the banner announces a turn the strip announces the
                  same one, with nothing lit: the board is still showing the
                  turn that just ended, but the turn it belongs to is over.
                  Nobody's turn has started while hands are being settled
                  either, so the strip names that decision instead of a step. */}
              <div className="turn-status">
                <strong>
                  {strip.pregame
                    ? "Keep or mull"
                    : strip.active === "You"
                      ? "Your turn"
                      : "Opponent’s turn"}
                </strong>
                <span>{strip.pregame ? "Opening hand" : `Turn ${strip.turn}`}</span>
              </div>
              <ol
                className="phase-track"
                aria-label={
                  strip.step === null
                    ? "Between turns. Click a phase to set or remove a stop."
                    : `Current step: ${strip.step}. Click a phase to set or remove a stop.`
                }
              >
                {turnPhases.map((phase) => {
                  const current = phase.steps.some((step) => step === strip.step);
                  const stopped = state.phaseStops.includes(phase.label);
                  return (
                    <li
                      className={`${current ? "phase-current" : ""} ${stopped ? "phase-stopped" : ""}`}
                      key={phase.label}
                    >
                      <button
                        type="button"
                        aria-pressed={stopped}
                        title={`${stopped ? "Remove" : "Set"} stop on ${phase.title}`}
                        onClick={() => togglePhaseStop(phase.label, !stopped)}
                      >
                        <span>{phase.title}</span>
                        {current && <small>{strip.step}</small>}
                        {stopped && <i aria-label="Stop set">STOP</i>}
                      </button>
                    </li>
                  );
                })}
              </ol>
              <button
                type="button"
                className={`autopass-toggle ${state.autopassEnabled ? "is-on" : ""}`}
                aria-pressed={state.autopassEnabled}
                title="Automatically yield routine priority windows"
                onClick={() => toggleAutopass(!state.autopassEnabled)}
              >
                <span>Auto-pass</span>
                <i>{state.autopassEnabled ? "On" : "Off"}</i>
              </button>
              <PlayerBar
                player={state.human}
                targetable={isPlayerTargetable("human")}
                selected={
                  fireballTargets.includes(playerTargetKey("human")) ||
                  dragOverTarget === playerTargetKey("human")
                }
                onTarget={() => selectPlayer("human")}
                onDragOverTarget={() => handleTargetDragOver(playerTargetKey("human"))}
                onDragLeaveTarget={() => handleTargetDragLeave(playerTargetKey("human"))}
                onDropTarget={() => handleTargetDrop(playerTargetKey("human"))}
              />
            </div>

            <HandZone
              cards={state.human.hand}
              cardArtMode={cardArtMode}
              actionCount={cardActions}
              isDraggable={cardIsDraggable}
              isTargetable={isTargetable}
              selectedCard={selectedCard}
              previewManaSourceIds={previewedPayment?.manaSourceIds ?? []}
              onSelect={selectCard}
              onDragStartCard={beginCardDrag}
              onDragEndCard={finishCardDrag}
              onPaymentPreviewStart={previewPaymentForCard}
              onPaymentPreviewEnd={clearPaymentPreview}
            />
          </section>

          <aside
            className={`decision-panel ${watchingOpponent ? "is-watching-opponent" : ""}`}
            aria-label="Legal actions"
            aria-busy={watchingOpponent}
          >
            <div className="decision-heading">
              <div>
                <span>{watchingOpponent ? "OPPONENT ACTING" : "YOUR DECISION"}</span>
                <strong>
                  {watchingOpponent
                    ? actionStepsRemaining > 0
                      ? `${actionStepsRemaining} action${actionStepsRemaining === 1 ? "" : "s"}`
                      : "New turn"
                    : isTriggerOrderDecision(state.decision)
                      ? "Order your triggers"
                      : isTriggerPlacementDecision(state.decision)
                        ? "Put your trigger on the stack"
                        : assigningDamageFor != null
                          ? `Assign ${cardName(state, assigningDamageFor)} damage`
                          : declaringBlockers
                            ? "Assign your blockers"
                            : preparingBlockers
                              ? "Prepare your blockers"
                              : draggingCardId !== null
                                ? "Drop on a valid target"
                                : choosingFireballTargets
                                  ? "Select Fireball targets"
                                  : choosingTarget
                                    ? "Choose a highlighted target"
                                    : panelActions.some(
                                          (action) => action.kind === "primary",
                                        )
                                      ? "Choose an option"
                                      : "Choose a card or pass"}
                </strong>
              </div>
              {watchingOpponent ? (
                <button className="skip-opponent-queue" onClick={skipOpponentActions}>
                  Skip animations
                </button>
              ) : selectedCard !== null && (
                <button onClick={clearCardSelection}>
                  {choosingTarget || choosingFireballTargets || choosingSacrifice
                    ? "Cancel"
                    : "Clear filter"}
                </button>
              )}
            </div>
            <div className="action-list">
              {state.decision && (
                isTriggerOrderDecision(state.decision) ? (
                  <div
                    className="engine-decision trigger-order-decision"
                    role="group"
                    aria-label={state.decision.prompt}
                    data-decision-kind="TriggerOrder"
                  >
                    <div className="target-prompt" role="status">
                      <strong>{state.decision.prompt}</strong>
                      <span>
                        Arrange the abilities in resolution order. The first resolves first.
                      </span>
                    </div>
                    <ol className="trigger-order-list">
                      {triggerResolutionOrder.map((optionId, index) => {
                        const option = state.decision!.options.find(
                          (candidate) => candidate.id === optionId,
                        );
                        if (!option) return null;
                        const triggerLabel = option.abilityText ?? option.label;
                        return (
                          <li key={option.id}>
                            <span className="trigger-order-position" aria-hidden="true">
                              {index + 1}
                            </span>
                            <span className="trigger-order-label">
                              <strong title={triggerLabel}>{triggerLabel}</strong>
                              <small>
                                {option.cardName ?? "Triggered ability"}
                                {option.zone !== "None" ? ` · ${option.zone}` : ""}
                              </small>
                            </span>
                            <span className="trigger-order-controls">
                              <button
                                type="button"
                                aria-label={`Move ${triggerLabel} earlier`}
                                title="Resolve sooner"
                                disabled={index === 0 || watchingOpponent}
                                onClick={() => moveTriggerOrderOption(option.id, -1)}
                              >
                                ↑
                              </button>
                              <button
                                type="button"
                                aria-label={`Move ${triggerLabel} later`}
                                title="Resolve later"
                                disabled={
                                  index === triggerResolutionOrder.length - 1 ||
                                  watchingOpponent
                                }
                                onClick={() => moveTriggerOrderOption(option.id, 1)}
                              >
                                ↓
                              </button>
                            </span>
                          </li>
                        );
                      })}
                    </ol>
                    <button
                      className="finalize-decision finalize-trigger-order"
                      disabled={watchingOpponent}
                      onClick={() => submitDecision(triggerResolutionOrder)}
                    >
                      <strong>Confirm resolution order</strong>
                      <small>{triggerResolutionOrder.length} mandatory triggers</small>
                    </button>
                  </div>
                ) : (
                  <div
                    className={`engine-decision ${
                      isTriggerPlacementDecision(state.decision)
                        ? "trigger-placement-decision"
                        : ""
                    }`}
                    role="group"
                    aria-label={state.decision.prompt}
                    data-decision-kind={state.decision.kind}
                  >
                    <div className="target-prompt" role="status">
                      <strong>{state.decision.prompt}</strong>
                      <span>
                        {state.decision.minimum === state.decision.maximum
                          ? `Choose ${state.decision.minimum}`
                          : state.decision.minimum === 0
                            ? `Choose up to ${state.decision.maximum}`
                            : `Choose ${state.decision.minimum}–${state.decision.maximum}`}
                        {isTriggerPlacementDecision(state.decision)
                          ? " · Put on the stack"
                          : ""}
                        {state.decision.visibility === "Private" ? " · Private" : ""}
                      </span>
                    </div>
                    <div className="decision-options">
                      {state.decision.options.map((option) => (
                        <button
                          key={option.id}
                          className={decisionSelection.includes(option.id) ? "is-selected" : ""}
                          onClick={() => chooseDecisionOption(option.id)}
                          disabled={watchingOpponent}
                        >
                          <strong>{option.label}</strong>
                          {(option.abilityText || option.zone !== "None") && (
                            <small>{option.abilityText ?? option.zone}</small>
                          )}
                        </button>
                      ))}
                    </div>
                    {/* Every decision that does not submit on the first click
                        needs a way to commit. A search is the case that made
                        this matter: it allows exactly one card but does not
                        require it, so gating on `maximum > 1` left the player
                        able to select a card and unable to confirm anything. */}
                    {!(state.decision.minimum === 1 && state.decision.maximum === 1) && (
                      <button
                        className="finalize-decision"
                        disabled={
                          decisionSelection.length < state.decision.minimum ||
                          watchingOpponent
                        }
                        onClick={() => submitDecision(decisionSelection)}
                      >
                        <strong>
                          {decisionSelection.length === 0 && state.decision.minimum === 0
                            ? "Choose none"
                            : "Confirm selection"}
                        </strong>
                        <small>
                          {decisionSelection.length} / {state.decision.maximum} selected
                        </small>
                      </button>
                    )}
                    {cancelDecisionAction && (
                      <button
                        className="cancel-decision"
                        onClick={() => prepareAction(cancelDecisionAction)}
                        disabled={watchingOpponent}
                      >
                        Cancel
                      </button>
                    )}
                  </div>
                )
              )}
              {attackAllCount > 0 && !state.canCancelAttackers && (
                <button className="attack-all" onClick={attackAll} disabled={watchingOpponent}>
                  <span aria-hidden="true">⚔</span>
                  <strong>Attack all</strong>
                  <small>{attackAllCount} creature{attackAllCount === 1 ? "" : "s"}</small>
                </button>
              )}
              {state.canCancelAttackers && (
                <button
                  className="cancel-attack"
                  onClick={cancelAttackers}
                  disabled={watchingOpponent}
                >
                  <span aria-hidden="true">↶</span>
                  <strong>Cancel</strong>
                  <small>Take back these attackers</small>
                </button>
              )}
              {choosingFireballTargets && (
                <div className="fireball-target-controls">
                  <div className="target-prompt" role="status">
                    <strong>Fireball · X = {selectedX}</strong>
                    <span>
                      Choose one or more highlighted creatures or players. Each target
                      beyond the first costs 1 additional mana.
                    </span>
                  </div>
                  <div className="fireball-target-summary">
                    {fireballTargets.length === 0 ? (
                      <span>No targets selected</span>
                    ) : (
                      fireballTargets.map((target) => {
                        const [kind, value] = target.split(":");
                        const label =
                          kind === "player"
                            ? value === "human"
                              ? "You"
                              : "Opponent"
                            : state.battlefield.find((card) => card.id === Number(value))
                                ?.name ?? `Card ${value}`;
                        return <span key={target}>{label}</span>;
                      })
                    )}
                  </div>
                  <div className="fireball-cost-summary">
                    <span>X</span>
                    <strong>{selectedX}</strong>
                    <i>+</i>
                    <span>Extra targets</span>
                    <strong>{Math.max(0, fireballTargets.length - 1)}</strong>
                    <i>+</i>
                    <span>Red</span>
                    <strong>R</strong>
                  </div>
                  <button
                    className="cast-fireball"
                    disabled={!fireballCastAction}
                    onClick={() => fireballCastAction && prepareAction(fireballCastAction)}
                  >
                    <strong>Cast Fireball</strong>
                    <small>
                      {fireballTargets.length === 0
                        ? "Choose at least one target"
                        : `${fireballTargets.length} target${fireballTargets.length === 1 ? "" : "s"} · ${Math.max(0, fireballTargets.length - 1)} extra mana`}
                    </small>
                  </button>
                </div>
              )}
              {declaringBlockers && (
                <div className="block-controls">
                  <div className="target-prompt" role="status">
                    <strong>
                      {selectedBlocker === null
                        ? "Choose a blocker"
                        : "Choose an attacker"}
                    </strong>
                    <span>
                      Click one of your highlighted creatures, then the attacker it
                      should block. Click a selected blocker again to remove its block.
                    </span>
                  </div>
                  <button className="finalize-blocks" onClick={finalizeBlocks}>
                    <strong>
                      {Object.keys(blockAssignments).length === 0
                        ? "No blocks"
                        : "Declare blocks"}
                    </strong>
                    <small>
                      {Object.keys(blockAssignments).length === 0
                        ? "Skip blocking and continue"
                        : `${Object.keys(blockAssignments).length} assigned`}
                    </small>
                  </button>
                </div>
              )}
              {state.canUndoMana && (
                <button className="undo-mana" onClick={undoMana}>
                  <span>↶</span>
                  <strong>Undo last mana tap</strong>
                </button>
              )}
              {choosingTarget && (
                <div className="target-prompt" role="status">
                  <strong>{selectedAbilitySummary ?? "Choose a highlighted target"}</strong>
                  <span>
                    Click a highlighted card, player, or spell for{" "}
                    {selectedSource?.name ?? "this action"}
                    {selectedX !== null ? ` with X = ${selectedX}` : ""}.
                  </span>
                </div>
              )}
              {draggingCardId !== null && (
                <div className="target-prompt" role="status">
                  <strong>Drop on a highlighted target</strong>
                  <span>Release the dragged card over a legal card, player, or spell.</span>
                </div>
              )}
              {choosingSacrifice && (
                <div className="target-prompt" role="status">
                  <strong>Choose an artifact to sacrifice</strong>
                  <span>
                    Select a cost below to deal 2 damage to{" "}
                    {selectedTarget?.name ?? "that creature"}.
                  </span>
                </div>
              )}
              {!choosingFireballTargets && panelActions.map((action) => (
                <button
                  className={`action action-${action.kind}`}
                  key={action.index}
                  onClick={() => prepareAction(action)}
                  disabled={watchingOpponent}
                >
                  <span>{displayActionLabel(action, state)}</span>
                  <i aria-hidden="true">→</i>
                </button>
              ))}
            </div>
            {/* Which decks are playing and under what seed is something you
                read once, so it lives in here with the rest of the meta. */}
            <details className="game-menu">
              <summary>Game menu</summary>
              <p className="game-menu-match">
                <span>You · {humanDeck}</span>
                <i>versus</i>
                <span>{policy} · {botDeck}</span>
                <small>Seed {seed}</small>
              </p>
              <button onClick={openSetup}>New game</button>
              {dangerActions.map((action) => (
                <button key={action.index} onClick={() => act(action)} disabled={watchingOpponent}>
                  {action.label}
                </button>
              ))}
              <p className="fan-content-notice">
                Penta is unofficial Fan Content permitted under the Fan Content Policy. Not
                approved or endorsed by Wizards. Portions of the materials used are property of
                Wizards of the Coast. © Wizards of the Coast LLC.
              </p>
            </details>
            <details
              className="game-log"
              open={gameLogOpen}
              onToggle={(event) => setGameLogOpen(event.currentTarget.open)}
            >
              <summary>Game log</summary>
              <ol>
                {state.events.map((event, index) => (
                  <li key={`${event}-${index}`}>{event}</li>
                ))}
              </ol>
            </details>
          </aside>
        </div>
      )}

      {state?.result && (
        <div className="result-backdrop">
          <section className={`result-card result-${state.result.outcome}`}>
            <span>GAME OVER</span>
            <h1>{state.result.message}</h1>
            <p>
              Turn {state.turn} · {humanDeck} vs {botDeck}
            </p>
            <div>
              {/* New game takes the setup choices, so "Random" rolls again;
                  a rematch takes the decks that just played, which keeps the
                  matchup even when chance picked it. */}
              <button onClick={() => newGame(randomSeed())}>New game</button>
              <button
                className="result-primary"
                onClick={() => newGame(randomSeed(), humanDeck, botDeck)}
              >
                Rematch
              </button>
            </div>
          </section>
        </div>
      )}
    </main>
  );
}


function displayActionLabel(action: Action, state: GameState) {
  if (action.label !== "Pass priority") return action.label;
  // The engine simulates the pass and reports the real destination, so the
  // button never promises a stop the auto-pass policy will not honor.
  return state.passLabel ?? "Pass priority";
}

/// The player panel: whose it is, what they have left, and the thing you aim
/// a Lightning Bolt at. It is deliberately small — a deck name is something
/// you look up once, and it was costing a full row of the table.
/// Mana you are holding, written the way the cards write it: one number for
/// colourless and a lettered pip for each coloured point. Empty is the normal
/// state, so it shows nothing — but keeps its width, since mana appears while
/// you are paying for something and the row must not jump as you tap lands.
function ManaPool({ mana }: { mana: PlayerState["mana"] }) {
  const coloured = [
    ["white", mana.white, "W"],
    ["blue", mana.blue, "U"],
    ["black", mana.black, "B"],
    ["red", mana.red, "R"],
    ["green", mana.green, "G"],
  ] as const;
  const spoken = [
    mana.colorless > 0 ? `${mana.colorless} colourless` : null,
    ...coloured.map(([name, count]) => (count > 0 ? `${count} ${name}` : null)),
  ].filter(Boolean);
  return (
    <span
      className="mana-pool card-cost"
      aria-label={spoken.length ? `Mana pool: ${spoken.join(", ")}` : "Mana pool empty"}
      title={spoken.length ? `Mana pool: ${spoken.join(", ")}` : "No floating mana"}
    >
      {mana.colorless > 0 && <i className="mana-generic">{mana.colorless}</i>}
      {coloured.flatMap(([name, count, letter]) =>
        // A pip each, the way a cost is written — until there are more than
        // will fit, when the colour collapses to one counted pip rather than
        // growing the panel and squeezing the phase strip beside it.
        count > 3
          ? [
              <i className={`mana-${name}-symbol is-counted`} key={name}>
                {letter}
                {count}
              </i>,
            ]
          : Array.from({ length: count }, (_, index) => (
              <i className={`mana-${name}-symbol`} key={`${name}${index}`}>
                {letter}
              </i>
            )),
      )}
    </span>
  );
}

function PlayerBar({
  player,
  opponent = false,
  targetable = false,
  selected = false,
  onTarget,
  onDragOverTarget,
  onDragLeaveTarget,
  onDropTarget,
}: {
  player: PlayerState;
  opponent?: boolean;
  targetable?: boolean;
  selected?: boolean;
  onTarget?(): void;
  onDragOverTarget?(): void;
  onDragLeaveTarget?(): void;
  onDropTarget?(): void;
}) {
  return (
    <div
      className={`player-bar ${opponent ? "player-opponent" : ""} ${targetable ? "is-targetable" : ""} ${selected ? "is-selected-target" : ""}`}
    >
      <div className="player-name">
        <strong>{opponent ? "Opponent" : "You"}</strong>
      </div>
      <div className="zone-counts">
        <span title="Library">LIB {player.library}</span>
        <span title="Graveyard">GY {player.graveyard.length}</span>
      </div>
      <ManaPool mana={player.mana} />
      <div className="life-total">
        <small>LIFE</small>
        <strong>{player.life}</strong>
      </div>
      {targetable && (
        <button
          type="button"
          className="player-target-hitbox"
          aria-label={`Target ${opponent ? "opponent" : "yourself"}`}
          onClick={onTarget}
          onDragOver={(event) => {
            event.preventDefault();
            event.dataTransfer.dropEffect = "move";
            onDragOverTarget?.();
          }}
          onDragLeave={onDragLeaveTarget}
          onDrop={(event) => {
            event.preventDefault();
            onDropTarget?.();
          }}
        />
      )}
    </div>
  );
}

/// Arrows from each spell on the stack to whatever it is aimed at, so a
/// Divine Offering pointing at your Mox is visibly pointing at your Mox.
function StackTargetArrows({
  stack,
  state,
  tableRef,
}: {
  stack: GameState["stack"];
  state: GameState;
  tableRef: { current: HTMLElement | null };
}) {
  const [lines, setLines] = useState<
    Array<{ key: string; x1: number; y1: number; x2: number; y2: number }>
  >([]);

  useEffect(() => {
    const table = tableRef.current;
    if (!table) return;
    const update = () => {
      const tableBounds = table.getBoundingClientRect();
      const center = (el: Element) => {
        const bounds = el.getBoundingClientRect();
        return {
          x: bounds.left + bounds.width / 2 - tableBounds.left,
          y: bounds.top + bounds.height / 2 - tableBounds.top,
        };
      };
      const next: Array<{ key: string; x1: number; y1: number; x2: number; y2: number }> = [];
      for (const item of stack) {
        const source = table.querySelector<HTMLElement>(
          `.stack-zone [data-card-id="${item.id}"]`,
        );
        if (!source) continue;
        const from = center(source);
        const targets: Array<{ key: string; el: Element | null }> = [
          ...item.targetCardIds.map((id) => ({
            key: `card:${id}`,
            el: table.querySelector(`[data-card-id="${id}"]:not(.stack-zone *)`),
          })),
          ...item.targetPlayers.map((owner) => ({
            key: `player:${owner}`,
            el: table.querySelector(
              owner === "opponent" ? ".player-opponent" : ".player-bar:not(.player-opponent)",
            ),
          })),
          ...item.targetStackIds.map((stackId) => {
            const targetItem = state.stack.find((candidate) => candidate.id === stackId);
            return {
              key: `stack:${stackId}`,
              el: targetItem
                ? table.querySelector(`.stack-zone [data-card-id="${targetItem.id}"]`)
                : null,
            };
          }),
        ];
        for (const target of targets) {
          if (!target.el) continue;
          const to = center(target.el);
          next.push({ key: `${item.id}->${target.key}`, x1: from.x, y1: from.y, x2: to.x, y2: to.y });
        }
      }
      setLines((current) =>
        JSON.stringify(current) === JSON.stringify(next) ? current : next,
      );
    };
    const frame = window.requestAnimationFrame(update);
    const interval = window.setInterval(update, 400);
    window.addEventListener("resize", update);
    return () => {
      window.cancelAnimationFrame(frame);
      window.clearInterval(interval);
      window.removeEventListener("resize", update);
    };
  }, [stack, state, tableRef]);

  if (lines.length === 0) return null;
  return (
    <svg className="stack-target-arrows" aria-label="Spell targets">
      <defs>
        <marker
          id="stack-arrow-head"
          markerWidth="8"
          markerHeight="8"
          refX="7"
          refY="4"
          orient="auto"
          markerUnits="strokeWidth"
        >
          <path d="M 0 0 L 8 4 L 0 8 z" />
        </marker>
      </defs>
      {lines.map((line) => (
        <line
          key={line.key}
          x1={line.x1}
          y1={line.y1}
          x2={line.x2}
          y2={line.y2}
          markerEnd="url(#stack-arrow-head)"
        />
      ))}
    </svg>
  );
}

function BlockArrows({
  assignments,
  tableRef,
}: {
  assignments: Record<number, number>;
  tableRef: { current: HTMLElement | null };
}) {
  const [lines, setLines] = useState<
    Array<{ blocker: number; attacker: number; x1: number; y1: number; x2: number; y2: number }>
  >([]);

  useEffect(() => {
    const table = tableRef.current;
    if (!table) return;
    const update = () => {
      const tableBounds = table.getBoundingClientRect();
      const next = Object.entries(assignments).flatMap(([blockerText, attacker]) => {
        const blocker = Number(blockerText);
        const blockerCard = table.querySelector<HTMLElement>(`[data-card-id="${blocker}"]`);
        const attackerCard = table.querySelector<HTMLElement>(`[data-card-id="${attacker}"]`);
        if (!blockerCard || !attackerCard) return [];
        const blockerBounds = blockerCard.getBoundingClientRect();
        const attackerBounds = attackerCard.getBoundingClientRect();
        return [{
          blocker,
          attacker,
          x1: blockerBounds.left + blockerBounds.width / 2 - tableBounds.left,
          y1: blockerBounds.top - tableBounds.top,
          x2: attackerBounds.left + attackerBounds.width / 2 - tableBounds.left,
          y2: attackerBounds.bottom - tableBounds.top,
        }];
      });
      setLines(next);
    };
    const frame = window.requestAnimationFrame(update);
    window.addEventListener("resize", update);
    table.addEventListener("scroll", update, true);
    return () => {
      window.cancelAnimationFrame(frame);
      window.removeEventListener("resize", update);
      table.removeEventListener("scroll", update, true);
    };
  }, [assignments, tableRef]);

  return (
    <svg className="block-arrows" aria-label="Current block assignments">
      <defs>
        <marker
          id="block-arrow-head"
          markerWidth="8"
          markerHeight="8"
          refX="7"
          refY="4"
          orient="auto"
          markerUnits="strokeWidth"
        >
          <path d="M 0 0 L 8 4 L 0 8 z" />
        </marker>
      </defs>
      {lines.map((line) => (
        <line
          key={line.blocker}
          x1={line.x1}
          y1={line.y1}
          x2={line.x2}
          y2={line.y2}
          markerEnd="url(#block-arrow-head)"
        />
      ))}
    </svg>
  );
}

function Zone({
  cards,
  cardArtMode,
  label,
  actionCount,
  isDraggable,
  isTargetable,
  onSelect,
  selectedCard,
  selectedCardIds = [],
  animatedCardId,
  previewManaSourceIds,
  onDragStartCard,
  onDragEndCard,
  dragOverTarget,
  onDragOverTarget,
  onDragLeaveTarget,
  onDropTarget,
  opponent = false,
}: {
  cards: Card[];
  cardArtMode: CardArtMode;
  label: string;
  actionCount(id: number): number;
  isDraggable(id: number): boolean;
  isTargetable(id: number): boolean;
  onSelect(id: number): void;
  selectedCard: number | null;
  selectedCardIds?: number[];
  animatedCardId: number | null;
  previewManaSourceIds: number[];
  onDragStartCard(id: number): void;
  onDragEndCard(): void;
  dragOverTarget: string | null;
  onDragOverTarget(target: string): void;
  onDragLeaveTarget(target: string): void;
  onDropTarget(target: string): void;
  opponent?: boolean;
}) {
  const lands = cards.filter((card) => card.kind === "land");
  const nonlands = cards.filter((card) => card.kind !== "land");
  const renderCards = (laneCards: Card[]) =>
    groupCardsIntoPiles(laneCards).map((pile) => (
      <CardPile
        key={pile.key}
        cards={pile.cards}
        cardArtMode={cardArtMode}
        actionCount={actionCount}
        isDraggable={isDraggable}
        isTargetable={isTargetable}
        onSelect={onSelect}
        selectedCard={selectedCard}
        selectedCardIds={selectedCardIds}
        animatedCardId={animatedCardId}
        previewManaSourceIds={previewManaSourceIds}
        onDragStartCard={onDragStartCard}
        onDragEndCard={onDragEndCard}
        dragOverTarget={dragOverTarget}
        onDragOverTarget={onDragOverTarget}
        onDragLeaveTarget={onDragLeaveTarget}
        onDropTarget={onDropTarget}
      />
    ));

  return (
    <div
      className={`battlefield-zone ${opponent ? "battlefield-opponent" : "battlefield-human"}`}
      aria-label={label}
    >
      <div className="battlefield-row battlefield-nonlands" aria-label="Nonland permanents">
        {renderCards(nonlands)}
      </div>
      <div className="battlefield-row battlefield-lands" aria-label="Lands">
        {renderCards(lands)}
      </div>
    </div>
  );
}

function groupCardsIntoPiles(cards: Card[]) {
  const piles = new Map<string, Card[]>();
  for (const card of cards) {
    // Visually different game states must remain in separate piles. Identical
    // cards in the same state can safely collapse while retaining each card id.
    const key = [
      card.name,
      card.kind,
      card.tapped ? "tapped" : "ready",
      card.attacking ? "attacking" : "idle",
      card.blocking ?? "",
      card.damage ?? 0,
      card.enteredThisTurn ? "entered" : "old",
    ].join(":");
    const pile = piles.get(key) ?? [];
    pile.push(card);
    piles.set(key, pile);
  }
  // The React key must not carry that state. A pile whose key changes is a pile
  // React unmounts, which throws away every card's DOM node inside it — so a
  // land losing its summoning sickness, which nothing on the board draws, still
  // made the card blink. Number the piles within each card name instead: a pile
  // keeps its identity while the cards in it tap, take damage, or grow old.
  const nth = new Map<string, number>();
  return Array.from(piles.values(), (pileCards) => {
    const name = `${pileCards[0].name}:${pileCards[0].kind}`;
    const index = nth.get(name) ?? 0;
    nth.set(name, index + 1);
    return { key: `${name}#${index}`, cards: pileCards };
  });
}

function CardPile({
  cards,
  cardArtMode,
  actionCount,
  isDraggable,
  isTargetable,
  onSelect,
  selectedCard,
  selectedCardIds,
  animatedCardId,
  previewManaSourceIds,
  onDragStartCard,
  onDragEndCard,
  dragOverTarget,
  onDragOverTarget,
  onDragLeaveTarget,
  onDropTarget,
}: {
  cards: Card[];
  cardArtMode: CardArtMode;
  actionCount(id: number): number;
  isDraggable(id: number): boolean;
  isTargetable(id: number): boolean;
  onSelect(id: number): void;
  selectedCard: number | null;
  selectedCardIds: number[];
  animatedCardId: number | null;
  previewManaSourceIds: number[];
  onDragStartCard(id: number): void;
  onDragEndCard(): void;
  dragOverTarget: string | null;
  onDragOverTarget(target: string): void;
  onDragLeaveTarget(target: string): void;
  onDropTarget(target: string): void;
}) {
  const [expanded, setExpanded] = useState(false);
  const displaysFullCards =
    cardArtMode === "full" &&
    cards.every((card) => card.art && isScryfallId(card.art.scryfallId));
  const previewedCards = cards.filter((card) => previewManaSourceIds.includes(card.id)).length;
  const paymentExpanded = previewedCards > 0 && previewedCards < cards.length;
  const visuallyExpanded = expanded || paymentExpanded;
  const cardWidth = displaysFullCards ? 48 : 96;
  const spacing = visuallyExpanded ? (displaysFullCards ? 42 : 72) : 7;
  const visibleOffsetCount = visuallyExpanded
    ? cards.length - 1
    : Math.min(cards.length - 1, 3);
  const width = cardWidth + visibleOffsetCount * spacing;

  return (
    <div
      className={[
        "card-pile",
        displaysFullCards ? "card-pile-full" : "",
        visuallyExpanded ? "is-expanded" : "",
        paymentExpanded ? "is-payment-expanded" : "",
      ].filter(Boolean).join(" ")}
      style={{ width }}
      onMouseEnter={() => setExpanded(true)}
      onMouseLeave={() => setExpanded(false)}
      onFocusCapture={() => setExpanded(true)}
      onBlurCapture={(event) => {
        if (!event.currentTarget.contains(event.relatedTarget)) setExpanded(false);
      }}
      aria-label={cards.length > 1 ? `${cards.length} ${cards[0].name} cards` : undefined}
    >
      {cards.map((card, index) => {
        const offset = visuallyExpanded ? index * spacing : Math.min(index, 3) * spacing;
        return (
          <div
            className="battlefield-card-slot"
            key={card.id}
            style={{ transform: `translateX(${offset}px)`, zIndex: index + 1 }}
          >
            <GameCard
              card={card}
              cardArtMode={cardArtMode}
              zone="battlefield"
              actionable={actionCount(card.id) > 0}
              draggableAction={isDraggable(card.id)}
              targetable={isTargetable(card.id)}
              selected={selectedCard === card.id || selectedCardIds.includes(card.id)}
              animating={animatedCardId === card.id}
              previewMana={previewManaSourceIds.includes(card.id)}
              dragOverTarget={dragOverTarget === cardTargetKey(card.id)}
              onSelect={onSelect}
              onDragStartCard={onDragStartCard}
              onDragEndCard={onDragEndCard}
              onDragOverTarget={onDragOverTarget}
              onDragLeaveTarget={onDragLeaveTarget}
              onDropTarget={onDropTarget}
              compact
            />
          </div>
        );
      })}
      {cards.length > 1 && !visuallyExpanded && (
        <span className="card-pile-count" aria-hidden="true">{cards.length}</span>
      )}
    </div>
  );
}

function HandZone({
  cards,
  cardArtMode,
  actionCount,
  isDraggable,
  isTargetable,
  selectedCard,
  previewManaSourceIds,
  onSelect,
  onDragStartCard,
  onDragEndCard,
  onPaymentPreviewStart,
  onPaymentPreviewEnd,
}: {
  cards: Card[];
  cardArtMode: CardArtMode;
  actionCount(id: number): number;
  isDraggable(id: number): boolean;
  isTargetable(id: number): boolean;
  selectedCard: number | null;
  previewManaSourceIds: number[];
  onSelect(id: number): void;
  onDragStartCard(id: number): void;
  onDragEndCard(): void;
  onPaymentPreviewStart(id: number): void;
  onPaymentPreviewEnd(): void;
}) {
  const [hoveredCard, setHoveredCard] = useState<number | null>(null);
  const [viewportWidth, setViewportWidth] = useState<number | null>(null);

  useEffect(() => {
    const updateViewportWidth = () => setViewportWidth(window.innerWidth);
    updateViewportWidth();
    window.addEventListener("resize", updateViewportWidth);
    return () => window.removeEventListener("resize", updateViewportWidth);
  }, []);

  const center = (cards.length - 1) / 2;
  const compactHand = viewportWidth !== null && viewportWidth <= 620;
  const cardWidth = compactHand ? 112 : 132;
  const handPadding = compactHand ? 14 : 36;
  const rotationAllowance = compactHand ? 42 : 0;
  const availableWidth = viewportWidth === null
    ? 700
    : Math.max(0, viewportWidth - cardWidth - handPadding - rotationAllowance);
  const spacing = Math.min(
    78,
    Math.max(compactHand ? 26 : 42, availableWidth / Math.max(cards.length - 1, 1)),
  );

  return (
    <div className="hand-zone" aria-label="Your hand">
      {cards.map((card, index) => {
          const relative = index - center;
          const isHovered = hoveredCard === card.id;
          const hoveredIndex = cards.findIndex((candidate) => candidate.id === hoveredCard);
          const neighborSpread = hoveredIndex < 0 || isHovered
            ? 0
            : index < hoveredIndex ? -18 : 18;
          return (
            <div
              className="hand-card-slot"
              key={card.id}
              style={{
                transform: `translateX(${relative * spacing + neighborSpread}px) translateY(${isHovered ? -52 : Math.abs(relative) * 3}px) rotate(${isHovered ? 0 : relative * 1.8}deg) scale(${isHovered ? 1.16 : 1})`,
                zIndex: isHovered ? 100 : index + 1,
              }}
            >
              <GameCard
                card={card}
                cardArtMode={cardArtMode}
                zone="hand"
                actionable={actionCount(card.id) > 0}
                draggableAction={isDraggable(card.id)}
                targetable={isTargetable(card.id)}
                selected={selectedCard === card.id}
                onSelect={onSelect}
                previewMana={previewManaSourceIds.includes(card.id)}
                onDragStartCard={onDragStartCard}
                onDragEndCard={onDragEndCard}
                onPaymentPreviewStart={onPaymentPreviewStart}
                onPaymentPreviewEnd={onPaymentPreviewEnd}
                onHoverChange={(hovered) => setHoveredCard(hovered ? card.id : null)}
              />
            </div>
          );
        })}
      {cards.length === 0 && <span className="zone-empty">Your hand is empty</span>}
    </div>
  );
}

function GameCard({
  card,
  cardArtMode,
  zone = "battlefield",
  targetKey,
  actionable,
  draggableAction = false,
  targetable = false,
  selected,
  animating = false,
  previewMana = false,
  dragOverTarget = false,
  onSelect,
  onDragStartCard,
  onDragEndCard,
  onDragOverTarget,
  onDragLeaveTarget,
  onDropTarget,
  onPaymentPreviewStart,
  onPaymentPreviewEnd,
  onHoverChange,
  compact = false,
}: {
  card: Card;
  cardArtMode: CardArtMode;
  zone?: string;
  targetKey?: string;
  actionable: boolean;
  draggableAction?: boolean;
  targetable?: boolean;
  selected: boolean;
  animating?: boolean;
  previewMana?: boolean;
  dragOverTarget?: boolean;
  onSelect(id: number): void;
  onDragStartCard?(id: number): void;
  onDragEndCard?(): void;
  onDragOverTarget?(target: string): void;
  onDragLeaveTarget?(target: string): void;
  onDropTarget?(target: string): void;
  onPaymentPreviewStart?(id: number): void;
  onPaymentPreviewEnd?(): void;
  onHoverChange?(hovered: boolean): void;
  compact?: boolean;
}) {
  const [previewPosition, setPreviewPosition] = useState<{
    left: number;
    top: number;
  } | null>(null);
  const validArtId =
    card.art && isScryfallId(card.art.scryfallId) ? card.art.scryfallId : null;
  const artRequestKey =
    cardArtMode !== "off" && validArtId ? `${cardArtMode}:${validArtId}` : null;
  const [failedArtRequest, setFailedArtRequest] = useState<string | null>(null);
  const renderedCardArtMode =
    artRequestKey && artRequestKey === failedArtRequest ? "off" : cardArtMode;
  const hasFullArt = renderedCardArtMode === "full" && validArtId !== null;
  const currentKind = card.kind.replace("artifactcreature", "artifact creature");
  const type =
    card.isLand && card.kind !== "land"
      ? `Land · ${currentKind}`
      : (card.typeLine || currentKind);
  const isRed =
    !card.kind.includes("artifact") &&
    !card.isLand &&
    ((card.manaCost?.red ?? 0) > 0 || (card.manaCost?.whiteRedHybrid ?? 0) > 0);
  const showZeroCost =
    !card.isLand &&
    card.manaCost?.generic === 0 &&
    card.manaCost.white === 0 &&
    card.manaCost.blue === 0 &&
    card.manaCost.black === 0 &&
    card.manaCost.red === 0 &&
    card.manaCost.green === 0 &&
    card.manaCost.whiteRedHybrid === 0 &&
    !card.manaCost.x;
  const manaSymbolCount = card.manaCost
    ? (card.manaCost.x ? 1 : 0) +
      (card.manaCost.generic > 0 || showZeroCost ? 1 : 0) +
      card.manaCost.white +
      card.manaCost.blue +
      card.manaCost.black +
      card.manaCost.red +
      card.manaCost.green +
      card.manaCost.whiteRedHybrid
    : 0;
  const manaCost = formatManaCost(card);
  const battlefieldState = [
    card.owner ? (card.tapped ? "Tapped" : "Untapped") : null,
    card.attacking ? "Attacking" : null,
    card.flying ? "Flying" : null,
    card.damage ? `${card.damage} damage marked` : null,
    card.enteredThisTurn ? "Played this turn." : null,
  ].filter(Boolean);

  const showPreview = (element: HTMLButtonElement) => {
    const bounds = element.getBoundingClientRect();
    const previewWidth = 260;
    const previewHeight = 320;
    const gutter = 10;
    let left = bounds.right + gutter;
    if (left + previewWidth > window.innerWidth - gutter) {
      left = bounds.left - previewWidth - gutter;
    }
    left = Math.max(gutter, Math.min(left, window.innerWidth - previewWidth - gutter));
    const centeredTop = bounds.top + (bounds.height - previewHeight) / 2;
    const battlefieldTop =
      bounds.top + bounds.height / 2 < window.innerHeight / 2
        ? bounds.top - previewHeight - gutter
        : bounds.bottom + gutter;
    const top = Math.max(
      gutter,
      Math.min(
        compact ? battlefieldTop : centeredTop,
        window.innerHeight - previewHeight - gutter,
      ),
    );
    setPreviewPosition({ left, top });
  };

  const previewId = `card-rules-${card.id}`;
  return (
    <>
      <button
        data-card-id={card.id}
        className={[
          "game-card",
          compact ? "game-card-compact" : "",
          hasFullArt ? "has-full-art" : "",
          `card-${card.kind}`,
          isRed ? "card-red" : "",
          card.tapped ? "is-tapped" : "",
          card.attacking ? "is-attacking" : "",
          actionable ? "is-actionable" : "",
          targetable ? "is-targetable" : "",
          selected ? "is-selected" : "",
          dragOverTarget ? "is-drag-over-target" : "",
          animating ? "is-opponent-action-card" : "",
          previewMana ? "is-autotap-preview" : "",
        ].join(" ")}
        aria-label={
          targetable
            ? `Target ${card.name}`
            : actionable
              ? `Choose an action for ${card.name}`
              : `Inspect ${card.name}`
        }
        aria-describedby={previewPosition ? previewId : undefined}
        data-card-owner={card.owner ?? "human"}
        data-card-name={card.name}
        data-card-zone={zone}
        draggable={draggableAction && !targetable}
        onDragStart={(event) => {
          event.dataTransfer.effectAllowed = "move";
          event.dataTransfer.setData("text/plain", String(card.id));
          onDragStartCard?.(card.id);
        }}
        onDragEnd={() => onDragEndCard?.()}
        onDragOver={(event) => {
          if (!targetable) return;
          event.preventDefault();
          event.dataTransfer.dropEffect = "move";
          onDragOverTarget?.(targetKey ?? cardTargetKey(card.id));
        }}
        onDragLeave={() => onDragLeaveTarget?.(targetKey ?? cardTargetKey(card.id))}
        onDrop={(event) => {
          if (!targetable) return;
          event.preventDefault();
          onDropTarget?.(targetKey ?? cardTargetKey(card.id));
        }}
        onMouseEnter={(event) => {
          showPreview(event.currentTarget);
          onPaymentPreviewStart?.(card.id);
          onHoverChange?.(true);
        }}
        onMouseLeave={() => {
          setPreviewPosition(null);
          onPaymentPreviewEnd?.();
          onHoverChange?.(false);
        }}
        onFocus={(event) => {
          showPreview(event.currentTarget);
          onPaymentPreviewStart?.(card.id);
          onHoverChange?.(true);
        }}
        onBlur={() => {
          setPreviewPosition(null);
          onPaymentPreviewEnd?.();
          onHoverChange?.(false);
        }}
        onClick={(event) => {
          event.currentTarget.blur();
          if (actionable) onSelect(card.id);
        }}
      >
        <span className={`card-header ${manaSymbolCount >= 3 ? "card-header-dense" : ""}`}>
          <span className="card-title">{card.name}</span>
          {card.manaCost && !card.isLand && (
            <span className="card-cost" aria-label={`Casting cost for ${card.name}`}>
              {card.manaCost.x && <i className="mana-generic">X</i>}
              {card.manaCost.generic > 0 && (
                <i className="mana-generic">{card.manaCost.generic}</i>
              )}
              {showZeroCost && <i className="mana-generic">0</i>}
              {Array.from({ length: card.manaCost.white }, (_, index) => (
                <i className="mana-white-symbol" key={`w${index}`}>W</i>
              ))}
              {Array.from({ length: card.manaCost.blue }, (_, index) => (
                <i className="mana-blue-symbol" key={`u${index}`}>U</i>
              ))}
              {Array.from({ length: card.manaCost.black }, (_, index) => (
                <i className="mana-black-symbol" key={`b${index}`}>B</i>
              ))}
              {Array.from({ length: card.manaCost.red }, (_, index) => (
                <i className="mana-red-symbol" key={`r${index}`}>R</i>
              ))}
              {Array.from({ length: card.manaCost.green }, (_, index) => (
                <i className="mana-green-symbol" key={`g${index}`}>G</i>
              ))}
              {Array.from({ length: card.manaCost.whiteRedHybrid }, (_, index) => (
                <i className="mana-white-red-symbol" key={`rw${index}`}>R/W</i>
              ))}
            </span>
          )}
        </span>
        {card.kind.includes("ability") ? (
          <span className="card-art" aria-hidden="true">
            <i>✦</i>
          </span>
        ) : (
          <CardArt
            mode={renderedCardArtMode}
            cardKind={card.kind}
            scryfallId={validArtId ?? ""}
            fullImageSizes={compact ? "48px" : "(max-width: 620px) 112px, 132px"}
            onImageError={() => {
              if (artRequestKey) setFailedArtRequest(artRequestKey);
            }}
          />
        )}
        <span className="card-type">{type}</span>
        <span className="card-text">{card.attacking ? "Attacking" : card.rulesText}</span>
        {card.power !== null && card.power !== undefined && (
          <strong className="card-stats">
            {card.power}/{card.toughness}
            {card.damage ? <small> · {card.damage} marked</small> : null}
          </strong>
        )}
      </button>
      {previewPosition &&
        createPortal(
          <span
            className="card-hover-stats"
            id={previewId}
            role="tooltip"
            style={previewPosition}
          >
            <strong>{card.name}</strong>
            <span className="card-hover-type">{type}</span>
            <span className="card-hover-rules">{card.rulesText}</span>
            {card.metadataOnly && (
              <span className="card-hover-support">
                Staged support: only baseline creature and land/mana behavior is active.
              </span>
            )}
            <span className="card-hover-details">
              <span><b>Cost</b> {manaCost}</span>
              {card.xValue !== null && card.xValue !== undefined && (
                <span><b>X</b> {card.xValue}</span>
              )}
              {card.power !== null && card.power !== undefined && (
                <span><b>Power / toughness</b> {card.power} / {card.toughness}</span>
              )}
            </span>
            {battlefieldState.length > 0 && (
              <span className="card-hover-state">{battlefieldState.join(" · ")}</span>
            )}
            {renderedCardArtMode !== "off" && validArtId && card.art && (
              <span className="card-hover-credit">
                Illustration: {card.art.artist} · Card art © Wizards of the Coast LLC · Image via Scryfall
              </span>
            )}
          </span>,
          document.body,
        )}
    </>
  );
}

function formatManaCost(card: Card) {
  if (card.isLand) return "—";
  if (!card.manaCost) return "Unknown";
  const parts = [
    card.manaCost.x ? "X" : "",
    card.manaCost.generic > 0 ? String(card.manaCost.generic) : "",
    "W".repeat(card.manaCost.white),
    "U".repeat(card.manaCost.blue),
    "B".repeat(card.manaCost.black),
    "R".repeat(card.manaCost.red),
    "G".repeat(card.manaCost.green),
    "{R/W}".repeat(card.manaCost.whiteRedHybrid),
  ].filter(Boolean);
  return parts.length > 0 ? parts.join(" ") : "0";
}
