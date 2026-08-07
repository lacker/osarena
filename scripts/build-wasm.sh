#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output_dir="${WASM_OUT_DIR:-$repo_root/web/app/wasm}"

cd "$repo_root"

cargo build \
  --package penta-wasm \
  --target wasm32-unknown-unknown \
  --release \
  --locked

wasm_input="$repo_root/target/wasm32-unknown-unknown/release/penta_wasm.wasm"
bindgen_version="$(wasm-bindgen --version)"

if command -v sha256sum >/dev/null 2>&1; then
  wasm_hash="$(sha256sum "$wasm_input" | cut -d ' ' -f 1)"
else
  wasm_hash="$(shasum -a 256 "$wasm_input" | cut -d ' ' -f 1)"
fi

cache_key="wasm=$wasm_hash bindgen=$bindgen_version target=web"
cache_file="$output_dir/.build-cache-key"
generated_files=(
  "$output_dir/penta_wasm.d.ts"
  "$output_dir/penta_wasm.js"
  "$output_dir/penta_wasm_bg.wasm"
  "$output_dir/penta_wasm_bg.wasm.d.ts"
)

bindings_complete=true
for generated_file in "${generated_files[@]}"; do
  if [[ ! -f "$generated_file" ]]; then
    bindings_complete=false
    break
  fi
done

if [[ "$bindings_complete" == true && -f "$cache_file" && "$(<"$cache_file")" == "$cache_key" ]]; then
  echo "WASM bindings are up to date"
  exit 0
fi

mkdir -p "$output_dir"
wasm-bindgen \
  "$wasm_input" \
  --out-dir "$output_dir" \
  --target web
printf '%s\n' "$cache_key" > "$cache_file"
