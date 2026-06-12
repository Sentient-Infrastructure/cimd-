pub struct ThresholdValidator;

impl ThresholdValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_threshold(threshold: u32, total: u32) -> bool {
        threshold > 0 && threshold <= total
    }

    pub fn calculate_byzantine_threshold(total: u32) -> u32 {
        total / 3
    }
}

impl Default for ThresholdValidator {
    fn default() -> Self {
        Self::new()
    }
}
