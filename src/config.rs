use crate::i18n::Language;
use directories::ProjectDirs;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "jorjanion";
const APPLICATION: &str = "AirDropBridge";
pub const DEFAULT_PORT: u16 = 53317;

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub device_name: String,
    pub port: u16,
    pub token: String,
    pub download_dir: PathBuf,
    pub auto_update: bool,
    pub autostart: bool,
    #[serde(default)]
    pub shortcut_url: Option<String>,
    #[serde(default = "default_true")]
    pub notifications_enabled: bool,
    #[serde(default = "default_true")]
    pub play_sound: bool,
    #[serde(default = "default_false")]
    pub open_folder_on_receive: bool,
    #[serde(default = "default_true")]
    pub clipboard_sync: bool,
    #[serde(default)]
    pub language: Language,
    #[serde(default)]
    pub custom_host: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NetworkEndpoint {
    pub name: String,
    pub ip: String,
    pub is_tailscale: bool,
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
        if let Ok(hostname) = std::env::var("COMPUTERNAME")
            && !hostname.trim().is_empty()
        {
            return hostname;
        }
        if let Ok(hostname) = std::env::var("HOSTNAME")
            && !hostname.trim().is_empty()
        {
            return hostname;
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
                shortcut_url: None,
                notifications_enabled: true,
                play_sound: true,
                open_folder_on_receive: false,
                clipboard_sync: true,
                language: Language::En,
                custom_host: None,
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
            shortcut_url: None,
            notifications_enabled: true,
            play_sound: true,
            open_folder_on_receive: false,
            clipboard_sync: true,
            language: Language::En,
            custom_host: None,
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

    /// Discovers all available network IPv4 endpoints (LAN, Tailscale, VPNs)
    pub fn get_available_endpoints() -> Vec<NetworkEndpoint> {
        let mut endpoints = Vec::new();
        if let Ok(ifaces) = local_ip_address::list_afinet_netifas() {
            for (name, ip) in ifaces {
                if let std::net::IpAddr::V4(ipv4) = ip {
                    if ipv4.is_loopback() || ipv4.is_unspecified() {
                        continue;
                    }
                    let octets = ipv4.octets();
                    // Tailscale CGNAT range is 100.64.0.0/10 (100.64.0.0 - 100.127.255.255)
                    let is_tailscale = (octets[0] == 100 && (64..=127).contains(&octets[1]))
                        || name.to_lowercase().contains("tailscale");
                    endpoints.push(NetworkEndpoint {
                        name,
                        ip: ipv4.to_string(),
                        is_tailscale,
                    });
                }
            }
        }
        endpoints
    }

    /// Resolves the primary host or IP to advertise for pairing
    pub fn resolve_primary_host(&self) -> String {
        if let Some(ref custom) = self.custom_host {
            let trimmed = custom.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
        local_ip_address::local_ip()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|_| "127.0.0.1".to_string())
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
            shortcut_url: Some("https://www.icloud.com/shortcuts/87c14547b1de4195b903ce1d18495c2f".to_string()),
            notifications_enabled: true,
            play_sound: true,
            open_folder_on_receive: false,
            clipboard_sync: true,
            language: Language::Es,
            custom_host: Some("100.115.92.10".to_string()),
        };

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&serialized).unwrap();
        assert_eq!(config.device_name, deserialized.device_name);
        assert_eq!(config.port, deserialized.port);
        assert_eq!(config.token, deserialized.token);
        assert_eq!(config.shortcut_url, deserialized.shortcut_url);
        assert_eq!(config.notifications_enabled, deserialized.notifications_enabled);
        assert_eq!(config.play_sound, deserialized.play_sound);
        assert_eq!(config.open_folder_on_receive, deserialized.open_folder_on_receive);
        assert_eq!(config.clipboard_sync, deserialized.clipboard_sync);
        assert_eq!(config.language, deserialized.language);
        assert_eq!(config.custom_host, deserialized.custom_host);
    }

    #[test]
    fn test_resolve_primary_host_prefers_custom() {
        let mut config = AppConfig {
            device_name: "TestPC".to_string(),
            port: 53317,
            token: "token".to_string(),
            download_dir: PathBuf::from("./downloads"),
            auto_update: false,
            autostart: false,
            shortcut_url: None,
            notifications_enabled: true,
            play_sound: true,
            open_folder_on_receive: false,
            clipboard_sync: true,
            language: Language::En,
            custom_host: Some("my-pc.tailnet-xyz.ts.net".to_string()),
        };

        assert_eq!(config.resolve_primary_host(), "my-pc.tailnet-xyz.ts.net");

        config.custom_host = None;
        assert!(!config.resolve_primary_host().is_empty());
    }

    #[test]
    fn test_available_endpoints_detection() {
        let endpoints = AppConfig::get_available_endpoints();
        for ep in &endpoints {
            assert!(!ep.ip.is_empty());
            assert!(!ep.name.is_empty());
            if ep.is_tailscale {
                let parts: Vec<&str> = ep.ip.split('.').collect();
                assert_eq!(parts[0], "100");
            }
        }
    }
}
