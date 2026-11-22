#[cfg(windows)]
pub mod win_ui {
    use crate::clipboard::ClipboardManager;
    use crate::config::AppConfig;
    use crate::notification::NotificationManager;
    use crate::qr::QrManager;
    use local_ip_address::local_ip;
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::sync::{Arc, OnceLock};
    use tokio::sync::RwLock;
    use tracing::error;
    use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
    use windows_sys::Win32::Graphics::Dwm::{DWMWA_USE_IMMERSIVE_DARK_MODE, DwmSetWindowAttribute};
    use windows_sys::Win32::Graphics::Gdi::{
        BI_RGB, BITMAPINFO, BITMAPINFOHEADER, BeginPaint, CreateFontW, CreateSolidBrush,
        DIB_RGB_COLORS, DeleteObject, EndPaint, FW_BOLD, FW_NORMAL, FillRect, InvalidateRect,
        PAINTSTRUCT, SRCCOPY, SelectObject, SetBkMode, SetTextColor, StretchDIBits, TRANSPARENT,
        TextOutW,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        BS_PUSHBUTTON, CreateIconFromResourceEx, CreateWindowExW, DefWindowProcW, DestroyWindow,
        DispatchMessageW, GetClientRect, GetDlgItem, GetMessageW, GetSystemMetrics, HMENU,
        ICON_BIG, ICON_SMALL, IDI_APPLICATION, IMAGE_ICON, LR_DEFAULTCOLOR, LoadIconW, LoadImageW,
        MSG, PostQuitMessage, RegisterClassExW, SM_CXICON, SM_CXSMICON, SM_CYICON, SM_CYSMICON,
        SW_HIDE, SW_RESTORE, SW_SHOW, SW_SHOWNORMAL, SendMessageW, SetForegroundWindow,
        SetWindowTextW, ShowWindow, TranslateMessage, WM_CLOSE, WM_COMMAND, WM_CREATE, WM_DESTROY,
        WM_PAINT, WM_SETFONT, WM_SETICON, WNDCLASSEXW, WS_CHILD, WS_MINIMIZEBOX, WS_OVERLAPPED,
        WS_SYSMENU, WS_VISIBLE,
    };

