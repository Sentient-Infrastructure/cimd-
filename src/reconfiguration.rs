use crate::network::NetworkConfiguration;
use crate::error::{CimdError, Result};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconfigurationProposal {
    pub id: String,
    pub current_config: NetworkConfiguration,
    pub proposed_config: NetworkConfiguration,
    pub reason: String,
    pub proposed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReconfigurationStatus {
    Proposed,
    Validated,
    Approved,
    Rejected,
    Applied,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconfigurationDecision {
    pub proposal_id: String,
    pub status: ReconfigurationStatus,
    pub reason: String,
    pub decided_at: DateTime<Utc>,
    pub violations: Vec<String>,
}

impl ReconfigurationProposal {
    pub fn new(
        current_config: NetworkConfiguration,
        proposed_config: NetworkConfiguration,
        reason: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            current_config,
            proposed_config,
            reason,
            proposed_at: Utc::now(),
        }
    }

    pub fn validate(&self) -> Result<ReconfigurationDecision> {
        let mut violations = Vec::new();

        let from_bft = self.current_config.byzantine_fault_tolerance();
        let to_bft = self.proposed_config.byzantine_fault_tolerance();

        if to_bft < 0.333 {
            violations.push("Byzantine resilience below 1/3 threshold".to_string());
        }

        if to_bft < from_bft * 0.9 {
            violations.push("BFT degradation exceeds 10%".to_string());
        }

        let status = if violations.is_empty() {
            ReconfigurationStatus::Approved
        } else {
            ReconfigurationStatus::Rejected
        };

        Ok(ReconfigurationDecision {
            proposal_id: self.id.clone(),
            status,
            reason: format!("Reconfiguration {}: {}", if status == ReconfigurationStatus::Approved { "approved" } else { "rejected" }, self.reason),
            decided_at: Utc::now(),
            violations,
        })
    }
}

pub use uuid;
