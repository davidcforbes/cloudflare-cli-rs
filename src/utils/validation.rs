use crate::error::{CfadError, Result};

pub fn validate_not_empty(value: &str, field_name: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(CfadError::validation(format!(
            "{} cannot be empty",
            field_name
        )));
    }
    Ok(())
}

pub fn validate_url(url: &str) -> Result<()> {
    url::Url::parse(url).map_err(|_| CfadError::validation("Invalid URL"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_not_empty_valid() {
        assert!(validate_not_empty("some value", "field").is_ok());
        assert!(validate_not_empty("x", "field").is_ok());
        assert!(validate_not_empty("  value  ", "field").is_ok());
    }

    #[test]
    fn test_validate_not_empty_invalid() {
        let result = validate_not_empty("", "username");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Validation error: username cannot be empty"
        );

        let result = validate_not_empty("   ", "password");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Validation error: password cannot be empty"
        );
    }

    #[test]
    fn test_validate_url_valid() {
        assert!(validate_url("https://example.com").is_ok());
        assert!(validate_url("http://example.com").is_ok());
        assert!(validate_url("https://example.com/path").is_ok());
        assert!(validate_url("https://example.com:8080").is_ok());
        assert!(validate_url("https://example.com/path?query=value").is_ok());
        assert!(validate_url("https://sub.domain.example.com").is_ok());
    }

    #[test]
    fn test_validate_url_invalid() {
        assert!(validate_url("not a url").is_err());
        assert!(validate_url("").is_err());
        assert!(validate_url("example.com").is_err()); // Missing scheme
        assert!(validate_url("http://").is_err()); // Missing domain
        assert!(validate_url("://example.com").is_err()); // Missing scheme
    }
}
