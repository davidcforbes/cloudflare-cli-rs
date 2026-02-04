use crate::error::{CfadError, Result};
use regex::Regex;

pub fn validate_config(_profile: &str) -> Result<()> {
    // Validation logic for config profiles
    Ok(())
}

pub fn validate_email(email: &str) -> Result<()> {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .map_err(|_| CfadError::validation("Invalid regex"))?;

    if !email_regex.is_match(email) {
        return Err(CfadError::validation("Invalid email address"));
    }

    Ok(())
}

pub fn validate_domain(domain: &str) -> Result<()> {
    let domain_regex = Regex::new(r"^([a-zA-Z0-9-]+\.)*[a-zA-Z0-9-]+\.[a-zA-Z]{2,}$")
        .map_err(|_| CfadError::validation("Invalid regex"))?;

    if !domain_regex.is_match(domain) {
        return Err(CfadError::validation("Invalid domain name"));
    }

    Ok(())
}

pub fn validate_ip(ip: &str) -> Result<()> {
    ip.parse::<std::net::IpAddr>()
        .map_err(|_| CfadError::validation("Invalid IP address"))?;
    Ok(())
}

pub fn validate_zone_id(zone_id: &str) -> Result<()> {
    if zone_id.len() != 32 || !zone_id.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(CfadError::validation("Invalid zone ID format"));
    }
    Ok(())
}

pub fn validate_record_id(record_id: &str) -> Result<()> {
    if record_id.len() != 32 || !record_id.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(CfadError::validation("Invalid record ID format"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_config() {
        assert!(validate_config("default").is_ok());
        assert!(validate_config("production").is_ok());
        assert!(validate_config("").is_ok());
    }

    #[test]
    fn test_validate_email_valid() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("test.user+tag@domain.co.uk").is_ok());
        assert!(validate_email("name123@test-domain.com").is_ok());
    }

    #[test]
    fn test_validate_email_invalid() {
        assert!(validate_email("invalid").is_err());
        assert!(validate_email("@example.com").is_err());
        assert!(validate_email("user@").is_err());
        assert!(validate_email("user@domain").is_err());
        assert!(validate_email("user domain@example.com").is_err());
    }

    #[test]
    fn test_validate_domain_valid() {
        assert!(validate_domain("example.com").is_ok());
        assert!(validate_domain("subdomain.example.com").is_ok());
        assert!(validate_domain("deep.sub.domain.example.co.uk").is_ok());
        assert!(validate_domain("test-domain.com").is_ok());
    }

    #[test]
    fn test_validate_domain_invalid() {
        assert!(validate_domain("invalid").is_err());
        assert!(validate_domain(".example.com").is_err());
        assert!(validate_domain("example.").is_err());
        assert!(validate_domain("example .com").is_err());
        assert!(validate_domain("").is_err());
    }

    #[test]
    fn test_validate_ip_valid() {
        assert!(validate_ip("192.168.1.1").is_ok());
        assert!(validate_ip("10.0.0.1").is_ok());
        assert!(validate_ip("2001:0db8:85a3:0000:0000:8a2e:0370:7334").is_ok());
        assert!(validate_ip("::1").is_ok());
        assert!(validate_ip("fe80::1").is_ok());
    }

    #[test]
    fn test_validate_ip_invalid() {
        assert!(validate_ip("256.1.1.1").is_err());
        assert!(validate_ip("192.168.1").is_err());
        assert!(validate_ip("not-an-ip").is_err());
        assert!(validate_ip("").is_err());
    }

    #[test]
    fn test_validate_zone_id_valid() {
        assert!(validate_zone_id("a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6").is_ok());
        assert!(validate_zone_id("12345678901234567890123456789012").is_ok());
        assert!(validate_zone_id("abcdefghijklmnopqrstuvwxyz012345").is_ok());
    }

    #[test]
    fn test_validate_zone_id_invalid_length() {
        assert!(validate_zone_id("short").is_err());
        assert!(validate_zone_id("toolongzoneidthatexceedsthirtytwocharacters").is_err());
        assert!(validate_zone_id("").is_err());
    }

    #[test]
    fn test_validate_zone_id_invalid_chars() {
        assert!(validate_zone_id("a1b2c3d4-5f6g7h8i9j0k1l2m3n4o5p6").is_err()); // hyphen
        assert!(validate_zone_id("a1b2c3d4 5f6g7h8i9j0k1l2m3n4o5p6").is_err()); // space
        assert!(validate_zone_id("a1b2c3d4@5f6g7h8i9j0k1l2m3n4o5p6").is_err()); // special char
    }

    #[test]
    fn test_validate_record_id_valid() {
        assert!(validate_record_id("rec123456789abcdefghijklmnopqrst").is_ok()); // Exactly 32 chars
        assert!(validate_record_id("12345678901234567890123456789012").is_ok());
        // Exactly 32 chars
    }

    #[test]
    fn test_validate_record_id_invalid_length() {
        assert!(validate_record_id("short").is_err());
        assert!(validate_record_id("toolongrecordidthatexceedsthirtytwocharacters").is_err());
    }

    #[test]
    fn test_validate_record_id_invalid_chars() {
        assert!(validate_record_id("rec12345-7890abcdefghijklmnopqr").is_err());
        assert!(validate_record_id("rec12345 7890abcdefghijklmnopqr").is_err());
    }
}
