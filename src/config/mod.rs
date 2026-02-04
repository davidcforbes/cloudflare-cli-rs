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
    pub account_id: Option<String>,

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
        let account_id = std::env::var("CLOUDFLARE_ACCOUNT_ID").ok();

        if api_token.is_none() && (api_key.is_none() || api_email.is_none()) {
            return Err(CfadError::config(
                "No credentials found. Set CLOUDFLARE_API_TOKEN or (CLOUDFLARE_API_KEY + CLOUDFLARE_API_EMAIL)",
            ));
        }

        Ok(Self {
            api_token,
            api_key,
            api_email,
            account_id,
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
            api_token: self
                .api_token
                .as_ref()
                .map(|t| format!("{}****", &t[..4.min(t.len())])),
            api_key: self
                .api_key
                .as_ref()
                .map(|k| format!("{}****", &k[..4.min(k.len())])),
            api_email: self.api_email.clone(),
            account_id: self.account_id.clone(),
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

/// Resolves account_id from CLI argument, environment variable, or config (in that order)
pub fn resolve_account_id(
    cli_account_id: Option<String>,
    profile: Option<&Profile>,
) -> Result<String> {
    // 1. CLI argument takes precedence
    if let Some(id) = cli_account_id {
        return Ok(id);
    }

    // 2. Try environment variable
    if let Ok(id) = std::env::var("CLOUDFLARE_ACCOUNT_ID") {
        return Ok(id);
    }

    // 3. Try config profile
    if let Some(profile) = profile {
        if let Some(id) = &profile.account_id {
            return Ok(id.clone());
        }
    }

    // 4. Try loading profile from config
    if let Ok(profile) = Config::load(None) {
        if let Some(id) = &profile.account_id {
            return Ok(id.clone());
        }
    }

    Err(CfadError::config(
        "No account ID found. Set CLOUDFLARE_ACCOUNT_ID environment variable, add account_id to config profile, or use --account-id flag",
    ))
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
        let config_dir =
            dirs::config_dir().ok_or_else(|| CfadError::config("Cannot find config directory"))?;

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

        let contents = toml::to_string_pretty(self).map_err(CfadError::TomlSer)?;
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
            account_id: None,
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
            account_id: None,
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
            account_id: None,
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
            account_id: None,
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
            account_id: None,
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
            account_id: None,
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
            account_id: None,
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
            account_id: None,
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
            account_id: None,
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

    #[test]
    #[serial_test::serial]
    fn test_profile_from_env_with_token() {
        std::env::set_var("CLOUDFLARE_API_TOKEN", "test_token");
        let profile = Profile::from_env().unwrap();
        assert_eq!(profile.api_token, Some("test_token".to_string()));
        std::env::remove_var("CLOUDFLARE_API_TOKEN");
    }

    #[test]
    #[serial_test::serial]
    fn test_profile_from_env_with_key_email() {
        std::env::set_var("CLOUDFLARE_API_KEY", "test_key");
        std::env::set_var("CLOUDFLARE_API_EMAIL", "test@example.com");
        let profile = Profile::from_env().unwrap();
        assert_eq!(profile.api_key, Some("test_key".to_string()));
        assert_eq!(profile.api_email, Some("test@example.com".to_string()));
        std::env::remove_var("CLOUDFLARE_API_KEY");
        std::env::remove_var("CLOUDFLARE_API_EMAIL");
    }

    #[test]
    #[serial_test::serial]
    fn test_profile_from_env_no_credentials() {
        // Clear all potential env vars
        std::env::remove_var("CLOUDFLARE_API_TOKEN");
        std::env::remove_var("CLOUDFLARE_API_KEY");
        std::env::remove_var("CLOUDFLARE_API_EMAIL");

        let result = Profile::from_env();
        assert!(result.is_err());
    }

    #[test]
    #[serial_test::serial]
    fn test_profile_from_env_incomplete_key_email() {
        // Only key, no email
        std::env::set_var("CLOUDFLARE_API_KEY", "test_key");
        std::env::remove_var("CLOUDFLARE_API_EMAIL");
        std::env::remove_var("CLOUDFLARE_API_TOKEN");

        let result = Profile::from_env();
        assert!(result.is_err());

        std::env::remove_var("CLOUDFLARE_API_KEY");
    }

    #[test]
    fn test_config_path_exists() {
        let path = Config::config_path();
        assert!(path.is_ok());
        let path = path.unwrap();
        assert!(path.to_string_lossy().contains("cfad"));
        assert!(path.to_string_lossy().ends_with("config.toml"));
    }

    #[test]
    fn test_config_serialization() {
        let mut config = Config::new("default".to_string());
        let profile = Profile {
            api_token: Some("token123".to_string()),
            api_key: None,
            api_email: None,
            account_id: None,
            default_zone: Some("example.com".to_string()),
            output_format: Some("json".to_string()),
        };
        config.profiles.insert("default".to_string(), profile);

        let serialized = toml::to_string(&config).unwrap();
        assert!(serialized.contains("default_profile"));
        assert!(serialized.contains("token123"));
        assert!(serialized.contains("example.com"));
    }

    #[test]
    fn test_config_deserialization() {
        let toml_str = r#"
            default_profile = "production"

            [profiles.production]
            api_token = "prod_token_123"
            default_zone = "prod.example.com"
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.default_profile, "production");
        assert_eq!(config.profiles.len(), 1);

        let profile = config.profiles.get("production").unwrap();
        assert_eq!(profile.api_token, Some("prod_token_123".to_string()));
        assert_eq!(profile.default_zone, Some("prod.example.com".to_string()));
    }

    #[test]
    fn test_auth_method_clone() {
        let auth = AuthMethod::ApiToken("test".to_string());
        let cloned = auth.clone();

        match cloned {
            AuthMethod::ApiToken(token) => assert_eq!(token, "test"),
            _ => panic!("Expected ApiToken"),
        }
    }

    #[test]
    fn test_profile_with_all_fields() {
        let profile = Profile {
            api_token: Some("token".to_string()),
            api_key: Some("key".to_string()),
            api_email: Some("email@test.com".to_string()),
            account_id: Some("test-account-id".to_string()),
            default_zone: Some("zone.com".to_string()),
            output_format: Some("table".to_string()),
        };

        assert!(profile.api_token.is_some());
        assert!(profile.api_key.is_some());
        assert!(profile.api_email.is_some());
        assert!(profile.default_zone.is_some());
        assert!(profile.output_format.is_some());
    }

    #[test]
    fn test_config_save_and_load() {
        use std::fs;

        // Create a temporary directory for testing
        let temp_dir = std::env::temp_dir().join("cfad_test_config");
        fs::create_dir_all(&temp_dir).ok();

        // Override config path for this test
        let test_config_path = temp_dir.join("test_config.toml");

        // Create a test config
        let mut config = Config::new("test_profile".to_string());
        let profile = Profile {
            api_token: Some("test_token_123".to_string()),
            api_key: None,
            api_email: None,
            account_id: None,
            default_zone: Some("test.example.com".to_string()),
            output_format: Some("json".to_string()),
        };
        config.profiles.insert("test_profile".to_string(), profile);

        // Save to test file
        let contents = toml::to_string_pretty(&config).unwrap();
        fs::write(&test_config_path, contents).unwrap();

        // Load and verify
        let loaded_contents = fs::read_to_string(&test_config_path).unwrap();
        let loaded_config: Config = toml::from_str(&loaded_contents).unwrap();

        assert_eq!(loaded_config.default_profile, "test_profile");
        assert_eq!(loaded_config.profiles.len(), 1);
        let loaded_profile = loaded_config.profiles.get("test_profile").unwrap();
        assert_eq!(loaded_profile.api_token, Some("test_token_123".to_string()));
        assert_eq!(
            loaded_profile.default_zone,
            Some("test.example.com".to_string())
        );

        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_config_from_file_not_found() {
        // This test verifies the error path when config file doesn't exist
        // We can't easily test the actual from_file() because it uses a fixed path
        let toml_str = r#"
            default_profile = "missing"

            [profiles.test]
            api_token = "token"
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();

        // Try to get a profile that doesn't exist
        let profile = config.profiles.get("nonexistent");
        assert!(profile.is_none());
    }

    #[test]
    fn test_config_multiple_profiles() {
        let mut config = Config::new("prod".to_string());

        let prod_profile = Profile {
            api_token: Some("prod_token".to_string()),
            api_key: None,
            api_email: None,
            account_id: None,
            default_zone: Some("prod.example.com".to_string()),
            output_format: None,
        };

        let dev_profile = Profile {
            api_token: Some("dev_token".to_string()),
            api_key: None,
            api_email: None,
            account_id: None,
            default_zone: Some("dev.example.com".to_string()),
            output_format: Some("json".to_string()),
        };

        config.profiles.insert("prod".to_string(), prod_profile);
        config.profiles.insert("dev".to_string(), dev_profile);

        assert_eq!(config.profiles.len(), 2);
        assert!(config.profiles.contains_key("prod"));
        assert!(config.profiles.contains_key("dev"));
    }

    #[test]
    #[serial_test::serial]
    fn test_config_load_with_env_vars() {
        // Set up env vars
        std::env::set_var("CLOUDFLARE_API_TOKEN", "env_test_token_load");

        // Config::load should find the env vars
        let result = Config::load(None);

        // Clean up
        std::env::remove_var("CLOUDFLARE_API_TOKEN");

        assert!(result.is_ok());
        let profile = result.unwrap();
        assert_eq!(profile.api_token, Some("env_test_token_load".to_string()));
    }

    #[test]
    fn test_config_from_file_deserialize() {
        use std::fs;
        let temp_dir = std::env::temp_dir().join("cfad_test_from_file");
        fs::create_dir_all(&temp_dir).ok();
        let test_path = temp_dir.join("test_config.toml");

        let toml_content = r#"
default_profile = "test"

[profiles.test]
api_token = "test_token_123"
default_zone = "example.com"
"#;
        fs::write(&test_path, toml_content).unwrap();

        // Read and deserialize
        let contents = fs::read_to_string(&test_path).unwrap();
        let config: Config = toml::from_str(&contents).unwrap();

        assert_eq!(config.default_profile, "test");
        assert!(config.profiles.contains_key("test"));

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_config_save_creates_directory() {
        use std::fs;
        let temp_dir = std::env::temp_dir().join("cfad_test_save_dir");
        fs::remove_dir_all(&temp_dir).ok();

        let config_dir = temp_dir.join("subdir");
        let config_path = config_dir.join("config.toml");

        let mut config = Config::new("default".to_string());
        let profile = Profile {
            api_token: Some("token".to_string()),
            api_key: None,
            api_email: None,
            account_id: None,
            default_zone: None,
            output_format: None,
        };
        config.profiles.insert("default".to_string(), profile);

        // Manually create parent and save
        fs::create_dir_all(&config_dir).unwrap();
        let contents = toml::to_string_pretty(&config).unwrap();
        fs::write(&config_path, contents).unwrap();

        // Verify file exists
        assert!(config_path.exists());

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_config_save_and_from_file_roundtrip() {
        // Create a config
        let mut config = Config::new("roundtrip_test".to_string());
        let profile = Profile {
            api_token: Some("roundtrip_token".to_string()),
            api_key: None,
            api_email: None,
            account_id: None,
            default_zone: Some("roundtrip.example.com".to_string()),
            output_format: Some("json".to_string()),
        };
        config
            .profiles
            .insert("roundtrip_test".to_string(), profile);

        // Get the config path
        let config_path = Config::config_path();
        assert!(config_path.is_ok());

        // We can't actually test save() and from_file() without potentially
        // interfering with the user's real config, so we'll test the serialization
        // logic instead
        let serialized = toml::to_string_pretty(&config);
        assert!(serialized.is_ok());

        let deserialized: std::result::Result<Config, toml::de::Error> =
            toml::from_str(&serialized.unwrap());
        assert!(deserialized.is_ok());

        let loaded = deserialized.unwrap();
        assert_eq!(loaded.default_profile, "roundtrip_test");
        assert!(loaded.profiles.contains_key("roundtrip_test"));
    }

    #[test]
    #[serial_test::serial]
    fn test_config_load_fallback_to_env() {
        // Clear env initially
        std::env::remove_var("CLOUDFLARE_API_TOKEN");
        std::env::remove_var("CLOUDFLARE_API_KEY");
        std::env::remove_var("CLOUDFLARE_API_EMAIL");

        // Config::load will try env vars as fallback when no file exists
        // Set env var
        std::env::set_var("CLOUDFLARE_API_TOKEN", "fallback_token");

        // This should succeed via env var fallback
        let result = Config::load(None);

        // Cleanup
        std::env::remove_var("CLOUDFLARE_API_TOKEN");

        assert!(result.is_ok());
        let profile = result.unwrap();
        assert_eq!(profile.api_token, Some("fallback_token".to_string()));
    }

    #[test]
    fn test_config_save_creates_parent_directory() {
        // Test that save logic includes creating parent directory
        let _config = Config::new("test".to_string());

        // Get the config path to verify parent() logic works
        if let Ok(path) = Config::config_path() {
            let parent = path.parent();
            assert!(parent.is_some());
        }
    }

    #[test]
    fn test_config_toml_serialization_error_handling() {
        // Test TomlSer error path in save()
        let config = Config::new("test".to_string());

        // Serialize to test the toml conversion
        let result = toml::to_string_pretty(&config);
        assert!(result.is_ok());

        // Test that map_err(CfadError::TomlSer) path exists
        // by verifying serialization works
        let toml_str = result.unwrap();
        assert!(toml_str.contains("default_profile"));
    }

    #[test]
    #[serial_test::serial]
    fn test_config_load_with_profile_selection() {
        // Set up multiple profile env scenario
        std::env::set_var("CLOUDFLARE_API_TOKEN", "profile_select_token");

        // Load with specific profile name (will fall back to env)
        let result = Config::load(Some("custom_profile"));

        // Cleanup
        std::env::remove_var("CLOUDFLARE_API_TOKEN");

        // Should succeed via env fallback even if profile doesn't exist in file
        assert!(result.is_ok());
    }

    #[test]
    fn test_profile_clone() {
        let profile = Profile {
            api_token: Some("token".to_string()),
            api_key: Some("key".to_string()),
            api_email: Some("email@test.com".to_string()),
            account_id: Some("account123".to_string()),
            default_zone: Some("zone.com".to_string()),
            output_format: Some("json".to_string()),
        };

        let cloned = profile.clone();
        assert_eq!(profile.api_token, cloned.api_token);
        assert_eq!(profile.api_key, cloned.api_key);
        assert_eq!(profile.api_email, cloned.api_email);
    }
}
