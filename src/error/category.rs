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
