use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SlotId(pub u64);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SlotPhase {
    Prepare,
    Commit,
    Externalize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slot {
    pub id: SlotId,
    pub phase: SlotPhase,
    pub sequence: u64,
    pub created_at: DateTime<Utc>,
    pub ballot_counter: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotState {
    pub slots: Vec<Slot>,
    pub timestamp: DateTime<Utc>,
}

impl Slot {
    pub fn new(id: u64) -> Self {
        Self {
            id: SlotId(id),
            phase: SlotPhase::Prepare,
            sequence: 0,
            created_at: Utc::now(),
            ballot_counter: 0,
        }
    }

    pub fn is_in_ballot_phase(&self) -> bool {
        self.phase != SlotPhase::Prepare
    }
}
