use crate::error::Result;
use crate::network::NetworkConfiguration;
use crate::slot::SlotState;
use crate::config::NetworkConfig;
use tokio::sync::RwLock;
use std::sync::Arc;
use chrono::Utc;

pub struct Monitor {
    peer_address: String,
    state: Arc<RwLock<MonitorState>>,
}

struct MonitorState {
    network_config: Option<NetworkConfiguration>,
    slot_state: Option<SlotState>,
    last_update: chrono::DateTime<Utc>,
}

impl Monitor {
    pub async fn new(config: NetworkConfig) -> Result<Self> {
        Ok(Self {
            peer_address: config.peer_address,
            state: Arc::new(RwLock::new(MonitorState {
                network_config: None,
                slot_state: None,
                last_update: Utc::now(),
            })),
        })
    }

    pub async fn get_network_state(&self) -> Result<(NetworkConfiguration, SlotState)> {
        let state = self.state.read().await;
        
        let config = state.network_config.clone()
            .ok_or_else(|| crate::error::CimdError::NetworkError("no network config available".to_string()))?;
        
        let slots = state.slot_state.clone()
            .ok_or_else(|| crate::error::CimdError::NetworkError("no slot state available".to_string()))?;
        
        Ok((config, slots))
    }

    pub async fn update_network_config(&self, config: NetworkConfiguration) {
        let mut state = self.state.write().await;
        state.network_config = Some(config);
        state.last_update = Utc::now();
    }

    pub async fn update_slot_state(&self, slots: SlotState) {
        let mut state = self.state.write().await;
        state.slot_state = Some(slots);
        state.last_update = Utc::now();
    }

    pub fn peer_address(&self) -> &str {
        &self.peer_address
    }
}
