use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub snapshot_id: u64,
    pub timestamp: DateTime<Utc>,
    pub version: u32,
}

impl StateSnapshot {
    pub fn new(snapshot_id: u64) -> Self {
        Self {
            snapshot_id,
            timestamp: Utc::now(),
            version: 1,
        }
    }

    pub fn age_seconds(&self) -> u64 {
        Utc::now()
            .signed_duration_since(self.timestamp)
            .num_seconds() as u64
    }
}
