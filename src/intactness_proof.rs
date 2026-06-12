use crate::network::NodeId;
use chrono::{DateTime, Utc};

pub struct IntactnessProof {
    pub proof_id: u64,
    pub verified_nodes: Vec<NodeId>,
    pub timestamp: DateTime<Utc>,
}

impl IntactnessProof {
    pub fn new(proof_id: u64, verified_nodes: Vec<NodeId>) -> Self {
        Self {
            proof_id,
            verified_nodes,
            timestamp: Utc::now(),
        }
    }

    pub fn node_count(&self) -> usize {
        self.verified_nodes.len()
    }
}
