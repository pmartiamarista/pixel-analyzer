# pixel-analyzer Makefile
# Standardized build and maintenance targets

.PHONY: all build wasm wasm-dev test lint fmt clippy clean check verify doc help

# Configuration
CARGO      := cargo
WASM_PACK  := wasm-pack
TARGET_WEB := --target web

# Default target
all: verify wasm

# Run all quality checks and tests
verify: check test

# Project Checks
check: fmt-check clippy

fmt-check:
	@$(CARGO) fmt -- --check

clippy:
	@$(CARGO) clippy -- -D warnings

fmt:
	@$(CARGO) fmt

# Production WASM Build
wasm:
	@$(WASM_PACK) build $(TARGET_WEB) --release

# Development WASM Build (no optimizations, faster)
wasm-dev:
	@$(WASM_PACK) build $(TARGET_WEB) --dev

# Testing
test:
	@$(CARGO) test

test-wasm:
	@$(WASM_PACK) test --headless --chrome

# Documentation
doc:
	@$(CARGO) doc --no-deps --open

# Cleanup
clean:
	@$(CARGO) clean
	@rm -rf pkg

# Utility HELP
help:
	@echo "Pixel Analyzer Build System"
	@echo "---------------------------"
	@echo "make verify    - Run fmt check and clippy"
	@echo "make test      - Run all unit and integration tests"
	@echo "make wasm      - Build production WASM package (opt level z)"
	@echo "make wasm-dev  - Build development WASM package"
	@echo "make doc       - Generate and open documentation"
	@echo "make clean     - Remove build artifacts and pkg folder"
	@echo ""
	@echo "Note: If wasm-pack fails due to target errors, ensure wasm32 is installed:"
	@echo "rustup target add wasm32-unknown-unknown"
