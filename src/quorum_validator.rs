use crate::network::{NodeId, NodeSet, QuorumSet, NetworkConfiguration};
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct QuorumValidationResult {
    pub is_valid: bool,
    pub failed_quorums: Vec<NodeId>,
    pub reason: String,
}

pub struct QuorumValidator;

impl QuorumValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_quorum_intersection(
        &self,
        config: &NetworkConfiguration,
        nodes: &NodeSet,
    ) -> Result<QuorumValidationResult> {
        let mut failed_quorums = Vec::new();

        for (node_id, quorum_set) in &config.quorum_sets {
            if !quorum_set.has_quorum(nodes) {
                failed_quorums.push(node_id.clone());
            }
        }

        let is_valid = failed_quorums.is_empty();
        let reason = if is_valid {
            "all quorums satisfied".to_string()
        } else {
            format!("{} quorum(s) failed", failed_quorums.len())
        };

        Ok(QuorumValidationResult {
            is_valid,
            failed_quorums,
            reason,
        })
    }

    pub fn get_quorum_size(&self, quorum_set: &QuorumSet) -> usize {
        quorum_set.validator_count()
    }

    pub fn check_threshold_breach(
        &self,
        quorum_set: &QuorumSet,
        available_nodes: &NodeSet,
    ) -> Result<bool> {
        let intersection = available_nodes.intersection(&quorum_set.validators);
        let has_threshold = intersection.len() >= quorum_set.threshold as usize;
        Ok(has_threshold)
    }
}

impl Default for QuorumValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quorum_validator_creation() {
        let validator = QuorumValidator::new();
        assert_eq!(validator.get_quorum_size(&QuorumSet::new(2, NodeSet::new())), 0);
    }
}
