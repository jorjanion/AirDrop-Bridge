#[cfg(windows)]
use crate::autostart::AutostartManager;
use crate::config::AppConfig;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
#[cfg(windows)]
use tracing::error;
#[cfg(not(windows))]
use tracing::info;

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
    #[cfg(windows)]
    pub fn spawn_tray(
        config: Arc<RwLock<AppConfig>>,
        cmd_tx: mpsc::Sender<TrayCommand>,
    ) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            if let Err(e) = run_windows_tray(config, cmd_tx) {
                error!("Windows tray error: {}", e);
            }
        })
    }

    #[cfg(not(windows))]
    pub fn spawn_tray(
        _config: Arc<RwLock<AppConfig>>,
        _cmd_tx: mpsc::Sender<TrayCommand>,
    ) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            info!("System tray initialized (headless/stub for non-windows)");
        })
    }
}

#[cfg(windows)]
fn run_windows_tray(
    config: Arc<RwLock<AppConfig>>,
    cmd_tx: mpsc::Sender<TrayCommand>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::UI::Shell::{
        NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NOTIFYICONDATAW, Shell_NotifyIconW,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu, DestroyWindow,
        DispatchMessageW, GetCursorPos, GetMessageW, IDI_APPLICATION, LoadIconW, MF_CHECKED,
        MF_DISABLED, MF_GRAYED, MF_SEPARATOR, MF_STRING, MF_UNCHECKED, MSG, PostQuitMessage,
        RegisterClassExW, SetForegroundWindow, TPM_BOTTOMALIGN, TPM_LEFTALIGN, TrackPopupMenuEx,
        TranslateMessage, WM_APP, WM_RBUTTONUP, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
    };

    const WM_TRAYICON: u32 = WM_APP + 1;
    const ID_HEADER: usize = 1000;
    const ID_PAIR: usize = 1001;
    const ID_DOWNLOADS: usize = 1002;
    const ID_COPY_TOKEN: usize = 1003;
    const ID_AUTOSTART: usize = 1004;
    const ID_CHECK_UPDATE: usize = 1005;
    const ID_EXIT: usize = 1006;

    let class_name: Vec<u16> = OsStr::new("AirDropBridgeTrayClass")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let h_instance =
            windows_sys::Win32::System::LibraryLoader::GetModuleHandleW(std::ptr::null());

        let wnd_class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: 0,
            lpfnWndProc: Some(DefWindowProcW),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: LoadIconW(0 as _, IDI_APPLICATION),
            hCursor: std::ptr::null_mut(),
            hbrBackground: std::ptr::null_mut(),
            lpszMenuName: std::ptr::null(),
            lpszClassName: class_name.as_ptr(),
            hIconSm: std::ptr::null_mut(),
        };

        RegisterClassExW(&wnd_class);

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            class_name.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            0,
            0,
            0,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            h_instance,
            std::ptr::null_mut(),
        );

        if hwnd.is_null() {
            return Err("Failed to create tray window".into());
        }

        let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
        nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
        nid.hWnd = hwnd;
        nid.uID = 1;
        nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
        nid.uCallbackMessage = WM_TRAYICON;
        nid.hIcon = LoadIconW(0 as _, IDI_APPLICATION);

        let tip = "AirDrop Bridge - Conectado\0";
        for (i, c) in tip.encode_utf16().enumerate() {
            if i < nid.szTip.len() {
                nid.szTip[i] = c;
            }
        }

        Shell_NotifyIconW(NIM_ADD, &nid);

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            if msg.message == WM_TRAYICON && msg.lParam as u32 == WM_RBUTTONUP {
                let mut pt: POINT = std::mem::zeroed();
                GetCursorPos(&mut pt);

                let hmenu = CreatePopupMenu();

                let header_text =
                    format!("AirDrop Bridge (Puerto {})\0", config.blocking_read().port);
                let header_wide: Vec<u16> = header_text.encode_utf16().collect();
                AppendMenuW(
                    hmenu,
                    MF_STRING | MF_DISABLED | MF_GRAYED,
                    ID_HEADER,
                    header_wide.as_ptr(),
                );

                AppendMenuW(hmenu, MF_SEPARATOR, 0, std::ptr::null());

                let pair_text = "📱 Emparejar dispositivo (QR)...\0"
                    .encode_utf16()
                    .collect::<Vec<_>>();
                AppendMenuW(hmenu, MF_STRING, ID_PAIR, pair_text.as_ptr());

                let dl_text = "📂 Abrir carpeta de descargas\0"
                    .encode_utf16()
                    .collect::<Vec<_>>();
                AppendMenuW(hmenu, MF_STRING, ID_DOWNLOADS, dl_text.as_ptr());

                let token_text = "📋 Copiar token de seguridad\0"
                    .encode_utf16()
                    .collect::<Vec<_>>();
                AppendMenuW(hmenu, MF_STRING, ID_COPY_TOKEN, token_text.as_ptr());

                AppendMenuW(hmenu, MF_SEPARATOR, 0, std::ptr::null());

                let is_autostart = AutostartManager::is_enabled();
                let autostart_flag = if is_autostart {
                    MF_CHECKED
                } else {
                    MF_UNCHECKED
                };
                let autostart_text = "⚙️ Iniciar con Windows\0"
                    .encode_utf16()
                    .collect::<Vec<_>>();
                AppendMenuW(
                    hmenu,
                    MF_STRING | autostart_flag,
                    ID_AUTOSTART,
                    autostart_text.as_ptr(),
                );

                let update_text = "🔄 Buscar actualizaciones...\0"
                    .encode_utf16()
                    .collect::<Vec<_>>();
                AppendMenuW(hmenu, MF_STRING, ID_CHECK_UPDATE, update_text.as_ptr());

                AppendMenuW(hmenu, MF_SEPARATOR, 0, std::ptr::null());

                let exit_text = "❌ Salir\0".encode_utf16().collect::<Vec<_>>();
                AppendMenuW(hmenu, MF_STRING, ID_EXIT, exit_text.as_ptr());

                SetForegroundWindow(hwnd);
                let cmd = TrackPopupMenuEx(
                    hmenu,
                    TPM_LEFTALIGN | TPM_BOTTOMALIGN | 0x0100, // TPM_RETURNCMD
                    pt.x,
                    pt.y,
                    hwnd,
                    std::ptr::null_mut(),
                );
                DestroyMenu(hmenu);

                match cmd as usize {
                    ID_PAIR => {
                        let _ = cmd_tx.blocking_send(TrayCommand::PairDevice);
                    }
                    ID_DOWNLOADS => {
                        let _ = cmd_tx.blocking_send(TrayCommand::OpenDownloads);
                    }
                    ID_COPY_TOKEN => {
                        let _ = cmd_tx.blocking_send(TrayCommand::CopyToken);
                    }
                    ID_AUTOSTART => {
                        let _ = cmd_tx.blocking_send(TrayCommand::ToggleAutostart);
                    }
                    ID_CHECK_UPDATE => {
                        let _ = cmd_tx.blocking_send(TrayCommand::CheckUpdate);
                    }
                    ID_EXIT => {
                        let _ = cmd_tx.blocking_send(TrayCommand::Exit);
                        Shell_NotifyIconW(NIM_DELETE, &nid);
                        DestroyWindow(hwnd);
                        PostQuitMessage(0);
                        break;
                    }
                    _ => {}
                }
            }
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        Shell_NotifyIconW(NIM_DELETE, &nid);
    }

    Ok(())
}
