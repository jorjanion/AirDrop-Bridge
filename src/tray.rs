use crate::autostart::AutostartManager;
use crate::clipboard::ClipboardManager;
use crate::config::AppConfig;
use crate::i18n::{Language, t};
use crate::notification::NotificationManager;
use crate::quickshare::QuickShareManager;
use crate::ui::win_ui;
use muda::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tracing::{error, info};
use tray_icon::{MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent};

pub enum TrayCommand {
    PairDevice,
    OpenDownloads,
    CopyToken,
    ToggleAutostart,
    ToggleQuickShare(bool),
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
    #[cfg(target_os = "linux")]
    gtk::init().map_err(|e| format!("Failed to initialize GTK for system tray: {}", e))?;

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
    let is_notif = conf.notifications_enabled;
    let is_sound = conf.play_sound;
    let is_open_folder = conf.open_folder_on_receive;
    let is_clipboard = conf.clipboard_sync;
    let is_quickshare = conf.quickshare_enabled;
    let is_auto_update = conf.auto_update;
    let current_custom_host = conf.custom_host.clone();
    let mut current_lang = conf.language;
    drop(conf);

    let strings = t(current_lang);

    // Build context menu with muda
    let menu = Menu::new();

    let header_title = format!("Drop Bridge v{} (:{})", env!("CARGO_PKG_VERSION"), port);
    let header_item = MenuItem::new(header_title, false, None);
    let sep1 = PredefinedMenuItem::separator();

    let open_item = MenuItem::new(strings.tray_open, true, None);
    let dl_item = MenuItem::new(strings.tray_downloads, true, None);
    let token_item = MenuItem::new(strings.tray_copy_token, true, None);
    let web_item = MenuItem::new(strings.tray_browser, true, None);
    let sep2 = PredefinedMenuItem::separator();

    // Settings submenu
    let settings_submenu = Submenu::new(strings.tray_settings, true);
    let autostart_item = CheckMenuItem::new(strings.tray_autostart, true, is_autostart, None);
    let notif_item = CheckMenuItem::new(strings.tray_notifications, true, is_notif, None);
    let sound_item = CheckMenuItem::new(strings.tray_sound, true, is_sound, None);
    let open_folder_item =
        CheckMenuItem::new(strings.tray_open_folder, true, is_open_folder, None);
    let clipboard_item =
        CheckMenuItem::new(strings.tray_clipboard, true, is_clipboard, None);
    let quickshare_supported = QuickShareManager::is_supported();
    let quickshare_item = CheckMenuItem::new(
        strings.tray_quickshare,
        quickshare_supported,
        quickshare_supported && is_quickshare,
        None,
    );
    let auto_update_item =
        CheckMenuItem::new(strings.tray_auto_update, true, is_auto_update, None);

    // Network Interface submenu
    let network_submenu = Submenu::new("🌐 Network Interface", true);
    let endpoints = AppConfig::get_available_endpoints();
    let is_auto = current_custom_host.is_none();
    let auto_item = CheckMenuItem::new("Auto (LAN)", true, is_auto, None);
    network_submenu.append(&auto_item)?;

    let mut endpoint_items: Vec<(Option<String>, CheckMenuItem)> = Vec::new();
    endpoint_items.push((None, auto_item));

    for ep in &endpoints {
        let label = format!("{} ({})", ep.name, ep.ip);
        let is_selected = current_custom_host.as_deref() == Some(&ep.ip);
        let ep_item = CheckMenuItem::new(&label, true, is_selected, None);
        network_submenu.append(&ep_item)?;
        endpoint_items.push((Some(ep.ip.clone()), ep_item));
    }

    // Language submenu
    let lang_submenu = Submenu::new(strings.tray_language, true);
    let lang_items: Vec<(Language, CheckMenuItem)> = Language::ALL
        .iter()
        .map(|&l| {
            let item = CheckMenuItem::new(l.display_name(), true, current_lang == l, None);
            (l, item)
        })
        .collect();

    for (_, item) in &lang_items {
        lang_submenu.append(item)?;
    }

    settings_submenu.append_items(&[
        &autostart_item,
        &notif_item,
        &sound_item,
        &open_folder_item,
        &clipboard_item,
        &quickshare_item,
        &auto_update_item,
        &network_submenu,
        &lang_submenu,
    ])?;

    let update_item = MenuItem::new(strings.tray_check_updates, true, None);
    let sep3 = PredefinedMenuItem::separator();

    let exit_item = MenuItem::new(strings.tray_exit, true, None);

    let open_id = open_item.id().clone();
    let dl_id = dl_item.id().clone();
    let token_id = token_item.id().clone();
    let web_id = web_item.id().clone();

