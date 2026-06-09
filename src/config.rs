use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub validation: ValidationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub peer_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub check_interval_ms: u64,
    pub alert_threshold: AlertThreshold,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertThreshold {
    Warn,
    Block,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            check_interval_ms: 1000,
            alert_threshold: AlertThreshold::Block,
            log_level: "info".to_string(),
        }
    }
}
