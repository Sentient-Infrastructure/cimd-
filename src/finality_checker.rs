use crate::slot::{SlotState, SlotPhase};
use crate::error::Result;

pub struct FinalityChecker;

impl FinalityChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn check_slot_finality(&self, slots: &SlotState) -> Result<bool> {
        let in_ballot = slots.slots.iter()
            .any(|s| s.phase == SlotPhase::Commit || s.phase == SlotPhase::Externalize);
        Ok(!in_ballot)
    }

    pub fn slots_in_progress(&self, slots: &SlotState) -> usize {
        slots.slots.iter()
            .filter(|s| s.phase != SlotPhase::Prepare)
            .count()
    }
}

impl Default for FinalityChecker {
    fn default() -> Self {
        Self::new()
    }
}
