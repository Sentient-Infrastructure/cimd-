use std::collections::HashMap;

pub struct ConsensusRoundTracker {
    rounds: HashMap<u64, u32>,
}

impl ConsensusRoundTracker {
    pub fn new() -> Self {
        Self {
            rounds: HashMap::new(),
        }
    }

    pub fn increment_round(&mut self, slot_id: u64) {
        *self.rounds.entry(slot_id).or_insert(0) += 1;
    }

    pub fn get_round(&self, slot_id: u64) -> u32 {
        self.rounds.get(&slot_id).copied().unwrap_or(0)
    }
}

impl Default for ConsensusRoundTracker {
    fn default() -> Self {
        Self::new()
    }
}
