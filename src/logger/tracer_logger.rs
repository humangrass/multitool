use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;
use serde::Serialize;

/// `LogLevel` defines the different levels of logging that can be used
/// within the application. These levels correspond to the common logging
/// levels found in Rust's logging libraries.
///
/// This enum supports deserialization via Serde and is compatible with
/// command-line arguments using Clap.
#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum LogLevel {
    /// Log level for informational messages.
    #[default]
    Info,
    /// Log level for detailed trace-level messages.
    Trace,
    /// Log level for debugging messages.
    Debug,
    /// Log level for warnings.
    Warn,
    /// Log level for errors.
    Error,
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    /// Converts a string into a `LogLevel`. The string is case-insensitive and
    /// should match one of the log levels: info, trace, debug, warn, or error.
    ///
    /// # Errors
    /// Returns an error if the input string does not match a valid log level.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "info" => Ok(LogLevel::Info),
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            _ => Err(format!("Invalid log level: {}", s)),
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "info"),
            LogLevel::Trace => write!(f, "trace"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}


/// Initializes a new tracing-based logger with the provided `LogLevel`.
///
/// This function configures a logger using the `tracing-subscriber` crate,
/// enabling structured logging with different log levels. The logger includes
/// thread information, target, and level in its output, using a compact format.
///
/// # Parameters
/// - `log_level`: The level of logging that should be used. This can be one of
///   `Info`, `Debug`, `Error`, `Warn`, or `Trace`.
///
/// # Example
/// ```rust
/// use multitool_hg::logger::tracer_logger::{LogLevel, new_tracer_logger};
/// use log::info;
///
/// fn main() {
///     new_tracer_logger(LogLevel::Info);
///     info!("Hello, world!")
///     // Your application logic here...
/// }
/// ```
pub fn new_tracer_logger(log_level: LogLevel) {
    let log_level_filter = match log_level {
        LogLevel::Trace => LevelFilter::TRACE,
        LogLevel::Debug => LevelFilter::DEBUG,
        LogLevel::Info => LevelFilter::INFO,
        LogLevel::Warn => LevelFilter::WARN,
        LogLevel::Error => LevelFilter::ERROR,
    };

    let format = fmt::format()
        .with_level(true) // include levels in formatted output
        .with_target(true) // include targets
        .with_thread_ids(true) // include the thread ID of the current thread
        .with_thread_names(true) // include the name of the current thread
        .compact(); // use the `Compact` formatting style.

    tracing_subscriber::fmt()
        .event_format(format)
        .with_max_level(log_level_filter)
        .init();
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    /// Test parsing valid log levels from strings.
    #[test]
    fn test_from_str_valid_levels() {
        assert_eq!(LogLevel::from_str("info").unwrap(), LogLevel::Info);
        assert_eq!(LogLevel::from_str("trace").unwrap(), LogLevel::Trace);
        assert_eq!(LogLevel::from_str("debug").unwrap(), LogLevel::Debug);
        assert_eq!(LogLevel::from_str("warn").unwrap(), LogLevel::Warn);
        assert_eq!(LogLevel::from_str("error").unwrap(), LogLevel::Error);
    }

    /// Test converting log levels to string format.
    #[test]
    fn test_display() {
        assert_eq!(LogLevel::Info.to_string(), "info");
        assert_eq!(LogLevel::Trace.to_string(), "trace");
        assert_eq!(LogLevel::Debug.to_string(), "debug");
        assert_eq!(LogLevel::Warn.to_string(), "warn");
        assert_eq!(LogLevel::Error.to_string(), "error");
    }

    /// Test parsing invalid log levels from strings.
    #[test]
    fn test_from_str_invalid_level() {
        assert!(LogLevel::from_str("invalid").is_err());
    }
}
