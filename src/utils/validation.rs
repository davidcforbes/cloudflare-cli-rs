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