    let autostart_id = autostart_item.id().clone();
    let notif_id = notif_item.id().clone();
    let sound_id = sound_item.id().clone();
    let open_folder_id = open_folder_item.id().clone();
    let clipboard_id = clipboard_item.id().clone();
    let quickshare_id = quickshare_item.id().clone();
    let auto_update_id = auto_update_item.id().clone();

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
        &settings_submenu,
        &update_item,
        &sep3,
        &exit_item,
    ])?;

    let tray_icon: TrayIcon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip(strings.tray_tooltip)
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

        #[cfg(target_os = "linux")]
        {
            while gtk::events_pending() {
                gtk::main_iteration_do(false);
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }

        #[cfg(all(not(windows), not(target_os = "linux")))]
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
                    let strings = t(conf.language);
                    NotificationManager::notify_message(strings.ui_title, strings.token_copied);
                }
            } else if event.id == web_id {
                let _ = cmd_tx.blocking_send(TrayCommand::PairDevice);
            } else if event.id == autostart_id {
                let is_enabled = AutostartManager::is_enabled();
                let new_state = !is_enabled;
                if let Err(e) = AutostartManager::set_enabled(new_state) {
                    error!("Failed to toggle autostart: {}", e);
                } else {
                    autostart_item.set_checked(new_state);
                    let mut conf = config.blocking_write();
                    conf.autostart = new_state;
                    let _ = conf.save();
                }
            } else if event.id == notif_id {
                let mut conf = config.blocking_write();
                conf.notifications_enabled = !conf.notifications_enabled;
                notif_item.set_checked(conf.notifications_enabled);
                let _ = conf.save();
            } else if event.id == sound_id {
                let mut conf = config.blocking_write();
                conf.play_sound = !conf.play_sound;
                sound_item.set_checked(conf.play_sound);
                let _ = conf.save();
            } else if event.id == open_folder_id {
                let mut conf = config.blocking_write();
                conf.open_folder_on_receive = !conf.open_folder_on_receive;
                open_folder_item.set_checked(conf.open_folder_on_receive);
                let _ = conf.save();
            } else if event.id == clipboard_id {
                let mut conf = config.blocking_write();
                conf.clipboard_sync = !conf.clipboard_sync;
                clipboard_item.set_checked(conf.clipboard_sync);
                let _ = conf.save();
            } else if event.id == quickshare_id {
                let new_state = {
                    let mut conf = config.blocking_write();
                    conf.quickshare_enabled = !conf.quickshare_enabled;
                    conf.quickshare_visibility = if conf.quickshare_enabled {
                        crate::config::QuickShareVisibility::Visible
                    } else {
                        crate::config::QuickShareVisibility::Invisible
                    };
                    quickshare_item.set_checked(conf.quickshare_enabled);
                    let _ = conf.save();
                    conf.quickshare_enabled
                };
                let _ = cmd_tx.blocking_send(TrayCommand::ToggleQuickShare(new_state));
            } else if event.id == auto_update_id {
                let mut conf = config.blocking_write();
                conf.auto_update = !conf.auto_update;
                auto_update_item.set_checked(conf.auto_update);
                let _ = conf.save();
            } else if let Some(&(lang, _)) =
                lang_items.iter().find(|(_, item)| item.id() == &event.id)
            {
                if current_lang != lang {
                    current_lang = lang;
                    let mut conf = config.blocking_write();
                    conf.language = lang;
                    let _ = conf.save();
                    drop(conf);

                    for (l, item) in &lang_items {
                        item.set_checked(*l == current_lang);
                    }

                    let new_strings = t(current_lang);
                    open_item.set_text(new_strings.tray_open);
                    dl_item.set_text(new_strings.tray_downloads);
                    token_item.set_text(new_strings.tray_copy_token);
                    web_item.set_text(new_strings.tray_browser);
                    settings_submenu.set_text(new_strings.tray_settings);
                    autostart_item.set_text(new_strings.tray_autostart);
                    notif_item.set_text(new_strings.tray_notifications);
                    sound_item.set_text(new_strings.tray_sound);
                    open_folder_item.set_text(new_strings.tray_open_folder);
                    clipboard_item.set_text(new_strings.tray_clipboard);
                    quickshare_item.set_text(new_strings.tray_quickshare);
                    auto_update_item.set_text(new_strings.tray_auto_update);
                    lang_submenu.set_text(new_strings.tray_language);
                    update_item.set_text(new_strings.tray_check_updates);
                    exit_item.set_text(new_strings.tray_exit);
                    let _ = tray_icon.set_tooltip(Some(new_strings.tray_tooltip));

                    win_ui::update_language();
                } else {
                    for (l, item) in &lang_items {
                        item.set_checked(*l == current_lang);
                    }
                }
            } else if let Some((custom_ip_opt, _)) =
                endpoint_items.iter().find(|(_, item)| item.id() == &event.id)
            {
                let selected_ip = custom_ip_opt.clone();
                let mut conf = config.blocking_write();
                conf.custom_host = selected_ip.clone();
                let _ = conf.save();
                drop(conf);

                for (ip_opt, item) in &endpoint_items {
                    item.set_checked(ip_opt == &selected_ip);
                }

                win_ui::refresh_pairing_state();

                let host_display = selected_ip.unwrap_or_else(|| "Auto".to_string());
                NotificationManager::notify_message(
                    "Drop Bridge",
                    &format!("Network host set to: {}", host_display),
                );
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
