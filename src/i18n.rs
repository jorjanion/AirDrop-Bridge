use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    En,
    Es,
}

impl Language {
    pub const ALL: &'static [Language] = &[Language::En, Language::Es];

    pub fn code(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Es => "es",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::En => "English",
            Language::Es => "Español",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code.to_lowercase().trim() {
            "es" | "es-es" | "es-mx" | "es-ar" | "spanish" => Language::Es,
            _ => Language::En,
        }
    }
}

pub struct Translations {
    // Tray Menu
    pub tray_open: &'static str,
    pub tray_downloads: &'static str,
    pub tray_copy_token: &'static str,
    pub tray_browser: &'static str,
    pub tray_settings: &'static str,
    pub tray_autostart: &'static str,
    pub tray_notifications: &'static str,
    pub tray_sound: &'static str,
    pub tray_open_folder: &'static str,
    pub tray_clipboard: &'static str,
    pub tray_auto_update: &'static str,
    pub tray_language: &'static str,
    pub tray_check_updates: &'static str,
    pub tray_exit: &'static str,
    pub tray_tooltip: &'static str,
    pub token_copied: &'static str,

    // UI Window
    pub ui_title: &'static str,
    pub ui_status_online: &'static str,
    pub ui_host_prefix: &'static str,
    pub ui_scan_tip: &'static str,
    pub ui_footer_hint: &'static str,
    pub ui_btn_downloads: &'static str,
    pub ui_btn_token: &'static str,
    pub ui_btn_browser: &'static str,
    pub ui_btn_minimize: &'static str,

    // Notifications
    pub notif_file_received_title: &'static str,
    pub notif_clipboard_title: &'static str,
    pub notif_copied: &'static str,
    pub notif_saved_in: &'static str,

    // Updater
    pub update_offline: &'static str,
    pub update_latest_prefix: &'static str,
    pub update_silent_done_fmt: &'static str,

    // Web Setup
    pub setup_online_badge: &'static str,
    pub setup_connection_with: &'static str,
    pub setup_host_mdns: &'static str,
    pub setup_lan_ip: &'static str,
    pub setup_security: &'static str,
    pub setup_authenticated: &'static str,
    pub setup_pending: &'static str,
    pub setup_btn_install: &'static str,
    pub setup_btn_connect: &'static str,
    pub setup_btn_download_file: &'static str,
    pub setup_instructions_title: &'static str,
    pub setup_step1: &'static str,
    pub setup_step2: &'static str,
    pub setup_step3: &'static str,
}

pub const EN_TRANSLATIONS: Translations = Translations {
    tray_open: "📱 Open AirDrop Bridge",
    tray_downloads: "📂 Open Downloads",
    tray_copy_token: "📋 Copy Token",
    tray_browser: "🌐 Open in Browser",
    tray_settings: "⚙️ Settings",
    tray_autostart: "Start with Windows",
    tray_notifications: "System Notifications",
    tray_sound: "Sound on Receive",
    tray_open_folder: "Open Folder on Receive",
    tray_clipboard: "Sync Clipboard",
    tray_auto_update: "Automatic Updates",
    tray_language: "🌐 Language",
    tray_check_updates: "🔄 Check for Updates...",
    tray_exit: "❌ Exit",
    tray_tooltip: "AirDrop Bridge - Online",
    token_copied: "Security token copied to clipboard",

    ui_title: "AirDrop Bridge",
    ui_status_online: "● Online  -  Ready to receive",
    ui_host_prefix: "Host:",
    ui_scan_tip: "Scan with your iPhone Camera:",
    ui_footer_hint: "The iOS shortcut will auto-configure in one tap.",
    ui_btn_downloads: "Open Downloads",
    ui_btn_token: "Copy Token",
    ui_btn_browser: "Open in Browser",
    ui_btn_minimize: "Minimize to Tray",

    notif_file_received_title: "AirDrop Bridge - File received",
    notif_clipboard_title: "AirDrop Bridge - Clipboard",
    notif_copied: "Copied to clipboard:",
    notif_saved_in: "Saved in",

    update_offline: "Could not check for updates (offline)",
    update_latest_prefix: "AirDrop Bridge is up to date",
    update_silent_done_fmt: "AirDrop Bridge was silently updated to {}. Changes will apply on next launch.",

    setup_online_badge: "Online",
    setup_connection_with: "Connection with",
    setup_host_mdns: "mDNS Host:",
    setup_lan_ip: "LAN IP:",
    setup_security: "Security:",
    setup_authenticated: "Authenticated ✓",
    setup_pending: "Pending",
    setup_btn_install: "1. Download & Install AirDrop Bridge Shortcut",
    setup_btn_connect: "2. Link & Configure this PC",
    setup_btn_download_file: "Download .shortcut file directly",
    setup_instructions_title: "Instructions:",
    setup_step1: "1. Tap step 1 to download and install the AirDrop Bridge Shortcut on your iPhone.",
    setup_step2: "2. Tap step 2 to import the security token and link this PC automatically.",
    setup_step3: "3. Ready! Share photos, videos, or documents directly from the iOS Share Sheet.",
};

