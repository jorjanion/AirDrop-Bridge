use crate::i18n::Language;
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
    pub fn notify_file_received(filename: &str, _path: &Path, size_bytes: u64, lang: Language) {
        let strings = crate::i18n::t(lang);
        let human_size = format_size(size_bytes);
        let title = strings.notif_file_received_title;
        let message = format!("{} ({})", filename, human_size);

        info!("Notification: {} - {}", title, message);

        #[cfg(windows)]
        {
            let path_clone = _path.to_path_buf();
            let title_clone = title.to_string();
            let msg_clone = message.clone();
            let saved_in_prefix = strings.notif_saved_in.to_string();

            std::thread::spawn(move || {
                show_windows_toast(&title_clone, &msg_clone, Some(&path_clone), Some(&saved_in_prefix));
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
    pub fn notify_clipboard(_content_type: &str, summary: &str, lang: Language) {
        let strings = crate::i18n::t(lang);
        let title = strings.notif_clipboard_title;
        let message = format!("{} {}", strings.notif_copied, summary);

        info!("Notification: {} - {}", title, message);

        #[cfg(windows)]
        {
            let title_clone = title.to_string();
            let msg_clone = message.clone();

            std::thread::spawn(move || {
                show_windows_toast(&title_clone, &msg_clone, None, None);
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

    /// Displays a general application notification
    pub fn notify_message(title: &str, message: &str) {
        info!("Notification: {} - {}", title, message);

        #[cfg(windows)]
        {
            let title_clone = title.to_string();
            let msg_clone = message.to_string();

            std::thread::spawn(move || {
                show_windows_toast(&title_clone, &msg_clone, None, None);
            });
        }

        #[cfg(not(windows))]
        {
            let _ = notify_rust::Notification::new()
                .summary(title)
                .body(message)
                .icon("dialog-information")
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
fn show_windows_toast(
    title: &str,
    message: &str,
    target_path: Option<&PathBuf>,
    saved_in_prefix: Option<&str>,
) {
    let clean_title = title.replace('\'', "''");
    let clean_msg = message.replace('\'', "''");

    let script = if let Some(p) = target_path {
        let clean_path = p.display().to_string().replace('\'', "''");
        let prefix = saved_in_prefix.unwrap_or("Saved in");
        let full_msg = format!("{} ({} {})", clean_msg, prefix, clean_path);
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

    #[cfg(windows)]
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let mut cmd = std::process::Command::new("powershell");
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    let _ = cmd
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
