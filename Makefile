.PHONY: build test lint check clean fmt help

help:
	@echo "cimd - Cumulative Intactness Monitor Daemon"
	@echo ""
	@echo "Available targets:"
	@echo "  build       Build the release binary"
	@echo "  test        Run all tests"
	@echo "  lint        Run clippy linter"
	@echo "  fmt         Format code with rustfmt"
	@echo "  check       Check code without building"
	@echo "  clean       Remove build artifacts"

build:
	cargo build --release

test:
	cargo test --workspace --verbose

lint:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt --all

check:
	cargo check --all

clean:
	cargo clean
