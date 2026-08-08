#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

version="$({
    awk '/^name = "wasm-bindgen"$/ { found = 1; next }
         found && /^version = / { gsub(/[",]/, "", $3); print $3; exit }' \
        "$repo_root/Cargo.lock"
})"

if [[ -z "$version" ]]; then
    echo "could not find wasm-bindgen in $repo_root/Cargo.lock" >&2
    exit 1
fi

printf '%s\n' "$version"
