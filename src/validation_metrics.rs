use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationMetrics {
    pub checks_passed: u64,
    pub checks_failed: u64,
    pub avg_check_time_ms: f64,
}

impl ValidationMetrics {
    pub fn new() -> Self {
        Self {
            checks_passed: 0,
            checks_failed: 0,
            avg_check_time_ms: 0.0,
        }
    }

    pub fn total_checks(&self) -> u64 {
        self.checks_passed + self.checks_failed
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_checks() == 0 {
            1.0
        } else {
            self.checks_passed as f64 / self.total_checks() as f64
        }
    }
}

impl Default for ValidationMetrics {
    fn default() -> Self {
        Self::new()
    }
}
