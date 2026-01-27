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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_api_key_display() {
        let err = ParclError::MissingApiKey;
        assert!(err.to_string().contains("API key not provided"));
        assert!(err.to_string().contains("PARCL_LABS_API_KEY"));
    }

    #[test]
    fn api_error_display() {
        let err = ParclError::ApiError {
            status: 404,
            message: "Not found".into(),
        };
        assert_eq!(err.to_string(), "API error (404): Not found");
    }

    #[test]
    fn invalid_parameter_display() {
        let err = ParclError::InvalidParameter("limit must be positive".into());
        assert_eq!(err.to_string(), "Invalid parameter: limit must be positive");
    }

    #[test]
    fn parse_error_from_serde() {
        let json_err = serde_json::from_str::<i32>("not a number").unwrap_err();
        let err: ParclError = json_err.into();
        assert!(matches!(err, ParclError::ParseError(_)));
        assert!(err.to_string().contains("Failed to parse response"));
    }
}
