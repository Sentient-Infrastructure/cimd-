pub struct AnomalyDetector {
    threshold: f64,
}

impl AnomalyDetector {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    pub fn detect_anomaly(&self, value: f64, baseline: f64) -> bool {
        let deviation = (value - baseline).abs() / baseline.max(0.1);
        deviation > self.threshold
    }
}

impl Default for AnomalyDetector {
    fn default() -> Self {
        Self::new(0.5)
    }
}
