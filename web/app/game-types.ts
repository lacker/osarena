import type { FormatId } from "./game-config";

export type Owner = "human" | "opponent";

export type CardArtMetadata = {
  scryfallId: string;
  artist: string;
};

export type StackObjectKind = "Spell" | "ActivatedAbility" | "TriggeredAbility";

export type DecisionKind = "Choice" | "TriggerOrder" | "TriggerPlacement";

export type SpellFormMetadata =
  | { kind: "part"; partId: number }
  | { kind: "combined"; partIds: number[] };

export type CastTargetSelectionMetadata = {
  slotId: number;
  targetCardIds: number[];
  targetPlayers: Owner[];
  targetStackIds: number[];
};

export type CastSignatureMetadata = {
  playOptionId: number;
  form: SpellFormMetadata;
  modeIds: number[];
  alternativeCostId?: number | null;
  additionalCostIds: number[];
  x: number;
  targetSelections: CastTargetSelectionMetadata[];
};

export type Card = {
  /** Current-zone GameObjectId used by actions, targets, and DOM identity. */
  id: number;
  /** Logical card part supplying this permanent's visible characteristics. */
  partId?: number;
  name: string;
  art: CardArtMetadata | null;
  kind: string;
  typeLine?: string;
  metadataOnly?: boolean;
  isLand?: boolean;
  rulesText: string;
  manaCost?: {
    generic: number;
    white: number;
    blue: number;
    black: number;
    red: number;
    green: number;
    whiteRedHybrid: number;
    x: boolean;
  } | null;
  owner?: Owner;
  tapped?: boolean;
  power?: number | null;
  toughness?: number | null;
  damage?: number;
  attacking?: boolean;
  blocking?: number | null;
  flying?: boolean;
  canAttack?: boolean;
  enteredThisTurn?: boolean;
  xValue?: number | null;
};

export type Action = {
  index: number;
  label: string;
  kind: "primary" | "combat" | "pass" | "danger";
  /** Current source GameObjectId, despite the legacy JSON field name. */
  cardId?: number | null;
  targetCardId?: number | null;
  targetPlayer?: Owner | null;
  targetStackId?: number | null;
  targetCardIds?: number[];
  targetPlayers?: Owner[];
  targetStackIds?: number[];
  targetCount?: number;
  abilitySummary?: string | null;
  manaAbility?: boolean;
  spellAction?: boolean;
  playOptionId?: number | null;
  modeIds?: number[] | null;
  sacrificeCardIds?: number[];
  combatDamageAttacker?: number | null;
  x?: number | null;
  paymentAction?: boolean;
  manaSourceIds?: number[];
  decisionId?: number | null;
  decisionOptionIds?: number[];
};

export type OpponentAction = {
  label: string;
  // "turn" beats carry no action of their own — they exist so a turn nobody
  // acted on is still announced.
  kind: "land" | "spell" | "ability" | "combat" | "choice" | "turn" | "draw";
  card?: string | null;
  cardId?: number | null;
  manaSources?: string[];
  state: GameState;
};

export type PlayerState = {
  life: number;
  library: number;
  mana: {
    white: number;
    blue: number;
    black: number;
    red: number;
    green: number;
    colorless: number;
  };
  hand?: Card[];
  handSize?: number;
  graveyard: string[];
};

export type DecisionOption = {
  id: number;
  label: string;
  cardId?: number | null;
  cardName?: string | null;
  /** Pending trigger identity, when this option represents a trigger. */
  triggerId?: number | null;
  /** The exact ability text, when it is narrower than the source card text. */
  abilityText?: string | null;
  zone: string;
};

export type DecisionState = {
  id: number;
  kind: DecisionKind;
  /** Trigger-order submissions are always listed in desired resolution order. */
  orderSemantics?: "resolution";
  prompt: string;
  minimum: number;
  maximum: number;
  cancellable: boolean;
  visibility: string;
  options: DecisionOption[];
};

export type GameState = {
  format: FormatId;
  turn: number;
  gameTurn: number;
  step: string;
  /// Opening hands are still being settled, so no turn or step has begun.
  pregame: boolean;
  active: string;
  priority: string;
  human: PlayerState & { hand: Card[] };
  opponent: PlayerState & { handSize: number };
  battlefield: Card[];
  stack: Array<{
    id: number;
    /** Deprecated JSON compatibility alias for `id`; never use for targeting. */
    cardId?: number;
    /** Historical source game-object ID for an ability. */
    sourceId?: number | null;
    /** Stable identifier of the printed or granted ability that created this object. */
    abilityId?: number | null;
    signature?: CastSignatureMetadata | null;
    name: string;
    art: CardArtMetadata | null;
    owner: Owner;
    kind: StackObjectKind;
    cardKind: string;
    typeLine?: string;
    metadataOnly?: boolean;
    isLand?: boolean;
    manaCost?: Card["manaCost"];
    rulesText: string;
    /** Exact stack ability text, when available separately from source rules. */
    abilityText?: string | null;
    power?: number | null;
    toughness?: number | null;
    x: number;
    targetCardIds: number[];
    targetPlayers: Owner[];
    targetStackIds: number[];
  }>;
  actions: Action[];
  passLabel: string | null;
  decision: DecisionState | null;
  opponentActions?: OpponentAction[];
  /// The board the moment your own action landed, before the game answered.
  /// Only sent when there is a replay to run; the replay starts from here.
  afterYourAction?: GameState | null;
  canUndoMana: boolean;
  canCancelAttackers: boolean;
  phaseStops: string[];
  autopassEnabled: boolean;
  result: null | { outcome: "win" | "loss" | "draw"; message: string };
  events: string[];
};
