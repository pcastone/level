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
}
