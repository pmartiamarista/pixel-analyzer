.PHONY: all build wasm test lint fmt clippy clean check

all: lint test build

build:
	@cargo build

wasm:
	@wasm-pack build --target web --release

test:
	@cargo test

lint: fmt clippy

check:
	@cargo fmt -- --check
	@cargo clippy -- -D warnings

fmt:
	@cargo fmt

clippy:
	@cargo clippy -- -D warnings

clean:
	@cargo clean
	@rm -rf pkg
