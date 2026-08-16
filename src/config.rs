use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 8402,
            host: "127.0.0.1".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OmniRoutePluginConfig {
    pub enabled: bool,
    pub url: String,
}

impl Default for OmniRoutePluginConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            url: "http://localhost:3000/v1".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct IgniteConfig {
    pub server: ServerConfig,
    pub omniroute_plugin: OmniRoutePluginConfig,
}

impl IgniteConfig {
    pub fn load() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let config_path = home_dir.join(".igniterouter").join("config.yaml");

        if config_path.exists() {
            if let Ok(content) = fs::read_to_string(config_path) {
                if let Ok(cfg) = serde_yaml::from_str(&content) {
                    return cfg;
                }
            }
        }

        Self::default()
    }
}

mod dirs {
    use std::path::PathBuf;

    pub fn home_dir() -> Option<PathBuf> {
        std::env::var_os("USERPROFILE")
            .or_else(|| std::env::var_os("HOME"))
            .map(PathBuf::from)
    }
}
