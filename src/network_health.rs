use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkHealth {
    pub healthy_nodes: usize,
    pub unhealthy_nodes: usize,
    pub health_ratio: f64,
}

impl NetworkHealth {
    pub fn new(healthy: usize, unhealthy: usize) -> Self {
        let total = healthy + unhealthy;
        let ratio = if total == 0 { 0.0 } else { healthy as f64 / total as f64 };
        Self {
            healthy_nodes: healthy,
            unhealthy_nodes: unhealthy,
            health_ratio: ratio,
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.health_ratio > 0.666
    }
}
