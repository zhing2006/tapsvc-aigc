use std::time::Duration;

use tapsvc_aigc_core::{Retryable, is_retryable_status};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
        retry_after: Option<Duration>,
    },

    #[error("DashScope service error ({code}): {message}")]
    Service { code: String, message: String },

    #[error("Invalid DashScope response: {0}")]
    InvalidResponse(String),

    #[error("Failed to deserialize response: {0}")]
    Deserialize(#[from] serde_json::Error),
}

impl Retryable for Error {
    fn is_retryable(&self) -> bool {
        match self {
            Error::Request(_) => true,
            Error::Api { status, .. } => is_retryable_status(*status),
            Error::Service { .. } | Error::InvalidResponse(_) | Error::Deserialize(_) => false,
        }
    }

    fn retry_after(&self) -> Option<Duration> {
        match self {
            Error::Api { retry_after, .. } => *retry_after,
            _ => None,
        }
    }
}
