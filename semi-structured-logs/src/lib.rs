// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::fmt::{self, Display, Formatter};

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let log_text = match self {
            Self::Error => "ERROR",
            Self::Warning => "WARNING",
            Self::Info => "INFO",
            Self::Debug => "DEBUG"
        };

        write!(f, "[{}]", log_text)
    }
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
        LogLevel::Debug => debug(message)
    }
}

pub fn info(message: &str) -> String {
    format!("{}: {}", LogLevel::Info, message)
}

pub fn warn(message: &str) -> String {
    format!("{}: {}", LogLevel::Warning, message)
}

pub fn error(message: &str) -> String {
    format!("{}: {}", LogLevel::Error, message)
}

pub fn debug(message: &str) -> String {
    format!("{}: {}", LogLevel::Debug, message)
}