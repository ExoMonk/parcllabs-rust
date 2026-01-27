use thiserror::Error;

/// Errors that can occur when interacting with the Parcl Labs API
#[derive(Error, Debug)]
pub enum ParclError {
    #[error("API key not provided. Set PARCL_LABS_API_KEY environment variable or pass it to the client")]
    MissingApiKey,

    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("API error ({status}): {message}")]
    ApiError { status: u16, message: String },

    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

pub type Result<T> = std::result::Result<T, ParclError>;
