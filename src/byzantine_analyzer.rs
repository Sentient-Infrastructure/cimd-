use crate::network::NetworkConfiguration;
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct ByzantineFaultAnalysis {
    pub total_validators: usize,
    pub fault_tolerance_ratio: f64,
    pub max_byzantine_nodes: usize,
    pub is_resilient: bool,
}

pub struct ByzantineAnalyzer;

impl ByzantineAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, config: &NetworkConfiguration) -> Result<ByzantineFaultAnalysis> {
        let total_validators = self.count_unique_validators(config);
        let max_byzantine = total_validators / 3;
        let fault_tolerance_ratio = config.byzantine_fault_tolerance();
        let is_resilient = fault_tolerance_ratio >= 0.333;

        Ok(ByzantineFaultAnalysis {
            total_validators,
            fault_tolerance_ratio,
            max_byzantine_nodes: max_byzantine,
            is_resilient,
        })
    }

    fn count_unique_validators(&self, config: &NetworkConfiguration) -> usize {
        config
            .quorum_sets
            .values()
            .flat_map(|qs| qs.validators.iter())
            .collect::<std::collections::HashSet<_>>()
            .len()
    }

    pub fn verify_resilience(&self, config: &NetworkConfiguration) -> Result<bool> {
        let analysis = self.analyze(config)?;
        Ok(analysis.is_resilient)
    }
}

impl Default for ByzantineAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byzantine_analyzer_creation() {
        let analyzer = ByzantineAnalyzer::new();
        assert_eq!(analyzer.count_unique_validators(&NetworkConfiguration::new(std::collections::HashMap::new())), 0);
    }
}
