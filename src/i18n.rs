use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    En,
    Es,
    Zh,
    Pt,
    Fr,
    De,
    It,
    Ja,
}

impl Language {
    pub const ALL: &'static [Language] = &[
        Language::En,
        Language::Es,
        Language::Zh,
        Language::Pt,
        Language::Fr,
        Language::De,
        Language::It,
        Language::Ja,
    ];

    pub fn code(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Es => "es",
            Language::Zh => "zh",
            Language::Pt => "pt",
            Language::Fr => "fr",
            Language::De => "de",
            Language::It => "it",
            Language::Ja => "ja",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::En => "English",
            Language::Es => "Español",
            Language::Zh => "简体中文",
            Language::Pt => "Português",
            Language::Fr => "Français",
            Language::De => "Deutsch",
            Language::It => "Italiano",
            Language::Ja => "日本語",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code.to_lowercase().trim() {
            "es" | "es-es" | "es-mx" | "es-ar" | "spanish" => Language::Es,
            "zh" | "zh-cn" | "zh-hans" | "zh-tw" | "zh-hant" | "chinese" => Language::Zh,
            "pt" | "pt-pt" | "pt-br" | "portuguese" => Language::Pt,
            "fr" | "fr-fr" | "fr-ca" | "french" => Language::Fr,
            "de" | "de-de" | "de-at" | "de-ch" | "german" => Language::De,
            "it" | "it-it" | "italian" => Language::It,
            "ja" | "ja-jp" | "japanese" => Language::Ja,
            "en" | "en-us" | "en-gb" | "english" => Language::En,
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
    pub tray_quickshare: &'static str,
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
    pub setup_version: &'static str,
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
    tray_open: "📱 Open Drop Bridge",
    tray_downloads: "📂 Open Downloads",
    tray_copy_token: "📋 Copy Token",
    tray_browser: "🌐 Open in Browser",
    tray_settings: "⚙️ Settings",
    tray_autostart: "Start with Windows",
    tray_notifications: "System Notifications",
    tray_sound: "Sound on Receive",
    tray_open_folder: "Open Folder on Receive",
    tray_clipboard: "Sync Clipboard",
    tray_quickshare: "Quick Share (Android)",
    tray_auto_update: "Automatic Updates",
    tray_language: "🌐 Language",
    tray_check_updates: "🔄 Check for Updates...",
    tray_exit: "❌ Exit",
    tray_tooltip: "Drop Bridge - Online",
    token_copied: "Security token copied to clipboard",

    ui_title: "Drop Bridge",
    ui_status_online: "● Online  -  Ready to receive",
    ui_host_prefix: "Host:",
    ui_scan_tip: "Scan with your iPhone Camera:",
    ui_footer_hint: "The iOS shortcut will auto-configure in one tap.",
    ui_btn_downloads: "Open Downloads",
    ui_btn_token: "Copy Token",
    ui_btn_browser: "Open in Browser",
    ui_btn_minimize: "Minimize to Tray",

    notif_file_received_title: "Drop Bridge - File received",
    notif_clipboard_title: "Drop Bridge - Clipboard",
    notif_copied: "Copied to clipboard:",
    notif_saved_in: "Saved in",

    update_offline: "Could not check for updates (offline)",
    update_latest_prefix: "Drop Bridge is up to date",
    update_silent_done_fmt: "Drop Bridge was silently updated to {}. Changes will apply on next launch.",

    setup_version: "Version:",
    setup_online_badge: "Online",
    setup_connection_with: "Connection with",
    setup_host_mdns: "mDNS Host:",
    setup_lan_ip: "LAN IP:",
    setup_security: "Security:",
    setup_authenticated: "Authenticated ✓",
    setup_pending: "Pending",
    setup_btn_install: "1. Download & Install Drop Bridge Shortcut",
    setup_btn_connect: "2. Link & Configure this PC",
    setup_btn_download_file: "Download .shortcut file directly",
    setup_instructions_title: "Instructions:",
    setup_step1: "1. Tap step 1 to download and install the Drop Bridge Shortcut on your iPhone.",
    setup_step2: "2. Tap step 2 to import the security token and link this PC automatically.",
    setup_step3: "3. Ready! Share photos, videos, or documents directly from the iOS Share Sheet.",
};

pub const ES_TRANSLATIONS: Translations = Translations {
    tray_open: "📱 Abrir Drop Bridge",
    tray_downloads: "📂 Abrir descargas",
    tray_copy_token: "📋 Copiar token",
    tray_browser: "🌐 Abrir en navegador",
    tray_settings: "⚙️ Ajustes",
    tray_autostart: "Iniciar con Windows",
    tray_notifications: "Notificaciones del sistema",
    tray_sound: "Sonido al recibir",
    tray_open_folder: "Abrir carpeta al recibir",
    tray_clipboard: "Sincronizar portapapeles",
    tray_quickshare: "Quick Share (Android)",
    tray_auto_update: "Actualizaciones automáticas",
    tray_language: "🌐 Idioma",
    tray_check_updates: "🔄 Buscar actualizaciones...",
    tray_exit: "❌ Salir",
    tray_tooltip: "Drop Bridge - En línea",
    token_copied: "Token copiado al portapapeles",

    ui_title: "Drop Bridge",
    ui_status_online: "● En línea  -  Listo para recibir",
    ui_host_prefix: "Host:",
    ui_scan_tip: "Escanea con la app Cámara de tu iPhone:",
    ui_footer_hint: "El atajo de iOS se autoconfigurará en un solo toque.",
    ui_btn_downloads: "Abrir Descargas",
    ui_btn_token: "Copiar Token",
    ui_btn_browser: "Abrir en Navegador",
    ui_btn_minimize: "Minimizar a la Bandeja",

    notif_file_received_title: "Drop Bridge - Archivo recibido",
    notif_clipboard_title: "Drop Bridge - Portapapeles",
    notif_copied: "Copiado al portapapeles:",
    notif_saved_in: "Guardado en",

    update_offline: "No se pudo comprobar actualizaciones (sin conexión)",
    update_latest_prefix: "Drop Bridge ya está actualizado",
    update_silent_done_fmt: "Drop Bridge se ha actualizado silenciosamente a {}. Se aplicará en el próximo inicio.",

    setup_version: "Versión:",
    setup_online_badge: "En línea",
    setup_connection_with: "Conexión con",
    setup_host_mdns: "Host mDNS:",
    setup_lan_ip: "IP LAN:",
    setup_security: "Seguridad:",
    setup_authenticated: "Autenticado ✓",
    setup_pending: "Pendiente",
    setup_btn_install: "1. Descargar e Instalar Atajo Drop Bridge",
    setup_btn_connect: "2. Vincular y Configurar este PC",
    setup_btn_download_file: "Descargar archivo .shortcut directamente",
    setup_instructions_title: "Instrucciones:",
    setup_step1: "1. Pulsa en el paso 1 para descargar e instalar el Atajo Drop Bridge en tu iPhone.",
    setup_step2: "2. Pulsa en el paso 2 para importar el token de seguridad y vincular este PC automáticamente.",
    setup_step3: "3. ¡Listo! Ya puedes compartir fotos, vídeos o documentos desde el menú Compartir de iOS.",
};

pub const ZH_TRANSLATIONS: Translations = Translations {
    tray_open: "📱 打开 Drop Bridge",
    tray_downloads: "📂 打开下载文件夹",
    tray_copy_token: "📋 复制令牌",
    tray_browser: "🌐 在浏览器中打开",
    tray_settings: "⚙️ 设置",
    tray_autostart: "开机自启动",
    tray_notifications: "系统通知",
    tray_sound: "接收时播放提示音",
    tray_open_folder: "接收后打开文件夹",
    tray_clipboard: "剪贴板同步",
    tray_quickshare: "Quick Share (Android)",
    tray_auto_update: "自动更新",
    tray_language: "🌐 语言",
    tray_check_updates: "🔄 检查更新...",
    tray_exit: "❌ 退出",
    tray_tooltip: "Drop Bridge - 在线",
    token_copied: "安全令牌已复制到剪贴板",

    ui_title: "Drop Bridge",
    ui_status_online: "● 在线  -  随时准备接收",
    ui_host_prefix: "主机:",
    ui_scan_tip: "使用 iPhone 相机扫描:",
    ui_footer_hint: "iOS 快捷指令将通过单次点击自动完成配置。",
    ui_btn_downloads: "打开下载文件夹",
    ui_btn_token: "复制令牌",
    ui_btn_browser: "在浏览器中打开",
    ui_btn_minimize: "最小化到托盘",

    notif_file_received_title: "Drop Bridge - 已收到文件",
    notif_clipboard_title: "Drop Bridge - 剪贴板",
    notif_copied: "已复制到剪贴板:",
    notif_saved_in: "已保存至",

    update_offline: "无法检查更新（离线）",
    update_latest_prefix: "Drop Bridge 已是最新版本",
    update_silent_done_fmt: "Drop Bridge 已静默更新至 {}。下次启动生效。",

    setup_version: "版本:",
    setup_online_badge: "在线",
    setup_connection_with: "连接至",
    setup_host_mdns: "mDNS 主机:",
    setup_lan_ip: "局域网 IP:",
    setup_security: "安全性:",
    setup_authenticated: "已认证 ✓",
    setup_pending: "待配对",
    setup_btn_install: "1. 下载并安装 Drop Bridge 快捷指令",
    setup_btn_connect: "2. 配对并配置此电脑",
    setup_btn_download_file: "直接下载 .shortcut 文件",
    setup_instructions_title: "使用指南:",
    setup_step1: "1. 轻点步骤 1，在 iPhone 上下载并安装 Drop Bridge 快捷指令。",
    setup_step2: "2. 轻点步骤 2，导入安全令牌并自动与此电脑完成配对。",
    setup_step3: "3. 大功告成！直接通过 iOS 分享面板快速发送照片、视频或各类文件。",
};

pub const PT_TRANSLATIONS: Translations = Translations {
    tray_open: "📱 Abrir Drop Bridge",
    tray_downloads: "📂 Abrir Downloads",
    tray_copy_token: "📋 Copiar Token",
    tray_browser: "🌐 Abrir no Navegador",
    tray_settings: "⚙️ Configurações",
    tray_autostart: "Iniciar com o Windows",
    tray_notifications: "Notificações do Sistema",
    tray_sound: "Som ao Receber",
    tray_open_folder: "Abrir Pasta ao Receber",
    tray_clipboard: "Sincronizar Área de Transferência",
    tray_quickshare: "Quick Share (Android)",
    tray_auto_update: "Atualizações Automáticas",
    tray_language: "🌐 Idioma",
    tray_check_updates: "🔄 Verificar Atualizações...",
    tray_exit: "❌ Sair",
    tray_tooltip: "Drop Bridge - Online",
    token_copied: "Token copiado para a área de transferência",

    ui_title: "Drop Bridge",
    ui_status_online: "● Online  -  Pronto para receber",
    ui_host_prefix: "Host:",
    ui_scan_tip: "Escaneie com a Câmera do seu iPhone:",
    ui_footer_hint: "O atalho do iOS será configurado automaticamente com um toque.",
    ui_btn_downloads: "Abrir Downloads",
    ui_btn_token: "Copiar Token",
    ui_btn_browser: "Abrir no Navegador",
    ui_btn_minimize: "Minimizar para a Bandeja",

    notif_file_received_title: "Drop Bridge - Arquivo recebido",
    notif_clipboard_title: "Drop Bridge - Área de Transferência",
    notif_copied: "Copiado para a área de transferência:",
    notif_saved_in: "Salvo em",

    update_offline: "Não foi possível verificar atualizações (offline)",
    update_latest_prefix: "O Drop Bridge está atualizado",
    update_silent_done_fmt: "O Drop Bridge foi atualizado silenciosamente para {}. As alterações serão aplicadas na próxima inicialização.",

    setup_version: "Versão:",
    setup_online_badge: "Online",
    setup_connection_with: "Conexão com",
    setup_host_mdns: "Host mDNS:",
    setup_lan_ip: "IP LAN:",
    setup_security: "Segurança:",
    setup_authenticated: "Autenticado ✓",
    setup_pending: "Pendente",
    setup_btn_install: "1. Baixar e Instalar Atalho Drop Bridge",
    setup_btn_connect: "2. Vincular e Configurar este PC",
    setup_btn_download_file: "Baixar arquivo .shortcut diretamente",
    setup_instructions_title: "Instruções:",
    setup_step1: "1. Toque no passo 1 para baixar e instalar o Atalho Drop Bridge no seu iPhone.",
    setup_step2: "2. Toque no passo 2 para importar o token de segurança e vincular este PC automaticamente.",
    setup_step3: "3. Pronto! Compartilhe fotos, vídeos ou documentos direto pelo menu Compartilhar do iOS.",
};

pub const FR_TRANSLATIONS: Translations = Translations {
    tray_open: "📱 Ouvrir Drop Bridge",
    tray_downloads: "📂 Ouvrir les Téléchargements",
    tray_copy_token: "📋 Copier le Jeton",
    tray_browser: "🌐 Ouvrir dans le Navigateur",
    tray_settings: "⚙️ Paramètres",
    tray_autostart: "Lancer avec Windows",
    tray_notifications: "Notifications Système",
    tray_sound: "Son à la Réception",
    tray_open_folder: "Ouvrir le Dossier à la Réception",
    tray_clipboard: "Synchroniser le Presse-papiers",
    tray_quickshare: "Quick Share (Android)",
    tray_auto_update: "Mises à Jour Automatiques",
    tray_language: "🌐 Langue",
    tray_check_updates: "🔄 Rechercher des Mises à Jour...",
    tray_exit: "❌ Quitter",
    tray_tooltip: "Drop Bridge - En ligne",
    token_copied: "Jeton de sécurité copié dans le presse-papiers",

    ui_title: "Drop Bridge",
    ui_status_online: "● En ligne  -  Prêt à recevoir",
    ui_host_prefix: "Hôte :",
    ui_scan_tip: "Scannez avec l'Appareil photo de votre iPhone :",
    ui_footer_hint: "Le raccourci iOS se configurera automatiquement en un geste.",
    ui_btn_downloads: "Ouvrir les Téléchargements",
    ui_btn_token: "Copier le Jeton",
    ui_btn_browser: "Ouvrir dans le Navigateur",
    ui_btn_minimize: "Réduire dans la Barre des Tâches",

    notif_file_received_title: "Drop Bridge - Fichier reçu",
    notif_clipboard_title: "Drop Bridge - Presse-papiers",
    notif_copied: "Copié dans le presse-papiers :",
    notif_saved_in: "Enregistré dans",

    update_offline: "Impossible de vérifier les mises à jour (hors ligne)",
    update_latest_prefix: "Drop Bridge est à jour",
    update_silent_done_fmt: "Drop Bridge a été mis à jour silencieusement vers {}. Appliqué au prochain démarrage.",

    setup_version: "Version :",
    setup_online_badge: "En ligne",
    setup_connection_with: "Connexion avec",
    setup_host_mdns: "Hôte mDNS :",
    setup_lan_ip: "IP LAN :",
    setup_security: "Sécurité :",
    setup_authenticated: "Authentifié ✓",
    setup_pending: "En attente",
    setup_btn_install: "1. Télécharger et Installer le Raccourci Drop Bridge",
    setup_btn_connect: "2. Associer et Configurer ce PC",
    setup_btn_download_file: "Télécharger directement le fichier .shortcut",
    setup_instructions_title: "Instructions :",
    setup_step1: "1. Touchez l'étape 1 pour télécharger et installer le raccourci Drop Bridge sur votre iPhone.",
    setup_step2: "2. Touchez l'étape 2 pour importer le jeton de sécurité et associer ce PC automatiquement.",
    setup_step3: "3. Prêt ! Partagez vos photos, vidéos ou documents directement depuis le menu Partager d'iOS.",
};

pub const DE_TRANSLATIONS: Translations = Translations {
    tray_open: "📱 Drop Bridge öffnen",
    tray_downloads: "📂 Downloads öffnen",
    tray_copy_token: "📋 Token kopieren",
    tray_browser: "🌐 Im Browser öffnen",
    tray_settings: "⚙️ Einstellungen",
    tray_autostart: "Mit Windows starten",
    tray_notifications: "Systembenachrichtigungen",
    tray_sound: "Ton beim Empfangen",
    tray_open_folder: "Ordner beim Empfangen öffnen",
    tray_clipboard: "Zwischenablage synchronisieren",
    tray_quickshare: "Quick Share (Android)",
    tray_auto_update: "Automatische Updates",
    tray_language: "🌐 Sprache",
    tray_check_updates: "🔄 Nach Updates suchen...",
    tray_exit: "❌ Beenden",
    tray_tooltip: "Drop Bridge - Online",
    token_copied: "Token in die Zwischenablage kopiert",

    ui_title: "Drop Bridge",
    ui_status_online: "● Online  -  Bereit zum Empfangen",
    ui_host_prefix: "Host:",
    ui_scan_tip: "Mit der iPhone-Kamera scannen:",
    ui_footer_hint: "Der iOS-Kurzbefehl richtet sich mit einem Fingertipp automatisch ein.",
    ui_btn_downloads: "Downloads öffnen",
    ui_btn_token: "Token kopieren",
    ui_btn_browser: "Im Browser öffnen",
    ui_btn_minimize: "In den Infobereich minimieren",

    notif_file_received_title: "Drop Bridge - Datei empfangen",
    notif_clipboard_title: "Drop Bridge - Zwischenablage",
    notif_copied: "In Zwischenablage kopiert:",
    notif_saved_in: "Gespeichert in",

    update_offline: "Updates konnten nicht geprüft werden (offline)",
    update_latest_prefix: "Drop Bridge ist auf dem neuesten Stand",
    update_silent_done_fmt: "Drop Bridge wurde im Hintergrund auf {} aktualisiert. Wird beim nächsten Start wirksam.",

    setup_version: "Version:",
    setup_online_badge: "Online",
    setup_connection_with: "Verbindung mit",
    setup_host_mdns: "mDNS Host:",
    setup_lan_ip: "LAN IP:",
    setup_security: "Sicherheit:",
    setup_authenticated: "Authentifiziert ✓",
    setup_pending: "Ausstehend",
    setup_btn_install: "1. Drop Bridge Kurzbefehl herunterladen & installieren",
    setup_btn_connect: "2. Diesen PC koppeln & konfigurieren",
    setup_btn_download_file: ".shortcut-Datei direkt herunterladen",
    setup_instructions_title: "Anleitung:",
    setup_step1: "1. Tippe auf Schritt 1, um den Drop Bridge Kurzbefehl auf deinem iPhone zu installieren.",
    setup_step2: "2. Tippe auf Schritt 2, um den Token zu importieren und diesen PC automatisch zu koppeln.",
    setup_step3: "3. Fertig! Teile Fotos, Videos oder Dokumente direkt über das iOS-Teilen-Menü.",
};

pub const IT_TRANSLATIONS: Translations = Translations {
    tray_open: "📱 Apri Drop Bridge",
    tray_downloads: "📂 Apri Download",
    tray_copy_token: "📋 Copia Token",
    tray_browser: "🌐 Apri nel Browser",
    tray_settings: "⚙️ Impostazioni",
    tray_autostart: "Avvia con Windows",
    tray_notifications: "Notifiche di Sistema",
    tray_sound: "Suono alla Ricezione",
    tray_open_folder: "Apri Cartella alla Ricezione",
    tray_clipboard: "Sincronizza Appunti",
    tray_quickshare: "Quick Share (Android)",
    tray_auto_update: "Aggiornamenti Automatici",
    tray_language: "🌐 Lingua",
    tray_check_updates: "🔄 Controlla Aggiornamenti...",
    tray_exit: "❌ Esci",
    tray_tooltip: "Drop Bridge - Online",
    token_copied: "Token copiato negli appunti",

    ui_title: "Drop Bridge",
    ui_status_online: "● Online  -  Pronto a ricevere",
    ui_host_prefix: "Host:",
    ui_scan_tip: "Scansiona con la Fotocamera dell'iPhone:",
    ui_footer_hint: "Il comando rapido di iOS si configurerà automaticamente con un tocco.",
    ui_btn_downloads: "Apri Download",
    ui_btn_token: "Copia Token",
    ui_btn_browser: "Apri nel Browser",
    ui_btn_minimize: "Riduci nella Barra delle Applicazioni",

    notif_file_received_title: "Drop Bridge - File ricevuto",
    notif_clipboard_title: "Drop Bridge - Appunti",
    notif_copied: "Copiato negli appunti:",
    notif_saved_in: "Salvato in",

    update_offline: "Impossibile controllare gli aggiornamenti (non in linea)",
    update_latest_prefix: "Drop Bridge è aggiornato",
    update_silent_done_fmt: "Drop Bridge è stato aggiornato in background a {}. Sarà effettivo al prossimo avvio.",

    setup_version: "Versione:",
    setup_online_badge: "Online",
    setup_connection_with: "Connessione con",
    setup_host_mdns: "Host mDNS:",
    setup_lan_ip: "IP LAN:",
    setup_security: "Sicurezza:",
    setup_authenticated: "Autenticato ✓",
    setup_pending: "In attesa",
    setup_btn_install: "1. Scarica e Installa il Comando Drop Bridge",
    setup_btn_connect: "2. Collega e Configura questo PC",
    setup_btn_download_file: "Scarica direttamente il file .shortcut",
    setup_instructions_title: "Istruzioni:",
    setup_step1: "1. Tocca il passaggio 1 per scaricare e installare il comando Drop Bridge sul tuo iPhone.",
    setup_step2: "2. Tocca il passaggio 2 per importare il token di sicurezza e collegare questo PC automaticamente.",
    setup_step3: "3. Fatto! Condividi foto, video o documenti direttamente dal menu di condivisione di iOS.",
};

pub const JA_TRANSLATIONS: Translations = Translations {
    tray_open: "📱 Drop Bridge を開く",
    tray_downloads: "📂 ダウンロードを開く",
    tray_copy_token: "📋 トークンをコピー",
    tray_browser: "🌐 ブラウザで開く",
    tray_settings: "⚙️ 設定",
    tray_autostart: "Windows 起動時に実行",
    tray_notifications: "システム通知",
    tray_sound: "受信時にサウンド再生",
    tray_open_folder: "受信時にフォルダを開く",
    tray_clipboard: "クリップボード同期",
    tray_quickshare: "Quick Share (Android)",
    tray_auto_update: "自動アップデート",
    tray_language: "🌐 言語",
    tray_check_updates: "🔄 アップデートを確認...",
    tray_exit: "❌ 終了",
    tray_tooltip: "Drop Bridge - オンライン",
    token_copied: "トークンをクリップボードにコピーしました",

    ui_title: "Drop Bridge",
    ui_status_online: "● オンライン  -  受信待機中",
    ui_host_prefix: "ホスト:",
    ui_scan_tip: "iPhoneのカメラでスキャン:",
    ui_footer_hint: "iOSショートカットはワンタップで自動設定されます。",
    ui_btn_downloads: "ダウンロードを開く",
    ui_btn_token: "トークンをコピー",
    ui_btn_browser: "ブラウザで開く",
    ui_btn_minimize: "トレイに最小化",

    notif_file_received_title: "Drop Bridge - ファイルを受信しました",
    notif_clipboard_title: "Drop Bridge - クリップボード",
    notif_copied: "クリップボードにコピー:",
    notif_saved_in: "保存先:",

    update_offline: "アップデートを確認できませんでした（オフライン）",
    update_latest_prefix: "Drop Bridge は最新バージョンです",
    update_silent_done_fmt: "Drop Bridge は {} に自動更新されました。次回起動時に反映されます。",

    setup_version: "バージョン:",
    setup_online_badge: "オンライン",
    setup_connection_with: "接続先:",
    setup_host_mdns: "mDNS ホスト:",
    setup_lan_ip: "LAN IP:",
    setup_security: "セキュリティ:",
    setup_authenticated: "認証済み ✓",
    setup_pending: "保留中",
    setup_btn_install: "1. Drop Bridge ショートカットをダウンロード＆インストール",
    setup_btn_connect: "2. このPCと連携・設定する",
    setup_btn_download_file: ".shortcut ファイルを直接ダウンロード",
    setup_instructions_title: "使用手順:",
    setup_step1: "1. ステップ1をタップして、iPhoneにDrop Bridgeショートカットをダウンロード＆インストールします。",
    setup_step2: "2. ステップ2をタップして、セキュリティトークンをインポートし、このPCと自動的に連携します。",
    setup_step3: "3. 完了！iOSの共有シートから写真、動画、書類を直接Windows PCへ送信できます。",
};

pub fn t(lang: Language) -> &'static Translations {
    match lang {
        Language::En => &EN_TRANSLATIONS,
        Language::Es => &ES_TRANSLATIONS,
        Language::Zh => &ZH_TRANSLATIONS,
        Language::Pt => &PT_TRANSLATIONS,
        Language::Fr => &FR_TRANSLATIONS,
        Language::De => &DE_TRANSLATIONS,
        Language::It => &IT_TRANSLATIONS,
        Language::Ja => &JA_TRANSLATIONS,
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
        assert_eq!(Language::Zh.code(), "zh");
        assert_eq!(Language::Pt.code(), "pt");
        assert_eq!(Language::Fr.code(), "fr");
        assert_eq!(Language::De.code(), "de");
        assert_eq!(Language::It.code(), "it");
        assert_eq!(Language::Ja.code(), "ja");

        assert_eq!(Language::En.display_name(), "English");
        assert_eq!(Language::Es.display_name(), "Español");
        assert_eq!(Language::Zh.display_name(), "简体中文");
        assert_eq!(Language::Pt.display_name(), "Português");
        assert_eq!(Language::Fr.display_name(), "Français");
        assert_eq!(Language::De.display_name(), "Deutsch");
        assert_eq!(Language::It.display_name(), "Italiano");
        assert_eq!(Language::Ja.display_name(), "日本語");
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
        assert_eq!(Language::from_code("zh"), Language::Zh);
        assert_eq!(Language::from_code("zh-CN"), Language::Zh);
        assert_eq!(Language::from_code("pt"), Language::Pt);
        assert_eq!(Language::from_code("pt-BR"), Language::Pt);
        assert_eq!(Language::from_code("fr"), Language::Fr);
        assert_eq!(Language::from_code("de"), Language::De);
        assert_eq!(Language::from_code("it"), Language::It);
        assert_eq!(Language::from_code("ja"), Language::Ja);
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
            assert!(!tr.setup_btn_connect.is_empty());
            assert!(!tr.setup_step1.is_empty());
            assert!(!tr.setup_step2.is_empty());
            assert!(!tr.setup_step3.is_empty());
        }
    }

    #[test]
    fn test_serde_roundtrip() {
        for &lang in Language::ALL {
            let serialized = serde_json::to_string(&lang).unwrap();
            let deserialized: Language = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, lang);
        }
    }
}
