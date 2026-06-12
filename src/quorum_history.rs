use crate::network::NetworkConfiguration;
use chrono::{DateTime, Utc};

pub struct QuorumConfigHistory {
    configs: Vec<(NetworkConfiguration, DateTime<Utc>)>,
}

impl QuorumConfigHistory {
    pub fn new() -> Self {
        Self {
            configs: Vec::new(),
        }
    }

    pub fn add_config(&mut self, config: NetworkConfiguration) {
        self.configs.push((config, Utc::now()));
    }

    pub fn latest(&self) -> Option<&NetworkConfiguration> {
        self.configs.last().map(|(c, _)| c)
    }

    pub fn history_len(&self) -> usize {
        self.configs.len()
    }
}

impl Default for QuorumConfigHistory {
    fn default() -> Self {
        Self::new()
    }
}
