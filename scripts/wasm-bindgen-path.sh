#!/usr/bin/env bash
set -euo pipefail

if command -v wasm-bindgen >/dev/null 2>&1; then
    command -v wasm-bindgen
    exit 0
fi

task_cargo_home="${CARGO_HOME:-${HOME:-}/.cargo}"
if [[ -x "$task_cargo_home/bin/wasm-bindgen" ]]; then
    printf '%s\n' "$task_cargo_home/bin/wasm-bindgen"
    exit 0
fi

exit 1
