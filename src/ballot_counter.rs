use std::collections::HashMap;
use crate::slot::SlotId;

pub struct BallotCounter {
    counters: HashMap<SlotId, u32>,
}

impl BallotCounter {
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
        }
    }

    pub fn increment(&mut self, slot_id: SlotId) {
        *self.counters.entry(slot_id).or_insert(0) += 1;
    }

    pub fn get(&self, slot_id: &SlotId) -> u32 {
        self.counters.get(slot_id).copied().unwrap_or(0)
    }

    pub fn reset(&mut self, slot_id: SlotId) {
        self.counters.remove(&slot_id);
    }
}

impl Default for BallotCounter {
    fn default() -> Self {
        Self::new()
    }
}
