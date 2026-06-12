use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NodeId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodeSet {
    nodes: HashSet<NodeId>,
}

impl NodeSet {
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
        }
    }

    pub fn with_nodes(nodes: HashSet<NodeId>) -> Self {
        Self { nodes }
    }

    pub fn add(&mut self, node: NodeId) {
        self.nodes.insert(node);
    }

    pub fn contains(&self, node: &NodeId) -> bool {
        self.nodes.contains(node)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn intersection(&self, other: &NodeSet) -> NodeSet {
        let nodes = self.nodes.intersection(&other.nodes).cloned().collect();
        NodeSet::with_nodes(nodes)
    }

    pub fn union(&self, other: &NodeSet) -> NodeSet {
        let nodes = self.nodes.union(&other.nodes).cloned().collect();
        NodeSet::with_nodes(nodes)
    }

    pub fn iter(&self) -> impl Iterator<Item = &NodeId> {
        self.nodes.iter()
    }
}

impl Default for NodeSet {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuorumSet {
    pub threshold: u32,
    pub validators: NodeSet,
    pub inner_quorum_sets: Vec<QuorumSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfiguration {
    pub quorum_sets: HashMap<NodeId, QuorumSet>,
    pub timestamp: u64,
}

impl QuorumSet {
    pub fn new(threshold: u32, validators: NodeSet) -> Self {
        Self {
            threshold,
            validators,
            inner_quorum_sets: Vec::new(),
        }
    }

    pub fn has_quorum(&self, subset: &NodeSet) -> bool {
        let local_satisfied = subset.intersection(&self.validators).len() >= self.threshold as usize;
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
            .flat_map(|qs| qs.validators.iter())
            .collect::<HashSet<_>>()
            .len();
        
        if total_validators == 0 {
            return 0.0;
        }
        
        (total_validators as f64 - 1.0) / 3.0
    }
}
