use chrono::{DateTime, Utc};

pub struct ConsensusCheckpoint {
    pub checkpoint_id: u64,
    pub timestamp: DateTime<Utc>,
    pub slot_height: u64,
}

impl ConsensusCheckpoint {
    pub fn new(checkpoint_id: u64, slot_height: u64) -> Self {
        Self {
            checkpoint_id,
            timestamp: Utc::now(),
            slot_height,
        }
    }

    pub fn age_ms(&self) -> i64 {
        Utc::now()
            .signed_duration_since(self.timestamp)
            .num_milliseconds()
    }
}
