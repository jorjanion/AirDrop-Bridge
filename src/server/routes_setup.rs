use crate::qr::QrManager;
use crate::server::AppState;
use axum::{
    Json,
    extract::{Query, State},
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
};
use local_ip_address::local_ip;
use serde::Deserialize;
use serde_json::{Value, json};

pub const DEFAULT_SHORTCUT_ICLOUD_URL: &str =
    "https://www.icloud.com/shortcuts/406580108b4e44a48674eb1087bcc9c6";

#[derive(Deserialize)]
pub struct SetupQuery {
    pub token: Option<String>,
    pub lang: Option<String>,
}

pub async fn setup_page_handler(
    State(state): State<AppState>,
    Query(query): Query<SetupQuery>,
) -> impl IntoResponse {
    let conf = state.config.read().await;

    let lang = query
        .lang
        .as_deref()
        .map(crate::i18n::Language::from_code)
        .unwrap_or(conf.language);
    let strings = crate::i18n::t(lang);

    // Validate token if provided or warn
    let is_authorized = match &query.token {
        Some(t) => t == &conf.token,
        None => false,
    };

    let token_param = match &query.token {
        Some(t) => format!("&token={}", urlencoding::encode(t)),
        None => String::new(),
    };

    let is_en = lang == crate::i18n::Language::En;
    let en_color = if is_en { "var(--primary)" } else { "var(--text-muted)" };
    let en_weight = if is_en { "700" } else { "400" };
    let es_color = if !is_en { "var(--primary)" } else { "var(--text-muted)" };
    let es_weight = if !is_en { "700" } else { "400" };

    let local_ip_str = local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    let mdns_host = format!(
        "{}.local",
        conf.device_name.to_lowercase().replace(' ', "-")
    );

    let shortcut_config = json!({
        "token": conf.token,
        "host": mdns_host,
        "ip": local_ip_str,
        "port": conf.port.to_string(),
        "device_name": conf.device_name
    });

    let config_str = shortcut_config.to_string();
    let encoded_config = urlencoding::encode(&config_str);
    let shortcut_deep_link = format!(
        "shortcuts://run-shortcut?name=AirDrop%20Bridge&input=text&text={}",
        encoded_config
    );

    let shortcut_install_url = conf
        .shortcut_url
        .as_deref()
        .unwrap_or(DEFAULT_SHORTCUT_ICLOUD_URL);

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="{lang_code}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=no">
    <title>AirDrop Bridge</title>
    <style>
        :root {{
            --bg: #0f172a;
            --card-bg: #1e293b;
            --text: #f8fafc;
            --text-muted: #94a3b8;
            --primary: #38bdf8;
            --primary-hover: #0284c7;
            --accent: #10b981;
            --border: #334155;
        }}
        @media (prefers-color-scheme: light) {{
            :root {{
                --bg: #f1f5f9;
                --card-bg: #ffffff;
                --text: #0f172a;
                --text-muted: #64748b;
                --primary: #0284c7;
                --primary-hover: #0369a1;
                --accent: #059669;
                --border: #e2e8f0;
            }}
        }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Segoe UI", Roboto, sans-serif;
            background: var(--bg);
            color: var(--text);
            margin: 0;
            padding: 24px 16px;
            display: flex;
            flex-direction: column;
            align-items: center;
            min-height: 100vh;
            box-sizing: border-box;
        }}
        .card {{
            background: var(--card-bg);
            border: 1px solid var(--border);
            border-radius: 24px;
            padding: 28px 24px;
            max-width: 440px;
            width: 100%;
            box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 8px 10px -6px rgba(0, 0, 0, 0.1);
        }}
        .lang-picker {{
            display: flex;
            justify-content: flex-end;
            align-items: center;
            gap: 10px;
            font-size: 13px;
            margin-bottom: 12px;
        }}
        .lang-link {{
            text-decoration: none;
            transition: color 0.15s ease;
        }}
        .lang-sep {{
            color: var(--border);
        }}
        .header {{
            text-align: center;
            margin-bottom: 24px;
        }}
        .badge {{
            display: inline-flex;
            align-items: center;
            gap: 6px;
            background: rgba(16, 185, 129, 0.15);
            color: var(--accent);
            padding: 4px 12px;
            border-radius: 9999px;
            font-size: 13px;
            font-weight: 600;
            margin-bottom: 12px;
        }}
        .dot {{
            width: 8px;
            height: 8px;
            background: var(--accent);
            border-radius: 50%;
        }}
        h1 {{
            font-size: 26px;
            margin: 0 0 6px 0;
            font-weight: 700;
        }}
        p.subtitle {{
            color: var(--text-muted);
            font-size: 15px;
            margin: 0;
        }}
        .info-box {{
            background: rgba(56, 189, 248, 0.08);
            border: 1px solid rgba(56, 189, 248, 0.2);
            border-radius: 16px;
            padding: 16px;
            margin-bottom: 24px;
            font-size: 14px;
        }}
        .info-row {{
            display: flex;
            justify-content: space-between;
            margin-bottom: 6px;
        }}
        .info-row:last-child {{
            margin-bottom: 0;
        }}
        .info-label {{
            color: var(--text-muted);
        }}
        .info-val {{
            font-weight: 600;
            font-family: ui-monospace, Menlo, Consolas, monospace;
        }}
        .btn {{
            display: block;
            width: 100%;
            padding: 14px 18px;
            border-radius: 14px;
            font-size: 16px;
            font-weight: 600;
            text-align: center;
            text-decoration: none;
            box-sizing: border-box;
            margin-bottom: 12px;
            transition: all 0.2s ease;
            cursor: pointer;
            border: none;
        }}
        .btn-primary {{
            background: var(--primary);
            color: #ffffff;
        }}
        .btn-primary:active {{
            transform: scale(0.98);
            background: var(--primary-hover);
        }}
        .btn-secondary {{
            background: rgba(148, 163, 184, 0.15);
            color: var(--text);
        }}
        .btn-secondary:active {{
            transform: scale(0.98);
        }}
        .btn-outline {{
            background: transparent;
            color: var(--primary);
            border: 1px solid var(--border);
            font-size: 14px;
            padding: 10px 14px;
        }}
        .btn-outline:active {{
            transform: scale(0.98);
            background: rgba(56, 189, 248, 0.1);
        }}
        .instructions {{
            margin-top: 20px;
            border-top: 1px solid var(--border);
            padding-top: 16px;
            font-size: 13px;
            color: var(--text-muted);
            line-height: 1.5;
        }}
    </style>
