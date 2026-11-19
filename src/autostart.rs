use std::path::PathBuf;
use tracing::info;

pub struct AutostartManager;

const APP_NAME: &str = "AirDropBridge";

impl AutostartManager {
    /// Gets the current running executable's canonical path
    pub fn current_exe_path() -> Result<PathBuf, String> {
        std::env::current_exe().map_err(|e| format!("Failed to get current executable path: {}", e))
    }

    #[cfg(windows)]
    pub fn is_enabled() -> bool {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use windows_sys::Win32::System::Registry::{
            HKEY_CURRENT_USER, KEY_READ, RegCloseKey, RegOpenKeyExW, RegQueryValueExW,
        };

        unsafe {
            let subkey: Vec<u16> = OsStr::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();
            let value_name: Vec<u16> = OsStr::new(APP_NAME)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();

            let mut hkey = std::ptr::null_mut();
            if RegOpenKeyExW(HKEY_CURRENT_USER, subkey.as_ptr(), 0, KEY_READ, &mut hkey) != 0 {
                return false;
            }

            let mut data_type = 0u32;
            let mut data_len = 0u32;
            let status = RegQueryValueExW(
                hkey,
                value_name.as_ptr(),
                std::ptr::null_mut(),
                &mut data_type,
                std::ptr::null_mut(),
                &mut data_len,
            );

            RegCloseKey(hkey);
            status == 0
        }
    }

    #[cfg(windows)]
    pub fn set_enabled(enabled: bool) -> Result<(), String> {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use windows_sys::Win32::System::Registry::{
            HKEY_CURRENT_USER, KEY_SET_VALUE, REG_SZ, RegCloseKey, RegDeleteValueW, RegOpenKeyExW,
            RegSetValueExW,
        };

        let subkey: Vec<u16> = OsStr::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let value_name: Vec<u16> = OsStr::new(APP_NAME)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        unsafe {
            let mut hkey = std::ptr::null_mut();
            if RegOpenKeyExW(
                HKEY_CURRENT_USER,
                subkey.as_ptr(),
                0,
                KEY_SET_VALUE,
                &mut hkey,
            ) != 0
            {
                return Err("Failed to open Windows registry Run key".to_string());
            }

            let result = if enabled {
                let exe_path = Self::current_exe_path()?;
                let exe_str = format!("\"{}\"", exe_path.to_string_lossy());
                let exe_wide: Vec<u16> = OsStr::new(&exe_str)
                    .encode_wide()
                    .chain(std::iter::once(0))
                    .collect();

                let status = RegSetValueExW(
                    hkey,
                    value_name.as_ptr(),
                    0,
                    REG_SZ,
                    exe_wide.as_ptr() as *const u8,
                    (exe_wide.len() * 2) as u32,
                );
                if status == 0 {
                    info!("Enabled autostart in Windows registry");
                    Ok(())
                } else {
                    Err(format!("RegSetValueExW failed with code {}", status))
                }
            } else {
                let status = RegDeleteValueW(hkey, value_name.as_ptr());
                if status == 0 || status == 2 {
                    // 2 = ERROR_FILE_NOT_FOUND (already deleted)
                    info!("Disabled autostart in Windows registry");
                    Ok(())
                } else {
                    Err(format!("RegDeleteValueW failed with code {}", status))
                }
            };

            RegCloseKey(hkey);
            result
        }
    }

    #[cfg(not(windows))]
    pub fn is_enabled() -> bool {
        if let Some(user_dirs) = directories::BaseDirs::new() {
            let autostart_file = user_dirs
                .config_dir()
                .join("autostart")
                .join("airdrop-bridge.desktop");
            return autostart_file.exists();
        }
        false
    }

    #[cfg(not(windows))]
    pub fn set_enabled(enabled: bool) -> Result<(), String> {
        let user_dirs = directories::BaseDirs::new()
            .ok_or_else(|| "Could not determine base config directory".to_string())?;
        let autostart_dir = user_dirs.config_dir().join("autostart");
        let desktop_file = autostart_dir.join("airdrop-bridge.desktop");

        if enabled {
            std::fs::create_dir_all(&autostart_dir)
                .map_err(|e| format!("Failed to create autostart directory: {}", e))?;
            let exe_path = Self::current_exe_path()?;
            let content = format!(
                "[Desktop Entry]\nType=Application\nName=AirDrop Bridge\nExec=\"{}\"\nTerminal=false\n",
                exe_path.to_string_lossy()
            );
            std::fs::write(&desktop_file, content)
                .map_err(|e| format!("Failed to write desktop entry: {}", e))?;
            info!("Enabled Linux desktop autostart");
        } else if desktop_file.exists() {
            let _ = std::fs::remove_file(desktop_file);
            info!("Disabled Linux desktop autostart");
        }
        Ok(())
    }
}
