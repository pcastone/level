use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub deployment_mode: DeploymentMode,
    pub api_key: String,
    #[serde(default)]
    pub ssl: Option<SslSettings>,
    #[serde(default)]
    pub redis: Option<RedisSettings>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DeploymentMode {
    Skinny,
    Standard,
    Enterprise,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SslSettings {
    pub enabled: bool,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
    pub auto_generate: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisSettings {
    pub url: String,
}

impl Settings {
    pub fn load() -> anyhow::Result<Self> {
        let config_path =
            std::env::var("LEVEL_CONFIG").unwrap_or_else(|_| "config/settings.toml".to_string());

        let settings = if Path::new(&config_path).exists() {
            let content = std::fs::read_to_string(&config_path)?;
            toml::from_str(&content)?
        } else {
            // Default settings for development
            Self::default()
        };

        Ok(settings)
    }

    pub fn is_skinny(&self) -> bool {
        self.deployment_mode == DeploymentMode::Skinny
    }

    pub fn is_enterprise(&self) -> bool {
        self.deployment_mode == DeploymentMode::Enterprise
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerSettings {
                port: 8080,
                host: "0.0.0.0".to_string(),
            },
            database: DatabaseSettings {
                url: std::env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "postgres://level:level@localhost:5432/level".to_string()),
                max_connections: 10,
            },
            deployment_mode: DeploymentMode::Standard,
            api_key: std::env::var("API_KEY").unwrap_or_else(|_| "dev-api-key".to_string()),
            ssl: None,
            redis: None,
        }
    }
}
