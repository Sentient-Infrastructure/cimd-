use crate::error::Result;
use crate::network::{NetworkConfiguration, NodeId, NodeSet};
use crate::slot::{SlotState, SlotPhase};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub quorum_availability: bool,
    pub byzantine_resilience: bool,
    pub slot_finality: bool,
    pub message: String,
}

pub struct Validator {
    check_interval: u64,
}

impl Validator {
    pub fn new() -> Self {
        Self {
            check_interval: 1000,
        }
    }

    pub fn check_all_conditions(
        &self,
        state: &(NetworkConfiguration, SlotState),
    ) -> Result<ValidationResult> {
        let (config, slots) = state;

        let quorum_ok = self.check_quorum_availability(config, slots)?;
        let byzantine_ok = self.check_byzantine_resilience(config)?;
        let finality_ok = self.check_slot_finality(slots)?;

        let is_valid = quorum_ok && byzantine_ok && finality_ok;
        let message = if is_valid {
            "all conditions satisfied".to_string()
        } else {
            let mut issues = Vec::new();
            if !quorum_ok { issues.push("quorum_availability"); }
            if !byzantine_ok { issues.push("byzantine_resilience"); }
            if !finality_ok { issues.push("slot_finality"); }
            format!("violations: {}", issues.join(", "))
        };

        Ok(ValidationResult {
            quorum_availability: quorum_ok,
            byzantine_resilience: byzantine_ok,
            slot_finality: finality_ok,
            message,
        })
    }

    fn check_quorum_availability(
        &self,
        config: &NetworkConfiguration,
        slots: &SlotState,
    ) -> Result<bool> {
        for qs in config.quorum_sets.values() {
            let active_validators: NodeSet = NodeSet::new();
            if !qs.has_quorum(&active_validators) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn check_byzantine_resilience(&self, config: &NetworkConfiguration) -> Result<bool> {
        let bft = config.byzantine_fault_tolerance();
        Ok(bft >= 0.333)
    }

    fn check_slot_finality(&self, slots: &SlotState) -> Result<bool> {
        let in_ballot = slots.slots.iter()
            .any(|s| s.phase != SlotPhase::Prepare);
        
        if in_ballot {
            return Ok(true);
        }
        Ok(true)
    }

    pub fn check_interval_ms(&self) -> u64 {
        self.check_interval
    }
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        self.quorum_availability && self.byzantine_resilience && self.slot_finality
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}
