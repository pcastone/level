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
    #[serde(default)]
    pub logging: LoggingSettings,
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
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout_secs: u64,
}

fn default_connect_timeout() -> u64 {
    30
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DeploymentMode {
    Skinny,
    Standard,
    Enterprise,
}

impl std::fmt::Display for DeploymentMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeploymentMode::Skinny => write!(f, "skinny"),
            DeploymentMode::Standard => write!(f, "standard"),
            DeploymentMode::Enterprise => write!(f, "enterprise"),
        }
    }
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

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingSettings {
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default)]
    pub json_output: bool,
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Default for LoggingSettings {
    fn default() -> Self {
        Self {
            level: std::env::var("RUST_LOG").unwrap_or_else(|_| default_log_level()),
            json_output: std::env::var("LOG_JSON").map(|v| v == "true" || v == "1").unwrap_or(false),
        }
    }
}

impl Settings {
    /// Load settings with environment variable overrides
    ///
    /// Environment variables:
    /// - LEVEL_CONFIG: Path to config file (default: config/settings.toml)
    /// - DATABASE_URL: Database connection string
    /// - DATABASE_MAX_CONNECTIONS: Max pool connections
    /// - LEVEL_PORT: Server port
    /// - LEVEL_HOST: Server host
    /// - API_KEY: API authentication key
    /// - DEPLOYMENT_MODE: skinny, standard, or enterprise
    /// - REDIS_URL: Redis connection string (optional)
    /// - RUST_LOG: Log level filter
    /// - LOG_JSON: Set to "true" for JSON log output
    pub fn load() -> anyhow::Result<Self> {
        let config_path =
            std::env::var("LEVEL_CONFIG").unwrap_or_else(|_| "config/settings.toml".to_string());

        let mut settings = if Path::new(&config_path).exists() {
            let content = std::fs::read_to_string(&config_path)?;
            toml::from_str(&content)?
        } else {
            Self::default()
        };

        // Apply environment variable overrides
        settings.apply_env_overrides();

        Ok(settings)
    }

    /// Apply environment variable overrides to settings
    fn apply_env_overrides(&mut self) {
        if let Ok(url) = std::env::var("DATABASE_URL") {
            self.database.url = url;
        }
        if let Ok(max_conn) = std::env::var("DATABASE_MAX_CONNECTIONS") {
            if let Ok(n) = max_conn.parse() {
                self.database.max_connections = n;
            }
        }
        if let Ok(port) = std::env::var("LEVEL_PORT") {
            if let Ok(p) = port.parse() {
                self.server.port = p;
            }
        }
        if let Ok(host) = std::env::var("LEVEL_HOST") {
            self.server.host = host;
        }
        if let Ok(key) = std::env::var("API_KEY") {
            self.api_key = key;
        }
        if let Ok(mode) = std::env::var("DEPLOYMENT_MODE") {
            self.deployment_mode = match mode.to_lowercase().as_str() {
                "skinny" => DeploymentMode::Skinny,
                "enterprise" => DeploymentMode::Enterprise,
                _ => DeploymentMode::Standard,
            };
        }
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            self.redis = Some(RedisSettings { url: redis_url });
        }
        if let Ok(level) = std::env::var("RUST_LOG") {
            self.logging.level = level;
        }
        if let Ok(json) = std::env::var("LOG_JSON") {
            self.logging.json_output = json == "true" || json == "1";
        }
    }

    pub fn is_skinny(&self) -> bool {
        self.deployment_mode == DeploymentMode::Skinny
    }

    pub fn is_enterprise(&self) -> bool {
        self.deployment_mode == DeploymentMode::Enterprise
    }

    /// Get the log filter directive for tracing
    pub fn log_filter(&self) -> String {
        format!("level={},tower_http=debug", self.logging.level)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerSettings {
                port: std::env::var("LEVEL_PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(8080),
                host: std::env::var("LEVEL_HOST")
                    .unwrap_or_else(|_| "0.0.0.0".to_string()),
            },
            database: DatabaseSettings {
                url: std::env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "postgres://level:level@localhost:5432/level".to_string()),
                max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                    .ok()
                    .and_then(|m| m.parse().ok())
                    .unwrap_or(10),
                connect_timeout_secs: 30,
            },
            deployment_mode: std::env::var("DEPLOYMENT_MODE")
                .map(|m| match m.to_lowercase().as_str() {
                    "skinny" => DeploymentMode::Skinny,
                    "enterprise" => DeploymentMode::Enterprise,
                    _ => DeploymentMode::Standard,
                })
                .unwrap_or(DeploymentMode::Standard),
            api_key: std::env::var("API_KEY").unwrap_or_else(|_| "dev-api-key".to_string()),
            ssl: None,
            redis: std::env::var("REDIS_URL").ok().map(|url| RedisSettings { url }),
            logging: LoggingSettings::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();
        assert_eq!(settings.server.port, 8080);
        assert_eq!(settings.server.host, "0.0.0.0");
        assert_eq!(settings.database.max_connections, 10);
        assert_eq!(settings.deployment_mode, DeploymentMode::Standard);
    }

    #[test]
    fn test_deployment_mode_display() {
        assert_eq!(DeploymentMode::Skinny.to_string(), "skinny");
        assert_eq!(DeploymentMode::Standard.to_string(), "standard");
        assert_eq!(DeploymentMode::Enterprise.to_string(), "enterprise");
    }

    #[test]
    fn test_is_skinny() {
        let mut settings = Settings::default();
        assert!(!settings.is_skinny());

        settings.deployment_mode = DeploymentMode::Skinny;
        assert!(settings.is_skinny());
    }

    #[test]
    fn test_is_enterprise() {
        let mut settings = Settings::default();
        assert!(!settings.is_enterprise());

        settings.deployment_mode = DeploymentMode::Enterprise;
        assert!(settings.is_enterprise());
    }

    #[test]
    fn test_log_filter() {
        let settings = Settings::default();
        assert!(settings.log_filter().contains("level="));
    }
}
