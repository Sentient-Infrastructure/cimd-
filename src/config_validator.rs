use crate::network::NetworkConfiguration;
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct ConfigValidationError {
    pub field: String,
    pub message: String,
}

pub struct ConfigValidator;

impl ConfigValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_network_config(&self, config: &NetworkConfiguration) -> Result<Vec<ConfigValidationError>> {
        let mut errors = Vec::new();

        if config.quorum_sets.is_empty() {
            errors.push(ConfigValidationError {
                field: "quorum_sets".to_string(),
                message: "at least one quorum set required".to_string(),
            });
        }

        Ok(errors)
    }

    pub fn is_valid_config(&self, config: &NetworkConfiguration) -> Result<bool> {
        let errors = self.validate_network_config(config)?;
        Ok(errors.is_empty())
    }
}

impl Default for ConfigValidator {
    fn default() -> Self {
        Self::new()
    }
}
