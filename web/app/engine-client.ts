import initWasm, { WebGame as RustWebGame } from "./wasm/penta_wasm.js";
import type { GameState } from "./game-types";
import type { FormatId } from "./game-config";

export type EngineGame = RustWebGame;

export type EngineConfig = {
  format: FormatId;
  humanDeck: string;
  botDeck: string;
  policy: string;
  humanFirst: boolean;
  seed: number;
};

/** Loads the generated WASM module exactly once per browser session. */
export async function initializeEngine(): Promise<void> {
  await initWasm();
}

export function createEngineGame(config: EngineConfig): EngineGame {
  return new RustWebGame(
    config.humanDeck,
    config.botDeck,
    config.policy,
    config.humanFirst,
    config.seed,
    config.format,
  );
}

/** Decodes the single JSON boundary exposed by the Rust facade. */
export function readEngineState(game: EngineGame): GameState {
  return JSON.parse(game.state_json()) as GameState;
}
