use crate::models::RegistrationMessage;
use socket2::SockAddr;
use std::error::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WizError {
    #[error("IO: {0}")]
    IOErr(#[from] std::io::Error),
    #[error("Timeout elapsed: {0}")]
    TimeOut(#[from] tokio::time::error::Elapsed),
    #[error("Invalid format: {0}")]
    InvalidFormat(#[from] time::error::InvalidFormatDescription),
    #[error("Time offset error: {0}")]
    InvalidOffset(#[from] time::error::IndeterminateOffset),
    #[error("Time formatting error: {0}")]
    TimeFormat(#[from] time::error::Format),
    #[error("No identifier in {0}")]
    NoIdent(String),
    // #[error("Other error: {0}")]
    // Eyre(#[from] color_eyre::Report),
    #[error("No IP in registration message: {0:?}")]
    NoIP(RegistrationMessage),
    #[error("Registration result failed: {0:?}")]
    RegErr(RegistrationMessage),
    #[error("Tracing subscriber init fail")]
    TracingInitErr(String),
    #[error("Serde JSON error: {0}")]
    JSONErr(#[from] serde_json::Error),
    #[error("Template error: {0}")]
    TemplateErr(#[from] indicatif::style::TemplateError),
    #[error("No minimum value")]
    NoMinimum,
    #[error("No maximum value")]
    NoMaximum,
    #[error("{0:?} is not an IPv4 address")]
    IP6(SockAddr),
    #[error("Address parse error: {0}")]
    AddrParseError(#[from] std::net::AddrParseError),
}

impl From<Box<dyn std::error::Error + Send + Sync + 'static>> for WizError {
    fn from(value: Box<dyn Error + Send + Sync + 'static>) -> Self {
        Self::TracingInitErr(value.to_string())
    }
}

pub type Result<T> = std::result::Result<T, WizError>;