</head>
<body>
    <div class="card">
        <div class="lang-picker">
            <a href="?lang=en{token_param}" class="lang-link" style="color: {en_color}; font-weight: {en_weight};">English</a>
            <span class="lang-sep">|</span>
            <a href="?lang=es{token_param}" class="lang-link" style="color: {es_color}; font-weight: {es_weight};">Español</a>
        </div>
        <div class="header">
            <div class="badge"><div class="dot"></div> {badge_text} &bull; v{app_version}</div>
            <h1>AirDrop Bridge</h1>
            <p class="subtitle">{connection_prefix} <strong>{device_name}</strong></p>
        </div>

        <div class="info-box">
            <div class="info-row"><span class="info-label">{label_version}</span> <span class="info-val">v{app_version}</span></div>
            <div class="info-row"><span class="info-label">{label_mdns}</span> <span class="info-val">{mdns_host}</span></div>
            <div class="info-row"><span class="info-label">{label_lan}</span> <span class="info-val">{local_ip_str}:{port}</span></div>
            <div class="info-row"><span class="info-label">{label_security}</span> <span class="info-val">{auth_status}</span></div>
        </div>

        <a href="{shortcut_install_url}" class="btn btn-primary" target="_blank" rel="noopener">
            {btn_install}
        </a>

        <a href="{shortcut_deep_link}" class="btn btn-secondary">
            {btn_connect}
        </a>

        <div class="instructions">
            <strong>{instructions_title}</strong><br>
            {step1}<br>
            {step2}<br>
            {step3}
        </div>
    </div>
</body>
</html>"#,
        lang_code = lang.code(),
        token_param = token_param,
        en_color = en_color,
        en_weight = en_weight,
        es_color = es_color,
        es_weight = es_weight,
        badge_text = strings.setup_online_badge,
        app_version = env!("CARGO_PKG_VERSION"),
        label_version = strings.setup_version,
        connection_prefix = strings.setup_connection_with,
        device_name = conf.device_name,
        label_mdns = strings.setup_host_mdns,
        mdns_host = mdns_host,
        label_lan = strings.setup_lan_ip,
        local_ip_str = local_ip_str,
        port = conf.port,
        label_security = strings.setup_security,
        auth_status = if is_authorized {
            strings.setup_authenticated
        } else {
            strings.setup_pending
        },
        shortcut_install_url = shortcut_install_url,
        btn_install = strings.setup_btn_install,
        shortcut_deep_link = shortcut_deep_link,
        btn_connect = strings.setup_btn_connect,
        instructions_title = strings.setup_instructions_title,
        step1 = strings.setup_step1,
        step2 = strings.setup_step2,
        step3 = strings.setup_step3
    );

    Html(html)
}

pub async fn pair_url_handler(State(state): State<AppState>) -> Json<Value> {
    let conf = state.config.read().await;
    let local_ip_str = local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string());

    let url = QrManager::pairing_url(&local_ip_str, conf.port, &conf.token);

    Json(json!({
        "device_name": conf.device_name,
        "ip": local_ip_str,
        "port": conf.port,
        "url": url,
        "token": conf.token
    }))
}

pub async fn qr_image_handler(State(state): State<AppState>) -> Result<Response, StatusCode> {
    let conf = state.config.read().await;
    let local_ip_str = local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string());

    let url = QrManager::pairing_url(&local_ip_str, conf.port, &conf.token);
    let png_bytes =
        QrManager::render_png(&url, 300).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(([(header::CONTENT_TYPE, "image/png")], png_bytes).into_response())
}

pub async fn shortcut_download_handler() -> impl IntoResponse {
    let bytes = include_bytes!("../../resources/shortcut/AirDrop_Bridge.shortcut");
    (
        [
            (header::CONTENT_TYPE, "application/x-apple-shortcut"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"AirDrop Bridge.shortcut\"",
            ),
        ],
        bytes.as_slice(),
    )
}
