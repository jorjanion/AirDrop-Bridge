use crate::config::AppConfig;
use crate::i18n::Language;
use crate::notification::NotificationManager;
use semver::Version;
use serde::Deserialize;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

const REPO_OWNER: &str = "jorjanion";
const REPO_NAME: &str = "airdrop-bridge";
const USER_AGENT: &str = "AirDropBridge-Updater/0.1.0";

#[derive(Debug, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: Option<String>,
    pub html_url: String,
    pub body: Option<String>,
    pub assets: Vec<GitHubAsset>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

pub struct AutoUpdater;

impl AutoUpdater {
    /// Compares two version strings (handles optional 'v' prefix)
    pub fn is_newer(current: &str, remote: &str) -> bool {
        let clean_cur = current.trim_start_matches('v');
        let clean_rem = remote.trim_start_matches('v');

        match (Version::parse(clean_cur), Version::parse(clean_rem)) {
            (Ok(cur), Ok(rem)) => rem > cur,
            _ => false,
        }
    }

    /// Cleans up any leftover .old.exe or .part files from previous updates
    pub fn cleanup_old_binaries() {
        if let Ok(exe_path) = std::env::current_exe()
            && let Some(parent) = exe_path.parent()
        {
            let old_bin = parent.join(format!(
                "{}.old.exe",
                exe_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("airdrop-bridge")
            ));
            if old_bin.exists() {
                let _ = std::fs::remove_file(old_bin);
                info!("Cleaned up old binary file from previous update");
            }
        }
    }

