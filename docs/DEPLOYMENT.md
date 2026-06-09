# Deployment Guide

## Prerequisites

- Rust 1.70+
- Linux or macOS
- Access to an FBA network peer

## Build for Production

```bash
cargo build --release
```

The binary will be at `target/release/cimd`.

## Configuration

Create a production config file:

```toml
[network]
peer_address = "your-stellar-peer.example.com:11625"

[validation]
check_interval_ms = 500
alert_threshold = "block"
log_level = "warn"
```

## Running as Systemd Service

Create `/etc/systemd/system/cimd.service`:

```ini
[Unit]
Description=Cumulative Intactness Monitor Daemon
After=network.target

[Service]
Type=simple
User=cimd
WorkingDirectory=/opt/cimd
ExecStart=/opt/cimd/cimd --config /etc/cimd/config.toml
Restart=on-failure
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

Then:

```bash
sudo systemctl daemon-reload
sudo systemctl enable cimd
sudo systemctl start cimd
```

## Monitoring

Monitor the daemon with:

```bash
sudo journalctl -u cimd -f
```

## Troubleshooting

### Connection refused
- Verify the peer address is correct
- Check network connectivity to the peer
- Ensure the peer is running

### High alert rate
- Review configuration thresholds
- Check network health
- Examine logs for underlying issues
