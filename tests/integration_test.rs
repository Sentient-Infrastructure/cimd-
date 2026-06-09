use cimd::network::{NetworkConfiguration, QuorumSet, NodeId};
use cimd::slot::{SlotState, Slot, SlotPhase};
use cimd::validator::Validator;
use cimd::simulator::Simulator;
use cimd::reconfiguration::ReconfigurationProposal;
use chrono::Utc;
use std::collections::HashMap;

#[test]
fn test_full_validation_flow() {
    let validator = Validator::new();
    let mut quorum_sets = HashMap::new();

    for i in 0..4 {
        let node = NodeId(format!("validator-{}", i));
        let qs = QuorumSet::new(3, vec![node.clone()].into_iter().collect());
        quorum_sets.insert(node, qs);
    }

    let config = NetworkConfiguration::new(quorum_sets);
    let slots = SlotState {
        slots: vec![Slot::new(1), Slot::new(2)],
        timestamp: Utc::now(),
    };

    let result = validator.check_all_conditions(&(config, slots));
    assert!(result.is_ok());
}

#[test]
fn test_reconfiguration_validation() {
    let mut from_quorum = HashMap::new();
    for i in 0..4 {
        let node = NodeId(format!("validator-{}", i));
        let qs = QuorumSet::new(3, vec![node.clone()].into_iter().collect());
        from_quorum.insert(node, qs);
    }

    let mut to_quorum = HashMap::new();
    for i in 0..3 {
        let node = NodeId(format!("validator-{}", i));
        let qs = QuorumSet::new(2, vec![node.clone()].into_iter().collect());
        to_quorum.insert(node, qs);
    }

    let from_config = NetworkConfiguration::new(from_quorum);
    let to_config = NetworkConfiguration::new(to_quorum);

    let proposal = ReconfigurationProposal::new(from_config, to_config, "reduce validator set".to_string());
    let decision = proposal.validate();

    assert!(decision.is_ok());
}

#[test]
fn test_slot_phase_transitions() {
    let mut slot = Slot::new(1);
    assert_eq!(slot.phase, SlotPhase::Prepare);
    assert!(!slot.is_in_ballot_phase());

    slot.phase = SlotPhase::Commit;
    assert!(slot.is_in_ballot_phase());

    slot.phase = SlotPhase::Externalize;
    assert!(slot.is_in_ballot_phase());
}

#[test]
fn test_simulator_reconfiguration() {
    let simulator = Simulator::new();

    let mut from_quorum = HashMap::new();
    for i in 0..4 {
        let node = NodeId(format!("validator-{}", i));
        let qs = QuorumSet::new(3, vec![node.clone()].into_iter().collect());
        from_quorum.insert(node, qs);
    }

    let mut to_quorum = from_quorum.clone();
    let new_node = NodeId("validator-4".to_string());
    let qs = QuorumSet::new(3, vec![new_node.clone()].into_iter().collect());
    to_quorum.insert(new_node, qs);

    let from_config = NetworkConfiguration::new(from_quorum);
    let to_config = NetworkConfiguration::new(to_quorum);

    let plan = simulator.simulate_reconfiguration(&from_config, &to_config);
    assert!(plan.is_ok());
}

#[test]
fn test_network_bft_calculation() {
    let mut quorum_sets = HashMap::new();

    for i in 0..9 {
        let node = NodeId(format!("validator-{}", i));
        let qs = QuorumSet::new(6, vec![node.clone()].into_iter().collect());
        quorum_sets.insert(node, qs);
    }

    let config = NetworkConfiguration::new(quorum_sets);
    let bft = config.byzantine_fault_tolerance();

    assert!(bft > 0.3);
    assert!(bft < 4.0);
}
