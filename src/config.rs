use directories::ProjectDirs;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "jorjanion";
const APPLICATION: &str = "AirDropBridge";
pub const DEFAULT_PORT: u16 = 53317;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub device_name: String,
    pub port: u16,
    pub token: String,
    pub download_dir: PathBuf,
    pub auto_update: bool,
    pub autostart: bool,
}

impl AppConfig {
    /// Generates a cryptographically secure 256-bit random hex token
    pub fn generate_token() -> String {
        let mut bytes = [0u8; 32];
        let mut rng = rand::rng();
        rng.fill_bytes(&mut bytes);
        hex::encode(bytes)
    }

    /// Determines the default download directory: ~/Downloads/AirDropBridge
    pub fn default_download_dir() -> PathBuf {
        if let Some(user_dirs) = directories::UserDirs::new() {
            if let Some(downloads) = user_dirs.download_dir() {
                return downloads.join("AirDropBridge");
            }
            return user_dirs.home_dir().join("Downloads").join("AirDropBridge");
        }
        PathBuf::from("./downloads")
    }

    /// Resolves the default device hostname
    pub fn default_device_name() -> String {
        if let Ok(hostname) = std::env::var("COMPUTERNAME") {
            if !hostname.trim().is_empty() {
                return hostname;
            }
        }
        if let Ok(hostname) = std::env::var("HOSTNAME") {
            if !hostname.trim().is_empty() {
                return hostname;
            }
        }
        "AirDrop-Bridge-PC".to_string()
    }

    /// Path to the config file in %APPDATA%/AirDropBridge/config.json
    pub fn config_file_path() -> Option<PathBuf> {
        ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
            .map(|proj| proj.config_dir().join("config.json"))
    }

    /// Loads the configuration from disk, or generates and saves a new default configuration
    pub fn load_or_create() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(path) = Self::config_file_path() {
            if path.exists() {
                let content = fs::read_to_string(&path)?;
                if let Ok(config) = serde_json::from_str::<AppConfig>(&content) {
                    config.ensure_directories()?;
                    return Ok(config);
                }
            }

            // Create new configuration
            let config = AppConfig {
                device_name: Self::default_device_name(),
                port: DEFAULT_PORT,
                token: Self::generate_token(),
                download_dir: Self::default_download_dir(),
                auto_update: true,
                autostart: false,
            };

            config.save_to_path(&path)?;
            config.ensure_directories()?;
            return Ok(config);
        }

        // Fallback in-memory/local
        let fallback = AppConfig {
            device_name: Self::default_device_name(),
            port: DEFAULT_PORT,
            token: Self::generate_token(),
            download_dir: PathBuf::from("./downloads"),
            auto_update: true,
            autostart: false,
        };
        fallback.ensure_directories()?;
        Ok(fallback)
    }

    /// Saves the current configuration to disk
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(path) = Self::config_file_path() {
            self.save_to_path(&path)?;
        }
        Ok(())
    }

    fn save_to_path(&self, path: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Ensures download directory exists
    pub fn ensure_directories(&self) -> Result<(), std::io::Error> {
        if !self.download_dir.exists() {
            fs::create_dir_all(&self.download_dir)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_generation() {
        let token1 = AppConfig::generate_token();
        let token2 = AppConfig::generate_token();
        assert_eq!(token1.len(), 64);
        assert_eq!(token2.len(), 64);
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_config_serialization() {
        let config = AppConfig {
            device_name: "TestPC".to_string(),
            port: 53317,
            token: "abcdef123456".to_string(),
            download_dir: PathBuf::from("C:/test_downloads"),
            auto_update: true,
            autostart: false,
        };

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&serialized).unwrap();
        assert_eq!(config.device_name, deserialized.device_name);
        assert_eq!(config.port, deserialized.port);
        assert_eq!(config.token, deserialized.token);
    }
}
