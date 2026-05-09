# pixel-analyzer Makefile
# Standardized build and maintenance targets

.PHONY: all build wasm wasm-dev test test-wasm lint fmt clippy clean check verify doc help demo-install demo-dev demo-build

# Configuration
CARGO        := cargo
WASM_PACK    := wasm-pack

# --target bundler: WASM loading is delegated to the consumer's bundler (Vite, webpack).
# This eliminates the globalThis["fetch"] call in the generated JS glue that
# --target web produces, resolving the socket.dev network-access alert.
# Users of this npm package are expected to use a modern bundler.
# For unbundled browser usage, build from source with: wasm-pack build --target web
TARGET       := --target bundler

# Default target
all: verify wasm

# Run all quality checks and tests
verify: check test test-wasm

# Project Checks
check: fmt-check clippy

fmt-check:
	@$(CARGO) fmt -- --check

clippy:
	@$(CARGO) clippy -- -D warnings

fmt:
	@$(CARGO) fmt

# Production WASM Build (bundler target — no fetch in generated glue)
build: wasm

wasm:
	@$(WASM_PACK) build $(TARGET) --release
	@echo "Injecting publishConfig into pkg/package.json..."
	@node -e 'const fs=require("fs"); const pkg=JSON.parse(fs.readFileSync("./pkg/package.json")); pkg.publishConfig={access:"public", registry:"https://registry.npmjs.org"}; fs.writeFileSync("./pkg/package.json", JSON.stringify(pkg, null, 2));'

# Development WASM Build (no optimizations, faster)
wasm-dev:
	@$(WASM_PACK) build $(TARGET) --dev

# Testing
test:
	@$(CARGO) test

test-wasm:
	@$(WASM_PACK) test --node

# Documentation
doc:
	@$(CARGO) doc --no-deps --open

# Cleanup
clean:
	@$(CARGO) clean
	@rm -rf pkg
	@rm -rf demo/dist
	@rm -rf demo/node_modules

# Demo Management
demo-install:
	@cd demo && npm install

demo-dev: wasm-dev
	@cd demo && npm run dev

demo-build: wasm
	@cd demo && npm run build

# Utility HELP
help:
	@echo "Pixel Analyzer Build System"
	@echo "---------------------------"
	@echo "make verify    - Run fmt check, clippy, and all tests"
	@echo "make test      - Run all native integration tests"
	@echo "make test-wasm - Run WASM tests in Node.js via wasm-pack"
	@echo "make build     - Alias for wasm"
	@echo "make wasm      - Build production WASM package (--target bundler, -Oz)"
	@echo "make wasm-dev  - Build development WASM package (no wasm-opt)"
	@echo "make doc       - Generate and open Rust documentation"
	@echo "make clean     - Remove build artifacts, pkg, and demo artifacts"
	@echo ""
	@echo "Demo targets:"
	@echo "make demo-install - Install demo npm dependencies"
	@echo "make demo-dev     - Build dev WASM and start Vite dev server"
	@echo "make demo-build   - Build production WASM and demo dist"
	@echo ""
	@echo "Build target notes:"
	@echo "  Published package uses --target bundler (no fetch in JS glue)."
	@echo "  For unbundled browser use: wasm-pack build --target web"
	@echo "  WASM target required: rustup target add wasm32-unknown-unknown"