pub const ES_TRANSLATIONS: Translations = Translations {
    tray_open: "📱 Abrir AirDrop Bridge",
    tray_downloads: "📂 Abrir descargas",
    tray_copy_token: "📋 Copiar token",
    tray_browser: "🌐 Abrir en navegador",
    tray_settings: "⚙️ Ajustes",
    tray_autostart: "Iniciar con Windows",
    tray_notifications: "Notificaciones del sistema",
    tray_sound: "Sonido al recibir",
    tray_open_folder: "Abrir carpeta al recibir",
    tray_clipboard: "Sincronizar portapapeles",
    tray_auto_update: "Actualizaciones automáticas",
    tray_language: "🌐 Idioma",
    tray_check_updates: "🔄 Buscar actualizaciones...",
    tray_exit: "❌ Salir",
    tray_tooltip: "AirDrop Bridge - En línea",
    token_copied: "Token copiado al portapapeles",

    ui_title: "AirDrop Bridge",
    ui_status_online: "● En línea  -  Listo para recibir",
    ui_host_prefix: "Host:",
    ui_scan_tip: "Escanea con la app Cámara de tu iPhone:",
    ui_footer_hint: "El atajo de iOS se autoconfigurará en un solo toque.",
    ui_btn_downloads: "Abrir Descargas",
    ui_btn_token: "Copiar Token",
    ui_btn_browser: "Abrir en Navegador",
    ui_btn_minimize: "Minimizar a la Bandeja",

    notif_file_received_title: "AirDrop Bridge - Archivo recibido",
    notif_clipboard_title: "AirDrop Bridge - Portapapeles",
    notif_copied: "Copiado al portapapeles:",
    notif_saved_in: "Guardado en",

    update_offline: "No se pudo comprobar actualizaciones (sin conexión)",
    update_latest_prefix: "AirDrop Bridge ya está actualizado",
    update_silent_done_fmt: "AirDrop Bridge se ha actualizado silenciosamente a {}. Se aplicará en el próximo inicio.",

    setup_online_badge: "En línea",
    setup_connection_with: "Conexión con",
    setup_host_mdns: "Host mDNS:",
    setup_lan_ip: "IP LAN:",
    setup_security: "Seguridad:",
    setup_authenticated: "Autenticado ✓",
    setup_pending: "Pendiente",
    setup_btn_install: "1. Descargar e Instalar Atajo AirDrop Bridge",
    setup_btn_connect: "2. Vincular y Configurar este PC",
    setup_btn_download_file: "Descargar archivo .shortcut directamente",
    setup_instructions_title: "Instrucciones:",
    setup_step1: "1. Pulsa en el paso 1 para descargar e instalar el Atajo AirDrop Bridge en tu iPhone.",
    setup_step2: "2. Pulsa en el paso 2 para importar el token de seguridad y vincular este PC automáticamente.",
    setup_step3: "3. ¡Listo! Ya puedes compartir fotos, vídeos o documentos desde el menú Compartir de iOS.",
};

pub fn t(lang: Language) -> &'static Translations {
    match lang {
        Language::En => &EN_TRANSLATIONS,
        Language::Es => &ES_TRANSLATIONS,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_language_is_en() {
        assert_eq!(Language::default(), Language::En);
    }

    #[test]
    fn test_language_codes_and_display() {
        assert_eq!(Language::En.code(), "en");
        assert_eq!(Language::Es.code(), "es");
        assert_eq!(Language::En.display_name(), "English");
        assert_eq!(Language::Es.display_name(), "Español");
    }

    #[test]
    fn test_from_code_parsing() {
        assert_eq!(Language::from_code("en"), Language::En);
        assert_eq!(Language::from_code("EN"), Language::En);
        assert_eq!(Language::from_code("es"), Language::Es);
        assert_eq!(Language::from_code("ES"), Language::Es);
        assert_eq!(Language::from_code("es-ES"), Language::Es);
        assert_eq!(Language::from_code("es-MX"), Language::Es);
        assert_eq!(Language::from_code("spanish"), Language::Es);
        assert_eq!(Language::from_code("fr"), Language::En); // fallback to English
        assert_eq!(Language::from_code("unknown"), Language::En);
    }

    #[test]
    fn test_translations_coverage() {
        for &lang in Language::ALL {
            let tr = t(lang);
            assert!(!tr.tray_open.is_empty());
            assert!(!tr.ui_title.is_empty());
            assert!(!tr.notif_file_received_title.is_empty());
            assert!(!tr.update_offline.is_empty());
            assert!(!tr.setup_btn_install.is_empty());
        }
    }

    #[test]
    fn test_serde_roundtrip() {
        let serialized_en = serde_json::to_string(&Language::En).unwrap();
        assert_eq!(serialized_en, "\"en\"");
        let deserialized_en: Language = serde_json::from_str(&serialized_en).unwrap();
        assert_eq!(deserialized_en, Language::En);

        let serialized_es = serde_json::to_string(&Language::Es).unwrap();
        assert_eq!(serialized_es, "\"es\"");
        let deserialized_es: Language = serde_json::from_str(&serialized_es).unwrap();
        assert_eq!(deserialized_es, Language::Es);
    }
}
