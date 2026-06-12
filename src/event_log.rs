use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: u64,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

pub struct EventLog {
    events: Vec<Event>,
    next_id: u64,
}

impl EventLog {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            next_id: 0,
        }
    }

    pub fn log_event(&mut self, message: String) {
        self.events.push(Event {
            id: self.next_id,
            message,
            timestamp: Utc::now(),
        });
        self.next_id += 1;
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl Default for EventLog {
    fn default() -> Self {
        Self::new()
    }
}
