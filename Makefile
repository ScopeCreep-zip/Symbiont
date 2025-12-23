# Symbiont Development Makefile

.PHONY: all build test lint fmt clean release doc audit install-tools sim

# Default target
all: fmt lint test

# Build
build:
	cargo build

release:
	cargo build --release

# Testing
test:
	cargo test

test-verbose:
	cargo test -- --nocapture

# Linting and formatting
lint:
	cargo clippy --all-targets -- -D warnings

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

# Documentation
doc:
	cargo doc --no-deps --open

# Security
audit:
	cargo audit

deny:
	cargo deny check

# Clean
clean:
	cargo clean

# Install development tools
install-tools:
	cargo install cargo-audit
	cargo install cargo-deny
	cargo install cargo-tarpaulin
	pip install pre-commit
	pre-commit install

# Run simulations
sim-trust:
	cargo run --release -- run -s trust-emergence -n 20 -t 500 -v

sim-adversary:
	cargo run --release -- run -s strategic -n 20 -t 500 --inject-at 100 -v

sim-workflow:
	cargo run --release -- run -s workflow-chain -n 20 -t 500 -v

# Coverage
coverage:
	cargo tarpaulin --out Html --output-dir coverage

# Quick sanity check
check: fmt-check lint test
	@echo "All checks passed!"

# CI simulation (run what CI would run)
ci: fmt-check lint test audit
	@echo "CI checks passed!"
