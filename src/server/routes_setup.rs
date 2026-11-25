use crate::qr::QrManager;
use crate::server::AppState;
use axum::{
    Json,
    extract::{Query, State},
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::{Value, json};

pub const DEFAULT_SHORTCUT_ICLOUD_URL: &str =
    "https://www.icloud.com/shortcuts/406580108b4e44a48674eb1087bcc9c6";

#[derive(Deserialize)]
pub struct SetupQuery {
    pub token: Option<String>,
    pub lang: Option<String>,
    pub host: Option<String>,
}

pub async fn setup_page_handler(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
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

    let host_param = match &query.host {
        Some(h) => format!("&host={}", urlencoding::encode(h)),
        None => String::new(),
    };

    let lang_links = crate::i18n::Language::ALL
        .iter()
        .map(|&l| {
            let is_sel = l == lang;
            let color = if is_sel { "var(--primary)" } else { "var(--text-muted)" };
            let weight = if is_sel { "700" } else { "400" };
            format!(
                r#"<a href="?lang={code}{token_param}{host_param}" class="lang-link" style="color: {color}; font-weight: {weight};">{name}</a>"#,
                code = l.code(),
                token_param = token_param,
                host_param = host_param,
                color = color,
                weight = weight,
                name = l.display_name(),
            )
        })
        .collect::<Vec<_>>()
        .join(r#"<span class="lang-sep">|</span>"#);

    let header_host = headers
        .get(header::HOST)
        .and_then(|h| h.to_str().ok())
        .map(|h| {
            if let Some((host_part, _)) = h.split_once(':') {
                host_part.to_string()
            } else {
                h.to_string()
            }
        })
        .filter(|h| h != "localhost" && h != "127.0.0.1");

    let effective_host = if let Some(ref q_host) = query.host {
        let trimmed = q_host.trim();
        if !trimmed.is_empty() {
            trimmed.to_string()
        } else {
            conf.resolve_primary_host()
        }
    } else if let Some(ref c_host) = conf.custom_host {
        let trimmed = c_host.trim();
        if !trimmed.is_empty() {
            trimmed.to_string()
        } else {
            header_host.unwrap_or_else(|| conf.resolve_primary_host())
        }
    } else if let Some(h) = header_host {
        h
    } else {
        conf.resolve_primary_host()
    };

    let mdns_host = format!(
        "{}.local",
        conf.device_name.to_lowercase().replace(' ', "-")
    );

    let is_hostname = effective_host.contains('.') && !effective_host.chars().all(|c| c.is_ascii_digit() || c == '.');
    let target_host = if is_hostname {
        effective_host.clone()
    } else {
        mdns_host.clone()
    };

    let shortcut_config = json!({
        "token": conf.token,
        "host": target_host,
        "ip": effective_host,
        "port": conf.port.to_string(),
        "device_name": conf.device_name
    });

    let config_str = shortcut_config.to_string();
    let encoded_config = urlencoding::encode(&config_str);
    let shortcut_deep_link = format!(
        "shortcuts://run-shortcut?name=Drop%20Bridge&input=text&text={}",
        encoded_config
    );

    let shortcut_install_url = conf
        .shortcut_url
        .as_deref()
        .unwrap_or(DEFAULT_SHORTCUT_ICLOUD_URL);

    let available_endpoints = crate::config::AppConfig::get_available_endpoints();
    let mut endpoint_selector = String::new();
    if available_endpoints.len() > 1 || conf.custom_host.is_some() {
        endpoint_selector.push_str(r#"<div style="margin:14px 0;background:var(--surface);padding:10px 14px;border-radius:12px;border:1px solid var(--border);text-align:left;">"#);
        endpoint_selector.push_str(r#"<div style="font-size:11px;font-weight:600;margin-bottom:8px;color:var(--text-muted);text-transform:uppercase;letter-spacing:0.5px;">Network Endpoint (Tailscale / LAN):</div><div style="display:flex;flex-wrap:wrap;gap:6px;">"#);

        for ep in &available_endpoints {
            let is_active = ep.ip == effective_host;
            let bg = if is_active { "var(--primary)" } else { "rgba(255,255,255,0.06)" };
            let fg = if is_active { "#ffffff" } else { "var(--text)" };
            let border = if is_active { "var(--primary)" } else { "var(--border)" };
            let badge = if ep.is_tailscale { " 🔒 Tailscale" } else { "" };
            let href = format!("?host={}{token_param}&lang={lang_code}", urlencoding::encode(&ep.ip), lang_code = lang.code());
            endpoint_selector.push_str(&format!(
                r#"<a href="{href}" style="padding:5px 9px;border-radius:7px;background:{bg};color:{fg};border:1px solid {border};text-decoration:none;font-size:12px;font-weight:500;">{ip}{badge}</a>"#,
                href = href,
                bg = bg,
                fg = fg,
                border = border,
                ip = ep.ip,
                badge = badge
            ));
        }

        if let Some(ref custom) = conf.custom_host {
            let is_active = custom == &effective_host;
            let bg = if is_active { "var(--primary)" } else { "rgba(255,255,255,0.06)" };
            let fg = if is_active { "#ffffff" } else { "var(--text)" };
            let border = if is_active { "var(--primary)" } else { "var(--border)" };
            let href = format!("?host={}{token_param}&lang={lang_code}", urlencoding::encode(custom), lang_code = lang.code());
            endpoint_selector.push_str(&format!(
                r#"<a href="{href}" style="padding:5px 9px;border-radius:7px;background:{bg};color:{fg};border:1px solid {border};text-decoration:none;font-size:12px;font-weight:500;">{custom} (Custom)</a>"#,
                href = href,
                bg = bg,
                fg = fg,
                border = border,
                custom = custom
            ));
        }

        endpoint_selector.push_str(r#"</div></div>"#);
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="{lang_code}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=no">
    <title>Drop Bridge</title>
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
            flex-wrap: wrap;
            gap: 6px 10px;
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
            {lang_links}
        </div>
        <div class="header">
            <div class="badge"><div class="dot"></div> {badge_text} &bull; v{app_version}</div>
            <h1>Drop Bridge</h1>
            <p class="subtitle">{connection_prefix} <strong>{device_name}</strong></p>
        </div>

        <div class="info-box">
            <div class="info-row"><span class="info-label">{label_version}</span> <span class="info-val">v{app_version}</span></div>
            <div class="info-row"><span class="info-label">{label_mdns}</span> <span class="info-val">{target_host}</span></div>
            <div class="info-row"><span class="info-label">{label_lan}</span> <span class="info-val">{effective_host}:{port}</span></div>
            <div class="info-row"><span class="info-label">{label_security}</span> <span class="info-val">{auth_status}</span></div>
        </div>

        {endpoint_selector}

        <a href="{shortcut_install_url}" class="btn btn-primary">
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
        lang_links = lang_links,
        badge_text = strings.setup_online_badge,
        app_version = env!("CARGO_PKG_VERSION"),
        label_version = strings.setup_version,
        connection_prefix = strings.setup_connection_with,
        device_name = conf.device_name,
        label_mdns = strings.setup_host_mdns,
        target_host = target_host,
        label_lan = strings.setup_lan_ip,
        effective_host = effective_host,
        port = conf.port,
        label_security = strings.setup_security,
        auth_status = if is_authorized {
            strings.setup_authenticated
        } else {
            strings.setup_pending
        },
        endpoint_selector = endpoint_selector,
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
    let primary_host = conf.resolve_primary_host();

    let url = QrManager::pairing_url(&primary_host, conf.port, &conf.token);

    Json(json!({
        "device_name": conf.device_name,
        "ip": primary_host,
        "port": conf.port,
        "url": url,
        "token": conf.token,
        "custom_host": conf.custom_host,
        "endpoints": crate::config::AppConfig::get_available_endpoints()
    }))
}

pub async fn qr_image_handler(State(state): State<AppState>) -> Result<Response, StatusCode> {
    let conf = state.config.read().await;
    let primary_host = conf.resolve_primary_host();

    let url = QrManager::pairing_url(&primary_host, conf.port, &conf.token);
    let png_bytes =
        QrManager::render_png(&url, 300).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(([(header::CONTENT_TYPE, "image/png")], png_bytes).into_response())
}

pub async fn shortcut_download_handler() -> impl IntoResponse {
    let bytes = include_bytes!("../../resources/shortcut/Drop_Bridge.shortcut");
    (
        [
            (header::CONTENT_TYPE, "application/x-apple-shortcut"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"Drop Bridge.shortcut\"",
            ),
        ],
        bytes.as_slice(),
    )
}
