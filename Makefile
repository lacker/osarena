FILTER ?=
PATTERN ?=
override RUST_TEST_FILTER := $(value FILTER)
override TEST_PATTERN := $(value PATTERN)
export RUST_TEST_FILTER
export TEST_PATTERN

WEB_WASM_CONTRACT_SUITE := tests/wasm-contract.suite.mjs
WEB_WASM_CASTING_SUITE := tests/wasm-casting.suite.mjs
WEB_WASM_COMBAT_SUITE := tests/wasm-combat.suite.mjs
WEB_WASM_PACING_SUITE := tests/wasm-pacing.suite.mjs
WEB_WASM_STATE_SUITE := tests/wasm-state.suite.mjs
WEB_WASM_FAST_SUITES := $(WEB_WASM_CONTRACT_SUITE) $(WEB_WASM_CASTING_SUITE) \
	$(WEB_WASM_COMBAT_SUITE) $(WEB_WASM_PACING_SUITE) $(WEB_WASM_STATE_SUITE)
WEB_WASM_SLOW_SUITES := tests/wasm-combat-slow.suite.mjs tests/wasm-pacing-slow.suite.mjs
WEB_ROOT_TESTS := $(patsubst web/%,%,$(filter-out web/tests/wasm-game.test.mjs,$(wildcard web/tests/*.test.mjs)))
WEB_FAST_ROOT_TESTS := $(filter-out tests/rendered-html.test.mjs,$(WEB_ROOT_TESTS))

define run_web_tests
	cd web && if [ -n "$$TEST_PATTERN" ]; then \
		CI=true node --test --test-name-pattern="$$TEST_PATTERN" $(1); \
	else \
		CI=true node --test $(1); \
	fi
endef

define run_rust_tests
	if [ -n "$$RUST_TEST_FILTER" ]; then \
		cargo test --locked $(1) "$$RUST_TEST_FILTER" $(2); \
	else \
		cargo test --locked $(1) $(2); \
	fi
endef

.PHONY: help doctor fmt fmt-rust fmt-python-binding \
	lint lint-rust lint-web lint-infra lint-python-binding \
	test test-rust test-rust-full test-rust-slow \
	test-engine test-engine-unit test-engine-integration test-policy test-wasm-rust \
	build-wasm build-web \
	test-web test-web-fast test-web-unit test-web-full \
	test-web-wasm test-web-wasm-full test-web-wasm-slow \
	test-web-wasm-contract test-web-wasm-casting test-web-wasm-combat \
	test-web-wasm-pacing test-web-wasm-state typecheck-web \
	test-web-render test-slow \
	check-fast check check-rust check-web \
	check-bindings check-bindings-available check-bindings-c check-bindings-python ci

help: ## List the available validation and build targets.
	@awk 'BEGIN { FS = ":.*## " } /^[a-zA-Z0-9_.-]+:.*## / { printf "  %-28s %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
	@printf '\nOptional filters:\n'
	@printf '  FILTER=<substring>           Narrow a Rust test target.\n'
	@printf '  PATTERN=<regular-expression> Narrow a browser/WASM test target.\n'

doctor: ## Verify the local toolchain and exact generator versions.
	./scripts/doctor.sh

fmt-rust: ## Check formatting for the root Rust workspace.
	cargo fmt --all -- --check

fmt-python-binding: ## Check formatting for the standalone Python binding crate.
	cargo fmt --manifest-path bindings/penta-py/Cargo.toml -- --check

fmt: fmt-rust fmt-python-binding ## Check formatting for every Rust crate.

lint-rust: ## Lint every Rust workspace target and feature.
	cargo clippy --locked --workspace --all-targets --all-features -- -D warnings

lint-web: ## Lint the web client.
	cd web && CI=true pnpm lint

lint-infra: ## Statically check shell scripts and GitHub Actions workflows.
	shellcheck scripts/*.sh
	actionlint

lint-python-binding: ## Lint the standalone Python binding crate.
	cargo clippy --manifest-path bindings/penta-py/Cargo.toml --locked --all-targets --all-features -- -D warnings

lint: lint-rust lint-web lint-infra ## Run engine, web, and infrastructure linters.

test-engine: ## Run the normal tests for the core engine package.
	$(call run_rust_tests,-p penta,)

test-engine-unit: ## Run core engine library tests, optionally filtered.
	$(call run_rust_tests,-p penta --lib,)

test-engine-integration: ## Run engine integration tests, optionally filtered.
	$(call run_rust_tests,-p penta --test engine,)

test-policy: ## Run policy integration tests, optionally filtered.
	$(call run_rust_tests,-p penta --test policy,)

test-wasm-rust: ## Run native unit tests for the Rust WASM adapter.
	$(call run_rust_tests,-p penta-wasm --lib,)

test-rust: ## Run normal Rust tests; simulation sweeps stay deferred.
	$(call run_rust_tests,--workspace --all-targets,)

test-rust-slow: ## Run only ignored Rust simulation sweeps.
	$(call run_rust_tests,--workspace --all-targets,-- --ignored)

test-rust-full: ## Run every normal and slow Rust test in one pass.
	cargo test --locked --workspace --all-targets -- --include-ignored

build-wasm: ## Build the release WASM module and generated bindings.
	./scripts/build-wasm.sh

test-web-wasm-contract: build-wasm ## Run browser contract and packaging tests.
	$(call run_web_tests,$(WEB_WASM_CONTRACT_SUITE))

test-web-wasm-casting: build-wasm ## Run browser casting and targeting tests.
	$(call run_web_tests,$(WEB_WASM_CASTING_SUITE))

test-web-wasm-combat: build-wasm ## Run fast browser combat tests.
	$(call run_web_tests,$(WEB_WASM_COMBAT_SUITE))

test-web-wasm-pacing: build-wasm ## Run fast browser priority and pacing tests.
	$(call run_web_tests,$(WEB_WASM_PACING_SUITE))

test-web-wasm-state: build-wasm ## Run browser state and event-log tests.
	$(call run_web_tests,$(WEB_WASM_STATE_SUITE))

test-web-wasm: build-wasm ## Run all fast browser-facing WASM suites.
	$(call run_web_tests,$(WEB_WASM_FAST_SUITES))

test-web-wasm-slow: build-wasm ## Run only slow browser-facing WASM sweeps.
	$(call run_web_tests,$(WEB_WASM_SLOW_SUITES))

test-web-wasm-full: build-wasm ## Run every browser-facing WASM test unfiltered.
	cd web && CI=true node --test $(WEB_WASM_FAST_SUITES) $(WEB_WASM_SLOW_SUITES)

typecheck-web: build-wasm ## Type-check the web client without writing compiler state.
	cd web && CI=true pnpm exec tsc --noEmit --incremental false --pretty false

build-web: build-wasm ## Build the production web application.
	cd web && CI=true pnpm run build:app

test-web-render: build-web ## Test the built server-rendered application shell.
	cd web && CI=true node --test tests/rendered-html.test.mjs

test-web-unit: ## Run fast standalone Node tests outside the WASM suites.
	@if [ -n "$(strip $(WEB_FAST_ROOT_TESTS))" ]; then \
		cd web && CI=true node --test $(WEB_FAST_ROOT_TESTS); \
	else \
		echo "No standalone fast web tests discovered"; \
	fi

test-web-fast: test-web-unit test-web-wasm ## Run every fast web test without a production build.

test-web: test-web-fast test-web-render ## Run the normal web tests.

test-web-full: build-web ## Run every discovered web test unfiltered.
	cd web && CI=true node --test $(WEB_ROOT_TESTS) $(WEB_WASM_FAST_SUITES) $(WEB_WASM_SLOW_SUITES)

test: test-rust test-web ## Run normal Rust and web tests.

test-slow: test-rust-slow test-web-wasm-slow ## Run only simulation-heavy suites.

check-fast: fmt-rust lint test-rust typecheck-web test-web-fast ## Run the broad checkpoint without slow tests or a production web build.

check-rust: fmt-rust lint-rust test-rust-full ## Run the complete root Rust workspace gate.

check-web: lint-web typecheck-web test-web-full ## Run the complete web gate.

check: check-rust check-web lint-infra ## Run the complete engine, web, and tooling gate.

check-bindings-c: ## Build and smoke-test only the C ABI.
	./scripts/check-bindings.sh c

check-bindings-python: ## Build and smoke-test only the Python module.
	./scripts/check-bindings.sh python

check-bindings: fmt-python-binding lint-python-binding ## Strictly validate both bot bindings.
	./scripts/check-bindings.sh all

check-bindings-available: ## Smoke-test bindings available on this machine.
	./scripts/check-bindings.sh available

ci: check check-bindings ## Run every repository gate.
