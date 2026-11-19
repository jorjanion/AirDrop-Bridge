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

#[derive(Deserialize)]
pub struct SetupQuery {
    pub token: Option<String>,
}

pub async fn setup_page_handler(
    State(state): State<AppState>,
    Query(query): Query<SetupQuery>,
) -> impl IntoResponse {
    let conf = state.config.read().await;

    // Validate token if provided or warn
    let is_authorized = match &query.token {
        Some(t) => t == &conf.token,
        None => false,
    };

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
        "port": conf.port,
        "device_name": conf.device_name
    });

    let config_str = shortcut_config.to_string();
    let encoded_config = urlencoding::encode(&config_str);
    let shortcut_deep_link = format!(
        "shortcuts://run-shortcut?name=AirDrop%20Bridge&input=text&text={}",
        encoded_config
    );

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=no">
    <title>AirDrop Bridge - Emparejamiento</title>
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
        <div class="header">
            <div class="badge"><div class="dot"></div> En línea</div>
            <h1>AirDrop Bridge</h1>
            <p class="subtitle">Conexión con <strong>{device_name}</strong></p>
        </div>

        <div class="info-box">
            <div class="info-row"><span class="info-label">Host mDNS:</span> <span class="info-val">{mdns_host}</span></div>
            <div class="info-row"><span class="info-label">IP LAN:</span> <span class="info-val">{local_ip_str}:{port}</span></div>
            <div class="info-row"><span class="info-label">Seguridad:</span> <span class="info-val">{auth_status}</span></div>
        </div>

        <a href="https://www.icloud.com/shortcuts/" class="btn btn-secondary">
            1. Descargar Atajo de iOS
        </a>

        <a href="{shortcut_deep_link}" class="btn btn-primary">
            2. Conectar y Configurar este PC
        </a>

        <div class="instructions">
            <strong>Instrucciones:</strong><br>
            1. Pulsa en el paso 1 si aún no tienes el Atajo instalado.<br>
            2. Pulsa en el paso 2 para que el Atajo importe automáticamente el token de seguridad y la dirección de este ordenador.
        </div>
    </div>
</body>
</html>"#,
        device_name = conf.device_name,
        mdns_host = mdns_host,
        local_ip_str = local_ip_str,
        port = conf.port,
        auth_status = if is_authorized {
            "Autenticado ✓"
        } else {
            "Pendiente"
        },
        shortcut_deep_link = shortcut_deep_link
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
