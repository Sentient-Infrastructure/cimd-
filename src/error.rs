use thiserror::Error;

#[derive(Error, Debug)]
pub enum CimdError {
    #[error("network error: {0}")]
    NetworkError(String),

    #[error("validation failed: {0}")]
    ValidationError(String),

    #[error("configuration error: {0}")]
    ConfigError(String),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("intactness violation: {0}")]
    IntactnessViolation(String),
}

pub type Result<T> = std::result::Result<T, CimdError>;
