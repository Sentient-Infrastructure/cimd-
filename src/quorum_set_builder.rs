use crate::network::{NodeId, NodeSet, QuorumSet};

pub struct QuorumSetBuilder {
    threshold: u32,
    validators: NodeSet,
    inner_sets: Vec<QuorumSet>,
}

impl QuorumSetBuilder {
    pub fn new(threshold: u32) -> Self {
        Self {
            threshold,
            validators: NodeSet::new(),
            inner_sets: Vec::new(),
        }
    }

    pub fn add_validator(mut self, node: NodeId) -> Self {
        self.validators.add(node);
        self
    }

    pub fn add_inner_set(mut self, qs: QuorumSet) -> Self {
        self.inner_sets.push(qs);
        self
    }

    pub fn build(self) -> QuorumSet {
        let mut qs = QuorumSet::new(self.threshold, self.validators);
        qs.inner_quorum_sets = self.inner_sets;
        qs
    }
}
