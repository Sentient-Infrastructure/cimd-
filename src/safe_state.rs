pub struct SafeStatePredicate {
    min_healthy_ratio: f64,
}

impl SafeStatePredicate {
    pub fn new(min_healthy_ratio: f64) -> Self {
        Self { min_healthy_ratio }
    }

    pub fn is_safe(&self, healthy_nodes: usize, total_nodes: usize) -> bool {
        if total_nodes == 0 {
            return false;
        }
        (healthy_nodes as f64) / (total_nodes as f64) >= self.min_healthy_ratio
    }
}

impl Default for SafeStatePredicate {
    fn default() -> Self {
        Self::new(0.666)
    }
}
