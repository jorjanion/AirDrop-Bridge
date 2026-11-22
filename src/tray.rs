use crate::autostart::AutostartManager;
use crate::clipboard::ClipboardManager;
use crate::config::AppConfig;
use crate::notification::NotificationManager;
use crate::ui::win_ui;
use muda::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tracing::{error, info};
use tray_icon::{MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent};

pub enum TrayCommand {
    PairDevice,
    OpenDownloads,
    CopyToken,
    ToggleAutostart,
    CheckUpdate,
    Exit,
}

pub struct TrayManager;

impl TrayManager {
    pub fn spawn_tray(
        config: Arc<RwLock<AppConfig>>,
        cmd_tx: mpsc::Sender<TrayCommand>,
    ) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            if let Err(e) = run_tray_loop(config, cmd_tx) {
                error!("Tray loop error: {}", e);
            }
        })
    }
}

fn load_tray_icon() -> Result<tray_icon::Icon, Box<dyn std::error::Error + Send + Sync>> {
    let png_bytes = include_bytes!("../resources/icon.png");
    let img = image::load_from_memory(png_bytes)?.to_rgba8();
    let (width, height) = img.dimensions();
    let icon = tray_icon::Icon::from_rgba(img.into_raw(), width, height)?;
    Ok(icon)
}

fn run_tray_loop(
    config: Arc<RwLock<AppConfig>>,
    cmd_tx: mpsc::Sender<TrayCommand>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let icon = match load_tray_icon() {
        Ok(i) => i,
        Err(e) => {
            error!("Failed to load custom tray icon: {}", e);
            // Fallback: 16x16 transparent icon
            tray_icon::Icon::from_rgba(vec![0; 16 * 16 * 4], 16, 16)?
        }
    };

    let conf = config.blocking_read();
    let port = conf.port;
    let is_autostart = AutostartManager::is_enabled();
    drop(conf);

    // Build context menu with muda
    let menu = Menu::new();

    let header_title = format!("AirDrop Bridge (:{})", port);
    let header_item = MenuItem::new(header_title, false, None);
    let sep1 = PredefinedMenuItem::separator();

    let open_item = MenuItem::new("📱 Abrir AirDrop Bridge", true, None);
    let dl_item = MenuItem::new("📂 Abrir descargas", true, None);
    let token_item = MenuItem::new("📋 Copiar token", true, None);
    let web_item = MenuItem::new("🌐 Abrir en navegador", true, None);
    let sep2 = PredefinedMenuItem::separator();

    let autostart_item = CheckMenuItem::new("⚙️ Iniciar con Windows", true, is_autostart, None);
    let update_item = MenuItem::new("🔄 Buscar actualizaciones...", true, None);
    let sep3 = PredefinedMenuItem::separator();

    let exit_item = MenuItem::new("❌ Salir", true, None);

    let open_id = open_item.id().clone();
    let dl_id = dl_item.id().clone();
    let token_id = token_item.id().clone();
    let web_id = web_item.id().clone();
    let autostart_id = autostart_item.id().clone();
    let update_id = update_item.id().clone();
    let exit_id = exit_item.id().clone();

    menu.append_items(&[
        &header_item,
        &sep1,
        &open_item,
        &dl_item,
        &token_item,
        &web_item,
        &sep2,
        &autostart_item,
        &update_item,
        &sep3,
        &exit_item,
    ])?;

    let _tray_icon: TrayIcon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("AirDrop Bridge - En linea")
        .with_icon(icon)
        .build()?;

    info!("System Tray initialized successfully with native icon and context menu");

    // Event loop for tray clicks and menu selections
    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    'tray_loop: loop {
        #[cfg(windows)]
        unsafe {
            use windows_sys::Win32::UI::WindowsAndMessaging::{
                DispatchMessageW, MSG, MsgWaitForMultipleObjectsEx, PM_REMOVE, PeekMessageW,
                QS_ALLINPUT, TranslateMessage, WM_QUIT,
            };

            let mut msg: MSG = std::mem::zeroed();
            while PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
                if msg.message == WM_QUIT {
                    break 'tray_loop;
                }
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            // Wait for messages with low latency and 0% CPU consumption
            MsgWaitForMultipleObjectsEx(0, std::ptr::null(), 20, QS_ALLINPUT, 0);
        }

        #[cfg(not(windows))]
        std::thread::sleep(std::time::Duration::from_millis(20));

        // Handle menu clicks
        while let Ok(event) = menu_channel.try_recv() {
            if event.id == open_id {
                win_ui::show_window();
            } else if event.id == dl_id {
                let conf = config.blocking_read();
                NotificationManager::open_directory(&conf.download_dir);
            } else if event.id == token_id {
                let conf = config.blocking_read();
                if let Err(e) = ClipboardManager::set_text(&conf.token) {
                    error!("Failed to copy token: {}", e);
                } else {
                    NotificationManager::notify_clipboard("text", "Token copiado al portapapeles");
                }
            } else if event.id == web_id {
                let _ = cmd_tx.blocking_send(TrayCommand::PairDevice);
            } else if event.id == autostart_id {
                let _ = cmd_tx.blocking_send(TrayCommand::ToggleAutostart);
            } else if event.id == update_id {
                let _ = cmd_tx.blocking_send(TrayCommand::CheckUpdate);
            } else if event.id == exit_id {
                win_ui::close_window();
                let _ = cmd_tx.blocking_send(TrayCommand::Exit);
                break 'tray_loop;
            }
        }

        // Handle left click on tray icon -> show UI window!
        while let Ok(event) = tray_channel.try_recv() {
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    ..
                }
                | TrayIconEvent::DoubleClick {
                    button: MouseButton::Left,
                    ..
                } => {
                    win_ui::show_window();
                }
                _ => {}
            }
        }
    }

    Ok(())
}
