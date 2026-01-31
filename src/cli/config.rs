//! CLI configuration handling

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// CLI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    /// API server URL
    #[serde(default = "default_api_url")]
    pub api_url: String,

    /// API key for authentication
    pub api_key: Option<String>,

    /// Default SOW ID or short_name
    pub default_sow: Option<String>,

    /// Output format preference
    #[serde(default = "default_format")]
    pub output_format: String,
}

fn default_api_url() -> String {
    "http://localhost:3000".to_string()
}

fn default_format() -> String {
    "table".to_string()
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            api_url: default_api_url(),
            api_key: None,
            default_sow: None,
            output_format: default_format(),
        }
    }
}

impl CliConfig {
    /// Get the config file path
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::home_dir()
            .context("Could not find home directory")?
            .join(".level");

        Ok(config_dir.join("config.toml"))
    }

    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

        let config: CliConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", config_path.display()))?;

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
        }

        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        std::fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

        Ok(())
    }

    /// Update default SOW
    pub fn set_default_sow(&mut self, sow: String) -> Result<()> {
        self.default_sow = Some(sow);
        self.save()
    }

    /// Get the effective SOW ID (from argument or config)
    pub fn get_sow(&self, arg_sow: Option<&str>) -> Result<String> {
        arg_sow
            .map(|s| s.to_string())
            .or_else(|| self.default_sow.clone())
            .ok_or_else(|| anyhow::anyhow!(
                "No SOW specified. Use --sow or set a default with 'level sow set-default <sow>'"
            ))
    }

    /// Parse config from TOML string (for testing)
    pub fn from_toml(content: &str) -> Result<Self> {
        toml::from_str(content).context("Failed to parse TOML")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CliConfig::default();
        assert_eq!(config.api_url, "http://localhost:3000");
        assert!(config.api_key.is_none());
        assert!(config.default_sow.is_none());
        assert_eq!(config.output_format, "table");
    }

    #[test]
    fn test_parse_toml_minimal() {
        let toml = r#"
            api_url = "http://example.com:8080"
        "#;
        let config = CliConfig::from_toml(toml).unwrap();
        assert_eq!(config.api_url, "http://example.com:8080");
        assert!(config.api_key.is_none());
    }

    #[test]
    fn test_parse_toml_full() {
        let toml = r#"
            api_url = "https://api.level.io"
            api_key = "secret-key-123"
            default_sow = "IT"
            output_format = "json"
        "#;
        let config = CliConfig::from_toml(toml).unwrap();
        assert_eq!(config.api_url, "https://api.level.io");
        assert_eq!(config.api_key, Some("secret-key-123".to_string()));
        assert_eq!(config.default_sow, Some("IT".to_string()));
        assert_eq!(config.output_format, "json");
    }

    #[test]
    fn test_get_sow_with_arg() {
        let config = CliConfig::default();
        let result = config.get_sow(Some("HR"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "HR");
    }

    #[test]
    fn test_get_sow_with_default() {
        let mut config = CliConfig::default();
        config.default_sow = Some("IT".to_string());
        let result = config.get_sow(None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "IT");
    }

    #[test]
    fn test_get_sow_arg_overrides_default() {
        let mut config = CliConfig::default();
        config.default_sow = Some("IT".to_string());
        let result = config.get_sow(Some("HR"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "HR");
    }

    #[test]
    fn test_get_sow_none_fails() {
        let config = CliConfig::default();
        let result = config.get_sow(None);
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_config() {
        let mut config = CliConfig::default();
        config.api_key = Some("test-key".to_string());
        config.default_sow = Some("PROJECT".to_string());

        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("api_url"));
        assert!(toml_str.contains("test-key"));
        assert!(toml_str.contains("PROJECT"));
    }
}
