use crate::network::NetworkConfiguration;
use crate::slot::SlotState;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkState {
    pub config: NetworkConfiguration,
    pub slots: SlotState,
    pub snapshot_time: DateTime<Utc>,
    pub is_healthy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    pub from_state: NetworkState,
    pub to_state: NetworkState,
    pub changes: Vec<StateChange>,
    pub transition_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateChange {
    QuorumSetModified,
    SlotProgressed,
    ValidatorAdded,
    ValidatorRemoved,
    NetworkHealthDegraded,
    NetworkHealthRestored,
}

impl NetworkState {
    pub fn new(
        config: NetworkConfiguration,
        slots: SlotState,
    ) -> Self {
        Self {
            config,
            slots,
            snapshot_time: Utc::now(),
            is_healthy: true,
        }
    }

    pub fn with_health(mut self, is_healthy: bool) -> Self {
        self.is_healthy = is_healthy;
        self
    }
}

impl StateTransition {
    pub fn new(from_state: NetworkState, to_state: NetworkState) -> Self {
        let changes = Vec::new();
        let transition_time_ms = to_state
            .snapshot_time
            .signed_duration_since(from_state.snapshot_time)
            .num_milliseconds() as u64;

        Self {
            from_state,
            to_state,
            changes,
            transition_time_ms,
        }
    }

    pub fn is_safe(&self) -> bool {
        !self.changes.contains(&StateChange::NetworkHealthDegraded)
    }
}
