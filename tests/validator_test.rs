use cimd::validator::Validator;
use cimd::network::{NetworkConfiguration, QuorumSet, NodeId};
use cimd::slot::{SlotState, Slot};
use chrono::Utc;
use std::collections::HashMap;

#[test]
fn test_quorum_availability_check() {
    let validator = Validator::new();
    let mut quorum_sets = HashMap::new();
    let node = NodeId("node1".to_string());
    let qs = QuorumSet::new(1, vec![node].into_iter().collect());
    quorum_sets.insert(node, qs);

    let config = NetworkConfiguration::new(quorum_sets);
    let slots = SlotState {
        slots: vec![Slot::new(1)],
        timestamp: Utc::now(),
    };

    let result = validator.check_all_conditions(&(config, slots));
    assert!(result.is_ok());
}

#[test]
fn test_byzantine_resilience() {
    let validator = Validator::new();
    let mut quorum_sets = HashMap::new();

    for i in 0..4 {
        let node = NodeId(format!("node{}", i));
        let qs = QuorumSet::new(3, vec![node.clone()].into_iter().collect());
        quorum_sets.insert(node, qs);
    }

    let config = NetworkConfiguration::new(quorum_sets);
    let bft = config.byzantine_fault_tolerance();
    assert!(bft > 0.0);
}
