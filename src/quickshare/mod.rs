use crate::config::{AppConfig, QuickShareVisibility};
use crate::server::ServerEvent;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use tracing::info;

#[cfg(feature = "quickshare")]
mod native {
    use super::*;
    use crate::clipboard::ClipboardManager;
    use rqs_lib::channel::{ChannelAction, ChannelDirection, ChannelMessage, TransferType};
    use rqs_lib::{RQS, State, Visibility};
    use std::sync::Mutex;
    use tokio::sync::watch;
    use tracing::{error, warn};

    pub struct NativeQuickShare {
        visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
        shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
    }

    impl NativeQuickShare {
        pub async fn start(
            config: Arc<RwLock<AppConfig>>,
            event_tx: broadcast::Sender<ServerEvent>,
        ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
            let (download_dir, initial_vis) = {
                let conf = config.read().await;
                (conf.download_dir.clone(), map_visibility(conf.quickshare_visibility))
            };

            let mut rqs = RQS::new(initial_vis, None, Some(download_dir.clone()));
            let visibility_sender = rqs.visibility_sender.clone();
            let message_sender = rqs.message_sender.clone();

            let (_send_tx, _ble_rx) = rqs.run().await.map_err(|e| {
                format!("Failed to start Quick Share core: {}", e)
            })?;

            info!("Quick Share (Nearby Share) listener started");

            let mut msg_rx = message_sender.subscribe();
            let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel::<()>();

            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        _ = &mut shutdown_rx => {
                            info!("Stopping Quick Share listener...");
                            rqs.stop().await;
                            break;
                        }
                        recv = msg_rx.recv() => {
                            match recv {
                                Ok(msg) => {
                                    handle_channel_message(
                                        msg,
                                        &message_sender,
                                        &config,
                                        &event_tx,
                                    ).await;
                                }
                                Err(broadcast::error::RecvError::Lagged(n)) => {
                                    warn!("Quick Share channel lagged by {} messages", n);
                                }
                                Err(broadcast::error::RecvError::Closed) => {
                                    break;
                                }
                            }
                        }
                    }
                }
            });

            Ok(Self {
                visibility_sender,
                shutdown_tx: Some(shutdown_tx),
            })
        }

        pub fn set_visibility(&self, visibility: QuickShareVisibility) {
            let mapped = map_visibility(visibility);
            if let Ok(guard) = self.visibility_sender.lock() {
                guard.send_modify(|v| *v = mapped);
            }
        }
    }

    impl Drop for NativeQuickShare {
        fn drop(&mut self) {
            if let Some(tx) = self.shutdown_tx.take() {
                let _ = tx.send(());
            }
        }
    }

    fn map_visibility(vis: QuickShareVisibility) -> Visibility {
        match vis {
            QuickShareVisibility::Visible => Visibility::Visible,
            QuickShareVisibility::Invisible => Visibility::Invisible,
        }
    }

    async fn handle_channel_message(
        msg: ChannelMessage,
        message_sender: &broadcast::Sender<ChannelMessage>,
        config: &Arc<RwLock<AppConfig>>,
        event_tx: &broadcast::Sender<ServerEvent>,
    ) {
        if msg.direction != ChannelDirection::LibToFront {
            return;
        }
        if msg.rtype != Some(TransferType::Inbound) {
            return;
        }

        let Some(state) = msg.state else {
            return;
        };

        match state {
            State::WaitingForUserConsent => {
                let auto_accept = {
                    let conf = config.read().await;
                    conf.quickshare_auto_accept
                };

                let accepted = if auto_accept {
                    true
                } else {
                    let sender_name = msg
                        .meta
                        .as_ref()
                        .and_then(|m| m.source.as_ref())
                        .map(|s| s.name.clone())
                        .unwrap_or_else(|| "Android device".to_string());
                    let pin = msg
                        .meta
                        .as_ref()
                        .and_then(|m| m.pin_code.clone())
                        .unwrap_or_else(|| "----".to_string());
                    prompt_user_consent(sender_name, pin).await
                };

                let action = if accepted {
                    ChannelAction::AcceptTransfer
                } else {
                    ChannelAction::RejectTransfer
                };

                let _ = message_sender.send(ChannelMessage {
                    id: msg.id,
                    direction: ChannelDirection::FrontToLib,
                    action: Some(action),
                    rtype: None,
                    state: None,
                    meta: None,
                });
            }
            State::Finished => {
                let Some(meta) = msg.meta else {
                    return;
                };

                let (download_dir, clipboard_sync) = {
                    let conf = config.read().await;
                    (conf.download_dir.clone(), conf.clipboard_sync)
                };

                if let Some(files) = meta.files {
                    for raw_name in files {
                        let path = resolve_received_file_path(&download_dir, &raw_name);
                        let filename = path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| raw_name.clone());
                        let size_bytes = tokio::fs::metadata(&path)
                            .await
                            .map(|m| m.len())
                            .unwrap_or(meta.total_bytes);

                        info!("Quick Share file received: {} ({} bytes)", filename, size_bytes);
                        let _ = event_tx.send(ServerEvent::FileReceived {
                            filename,
                            path,
                            size_bytes,
                        });
                    }
                }

                if let Some(text) = meta.text_payload {
                    if clipboard_sync {
                        if let Err(e) = ClipboardManager::set_text(&text) {
                            error!("Failed to copy Quick Share text to clipboard: {}", e);
                        } else {
                            let summary = build_clipboard_summary(&text);
                            let _ = event_tx.send(ServerEvent::ClipboardReceived {
                                content_type: "text".to_string(),
                                summary,
                            });
                        }
                    }
                }
            }
            State::Rejected | State::Cancelled | State::Disconnected => {
                info!("Quick Share transfer ended with state: {:?}", state);
            }
            _ => {}
        }
    }

    async fn prompt_user_consent(sender_name: String, pin: String) -> bool {
        tokio::task::spawn_blocking(move || {
            let prompt = format!(
                "Accept Quick Share transfer from '{}'?\nPIN: {}",
                sender_name, pin
            );

            #[cfg(windows)]
            {
                use std::ffi::OsStr;
                use std::os::windows::ffi::OsStrExt;
                use windows_sys::Win32::UI::WindowsAndMessaging::{
                    IDYES, MB_ICONQUESTION, MB_SYSTEMMODAL, MB_YESNO, MessageBoxW,
                };

                let title_wide: Vec<u16> = OsStr::new("DropBridge Quick Share")
                    .encode_wide()
                    .chain(std::iter::once(0))
                    .collect();
                let text_wide: Vec<u16> = OsStr::new(&prompt)
                    .encode_wide()
                    .chain(std::iter::once(0))
                    .collect();

                let res = unsafe {
                    MessageBoxW(
                        std::ptr::null_mut(),
                        text_wide.as_ptr(),
                        title_wide.as_ptr(),
                        MB_YESNO | MB_ICONQUESTION | MB_SYSTEMMODAL,
                    )
                };
                return res == IDYES;
            }

            #[cfg(not(windows))]
            {
                if let Ok(status) = std::process::Command::new("zenity")
                    .args(["--question", "--title=DropBridge Quick Share", "--text", &prompt])
                    .status()
                {
                    return status.success();
                }
                if let Ok(status) = std::process::Command::new("kdialog")
                    .args(["--title", "DropBridge Quick Share", "--yesno", &prompt])
                    .status()
                {
                    return status.success();
                }
                warn!("Neither zenity nor kdialog found for Quick Share consent prompt; rejecting transfer");
                false
            }
        })
        .await
        .unwrap_or(false)
    }
}

