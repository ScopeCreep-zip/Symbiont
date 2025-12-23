# Contributing to Symbiont

Thank you for your interest in contributing to Symbiont! This document provides guidelines and instructions for contributing.

## Code of Conduct

Please be respectful and constructive in all interactions. We welcome contributors of all backgrounds and experience levels.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/symbiont.git`
3. Create a feature branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Run lints: `cargo clippy`
7. Format code: `cargo fmt`
8. Commit and push
9. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70+
- Cargo

### Building

```bash
cargo build
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Linting

```bash
# Run clippy
cargo clippy -- -D warnings

# Run rustfmt check
cargo fmt -- --check
```

## Code Style

- Follow Rust standard naming conventions
- Use `cargo fmt` before committing
- Address all `cargo clippy` warnings
- Write doc comments for public APIs
- Add tests for new functionality

## Commit Messages

Use clear, descriptive commit messages:

```
feat: add new detection algorithm for collusion rings

- Implement community detection using connected components
- Add CollusionCluster structure with metrics
- Add tests for cluster detection
```

Prefixes:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation
- `test:` - Tests
- `refactor:` - Code refactoring
- `perf:` - Performance improvement
- `ci:` - CI/CD changes

## Pull Request Process

1. Ensure all tests pass
2. Update documentation if needed
3. Add tests for new functionality
4. Keep PRs focused and reasonably sized
5. Respond to review feedback promptly

## Architecture

### Core Crate (`symbiont-core`)

The protocol implementation:

- `types.rs` - Core types with newtypes for safety
- `node.rs` - Node state and behavior
- `connection.rs` - Connection dynamics (Physarum equation)
- `trust.rs` - Trust computation
- `defense.rs` - Defense signaling
- `routing.rs` - Task routing
- `workflow.rs` - Workflow orchestration
- `detection.rs` - Adversary detection

### Simulation Crate (`symbiont-sim`)

Testing and validation:

- `network.rs` - Simulated network
- `agents.rs` - Agent behavior models
- `scenarios/` - Test scenarios

### CLI Crate (`symbiont-cli`)

User interface for running simulations.

## Questions?

Open an issue for questions or discussion.
