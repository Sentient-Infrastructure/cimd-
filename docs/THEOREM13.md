# Theorem 13: Cumulative Intactness

Theorem 13 defines three conditions that must hold across network reconfigurations to maintain safety guarantees for slots still in progress.

## Condition 1: Quorum Availability

Every active slot must retain a quorum in the new configuration.

```
For each active slot S:
  ∃ quorum Q in new_config such that Q ⊆ participants(S)
```

This ensures that no slot loses its ability to reach consensus due to a reconfiguration.

## Condition 2: Byzantine Resilience

The network must maintain 1/3 fault tolerance after reconfiguration.

```
For all quorum sets Q in new_config:
  |Q| ≥ total_validators / 3 + 1
```

This preserves the network's resilience against Byzantine failures.

## Condition 3: Slot Finality

No reconfigurations can retroactively affect slots already in ballot phase.

```
For each slot S in ballot_phase(current_config):
  config_lock(S) = current_config
```

Once a slot has entered the ballot/commit phase, it is immutably bound to the current configuration.

## Implementation

The daemon continuously verifies all three conditions:
1. On each check interval, the Validator inspects all active slots and configurations
2. If any condition fails, the Alerter notifies operators
3. Before applying a reconfiguration, the Simulator models its impact against all conditions
4. Only reconfigurations that satisfy all conditions are approved
