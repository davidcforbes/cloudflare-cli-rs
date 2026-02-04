use std::time::Duration;
use thiserror::Error;

pub mod category;

pub use category::ErrorCategory;

#[derive(Debug, Error)]
pub enum CfadError {
    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
        code: Option<u32>,
    },

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Rate limit exceeded")]
    RateLimit { retry_after: Option<Duration> },

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Resource not found: {resource_type} '{identifier}'")]
    NotFound {
        resource_type: String,
        identifier: String,
    },

    #[error("Operation timeout after {0:?}")]
    Timeout(Duration),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("TOML deserialize error: {0}")]
    TomlDe(#[from] toml::de::Error),

    #[error("TOML serialize error: {0}")]
    TomlSer(toml::ser::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("{0}")]
    Other(String),
}

impl CfadError {
    pub fn api(message: impl Into<String>) -> Self {
        Self::Api {
            status: 400,
            message: message.into(),
            code: None,
        }
    }

    pub fn api_with_code(status: u16, message: impl Into<String>, code: u32) -> Self {
        Self::Api {
            status,
            message: message.into(),
            code: Some(code),
        }
    }

    pub fn config(message: impl Into<String>) -> Self {
        Self::Config(message.into())
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    pub fn auth(message: impl Into<String>) -> Self {
        Self::Auth(message.into())
    }

    pub fn network(message: impl Into<String>) -> Self {
        Self::Network(message.into())
    }

    pub fn not_found(resource_type: impl Into<String>, identifier: impl Into<String>) -> Self {
        Self::NotFound {
            resource_type: resource_type.into(),
            identifier: identifier.into(),
        }
    }

    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Network(_) | Self::RateLimit { .. } | Self::Timeout(_)
        )
    }

    pub fn category(&self) -> ErrorCategory {
        match self {
            Self::Api { .. } => ErrorCategory::Api,
            Self::Auth(_) => ErrorCategory::Authentication,
            Self::Network(_) => ErrorCategory::Network,
            Self::RateLimit { .. } => ErrorCategory::RateLimit,
            Self::Config(_) => ErrorCategory::Configuration,
            Self::Validation(_) => ErrorCategory::Validation,
            Self::NotFound { .. } => ErrorCategory::NotFound,
            Self::Timeout(_) => ErrorCategory::Timeout,
            Self::Io(_) => ErrorCategory::FileSystem,
            Self::Json(_) | Self::TomlDe(_) | Self::TomlSer(_) => ErrorCategory::Serialization,
            Self::Http(_) => ErrorCategory::Network,
            Self::UrlParse(_) => ErrorCategory::Validation,
            Self::Other(_) => ErrorCategory::Other,
        }
    }

    pub fn from_cf_errors(errors: Vec<CfError>) -> Self {
        let messages: Vec<String> = errors
            .iter()
            .map(|e| format!("[{}] {}", e.code, e.message))
            .collect();

        Self::Api {
            status: 400,
            message: messages.join("; "),
            code: errors.first().map(|e| e.code),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CfError {
    pub code: u32,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct CfMessage {
    pub code: u32,
    pub message: String,
}

use serde::Deserialize;

pub type Result<T> = std::result::Result<T, CfadError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_api_constructor() {
        let error = CfadError::api("Test message");
        match error {
            CfadError::Api {
                status,
                message,
                code,
            } => {
                assert_eq!(status, 400);
                assert_eq!(message, "Test message");
                assert_eq!(code, None);
            }
            _ => panic!("Expected Api variant"),
        }
    }

    #[test]
    fn test_error_api_with_code_constructor() {
        let error = CfadError::api_with_code(403, "Forbidden", 1003);
        match error {
            CfadError::Api {
                status,
                message,
                code,
            } => {
                assert_eq!(status, 403);
                assert_eq!(message, "Forbidden");
                assert_eq!(code, Some(1003));
            }
            _ => panic!("Expected Api variant"),
        }
    }

    #[test]
    fn test_error_not_found_constructor() {
        let error = CfadError::not_found("DNS record", "abc123");
        match error {
            CfadError::NotFound {
                resource_type,
                identifier,
            } => {
                assert_eq!(resource_type, "DNS record");
                assert_eq!(identifier, "abc123");
            }
            _ => panic!("Expected NotFound variant"),
        }
    }

    #[test]
    fn test_error_is_retryable_network() {
        let error = CfadError::network("Connection failed");
        assert!(error.is_retryable());
    }

    #[test]
    fn test_error_is_retryable_rate_limit() {
        let error = CfadError::RateLimit { retry_after: None };
        assert!(error.is_retryable());
    }

    #[test]
    fn test_error_is_retryable_timeout() {
        let error = CfadError::Timeout(Duration::from_secs(30));
        assert!(error.is_retryable());
    }

    #[test]
    fn test_error_is_not_retryable_api() {
        let error = CfadError::api("Bad request");
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_error_is_not_retryable_auth() {
        let error = CfadError::auth("Invalid token");
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_error_is_not_retryable_validation() {
        let error = CfadError::validation("Invalid input");
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_error_category_api() {
        let error = CfadError::api("Test");
        assert!(matches!(error.category(), ErrorCategory::Api));
    }

    #[test]
    fn test_error_category_auth() {
        let error = CfadError::auth("Test");
        assert!(matches!(error.category(), ErrorCategory::Authentication));
    }

    #[test]
    fn test_error_category_network() {
        let error = CfadError::network("Test");
        assert!(matches!(error.category(), ErrorCategory::Network));
    }

    #[test]
    fn test_error_category_rate_limit() {
        let error = CfadError::RateLimit { retry_after: None };
        assert!(matches!(error.category(), ErrorCategory::RateLimit));
    }

    #[test]
    fn test_error_category_validation() {
        let error = CfadError::validation("Test");
        assert!(matches!(error.category(), ErrorCategory::Validation));
    }

    #[test]
    fn test_error_category_not_found() {
        let error = CfadError::not_found("Resource", "123");
        assert!(matches!(error.category(), ErrorCategory::NotFound));
    }

    #[test]
    fn test_error_from_cf_errors_single() {
        let cf_errors = vec![CfError {
            code: 1003,
            message: "Invalid zone".to_string(),
        }];

        let error = CfadError::from_cf_errors(cf_errors);
        match error {
            CfadError::Api { message, code, .. } => {
                assert_eq!(message, "[1003] Invalid zone");
                assert_eq!(code, Some(1003));
            }
            _ => panic!("Expected Api variant"),
        }
    }

    #[test]
    fn test_error_from_cf_errors_multiple() {
        let cf_errors = vec![
            CfError {
                code: 1003,
                message: "Invalid zone".to_string(),
            },
            CfError {
                code: 1004,
                message: "Invalid record".to_string(),
            },
        ];

        let error = CfadError::from_cf_errors(cf_errors);
        match error {
            CfadError::Api { message, code, .. } => {
                assert_eq!(message, "[1003] Invalid zone; [1004] Invalid record");
                assert_eq!(code, Some(1003)); // First error code
            }
            _ => panic!("Expected Api variant"),
        }
    }

    #[test]
    fn test_error_display_api() {
        let error = CfadError::api_with_code(404, "Not found", 1003);
        let display = format!("{}", error);
        assert!(display.contains("404"));
        assert!(display.contains("Not found"));
    }

    #[test]
    fn test_error_display_not_found() {
        let error = CfadError::not_found("Zone", "example.com");
        let display = format!("{}", error);
        assert!(display.contains("Zone"));
        assert!(display.contains("example.com"));
    }

    #[test]
    fn test_error_category_config() {
        let error = CfadError::config("Invalid config");
        assert!(matches!(error.category(), ErrorCategory::Configuration));
    }

    #[test]
    fn test_error_category_timeout() {
        let error = CfadError::Timeout(Duration::from_secs(30));
        assert!(matches!(error.category(), ErrorCategory::Timeout));
    }

    #[test]
    fn test_error_category_io() {
        let error = CfadError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        ));
        assert!(matches!(error.category(), ErrorCategory::FileSystem));
    }

    #[test]
    fn test_error_category_json() {
        let json_str = "{invalid json";
        let result: std::result::Result<serde_json::Value, serde_json::Error> =
            serde_json::from_str(json_str);
        if let Err(e) = result {
            let error = CfadError::from(e);
            assert!(matches!(error.category(), ErrorCategory::Serialization));
        }
    }

    #[test]
    fn test_error_category_url_parse() {
        use url::Url;
        let result = Url::parse("not a url");
        if let Err(e) = result {
            let error = CfadError::UrlParse(e);
            assert!(matches!(error.category(), ErrorCategory::Validation));
        }
    }

    #[test]
    fn test_error_category_other() {
        let error = CfadError::Other("Something went wrong".to_string());
        assert!(matches!(error.category(), ErrorCategory::Other));
    }

    #[test]
    fn test_error_category_toml_de() {
        // Test TomlDe error category
        let toml_str = "invalid = [toml";
        let result: std::result::Result<toml::Value, toml::de::Error> = toml::from_str(toml_str);
        if let Err(e) = result {
            let error = CfadError::from(e);
            assert!(matches!(error.category(), ErrorCategory::Serialization));
        }
    }

    #[test]
    fn test_error_category_http() {
        // Create a simple reqwest error by attempting to parse an invalid URL
        let url_result = reqwest::Url::parse("not a valid url");
        assert!(url_result.is_err());

        // Test with a client builder error
        let client_result = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(0))
            .build();

        if let Err(e) = client_result {
            let error = CfadError::from(e);
            assert!(matches!(error.category(), ErrorCategory::Network));
        }
    }
}
