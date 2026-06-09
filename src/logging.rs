use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

impl LogLevel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "debug" => Some(LogLevel::Debug),
            "info" => Some(LogLevel::Info),
            "warn" => Some(LogLevel::Warn),
            "error" => Some(LogLevel::Error),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub message: String,
    pub context: Option<String>,
}

impl LogEntry {
    pub fn new(level: LogLevel, message: impl Into<String>) -> Self {
        Self {
            level,
            timestamp: chrono::Utc::now(),
            message: message.into(),
            context: None,
        }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

pub struct Logger;

impl Logger {
    pub fn log(entry: LogEntry) {
        match entry.level {
            LogLevel::Debug => tracing::debug!("{}: {}", entry.level, entry.message),
            LogLevel::Info => tracing::info!("{}: {}", entry.level, entry.message),
            LogLevel::Warn => tracing::warn!("{}: {}", entry.level, entry.message),
            LogLevel::Error => tracing::error!("{}: {}", entry.level, entry.message),
        }
    }
}
