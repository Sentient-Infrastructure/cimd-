use crate::slot::SlotId;

pub struct SlotOrderingVerifier;

impl SlotOrderingVerifier {
    pub fn new() -> Self {
        Self
    }

    pub fn verify_ordering(slots: &[SlotId]) -> bool {
        for window in slots.windows(2) {
            if window[0].0 >= window[1].0 {
                return false;
            }
        }
        true
    }
}

impl Default for SlotOrderingVerifier {
    fn default() -> Self {
        Self::new()
    }
}
