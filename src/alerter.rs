use crate::error::Result;
use crate::config::ValidationConfig;
use crate::validator::ValidationResult;
use chrono::Utc;

pub struct Alerter {
    config: ValidationConfig,
}

#[derive(Debug)]
pub struct Alert {
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertSeverity {
    Warning,
    Blocking,
    Critical,
}

impl Alerter {
    pub fn new(config: ValidationConfig) -> Self {
        Self { config }
    }

    pub async fn alert(&self, result: &ValidationResult) -> Result<()> {
        let severity = self.determine_severity(&result);
        let alert = Alert {
            severity,
            message: result.message.clone(),
            timestamp: Utc::now(),
        };

        self.emit_alert(&alert).await?;
        Ok(())
    }

    fn determine_severity(&self, result: &ValidationResult) -> AlertSeverity {
        match &self.config.alert_threshold {
            crate::config::AlertThreshold::Warn => AlertSeverity::Warning,
            crate::config::AlertThreshold::Block => {
                if !result.quorum_availability {
                    AlertSeverity::Critical
                } else if !result.byzantine_resilience {
                    AlertSeverity::Blocking
                } else {
                    AlertSeverity::Warning
                }
            }
        }
    }

    async fn emit_alert(&self, alert: &Alert) -> Result<()> {
        tracing::warn!(
            severity = ?alert.severity,
            message = %alert.message,
            timestamp = %alert.timestamp,
            "CIMD ALERT"
        );

        match alert.severity {
            AlertSeverity::Critical => {
                tracing::error!("CRITICAL INTACTNESS VIOLATION: {}", alert.message);
            }
            AlertSeverity::Blocking => {
                tracing::warn!("BLOCKING CONDITION: {}", alert.message);
            }
            AlertSeverity::Warning => {
                tracing::info!("WARNING: {}", alert.message);
            }
        }

        Ok(())
    }
}
