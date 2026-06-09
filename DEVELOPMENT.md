# Development

## Project Structure

```
cimd-/
├── src/
│   ├── main.rs           # Daemon entry point
│   ├── lib.rs            # Library exports
│   ├── config.rs         # Configuration parsing
│   ├── monitor.rs        # Network state monitoring
│   ├── validator.rs      # Theorem 13 validation logic
│   ├── simulator.rs      # Reconfiguration simulation
│   ├── alerter.rs        # Alert notification system
│   ├── network.rs        # Network/quorum set types
│   ├── slot.rs           # Slot and phase tracking
│   ├── network_state.rs  # State transition tracking
│   ├── reconfiguration.rs # Reconfiguration proposals
│   ├── metrics.rs        # Metrics collection
│   └── logging.rs        # Logging utilities
├── tests/
│   ├── validator_test.rs     # Validator unit tests
│   └── integration_test.rs   # Full integration tests
├── docs/
│   ├── THEOREM13.md      # Theorem 13 explanation
│   └── DEPLOYMENT.md     # Deployment guide
├── .github/workflows/
│   └── ci.yml            # CI/CD pipeline
└── Cargo.toml            # Dependencies and metadata
```

## Quick Start

```bash
# Build
cargo build --release

# Run tests
cargo test

# Format
cargo fmt

# Lint
cargo clippy

# Run the daemon
./target/release/cimd --config config.example.toml
```

## Architecture Principles

- **Modularity**: Each component has a single responsibility (Monitor, Validator, Simulator, Alerter)
- **Async-first**: Uses Tokio for concurrent, non-blocking operations
- **Type safety**: Leverages Rust's type system to prevent configuration errors
- **Observable**: Integrated tracing and metrics for production monitoring
- **Testable**: Comprehensive test coverage with both unit and integration tests

## Adding Features

1. Define data types in appropriate modules (network.rs, slot.rs, etc.)
2. Implement validation logic in validator.rs if it concerns Theorem 13 conditions
3. Update config.rs if new configuration is needed
4. Add tests in tests/ directory
5. Update documentation in docs/
6. Run `cargo fmt` and `cargo clippy` before committing

## Common Tasks

### Adding a new metric
- Add field to `Metrics` struct in metrics.rs
- Update `MetricsCollector.snapshot()` to compute it
- Log it in alerter.rs if relevant

### Supporting a new alert threshold
- Add variant to `AlertThreshold` enum in config.rs
- Update `Alerter.determine_severity()` to handle it

### Modifying validator conditions
- Update logic in validator.rs
- Add test case to tests/integration_test.rs
- Update THEOREM13.md if implementation changes