pub struct QuickShareManager {
    #[cfg(feature = "quickshare")]
    inner: native::NativeQuickShare,
}

impl QuickShareManager {
    pub async fn start(
        config: Arc<RwLock<AppConfig>>,
        event_tx: broadcast::Sender<ServerEvent>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        #[cfg(feature = "quickshare")]
        {
            let inner = native::NativeQuickShare::start(config, event_tx).await?;
            Ok(Self { inner })
        }

        #[cfg(not(feature = "quickshare"))]
        {
            let _ = (config, event_tx);
            info!("Quick Share native backend is disabled (feature 'quickshare' not enabled)");
            Ok(Self {})
        }
    }

    pub fn set_visibility(&self, visibility: QuickShareVisibility) {
        #[cfg(feature = "quickshare")]
        {
            self.inner.set_visibility(visibility);
        }

        #[cfg(not(feature = "quickshare"))]
        {
            let _ = visibility;
        }
    }

    pub fn is_supported() -> bool {
        cfg!(feature = "quickshare")
    }
}

/// Resolves the path of a received Quick Share file inside `download_dir`
pub fn resolve_received_file_path(download_dir: &Path, raw_name: &str) -> PathBuf {
    let candidate = Path::new(raw_name);
    if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        let safe = sanitize_filename::sanitize(raw_name);
        if safe.is_empty() {
            download_dir.join("received_file")
        } else {
            download_dir.join(safe)
        }
    }
}

/// Builds a short single-line summary for clipboard notifications
pub fn build_clipboard_summary(text: &str) -> String {
    let trimmed = text.trim().replace('\n', " ");
    if trimmed.chars().count() > 60 {
        let truncated: String = trimmed.chars().take(57).collect();
        format!("{}...", truncated)
    } else {
        trimmed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_received_file_path_sanitizes_relative() {
        let dir = Path::new("/tmp/downloads");
        let resolved = resolve_received_file_path(dir, "../evil/photo.jpg");
        assert_eq!(resolved.parent(), Some(dir));
        let fname = resolved.file_name().unwrap().to_string_lossy();
        assert!(!fname.contains('/'));
        assert!(fname.ends_with("photo.jpg"));
    }

    #[test]
    fn test_build_clipboard_summary_truncates_long_text() {
        let long = "a".repeat(100);
        let summary = build_clipboard_summary(&long);
        assert_eq!(summary.chars().count(), 60);
        assert!(summary.ends_with("..."));
    }
}
