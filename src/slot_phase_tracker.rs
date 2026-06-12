use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SlotPhaseId(pub u64);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PhaseTransition {
    InitiatePrepare,
    AdvanceToCommit,
    AdvanceToExternalize,
    Finalize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseTransitionEvent {
    pub slot_id: u64,
    pub transition: PhaseTransition,
    pub timestamp: DateTime<Utc>,
    pub sequence_number: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotPhaseTracker {
    pub current_slot_id: u64,
    pub last_transition: Option<PhaseTransitionEvent>,
    pub transition_history: Vec<PhaseTransitionEvent>,
}

impl SlotPhaseTracker {
    pub fn new(slot_id: u64) -> Self {
        Self {
            current_slot_id: slot_id,
            last_transition: None,
            transition_history: Vec::new(),
        }
    }

    pub fn record_transition(&mut self, transition: PhaseTransition) -> PhaseTransitionEvent {
        let event = PhaseTransitionEvent {
            slot_id: self.current_slot_id,
            transition,
            timestamp: Utc::now(),
            sequence_number: self.transition_history.len() as u64,
        };

        self.transition_history.push(event.clone());
        self.last_transition = Some(event.clone());
        event
    }

    pub fn is_in_ballot_phase(&self) -> bool {
        self.last_transition
            .as_ref()
            .map(|t| matches!(t.transition, PhaseTransition::AdvanceToCommit | PhaseTransition::AdvanceToExternalize))
            .unwrap_or(false)
    }

    pub fn transition_count(&self) -> usize {
        self.transition_history.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot_phase_tracker_creation() {
        let tracker = SlotPhaseTracker::new(1);
        assert_eq!(tracker.current_slot_id, 1);
        assert_eq!(tracker.transition_count(), 0);
    }

    #[test]
    fn test_record_transition() {
        let mut tracker = SlotPhaseTracker::new(1);
        tracker.record_transition(PhaseTransition::InitiatePrepare);
        assert_eq!(tracker.transition_count(), 1);
    }
}