    /// Queries the latest GitHub release via GitHub API, with fallback to gh CLI
    pub async fn fetch_latest_release() -> Result<GitHubRelease, String> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            REPO_OWNER, REPO_NAME
        );

        let client_res = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(std::time::Duration::from_secs(10))
            .build();

        if let Ok(client) = client_res {
            if let Ok(resp) = client.get(&url).send().await {
                if resp.status().is_success() {
                    if let Ok(release) = resp.json::<GitHubRelease>().await {
                        return Ok(release);
                    }
                }
            }
        }

        // Fallback: query via gh CLI if installed
        Self::fetch_latest_release_via_gh_cli().await
    }

    /// Queries the latest GitHub release using the local GitHub CLI (gh)
    pub async fn fetch_latest_release_via_gh_cli() -> Result<GitHubRelease, String> {
        let repo_arg = format!("{}/{}", REPO_OWNER, REPO_NAME);
        let output = tokio::task::spawn_blocking(move || {
            #[cfg(windows)]
            use std::os::windows::process::CommandExt;
            #[cfg(windows)]
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            let mut cmd = std::process::Command::new("gh");
            #[cfg(windows)]
            cmd.creation_flags(CREATE_NO_WINDOW);
            cmd.args(["release", "view", "--repo", &repo_arg, "--json", "tagName,name,assets,url"])
                .output()
        })
        .await
        .map_err(|e| format!("gh task join error: {}", e))?
        .map_err(|e| format!("Failed to execute gh CLI: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("gh CLI returned error: {}", stderr.trim()));
        }

        #[derive(Deserialize)]
        struct GhCliRelease {
            #[serde(rename = "tagName")]
            tag_name: String,
            name: Option<String>,
            url: Option<String>,
            assets: Vec<GhCliAsset>,
        }

        #[derive(Deserialize)]
        struct GhCliAsset {
            name: String,
            url: String,
            size: Option<u64>,
        }

        let cli_rel: GhCliRelease = serde_json::from_slice(&output.stdout)
            .map_err(|e| format!("Failed to parse gh CLI JSON: {}", e))?;

        Ok(GitHubRelease {
            tag_name: cli_rel.tag_name,
            name: cli_rel.name,
            html_url: cli_rel.url.unwrap_or_default(),
            body: None,
            assets: cli_rel
                .assets
                .into_iter()
                .map(|a| GitHubAsset {
                    name: a.name,
                    browser_download_url: a.url,
                    size: a.size.unwrap_or(0),
                })
                .collect(),
        })
    }

    /// Checks for updates and performs hot-swap replacement if a newer version exists
    pub async fn check_and_update(notify_if_latest: bool, lang: Language) {
        let strings = crate::i18n::t(lang);
        info!("Checking for updates on GitHub...");
        let current_version = env!("CARGO_PKG_VERSION");

        let release = match Self::fetch_latest_release().await {
            Ok(r) => r,
            Err(e) => {
                warn!("Update check failed: {}", e);
                if notify_if_latest {
                    NotificationManager::notify_message(
                        strings.ui_title,
                        strings.update_offline,
                    );
                }
                return;
            }
        };

        if !Self::is_newer(current_version, &release.tag_name) {
            info!("AirDrop Bridge is up to date (version {})", current_version);
            if notify_if_latest {
                NotificationManager::notify_message(
                    strings.ui_title,
                    &format!("{} ({})", strings.update_latest_prefix, current_version),
                );
            }
            return;
        }

        info!(
            "Newer version found: {} (current: {})",
            release.tag_name, current_version
        );

        // Find matching binary asset for current OS and architecture
        #[cfg(all(windows, target_arch = "x86_64"))]
        let target_asset = release
            .assets
            .iter()
            .find(|a| {
                a.name.ends_with(".exe") && (a.name.contains("x64") || a.name.contains("x86_64"))
            })
            .or_else(|| release.assets.iter().find(|a| a.name.ends_with(".exe")));

        #[cfg(all(windows, target_arch = "aarch64"))]
        let target_asset = release
            .assets
            .iter()
            .find(|a| {
                a.name.ends_with(".exe") && (a.name.contains("arm64") || a.name.contains("aarch64"))
            })
            .or_else(|| release.assets.iter().find(|a| a.name.ends_with(".exe")));

        #[cfg(not(windows))]
        let target_asset = release
            .assets
            .iter()
            .find(|a| a.name.contains("linux") || !a.name.contains("windows"));

        if let Some(asset) = target_asset {
            info!("Found downloadable asset: {}", asset.name);
            match Self::download_and_apply(&asset.browser_download_url).await {
                Ok(_) => {
                    info!(
                        "AirDrop Bridge was silently updated to {} via GitHub",
                        release.tag_name
                    );
                    let msg = strings
                        .update_silent_done_fmt
                        .replace("{}", &release.tag_name);
                    NotificationManager::notify_message(strings.ui_title, &msg);
                }
                Err(e) => {
                    error!("Failed to apply update: {}", e);
                }
            }
        }
    }

    /// Downloads and replaces the executable in-place
    async fn download_and_apply(download_url: &str) -> Result<(), String> {
        let current_exe = std::env::current_exe()
            .map_err(|e| format!("Failed to get current executable path: {}", e))?;
        let parent_dir = current_exe
            .parent()
            .ok_or_else(|| "Failed to get exe directory".to_string())?;

        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .map_err(|e| e.to_string())?;

        let resp = client
            .get(download_url)
            .send()
            .await
            .map_err(|e| format!("Failed to download update payload: {}", e))?;

        let bytes = resp
            .bytes()
            .await
            .map_err(|e| format!("Failed to read update stream: {}", e))?;

        let new_exe_temp = parent_dir.join("airdrop-bridge.new.exe");
        std::fs::write(&new_exe_temp, &bytes)
            .map_err(|e| format!("Failed to write downloaded update file: {}", e))?;

        // Perform safe in-place replacement
        Self::swap_executables(&current_exe, &new_exe_temp)
    }

    /// Swaps current running executable with the new one
    fn swap_executables(current: &Path, new: &Path) -> Result<(), String> {
        #[cfg(windows)]
        {
            let old_exe = current.with_extension("old.exe");
            if old_exe.exists() {
                let _ = std::fs::remove_file(&old_exe);
            }

            // In Windows, a running exe can be renamed
            std::fs::rename(current, &old_exe)
                .map_err(|e| format!("Failed to rename running exe to .old.exe: {}", e))?;

            // Rename new exe to the active exe name
            if let Err(e) = std::fs::rename(new, current) {
                // Rollback on failure
                let _ = std::fs::rename(&old_exe, current);
                return Err(format!("Failed to install new exe, rolled back: {}", e));
            }

            info!("Executable hot-swap completed successfully!");
            Ok(())
        }

        #[cfg(not(windows))]
        {
            std::fs::rename(new, current)
                .map_err(|e| format!("Failed to replace binary: {}", e))?;
            info!("Binary replacement completed successfully!");
            Ok(())
        }
    }

    /// Spawns periodic silent background update check loop
    pub fn spawn_background_checker(config: Arc<RwLock<AppConfig>>) {
        tokio::spawn(async move {
            // Initial silent check 5 seconds after startup
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            {
                let conf = config.read().await;
                if conf.auto_update {
                    let lang = conf.language;
                    drop(conf);
                    info!("Running initial silent update check...");
                    Self::check_and_update(false, lang).await;
                }
            }

            // Periodic silent check every 1 hour
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(3600));
            loop {
                interval.tick().await;
                let conf = config.read().await;
                if conf.auto_update {
                    let lang = conf.language;
                    drop(conf);
                    info!("Running periodic silent background update check...");
                    Self::check_and_update(false, lang).await;
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        assert!(AutoUpdater::is_newer("0.1.0", "0.2.0"));
        assert!(AutoUpdater::is_newer("v0.1.0", "v0.1.1"));
        assert!(AutoUpdater::is_newer("0.1.0", "v1.0.0"));
        assert!(!AutoUpdater::is_newer("0.2.0", "0.1.0"));
        assert!(!AutoUpdater::is_newer("1.0.0", "1.0.0"));
        assert!(!AutoUpdater::is_newer("v1.0.0", "1.0.0"));
    }
}
