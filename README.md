# Cumulative Intactness Monitor Daemon (cimd-)

A live daemon that tracks all three conditions of Theorem 13's cumulative intactness across configuration changes in a running FBA network — alerting operators the moment a reconfiguration would retroactively invalidate safety guarantees for any slot still in progress.

[![Status](https://img.shields.io/badge/status-active-brightgreen)](.)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

---

## How It Works

1. The daemon connects to a live FBA network and monitors all quorum set configurations.
2. It continuously verifies Theorem 13's three cumulative intactness conditions across active slots.
3. When a reconfiguration is initiated, it simulates the change against all in-flight slots.
4. If the reconfiguration would invalidate safety guarantees for any slot, the daemon alerts operators immediately.
5. Only reconfiguraciones that preserve cumulative intactness across all active slots are permitted.

---

## Architecture

The daemon consists of:

- **Monitor** — tracks live network state and slot progress
- **Validator** — checks all three cumulative intactness conditions for each configuration
- **Simulator** — models the impact of proposed reconfigurations before they are applied
- **Alerter** — notifies operators of blocking conditions and state anomalies

---

## Prerequisites

| Requirement | Version |
|---|---|
| Rust | 1.70+ |
| Linux or macOS | Any recent version |

---

## Getting Started

### 1. Clone

```bash
git clone https://github.com/stellar/cimd-.git
cd cimd-
```

### 2. Build

```bash
cargo build --release
```

### 3. Configure

Create a configuration file with your FBA network details:

```toml
[network]
peer_address = "127.0.0.1:11625"

[validation]
check_interval_ms = 1000
alert_threshold = "block"
```

### 4. Run

```bash
./target/release/cimd- --config config.toml
```

The daemon will begin monitoring and reporting cumulative intactness status.

---

## Theorem 13 Conditions

The daemon enforces three intactness conditions across reconfigurations:

1. **Quorum Availability** — Every active slot retains a quorum in the new configuration.
2. **Byzantine Resilience** — The network maintains 1/3 fault tolerance after reconfiguration.
3. **Slot Finality** — No reconfigurations can retroactively affect slots already in ballot phase.

---

## Configuration

Key options:

| Option | Type | Default | Description |
|---|---|---|---|
| `peer_address` | string | `127.0.0.1:11625` | FBA peer connection address |
| `check_interval_ms` | integer | `1000` | How often to validate conditions (ms) |
| `alert_threshold` | enum | `block` | Alert on `warn` or `block` |
| `log_level` | string | `info` | Log verbosity: `debug`, `info`, `warn`, `error` |

---

## Contributing

1. Fork the repo
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Commit with conventional commits: `git commit -m "feat: add X"`
4. Open a pull request

---

## License

MIT — see [LICENSE](./LICENSE).

---

## Built on Stellar

cimd- safeguards Stellar FBA networks against state forks and retroactive ballot poisoning during dynamic reconfiguration.

- [Stellar Docs](https://developers.stellar.org)
- [FBA Consensus](https://developers.stellar.org/docs/learn/consensus)
