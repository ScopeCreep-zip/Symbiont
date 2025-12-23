# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial implementation of Symbiont Mycorrhizal Trust Protocol
- Core protocol library (`symbiont-core`)
  - Physarum-inspired connection dynamics
  - Trust computation with diversity cap
  - Defense signaling and threat propagation
  - Task routing based on trust and capability
  - Workflow orchestration (sequential, parallel, DAG)
  - Adversary detection (strategic, Sybil, collusion)
  - Convergence protocol with agree-to-disagree
- Simulation harness (`symbiont-sim`)
  - Network simulation
  - Agent behavior models (honest, strategic adversary, free rider, Sybil)
  - Metrics collection
  - Multiple test scenarios
- Command-line interface (`symbiont-cli`)
  - Run simulations
  - Export metrics to CSV
  - Verbose progress reporting

### Changed
- N/A

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

## [0.1.0] - 2025-12-22

### Added
- Initial release
- Complete implementation of Symbiont v0.1 specification
- 88 passing tests
- Full documentation

[Unreleased]: https://github.com/YOUR_USERNAME/symbiont/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/YOUR_USERNAME/symbiont/releases/tag/v0.1.0
