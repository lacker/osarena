#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
failures=0
bindgen_problem=false

pass() {
    printf 'ok   %s\n' "$1"
}

fail() {
    printf 'FAIL %s\n' "$1" >&2
    failures=$((failures + 1))
}

require_command() {
    local command_name="$1"
    if command -v "$command_name" >/dev/null 2>&1; then
        pass "$command_name: $(command -v "$command_name")"
        return 0
    fi
    fail "$command_name is not installed or not on PATH"
    return 1
}

cd "$repo_root"

require_command cargo || true
require_command rustc || true
require_command rustup || true
require_command node || true
require_command pnpm || true
bindgen_binary=""
if bindgen_binary="$(./scripts/wasm-bindgen-path.sh)"; then
    pass "wasm-bindgen: $bindgen_binary"
else
    fail "wasm-bindgen is not installed on PATH or in Cargo's bin directory"
    bindgen_problem=true
fi
require_command shellcheck || true
require_command actionlint || true
require_command cc || true
require_command python3 || true

if command -v rustc >/dev/null 2>&1; then
    required_rust="$(sed -n 's/^channel = "\([^"]*\)".*/\1/p' rust-toolchain.toml)"
    actual_rust="$(rustc --version | awk '{ print $2 }')"
    if [[ -n "$required_rust" && "$actual_rust" == "$required_rust" ]]; then
        pass "Rust: $actual_rust"
    else
        fail "Rust $actual_rust does not match rust-toolchain.toml ($required_rust)"
    fi
fi

if command -v cargo >/dev/null 2>&1; then
    if cargo fmt --version >/dev/null 2>&1; then
        pass "rustfmt: $(cargo fmt --version)"
    else
        fail "the pinned Rust toolchain is missing rustfmt"
    fi
    if cargo clippy --version >/dev/null 2>&1; then
        pass "Clippy: $(cargo clippy --version)"
    else
        fail "the pinned Rust toolchain is missing Clippy"
    fi
fi

if command -v rustup >/dev/null 2>&1; then
    if rustup target list --installed | grep -qx 'wasm32-unknown-unknown'; then
        pass "Rust target: wasm32-unknown-unknown"
    else
        fail "Rust target wasm32-unknown-unknown is missing"
    fi
fi

if command -v node >/dev/null 2>&1; then
    if node -e '
      const [major, minor] = process.versions.node.split(".").map(Number);
      process.exit(major > 22 || (major === 22 && minor >= 13) ? 0 : 1);
    '; then
        pass "Node: $(node --version)"
    else
        fail "Node $(node --version) does not satisfy >=22.13.0"
    fi
fi

if command -v pnpm >/dev/null 2>&1; then
    required_pnpm="$(sed -n 's/.*"packageManager": "pnpm@\([^"]*\)".*/\1/p' web/package.json)"
    actual_pnpm="$(pnpm --version)"
    if [[ -n "$required_pnpm" && "$actual_pnpm" == "$required_pnpm" ]]; then
        pass "pnpm: $actual_pnpm"
    else
        fail "pnpm $actual_pnpm does not match web/package.json ($required_pnpm)"
    fi
fi

required_bindgen="$(./scripts/wasm-bindgen-version.sh)"
if [[ -n "$bindgen_binary" ]]; then
    actual_bindgen="$("$bindgen_binary" --version | awk '{ print $2 }')"
    if [[ "$actual_bindgen" == "$required_bindgen" ]]; then
        pass "wasm-bindgen: $actual_bindgen"
    else
        fail "wasm-bindgen $actual_bindgen does not match Cargo.lock ($required_bindgen)"
        bindgen_problem=true
    fi
fi

if command -v python3 >/dev/null 2>&1; then
    if python3 -c 'import sys; raise SystemExit(sys.version_info < (3, 9))'; then
        pass "Python: $(python3 --version 2>&1)"
    else
        fail "$(python3 --version 2>&1) does not satisfy >=3.9"
    fi
fi

if (( failures > 0 )); then
    printf '\n%d prerequisite check(s) failed.\n' "$failures" >&2
    if [[ "$bindgen_problem" == true ]]; then
        printf 'Install wasm-bindgen with: cargo install wasm-bindgen-cli --version %s --locked\n' \
            "$required_bindgen" >&2
    fi
    exit 1
fi

printf '\nAll development prerequisites are ready.\n'
