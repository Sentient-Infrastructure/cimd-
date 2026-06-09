use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub timestamp: DateTime<Utc>,
    pub checks_performed: u64,
    pub violations_detected: u64,
    pub alerts_issued: u64,
    pub avg_check_time_ms: f64,
    pub network_health_score: f64,
}

#[derive(Debug, Clone, Default)]
pub struct MetricsCollector {
    checks_performed: u64,
    violations_detected: u64,
    alerts_issued: u64,
    check_times: Vec<u64>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_check(&mut self, duration_ms: u64) {
        self.checks_performed += 1;
        self.check_times.push(duration_ms);
    }

    pub fn record_violation(&mut self) {
        self.violations_detected += 1;
    }

    pub fn record_alert(&mut self) {
        self.alerts_issued += 1;
    }

    pub fn snapshot(&self) -> Metrics {
        let avg_check_time_ms = if self.check_times.is_empty() {
            0.0
        } else {
            let sum: u64 = self.check_times.iter().sum();
            sum as f64 / self.check_times.len() as f64
        };

        let health_score = if self.checks_performed == 0 {
            1.0
        } else {
            1.0 - (self.violations_detected as f64 / self.checks_performed as f64).min(1.0)
        };

        Metrics {
            timestamp: Utc::now(),
            checks_performed: self.checks_performed,
            violations_detected: self.violations_detected,
            alerts_issued: self.alerts_issued,
            avg_check_time_ms,
            network_health_score: health_score,
        }
    }
}
