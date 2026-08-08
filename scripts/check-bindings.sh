#!/usr/bin/env bash
# Builds the C ABI and Python bindings and runs their smoke tests: full games
# played through each surface against the built-in opponents, plus the error
# paths. The default aggregate is strict; `available` is the explicitly
# best-effort local mode that skips Python when it is unavailable.
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

suite="${1:-all}"
if [[ $# -gt 1 ]]; then
    echo "usage: $0 [all|available|c|python]" >&2
    exit 2
fi

out_dir="$(mktemp -d "${TMPDIR:-/tmp}/penta-bindings.XXXXXX")"
trap 'rm -rf "$out_dir"' EXIT

check_c() {
    echo "== C ABI =="
    cargo build --locked --release -p penta-ffi
    cc bindings/penta-ffi/smoke.c target/release/libpenta_ffi.a \
        -I bindings/penta-ffi -o "$out_dir/smoke"
    "$out_dir/smoke"
}

check_python() {
    echo "== Python =="
    if ! command -v python3 >/dev/null; then
        if [[ "$suite" == available ]]; then
            echo "python3 not found; skipping the Python binding"
            return 0
        fi
        echo "python3 is required for the Python binding check" >&2
        return 127
    fi
    (cd bindings/penta-py && cargo build --locked --release)
    case "$(uname)" in
        Darwin) built="bindings/penta-py/target/release/libpenta.dylib" ;;
        *) built="bindings/penta-py/target/release/libpenta.so" ;;
    esac
    cp "$built" "$out_dir/penta.so"
    cp bindings/penta-py/smoke.py "$out_dir/"
    (cd "$out_dir" && python3 smoke.py)
}

case "$suite" in
    all)
        check_c
        check_python
        ;;
    available)
        check_c
        check_python
        ;;
    c) check_c ;;
    python) check_python ;;
    *)
        echo "usage: $0 [all|available|c|python]" >&2
        exit 2
        ;;
esac
