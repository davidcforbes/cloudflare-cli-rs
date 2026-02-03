use crate::error::{CfadError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub mod validation;

pub use validation::validate_config;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub default_profile: String,
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Profile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_zone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
}

impl Profile {
    pub fn from_env() -> Result<Self> {
        let api_token = std::env::var("CLOUDFLARE_API_TOKEN").ok();
        let api_key = std::env::var("CLOUDFLARE_API_KEY").ok();
        let api_email = std::env::var("CLOUDFLARE_API_EMAIL").ok();

        if api_token.is_none() && (api_key.is_none() || api_email.is_none()) {
            return Err(CfadError::config(
                "No credentials found. Set CLOUDFLARE_API_TOKEN or (CLOUDFLARE_API_KEY + CLOUDFLARE_API_EMAIL)",
            ));
        }

        Ok(Self {
            api_token,
            api_key,
            api_email,
            default_zone: None,
            output_format: None,
        })
    }

    pub fn auth_method(&self) -> Result<AuthMethod> {
        if let Some(token) = &self.api_token {
            Ok(AuthMethod::ApiToken(token.clone()))
        } else if let (Some(key), Some(email)) = (&self.api_key, &self.api_email) {
            Ok(AuthMethod::ApiKeyEmail {
                key: key.clone(),
                email: email.clone(),
            })
        } else {
            Err(CfadError::config(
                "No valid authentication method configured",
            ))
        }
    }

    pub fn redacted(&self) -> Self {
        Self {
            api_token: self.api_token.as_ref().map(|t| {
                format!("{}****", &t[..4.min(t.len())])
            }),
            api_key: self.api_key.as_ref().map(|k| {
                format!("{}****", &k[..4.min(k.len())])
            }),
            api_email: self.api_email.clone(),
            default_zone: self.default_zone.clone(),
            output_format: self.output_format.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum AuthMethod {
    ApiToken(String),
    ApiKeyEmail { key: String, email: String },
}

impl Config {
    pub fn load(profile_name: Option<&str>) -> Result<Profile> {
        // 1. Try explicit CLI flags first (handled at CLI level)
        // 2. Try environment variables
        if let Ok(profile) = Profile::from_env() {
            return Ok(profile);
        }

        // 3. Try config file
        if let Ok(config_file) = Self::from_file() {
            let profile_key = profile_name.unwrap_or(&config_file.default_profile);

            if let Some(profile) = config_file.profiles.get(profile_key) {
                return Ok(profile.clone());
            }
        }

        // 4. Last fallback: try env again with more detailed error
        Profile::from_env()
    }

    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| CfadError::config("Cannot find config directory"))?;

        Ok(config_dir.join("cfad").join("config.toml"))
    }

    pub fn from_file() -> Result<Self> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Err(CfadError::config("Config file not found"));
        }
        let contents = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&contents)?)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self)
            .map_err(CfadError::TomlSer)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    pub fn new(default_profile: String) -> Self {
        Self {
            default_profile,
            profiles: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_auth_method_with_token() {
        let profile = Profile {
            api_token: Some("test_token_12345".to_string()),
            api_key: None,
            api_email: None,
            default_zone: None,
            output_format: None,
        };

        let auth = profile.auth_method().unwrap();
        match auth {
            AuthMethod::ApiToken(token) => {
                assert_eq!(token, "test_token_12345");
            }
            _ => panic!("Expected ApiToken variant"),
        }
    }

    #[test]
    fn test_profile_auth_method_with_key_email() {
        let profile = Profile {
            api_token: None,
            api_key: Some("test_key_12345".to_string()),
            api_email: Some("user@example.com".to_string()),
            default_zone: None,
            output_format: None,
        };

        let auth = profile.auth_method().unwrap();
        match auth {
            AuthMethod::ApiKeyEmail { key, email } => {
                assert_eq!(key, "test_key_12345");
                assert_eq!(email, "user@example.com");
            }
            _ => panic!("Expected ApiKeyEmail variant"),
        }
    }

    #[test]
    fn test_profile_auth_method_token_takes_precedence() {
        // When both token and key+email are present, token should be used
        let profile = Profile {
            api_token: Some("test_token".to_string()),
            api_key: Some("test_key".to_string()),
            api_email: Some("user@example.com".to_string()),
            default_zone: None,
            output_format: None,
        };

        let auth = profile.auth_method().unwrap();
        match auth {
            AuthMethod::ApiToken(_) => {}
            _ => panic!("Expected ApiToken to take precedence"),
        }
    }

    #[test]
    fn test_profile_auth_method_no_credentials() {
        let profile = Profile {
            api_token: None,
            api_key: None,
            api_email: None,
            default_zone: None,
            output_format: None,
        };

        let result = profile.auth_method();
        assert!(result.is_err());
    }

    #[test]
    fn test_profile_auth_method_missing_email() {
        let profile = Profile {
            api_token: None,
            api_key: Some("test_key".to_string()),
            api_email: None,
            default_zone: None,
            output_format: None,
        };

        let result = profile.auth_method();
        assert!(result.is_err());
    }

    #[test]
    fn test_profile_auth_method_missing_key() {
        let profile = Profile {
            api_token: None,
            api_key: None,
            api_email: Some("user@example.com".to_string()),
            default_zone: None,
            output_format: None,
        };

        let result = profile.auth_method();
        assert!(result.is_err());
    }

    #[test]
    fn test_profile_redacted_token() {
        let profile = Profile {
            api_token: Some("secret_token_12345".to_string()),
            api_key: None,
            api_email: None,
            default_zone: Some("example.com".to_string()),
            output_format: None,
        };

        let redacted = profile.redacted();
        assert_eq!(redacted.api_token, Some("secr****".to_string()));
        assert_eq!(redacted.default_zone, Some("example.com".to_string()));
    }

    #[test]
    fn test_profile_redacted_key() {
        let profile = Profile {
            api_token: None,
            api_key: Some("secret_key_12345".to_string()),
            api_email: Some("user@example.com".to_string()),
            default_zone: None,
            output_format: None,
        };

        let redacted = profile.redacted();
        assert_eq!(redacted.api_key, Some("secr****".to_string()));
        assert_eq!(redacted.api_email, Some("user@example.com".to_string()));
    }

    #[test]
    fn test_profile_redacted_short_token() {
        // Token shorter than 4 chars
        let profile = Profile {
            api_token: Some("abc".to_string()),
            api_key: None,
            api_email: None,
            default_zone: None,
            output_format: None,
        };

        let redacted = profile.redacted();
        assert_eq!(redacted.api_token, Some("abc****".to_string()));
    }

    #[test]
    fn test_config_new() {
        let config = Config::new("default".to_string());
        assert_eq!(config.default_profile, "default");
        assert_eq!(config.profiles.len(), 0);
    }
}
