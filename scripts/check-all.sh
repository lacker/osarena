#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Compatibility entry point. The root Makefile is the canonical task graph.
exec make -C "$repo_root" check
