use crate::error::Result;
use crate::network::NetworkConfiguration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconfigurationPlan {
    pub from_config: NetworkConfiguration,
    pub to_config: NetworkConfiguration,
    pub is_safe: bool,
    pub reason: String,
}

pub struct Simulator;

impl Simulator {
    pub fn new() -> Self {
        Self
    }

    pub fn simulate_reconfiguration(
        &self,
        current: &NetworkConfiguration,
        proposed: &NetworkConfiguration,
    ) -> Result<ReconfigurationPlan> {
        let from_bft = current.byzantine_fault_tolerance();
        let to_bft = proposed.byzantine_fault_tolerance();

        let is_safe = to_bft >= 0.333 && to_bft >= from_bft * 0.95;
        let reason = if is_safe {
            "reconfiguration maintains Byzantine resilience".to_string()
        } else {
            format!(
                "reconfiguration reduces BFT from {:.2} to {:.2}",
                from_bft, to_bft
            )
        };

        Ok(ReconfigurationPlan {
            from_config: current.clone(),
            to_config: proposed.clone(),
            is_safe,
            reason,
        })
    }

    pub fn validate_in_flight_slots(
        &self,
        plan: &ReconfigurationPlan,
        active_slot_count: usize,
    ) -> Result<bool> {
        if active_slot_count > 0 && !plan.is_safe {
            return Ok(false);
        }
        Ok(true)
    }
}

impl Default for Simulator {
    fn default() -> Self {
        Self::new()
    }
}
