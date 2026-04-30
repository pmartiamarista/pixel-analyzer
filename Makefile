# pixel-analyzer Makefile
# Standardized build and maintenance targets

.PHONY: all build wasm wasm-dev test test-wasm lint fmt clippy clean check verify doc help demo-install demo-dev demo-build

# Configuration
CARGO      := cargo
WASM_PACK  := wasm-pack
TARGET_WEB := --target web

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

# Production WASM Build
build: wasm

wasm:
	@$(WASM_PACK) build $(TARGET_WEB) --release
	@echo "Injecting publishConfig into pkg/package.json..."
	@node -e 'const fs=require("fs"); const pkg=JSON.parse(fs.readFileSync("./pkg/package.json")); pkg.publishConfig={access:"public", registry:"https://registry.npmjs.org"}; fs.writeFileSync("./pkg/package.json", JSON.stringify(pkg, null, 2));'

# Development WASM Build (no optimizations, faster)
wasm-dev:
	@$(WASM_PACK) build $(TARGET_WEB) --dev

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
	@echo "make verify    - Run fmt check and clippy"
	@echo "make test      - Run all unit and integration tests"
	@echo "make test-wasm - Run WASM specific integration tests in Node"
	@echo "make build     - Alias for wasm"
	@echo "make wasm      - Build production WASM package (opt level z)"
	@echo "make wasm-dev  - Build development WASM package"
	@echo "make doc          - Generate and open documentation"
	@echo "make clean        - Remove build artifacts, pkg, and demo artifacts"
	@echo "make demo-install - Install demo dependencies"
	@echo "make demo-dev     - Build dev WASM and start demo dev server"
	@echo "make demo-build   - Build production WASM and demo dist"
	@echo ""
	@echo "Note: If wasm-pack fails due to target errors, ensure wasm32 is installed:"
	@echo "rustup target add wasm32-unknown-unknown"
