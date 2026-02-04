#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    Api,
    Authentication,
    Network,
    RateLimit,
    Configuration,
    Validation,
    NotFound,
    Timeout,
    FileSystem,
    Serialization,
    Other,
}

impl ErrorCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Api => "API Error",
            Self::Authentication => "Authentication Error",
            Self::Network => "Network Error",
            Self::RateLimit => "Rate Limit",
            Self::Configuration => "Configuration Error",
            Self::Validation => "Validation Error",
            Self::NotFound => "Not Found",
            Self::Timeout => "Timeout",
            Self::FileSystem => "File System Error",
            Self::Serialization => "Serialization Error",
            Self::Other => "Error",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_category_as_str_api() {
        assert_eq!(ErrorCategory::Api.as_str(), "API Error");
    }

    #[test]
    fn test_error_category_as_str_authentication() {
        assert_eq!(
            ErrorCategory::Authentication.as_str(),
            "Authentication Error"
        );
    }

    #[test]
    fn test_error_category_as_str_network() {
        assert_eq!(ErrorCategory::Network.as_str(), "Network Error");
    }

    #[test]
    fn test_error_category_as_str_rate_limit() {
        assert_eq!(ErrorCategory::RateLimit.as_str(), "Rate Limit");
    }

    #[test]
    fn test_error_category_as_str_configuration() {
        assert_eq!(ErrorCategory::Configuration.as_str(), "Configuration Error");
    }

    #[test]
    fn test_error_category_as_str_validation() {
        assert_eq!(ErrorCategory::Validation.as_str(), "Validation Error");
    }

    #[test]
    fn test_error_category_as_str_not_found() {
        assert_eq!(ErrorCategory::NotFound.as_str(), "Not Found");
    }

    #[test]
    fn test_error_category_as_str_timeout() {
        assert_eq!(ErrorCategory::Timeout.as_str(), "Timeout");
    }

    #[test]
    fn test_error_category_as_str_file_system() {
        assert_eq!(ErrorCategory::FileSystem.as_str(), "File System Error");
    }

    #[test]
    fn test_error_category_as_str_serialization() {
        assert_eq!(ErrorCategory::Serialization.as_str(), "Serialization Error");
    }

    #[test]
    fn test_error_category_as_str_other() {
        assert_eq!(ErrorCategory::Other.as_str(), "Error");
    }

    #[test]
    fn test_error_category_equality() {
        assert_eq!(ErrorCategory::Api, ErrorCategory::Api);
        assert_ne!(ErrorCategory::Api, ErrorCategory::Network);
    }

    #[test]
    fn test_error_category_clone() {
        let cat1 = ErrorCategory::Timeout;
        let cat2 = cat1;
        assert_eq!(cat1, cat2);
    }
}
