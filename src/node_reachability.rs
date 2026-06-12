use crate::network::NodeId;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeStatus {
    Reachable,
    Unreachable,
    Degraded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeReachability {
    pub node_id: NodeId,
    pub status: NodeStatus,
    pub last_ping: DateTime<Utc>,
    pub consecutive_failures: u32,
    pub latency_ms: u64,
}

pub struct NodeReachabilityTracker {
    nodes: HashMap<NodeId, NodeReachability>,
    failure_threshold: u32,
}

impl NodeReachabilityTracker {
    pub fn new(failure_threshold: u32) -> Self {
        Self {
            nodes: HashMap::new(),
            failure_threshold,
        }
    }

    pub fn register_node(&mut self, node_id: NodeId) {
        if !self.nodes.contains_key(&node_id) {
            self.nodes.insert(
                node_id.clone(),
                NodeReachability {
                    node_id,
                    status: NodeStatus::Degraded,
                    last_ping: Utc::now(),
                    consecutive_failures: 0,
                    latency_ms: 0,
                },
            );
        }
    }

    pub fn record_success(&mut self, node_id: &NodeId, latency_ms: u64) {
        if let Some(reachability) = self.nodes.get_mut(node_id) {
            reachability.status = NodeStatus::Reachable;
            reachability.last_ping = Utc::now();
            reachability.consecutive_failures = 0;
            reachability.latency_ms = latency_ms;
        }
    }

    pub fn record_failure(&mut self, node_id: &NodeId) {
        if let Some(reachability) = self.nodes.get_mut(node_id) {
            reachability.consecutive_failures += 1;
            reachability.last_ping = Utc::now();
            reachability.status = if reachability.consecutive_failures >= self.failure_threshold {
                NodeStatus::Unreachable
            } else {
                NodeStatus::Degraded
            };
        }
    }

    pub fn get_reachable_nodes(&self) -> Vec<&NodeId> {
        self.nodes
            .values()
            .filter(|r| r.status == NodeStatus::Reachable)
            .map(|r| &r.node_id)
            .collect()
    }

    pub fn get_unreachable_nodes(&self) -> Vec<&NodeId> {
        self.nodes
            .values()
            .filter(|r| r.status == NodeStatus::Unreachable)
            .map(|r| &r.node_id)
            .collect()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_reachability_tracker_creation() {
        let tracker = NodeReachabilityTracker::new(3);
        assert_eq!(tracker.node_count(), 0);
    }

    #[test]
    fn test_register_node() {
        let mut tracker = NodeReachabilityTracker::new(3);
        let node = NodeId("node1".to_string());
        tracker.register_node(node);
        assert_eq!(tracker.node_count(), 1);
    }
}