    static WINDOW_HWND: AtomicPtr<std::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());

    const BTN_OPEN_DOWNLOADS: usize = 101;
    const BTN_COPY_TOKEN: usize = 102;
    const BTN_OPEN_BROWSER: usize = 103;
    const BTN_MINIMIZE: usize = 104;

    struct UiState {
        config: Arc<RwLock<AppConfig>>,
        qr_rgba: Vec<u8>,
        qr_size: u32,
        pairing_url: String,
        local_ip_str: String,
    }

    static GLOBAL_STATE: OnceLock<UiState> = OnceLock::new();

    pub fn start_ui_thread(config: Arc<RwLock<AppConfig>>) {
        std::thread::spawn(move || {
            if let Err(e) = run_ui_window(config) {
                error!("UI Window error: {}", e);
            }
        });
    }

    pub fn show_window() {
        let hwnd = WINDOW_HWND.load(Ordering::SeqCst) as HWND;
        if !hwnd.is_null() {
            update_language();
            unsafe {
                ShowWindow(hwnd, SW_SHOW);
                ShowWindow(hwnd, SW_RESTORE);
                SetForegroundWindow(hwnd);
            }
        }
    }

    pub fn update_language() {
        let hwnd = WINDOW_HWND.load(Ordering::SeqCst) as HWND;
        if !hwnd.is_null() {
            if let Some(state) = GLOBAL_STATE.get() {
                let lang = state.config.blocking_read().language;
                let strings = crate::i18n::t(lang);

                unsafe {
                    let set_btn = |id: usize, text: &str| {
                        let h_btn = GetDlgItem(hwnd, id as i32);
                        if !h_btn.is_null() {
                            let wide: Vec<u16> = OsStr::new(text)
                                .encode_wide()
                                .chain(std::iter::once(0))
                                .collect();
                            SetWindowTextW(h_btn, wide.as_ptr());
                        }
                    };

                    set_btn(BTN_OPEN_DOWNLOADS, strings.ui_btn_downloads);
                    set_btn(BTN_COPY_TOKEN, strings.ui_btn_token);
                    set_btn(BTN_OPEN_BROWSER, strings.ui_btn_browser);
                    set_btn(BTN_MINIMIZE, strings.ui_btn_minimize);

                    InvalidateRect(hwnd, std::ptr::null(), 1);
                }
            }
        }
    }

    pub fn hide_window() {
        let hwnd = WINDOW_HWND.load(Ordering::SeqCst) as HWND;
        if !hwnd.is_null() {
            unsafe {
                ShowWindow(hwnd, SW_HIDE);
            }
        }
    }

    pub fn close_window() {
        let hwnd = WINDOW_HWND.load(Ordering::SeqCst) as HWND;
        if !hwnd.is_null() {
            unsafe {
                DestroyWindow(hwnd);
            }
        }
    }

    fn run_ui_window(
        config: Arc<RwLock<AppConfig>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let conf = config.blocking_read();
        let local_ip_str = local_ip()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|_| "127.0.0.1".to_string());
        let pairing_url = QrManager::pairing_url(&local_ip_str, conf.port, &conf.token);

        // Generate QR code bitmap
        let qr_code = qrcode::QrCode::new(pairing_url.as_bytes())
            .map_err(|e| format!("QR Generation failed: {}", e))?;
        let qr_image = qr_code
            .render::<image::Rgba<u8>>()
            .max_dimensions(240, 240)
            .build();
        let qr_size = qr_image.width();
        let qr_rgba = qr_image.into_raw();

        drop(conf);

        let _ = GLOBAL_STATE.set(UiState {
            config,
            qr_rgba,
            qr_size,
            pairing_url,
            local_ip_str,
        });

        let class_name: Vec<u16> = OsStr::new("AirDropBridgeUIWindow")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let window_title: Vec<u16> = OsStr::new("AirDrop Bridge")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        unsafe {
            let h_instance =
                windows_sys::Win32::System::LibraryLoader::GetModuleHandleW(std::ptr::null());

            let bg_brush = CreateSolidBrush(0x00170F); // #0f172a in BGR

            let cx_icon = GetSystemMetrics(SM_CXICON);
            let cy_icon = GetSystemMetrics(SM_CYICON);
            let cx_smicon = GetSystemMetrics(SM_CXSMICON);
            let cy_smicon = GetSystemMetrics(SM_CYSMICON);

            let mut h_icon_big = LoadImageW(
                h_instance,
                1 as *const u16,
                IMAGE_ICON,
                cx_icon,
                cy_icon,
                LR_DEFAULTCOLOR,
            );
            let mut h_icon_small = LoadImageW(
                h_instance,
                1 as *const u16,
                IMAGE_ICON,
                cx_smicon,
                cy_smicon,
                LR_DEFAULTCOLOR,
            );

            // Fallback: load directly from embedded PNG bytes if resource isn't found
            let png_bytes = include_bytes!("../resources/icon.png");
            if h_icon_big.is_null() {
                h_icon_big = CreateIconFromResourceEx(
                    png_bytes.as_ptr() as *mut u8,
                    png_bytes.len() as u32,
                    1,
                    0x00030000,
                    cx_icon,
                    cy_icon,
                    LR_DEFAULTCOLOR,
                );
            }
            if h_icon_small.is_null() {
                h_icon_small = CreateIconFromResourceEx(
                    png_bytes.as_ptr() as *mut u8,
                    png_bytes.len() as u32,
                    1,
                    0x00030000,
                    cx_smicon,
                    cy_smicon,
                    LR_DEFAULTCOLOR,
                );
            }

            let final_icon_big = if !h_icon_big.is_null() {
                h_icon_big
            } else {
                LoadIconW(0 as _, IDI_APPLICATION)
            };
            let final_icon_small = if !h_icon_small.is_null() {
                h_icon_small
            } else {
                final_icon_big
            };

            let wnd_class = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: 0,
                lpfnWndProc: Some(wnd_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: h_instance,
                hIcon: final_icon_big,
                hCursor: std::ptr::null_mut(),
                hbrBackground: bg_brush,
                lpszMenuName: std::ptr::null(),
                lpszClassName: class_name.as_ptr(),
                hIconSm: final_icon_small,
            };

            RegisterClassExW(&wnd_class);

            // Window size 480 x 620 centered
            let win_w = 480;
            let win_h = 620;
            let screen_w = windows_sys::Win32::UI::WindowsAndMessaging::GetSystemMetrics(0);
            let screen_h = windows_sys::Win32::UI::WindowsAndMessaging::GetSystemMetrics(1);
            let win_x = (screen_w - win_w) / 2;
            let win_y = (screen_h - win_h) / 2;

            let hwnd = CreateWindowExW(
                0,
                class_name.as_ptr(),
                window_title.as_ptr(),
                WS_OVERLAPPED | WS_SYSMENU | WS_MINIMIZEBOX,
                win_x,
                win_y,
                win_w,
                win_h,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                h_instance,
                std::ptr::null_mut(),
            );

            if hwnd.is_null() {
                return Err("Failed to create UI window".into());
            }

            // Set big and small window icons explicitly (for Taskbar, Alt-Tab, and Titlebar)
            if !final_icon_big.is_null() {
                SendMessageW(hwnd, WM_SETICON, ICON_BIG as _, final_icon_big as _);
            }
            if !final_icon_small.is_null() {
                SendMessageW(hwnd, WM_SETICON, ICON_SMALL as _, final_icon_small as _);
            }

            // Enable Windows 10/11 Immersive Dark Mode for title bar
            let dark_mode: i32 = 1;
            DwmSetWindowAttribute(
                hwnd,
                DWMWA_USE_IMMERSIVE_DARK_MODE as u32,
                &dark_mode as *const _ as *const _,
                std::mem::size_of::<i32>() as u32,
            );

            WINDOW_HWND.store(hwnd as *mut _, Ordering::SeqCst);

            // Start initially visible so the user immediately sees the interface
            ShowWindow(hwnd, SW_SHOWNORMAL);

            let mut msg: MSG = std::mem::zeroed();
            while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }

        Ok(())
    }

    unsafe extern "system" fn wnd_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            match msg {
                WM_CREATE => {
                    let h_instance = windows_sys::Win32::System::LibraryLoader::GetModuleHandleW(
                        std::ptr::null(),
                    );

                    let btn_class: Vec<u16> = OsStr::new("BUTTON")
                        .encode_wide()
                        .chain(std::iter::once(0))
                        .collect();

                    let font = CreateFontW(
                        16,
                        0,
                        0,
                        0,
                        FW_NORMAL as i32,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        OsStr::new("Segoe UI")
                            .encode_wide()
                            .chain(std::iter::once(0))
                            .collect::<Vec<_>>()
                            .as_ptr(),
                    );

                    let lang = GLOBAL_STATE
                        .get()
                        .map(|s| s.config.blocking_read().language)
                        .unwrap_or_default();
                    let strings = crate::i18n::t(lang);

                    // Button 1: Open Downloads
                    let txt_dl = OsStr::new(strings.ui_btn_downloads)
                        .encode_wide()
                        .chain(std::iter::once(0))
                        .collect::<Vec<_>>();
                    let btn1 = CreateWindowExW(
                        0,
                        btn_class.as_ptr(),
                        txt_dl.as_ptr(),
                        WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON as u32,
                        40,
                        470,
                        180,
                        38,
                        hwnd,
                        BTN_OPEN_DOWNLOADS as HMENU,
                        h_instance,
                        std::ptr::null_mut(),
                    );
                    SendMessageW(btn1, WM_SETFONT, font as WPARAM, 1);

                    // Button 2: Copy Token
                    let txt_token = OsStr::new(strings.ui_btn_token)
                        .encode_wide()
                        .chain(std::iter::once(0))
                        .collect::<Vec<_>>();
                    let btn2 = CreateWindowExW(
                        0,
                        btn_class.as_ptr(),
                        txt_token.as_ptr(),
                        WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON as u32,
                        240,
                        470,
                        180,
                        38,
                        hwnd,
                        BTN_COPY_TOKEN as HMENU,
                        h_instance,
                        std::ptr::null_mut(),
                    );
                    SendMessageW(btn2, WM_SETFONT, font as WPARAM, 1);

                    // Button 3: Open in browser
                    let txt_web = OsStr::new(strings.ui_btn_browser)
                        .encode_wide()
                        .chain(std::iter::once(0))
                        .collect::<Vec<_>>();
                    let btn3 = CreateWindowExW(
                        0,
                        btn_class.as_ptr(),
                        txt_web.as_ptr(),
                        WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON as u32,
                        40,
                        520,
                        180,
                        38,
                        hwnd,
                        BTN_OPEN_BROWSER as HMENU,
                        h_instance,
                        std::ptr::null_mut(),
                    );
                    SendMessageW(btn3, WM_SETFONT, font as WPARAM, 1);

                    // Button 4: Minimize / Hide to tray
                    let txt_hide = OsStr::new(strings.ui_btn_minimize)
                        .encode_wide()
                        .chain(std::iter::once(0))
                        .collect::<Vec<_>>();
                    let btn4 = CreateWindowExW(
                        0,
                        btn_class.as_ptr(),
                        txt_hide.as_ptr(),
                        WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON as u32,
                        240,
                        520,
                        180,
                        38,
                        hwnd,
                        BTN_MINIMIZE as HMENU,
                        h_instance,
                        std::ptr::null_mut(),
                    );
                    SendMessageW(btn4, WM_SETFONT, font as WPARAM, 1);

                    0
                }

                WM_COMMAND => {
                    let id = wparam;
                    if let Some(state) = GLOBAL_STATE.get() {
                        match id {
                            BTN_OPEN_DOWNLOADS => {
                                let conf = state.config.blocking_read();
                                NotificationManager::open_directory(&conf.download_dir);
                            }
                            BTN_COPY_TOKEN => {
                                let conf = state.config.blocking_read();
                                let _ = ClipboardManager::set_text(&conf.token);
                                let strings = crate::i18n::t(conf.language);
                                NotificationManager::notify_message(
                                    strings.ui_title,
                                    strings.token_copied,
                                );
                            }
                            BTN_OPEN_BROWSER => {
                                NotificationManager::open_directory(&std::path::PathBuf::from(
                                    &state.pairing_url,
                                ));
                            }
                            BTN_MINIMIZE => {
                                ShowWindow(hwnd, SW_HIDE);
                            }
                            _ => {}
                        }
                    }
                    0
                }

                WM_PAINT => {
                    let mut ps: PAINTSTRUCT = std::mem::zeroed();
                    let hdc = BeginPaint(hwnd, &mut ps);

                    let mut rect: RECT = std::mem::zeroed();
                    GetClientRect(hwnd, &mut rect);

                    // Fill background: dark slate #0f172a (BGR: 0x002A170F)
                    let bg_brush = CreateSolidBrush(0x002a170f);
                    FillRect(hdc, &rect, bg_brush);
                    DeleteObject(bg_brush as _);

                    SetBkMode(hdc, TRANSPARENT as i32);

                    // Draw Header Title
                    let font_title = CreateFontW(
                        26,
                        0,
                        0,
                        0,
                        FW_BOLD as i32,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        OsStr::new("Segoe UI")
                            .encode_wide()
                            .chain(std::iter::once(0))
                            .collect::<Vec<_>>()
                            .as_ptr(),
                    );
                    let old_font = SelectObject(hdc, font_title as _);

                    let lang = GLOBAL_STATE
                        .get()
                        .map(|s| s.config.blocking_read().language)
                        .unwrap_or_default();
                    let strings = crate::i18n::t(lang);

                    SetTextColor(hdc, 0x00f8fafc); // White
                    let title_text = OsStr::new(strings.ui_title)
                        .encode_wide()
                        .collect::<Vec<_>>();
                    TextOutW(hdc, 40, 24, title_text.as_ptr(), title_text.len() as i32);

                    // Draw Status & Device Name
                    let font_sub = CreateFontW(
                        15,
                        0,
                        0,
                        0,
                        FW_NORMAL as i32,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        OsStr::new("Segoe UI")
                            .encode_wide()
                            .chain(std::iter::once(0))
                            .collect::<Vec<_>>()
                            .as_ptr(),
                    );
                    SelectObject(hdc, font_sub as _);

                    SetTextColor(hdc, 0x0081b910); // Emerald Green in BGR
                    let status_text = OsStr::new(strings.ui_status_online)
                        .encode_wide()
                        .collect::<Vec<_>>();
                    TextOutW(hdc, 40, 60, status_text.as_ptr(), status_text.len() as i32);

                    if let Some(state) = GLOBAL_STATE.get() {
                        SetTextColor(hdc, 0x00b8a394); // Muted grey in BGR (#94a3b8)
                        let info_text = format!("{} {}:53317", strings.ui_host_prefix, state.local_ip_str);
                        let info_wide = OsStr::new(&info_text).encode_wide().collect::<Vec<_>>();
                        TextOutW(hdc, 40, 84, info_wide.as_ptr(), info_wide.len() as i32);

                        let scan_tip = OsStr::new(strings.ui_scan_tip)
                            .encode_wide()
                            .collect::<Vec<_>>();
                        SetTextColor(hdc, 0x00f8fafc);
                        TextOutW(hdc, 40, 120, scan_tip.as_ptr(), scan_tip.len() as i32);

                        // Draw White background container for QR code
                        let qr_x = (rect.right - 240) / 2;
                        let qr_y = 155;
                        let qr_rect = RECT {
                            left: qr_x - 12,
                            top: qr_y - 12,
                            right: qr_x + 240 + 12,
                            bottom: qr_y + 240 + 12,
                        };
                        let white_brush = CreateSolidBrush(0x00ffffff);
                        FillRect(hdc, &qr_rect, white_brush);
                        DeleteObject(white_brush as _);

                        // Render QR Code onto HDC using StretchDIBits
                        // Convert RGBA to 32-bit BGRA for Win32 GDI
                        let mut bgra = Vec::with_capacity(state.qr_rgba.len());
                        for chunk in state.qr_rgba.chunks(4) {
                            bgra.push(chunk[2]); // B
                            bgra.push(chunk[1]); // G
                            bgra.push(chunk[0]); // R
                            bgra.push(chunk[3]); // A
                        }

                        let mut bmi: BITMAPINFO = std::mem::zeroed();
                        bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
                        bmi.bmiHeader.biWidth = state.qr_size as i32;
                        bmi.bmiHeader.biHeight = -(state.qr_size as i32); // Negative for top-down DIB
                        bmi.bmiHeader.biPlanes = 1;
                        bmi.bmiHeader.biBitCount = 32;
                        bmi.bmiHeader.biCompression = BI_RGB;

                        StretchDIBits(
                            hdc,
                            qr_x,
                            qr_y,
                            240,
                            240,
                            0,
                            0,
                            state.qr_size as i32,
                            state.qr_size as i32,
                            bgra.as_ptr() as _,
                            &bmi,
                            DIB_RGB_COLORS,
                            SRCCOPY,
                        );
                    }

                    // Instructions footer
                    let hint = OsStr::new(strings.ui_footer_hint)
                        .encode_wide()
                        .collect::<Vec<_>>();
                    SetTextColor(hdc, 0x00b8a394);
                    TextOutW(hdc, 60, 420, hint.as_ptr(), hint.len() as i32);

                    SelectObject(hdc, old_font);
                    DeleteObject(font_title as _);
                    DeleteObject(font_sub as _);

                    EndPaint(hwnd, &ps);
                    0
                }

                WM_CLOSE => {
                    // Intercept closing: hide to system tray instead of exiting!
                    ShowWindow(hwnd, SW_HIDE);
                    0
                }

                WM_DESTROY => {
                    WINDOW_HWND.store(std::ptr::null_mut(), Ordering::SeqCst);
                    PostQuitMessage(0);
                    0
                }

                _ => DefWindowProcW(hwnd, msg, wparam, lparam),
            }
        }
    }
}

#[cfg(not(windows))]
pub mod win_ui {
    use crate::config::AppConfig;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    pub fn start_ui_thread(_config: Arc<RwLock<AppConfig>>) {}
    pub fn show_window() {}
    pub fn hide_window() {}
    pub fn close_window() {}
    pub fn update_language() {}
}
