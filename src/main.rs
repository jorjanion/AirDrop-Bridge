#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod autostart;
pub mod clipboard;
pub mod config;
pub mod mdns;
pub mod notification;
pub mod qr;
pub mod server;
pub mod tray;
pub mod ui;
pub mod updater;

use autostart::AutostartManager;
use clipboard::ClipboardManager;
use config::AppConfig;
use mdns::MdnsResponder;
use notification::NotificationManager;
use qr::QrManager;
use server::{AppState, ServerEvent, create_router};
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tracing::{error, info};
use tray::{TrayCommand, TrayManager};
use ui::win_ui;
use updater::AutoUpdater;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "airdrop_bridge=info,tower_http=info".into()),
        )
        .init();

    info!("Starting AirDrop Bridge v{}...", env!("CARGO_PKG_VERSION"));

    // Clean up residual binaries from previous hot-swap updates
    AutoUpdater::cleanup_old_binaries();

    // Load or initialize persistent configuration
    let config = AppConfig::load_or_create()?;
    let port = config.port;
    let device_name = config.device_name.clone();

    // Start mDNS advertisement
    let _mdns = match MdnsResponder::start(&device_name, port) {
        Ok(m) => {
            info!("mDNS responder started for {}", device_name);
            Some(m)
        }
        Err(e) => {
            error!("Failed to start mDNS responder: {}", e);
            None
        }
    };

    // Initialize server state & broadcast channel
    let (state, mut event_rx) = AppState::new(config.clone());
    let router = create_router(state.clone());

    // Spawn server background task
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("HTTP Server listening on http://0.0.0.0:{}", port);

    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, router).await {
            error!("Server error: {}", e);
        }
    });

    // Spawn ServerEvent consumer (Notifications & Automation)
    let config_events = state.config.clone();
    tokio::spawn(async move {
        while let Ok(event) = event_rx.recv().await {
            let conf = config_events.read().await;
            match event {
                ServerEvent::FileReceived {
                    filename,
                    path,
                    size_bytes,
                } => {
                    if conf.play_sound {
                        NotificationManager::play_chime();
                    }
                    if conf.notifications_enabled {
                        NotificationManager::notify_file_received(&filename, &path, size_bytes);
                    }
                    if conf.open_folder_on_receive {
                        NotificationManager::open_directory(&conf.download_dir);
                    }
                }
                ServerEvent::ClipboardReceived {
                    content_type,
                    summary,
                } => {
                    if conf.play_sound {
                        NotificationManager::play_chime();
                    }
                    if conf.notifications_enabled {
                        NotificationManager::notify_clipboard(&content_type, &summary);
                    }
                }
            }
        }
    });

    // Start auto-updater background checker
    if config.auto_update {
        AutoUpdater::spawn_background_checker();
    }

    let config_arc = state.config.clone();

    // Start Native UI Window thread (starts visible, minimizes to tray on close)
    win_ui::start_ui_thread(config_arc.clone());

    // Set up System Tray & command channel
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<TrayCommand>(32);
    let _tray_thread = TrayManager::spawn_tray(config_arc.clone(), cmd_tx);

    info!("AirDrop Bridge is active in system tray. Zero-config ready!");

    // Main event loop handling system tray commands
    while let Some(cmd) = cmd_rx.recv().await {
        match cmd {
            TrayCommand::PairDevice => {
                let conf = config_arc.read().await;
                let pair_url = QrManager::pairing_url("localhost", conf.port, &conf.token);
                info!("Opening pairing portal: {}", pair_url);
                NotificationManager::open_directory(&std::path::PathBuf::from(&pair_url));
            }
            TrayCommand::OpenDownloads => {
                let conf = config_arc.read().await;
                NotificationManager::open_directory(&conf.download_dir);
            }
            TrayCommand::CopyToken => {
                let conf = config_arc.read().await;
                if let Err(e) = ClipboardManager::set_text(&conf.token) {
                    error!("Failed to copy token: {}", e);
                } else {
                    NotificationManager::notify_clipboard("text", "Token de seguridad copiado");
                }
            }
            TrayCommand::ToggleAutostart => {
                let is_enabled = AutostartManager::is_enabled();
                if let Err(e) = AutostartManager::set_enabled(!is_enabled) {
                    error!("Failed to toggle autostart: {}", e);
                } else {
                    let mut conf = config_arc.write().await;
                    conf.autostart = !is_enabled;
                    let _ = conf.save();
                }
            }
            TrayCommand::CheckUpdate => {
                tokio::spawn(async {
                    AutoUpdater::check_and_update(true).await;
                });
            }
            TrayCommand::Exit => {
                info!("Shutdown requested from tray. Exiting cleanly...");
                win_ui::close_window();
                break;
            }
        }
    }

    info!("AirDrop Bridge stopped.");
    Ok(())
}
