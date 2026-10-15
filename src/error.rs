use thiserror::Error;

#[derive(Error, Debug)]
pub enum TempMailError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API error ({code}): {message}")]
    Api { code: u16, message: String },

    #[error("No active mailbox configured. Create one with new_mailbox() first.")]
    NoActiveMailbox,

    #[error("Timeout reached while waiting for email or OTP code.")]
    Timeout,
}

pub type Result<T> = std::result::Result<T, TempMailError>;
