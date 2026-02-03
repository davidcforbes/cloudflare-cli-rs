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
