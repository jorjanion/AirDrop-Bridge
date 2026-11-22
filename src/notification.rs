use std::path::Path;
#[cfg(windows)]
use std::path::PathBuf;
use tracing::info;

pub struct NotificationManager;

impl NotificationManager {
    /// Plays a subtle native OS notification chime
    pub fn play_chime() {
        #[cfg(windows)]
        {
            use std::ffi::OsStr;
            use std::os::windows::ffi::OsStrExt;
            use windows_sys::Win32::Media::Audio::{PlaySoundW, SND_ALIAS, SND_ASYNC};

            unsafe {
                let sound_alias: Vec<u16> = OsStr::new("Notification.Default")
                    .encode_wide()
                    .chain(std::iter::once(0))
                    .collect();

                PlaySoundW(
                    sound_alias.as_ptr(),
                    std::ptr::null_mut(),
                    SND_ALIAS | SND_ASYNC,
                );
            }
        }
    }

    /// Opens a file in the default application
    pub fn open_file(path: &Path) {
        #[cfg(windows)]
        {
            let _ = std::process::Command::new("explorer")
                .arg(path.as_os_str())
                .spawn();
        }
        #[cfg(not(windows))]
        {
            let _ = std::process::Command::new("xdg-open")
                .arg(path.as_os_str())
                .spawn();
        }
    }

    /// Opens the folder containing the file and selects it
    pub fn open_folder_and_select(path: &Path) {
        #[cfg(windows)]
        {
            let arg = format!("/select,{}", path.display());
            let _ = std::process::Command::new("explorer").arg(arg).spawn();
        }
        #[cfg(not(windows))]
        {
            if let Some(parent) = path.parent() {
                let _ = std::process::Command::new("xdg-open")
                    .arg(parent.as_os_str())
                    .spawn();
            }
        }
    }

    /// Opens the downloads directory
    pub fn open_directory(dir: &Path) {
        #[cfg(windows)]
        {
            let _ = std::process::Command::new("explorer")
                .arg(dir.as_os_str())
                .spawn();
        }
        #[cfg(not(windows))]
        {
            let _ = std::process::Command::new("xdg-open")
                .arg(dir.as_os_str())
                .spawn();
        }
    }

    /// Displays an interactive native Toast notification when a file is received
    pub fn notify_file_received(filename: &str, _path: &Path, size_bytes: u64) {
        let human_size = format_size(size_bytes);
        let title = "AirDrop Bridge - Archivo recibido";
        let message = format!("{} ({})", filename, human_size);

        info!("Notification: {} - {}", title, message);

        #[cfg(windows)]
        {
            let path_clone = _path.to_path_buf();
            let title_clone = title.to_string();
            let msg_clone = message.clone();

            std::thread::spawn(move || {
                show_windows_toast(&title_clone, &msg_clone, Some(&path_clone));
            });
        }

        #[cfg(not(windows))]
        {
            let _ = notify_rust::Notification::new()
                .summary(title)
                .body(&message)
                .icon("document-save")
                .show();
        }
    }

    /// Displays a notification when text or image is copied to the clipboard
    pub fn notify_clipboard(_content_type: &str, summary: &str) {
        let title = "AirDrop Bridge - Portapapeles";
        let message = format!("Copiado al portapapeles: {}", summary);

        info!("Notification: {} - {}", title, message);

        #[cfg(windows)]
        {
            let title_clone = title.to_string();
            let msg_clone = message.clone();

            std::thread::spawn(move || {
                show_windows_toast(&title_clone, &msg_clone, None);
            });
        }

        #[cfg(not(windows))]
        {
            let _ = notify_rust::Notification::new()
                .summary(title)
                .body(&message)
                .icon("edit-paste")
                .show();
        }
    }
}

pub fn format_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let b = bytes as f64;
    if b >= GB {
        format!("{:.2} GB", b / GB)
    } else if b >= MB {
        format!("{:.1} MB", b / MB)
    } else if b >= KB {
        format!("{:.0} KB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(windows)]
fn show_windows_toast(title: &str, message: &str, target_path: Option<&PathBuf>) {
    let clean_title = title.replace('\'', "''");
    let clean_msg = message.replace('\'', "''");

    let script = if let Some(p) = target_path {
        let clean_path = p.display().to_string().replace('\'', "''");
        let full_msg = format!("{} (Guardado en {})", clean_msg, clean_path);
        format!(
            r#"[Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] > $null
$template = [Windows.UI.Notifications.ToastNotificationManager]::GetTemplateContent([Windows.UI.Notifications.ToastTemplateType]::ToastText02)
$textNodes = $template.GetElementsByTagName("text")
$textNodes.Item(0).AppendChild($template.CreateTextNode('{title}')) > $null
$textNodes.Item(1).AppendChild($template.CreateTextNode('{msg}')) > $null
$toast = [Windows.UI.Notifications.ToastNotification]::new($template)
$notifier = [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier("AirDrop Bridge")
$notifier.Show($toast)"#,
            title = clean_title,
            msg = full_msg
        )
    } else {
        format!(
            r#"[Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] > $null
$template = [Windows.UI.Notifications.ToastNotificationManager]::GetTemplateContent([Windows.UI.Notifications.ToastTemplateType]::ToastText02)
$textNodes = $template.GetElementsByTagName("text")
$textNodes.Item(0).AppendChild($template.CreateTextNode('{title}')) > $null
$textNodes.Item(1).AppendChild($template.CreateTextNode('{msg}')) > $null
$toast = [Windows.UI.Notifications.ToastNotification]::new($template)
$notifier = [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier("AirDrop Bridge")
$notifier.Show($toast)"#,
            title = clean_title,
            msg = clean_msg
        )
    };

    let _ = std::process::Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle",
            "Hidden",
            "-Command",
            &script,
        ])
        .output();
}
