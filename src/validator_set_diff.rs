use crate::network::{NodeId, NodeSet};
use std::collections::HashSet;

pub struct ValidatorSetDiff {
    pub added: Vec<NodeId>,
    pub removed: Vec<NodeId>,
    pub unchanged: Vec<NodeId>,
}

impl ValidatorSetDiff {
    pub fn compute(old_set: &NodeSet, new_set: &NodeSet) -> Self {
        let old_nodes: HashSet<_> = old_set.iter().cloned().collect();
        let new_nodes: HashSet<_> = new_set.iter().cloned().collect();

        let added: Vec<_> = new_nodes.difference(&old_nodes).cloned().collect();
        let removed: Vec<_> = old_nodes.difference(&new_nodes).cloned().collect();
        let unchanged: Vec<_> = old_nodes.intersection(&new_nodes).cloned().collect();

        Self {
            added,
            removed,
            unchanged,
        }
    }

    pub fn has_changes(&self) -> bool {
        !self.added.is_empty() || !self.removed.is_empty()
    }
}
