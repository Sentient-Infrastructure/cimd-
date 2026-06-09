# Architecture

## Component Overview

### Monitor
Tracks live network state and slot progress by polling the peer address and maintaining current network configuration and slot states.

### Validator
Implements Theorem 13's three cumulative intactness conditions:
- Quorum Availability: verifies all active slots retain quorum in new configuration
- Byzantine Resilience: ensures network maintains 1/3 fault tolerance
- Slot Finality: prevents reconfigurations from affecting slots in ballot phase

### Simulator
Models the impact of proposed reconfigurations before they are applied. Validates that in-flight slots remain safe across the reconfiguration.

### Alerter
Notifies operators of blocking conditions and state anomalies based on configured alert threshold (warn or block).

## Data Flow

1. Monitor polls network peer and updates internal state
2. Validator checks all three conditions against current state
3. If conditions fail, Alerter emits alerts based on severity
4. On reconfiguration proposal, Simulator models impact and validates safety
5. Daemon logs all state changes and violations

## Configuration

See `config.example.toml` for all available options.
