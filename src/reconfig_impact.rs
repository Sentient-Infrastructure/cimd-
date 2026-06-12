use crate::network::NetworkConfiguration;

#[derive(Debug, Clone)]
pub struct ReconfigurationImpact {
    pub affected_slots: usize,
    pub severity: u32,
}

impl ReconfigurationImpact {
    pub fn analyze(old: &NetworkConfiguration, new: &NetworkConfiguration) -> Self {
        let severity = if old.timestamp == new.timestamp { 0 } else { 1 };
        Self {
            affected_slots: 0,
            severity,
        }
    }
}
