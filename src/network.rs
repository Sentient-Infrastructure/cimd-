use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NodeId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuorumSet {
    pub threshold: u32,
    pub validators: HashSet<NodeId>,
    pub inner_quorum_sets: Vec<QuorumSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfiguration {
    pub quorum_sets: HashMap<NodeId, QuorumSet>,
    pub timestamp: u64,
}

impl QuorumSet {
    pub fn new(threshold: u32, validators: HashSet<NodeId>) -> Self {
        Self {
            threshold,
            validators,
            inner_quorum_sets: Vec::new(),
        }
    }

    pub fn has_quorum(&self, subset: &HashSet<NodeId>) -> bool {
        let local_satisfied = subset.intersection(&self.validators).count() >= self.threshold as usize;
        let inner_satisfied = self.inner_quorum_sets.iter()
            .filter(|qs| qs.has_quorum(subset))
            .count();
        
        local_satisfied && inner_satisfied >= self.inner_quorum_sets.len().saturating_sub(
            self.inner_quorum_sets.len().saturating_sub(self.threshold as usize)
        )
    }

    pub fn validator_count(&self) -> usize {
        self.validators.len()
    }
}

impl NetworkConfiguration {
    pub fn new(quorum_sets: HashMap<NodeId, QuorumSet>) -> Self {
        Self {
            quorum_sets,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    pub fn byzantine_fault_tolerance(&self) -> f64 {
        let total_validators: usize = self.quorum_sets.values()
            .flat_map(|qs| &qs.validators)
            .collect::<HashSet<_>>()
            .len();
        
        if total_validators == 0 {
            return 0.0;
        }
        
        (total_validators as f64 - 1.0) / 3.0
    }
}
