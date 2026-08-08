import { readFile } from "node:fs/promises";

import init, { WebGame } from "../app/wasm/penta_wasm.js";

let initialization;

export function initializeWasm() {
  initialization ??= readFile(
    new URL("../app/wasm/penta_wasm_bg.wasm", import.meta.url),
  ).then((bytes) => init({ module_or_path: bytes }));
  return initialization;
}

export { WebGame };
