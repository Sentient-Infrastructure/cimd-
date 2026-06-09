mod config;
mod monitor;
mod validator;
mod simulator;
mod alerter;
mod error;
mod network;
mod slot;

use tracing_subscriber;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(name = "cimd")]
#[command(about = "Cumulative Intactness Monitor Daemon")]
struct Args {
    #[arg(short, long)]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let cfg = config::Config::load(&args.config)?;

    let daemon = Daemon::new(cfg).await?;
    daemon.run().await?;

    Ok(())
}

struct Daemon {
    monitor: monitor::Monitor,
    validator: validator::Validator,
    simulator: simulator::Simulator,
    alerter: alerter::Alerter,
}

impl Daemon {
    async fn new(cfg: config::Config) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            monitor: monitor::Monitor::new(cfg.network.clone()).await?,
            validator: validator::Validator::new(),
            simulator: simulator::Simulator::new(),
            alerter: alerter::Alerter::new(cfg.validation.clone()),
        })
    }

    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("cimd daemon starting");
        
        loop {
            if let Err(e) = self.check_intactness().await {
                tracing::error!("intactness check failed: {}", e);
            }
            
            tokio::time::sleep(tokio::time::Duration::from_millis(
                self.validator.check_interval_ms(),
            )).await;
        }
    }

    async fn check_intactness(&self) -> Result<(), Box<dyn std::error::Error>> {
        let state = self.monitor.get_network_state().await?;
        let result = self.validator.check_all_conditions(&state)?;
        
        if !result.is_valid() {
            self.alerter.alert(&result).await?;
        }
        
        Ok(())
    }
}
