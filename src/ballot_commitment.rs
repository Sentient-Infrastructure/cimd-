use std::collections::HashMap;
use crate::slot::SlotId;

pub struct BallotCommitment {
    commitments: HashMap<SlotId, bool>,
}

impl BallotCommitment {
    pub fn new() -> Self {
        Self {
            commitments: HashMap::new(),
        }
    }

    pub fn commit(&mut self, slot_id: SlotId) {
        self.commitments.insert(slot_id, true);
    }

    pub fn is_committed(&self, slot_id: &SlotId) -> bool {
        self.commitments.get(slot_id).copied().unwrap_or(false)
    }

    pub fn committed_count(&self) -> usize {
        self.commitments.values().filter(|&&v| v).count()
    }
}

impl Default for BallotCommitment {
    fn default() -> Self {
        Self::new()
    }
}
