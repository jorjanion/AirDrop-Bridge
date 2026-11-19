pub mod middleware;
pub mod routes_clipboard;
pub mod routes_ping;
pub mod routes_setup;
pub mod routes_upload;

use crate::config::AppConfig;
use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, post},
};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[derive(Clone, Debug)]
pub enum ServerEvent {
    FileReceived {
        filename: String,
        path: PathBuf,
        size_bytes: u64,
    },
    ClipboardReceived {
        content_type: String,
        summary: String,
    },
}

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<RwLock<AppConfig>>,
    pub event_tx: broadcast::Sender<ServerEvent>,
}

impl AppState {
    pub fn new(config: AppConfig) -> (Self, broadcast::Receiver<ServerEvent>) {
        let (tx, rx) = broadcast::channel(100);
        let state = Self {
            config: Arc::new(RwLock::new(config)),
            event_tx: tx,
        };
        (state, rx)
    }
}

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/api/v1/ping", get(routes_ping::ping_handler))
        .route("/setup", get(routes_setup::setup_page_handler))
        .route("/setup/pair-url", get(routes_setup::pair_url_handler))
        .route("/setup/qr.png", get(routes_setup::qr_image_handler));

    // Protected API routes (Bearer token required)
    let protected_routes = Router::new()
        .route("/api/v1/upload", post(routes_upload::upload_handler))
        .route(
            "/api/v1/clipboard",
            post(routes_clipboard::set_clipboard_handler)
                .get(routes_clipboard::get_clipboard_handler),
        )
        .layer(from_fn_with_state(state.clone(), middleware::auth_middleware));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
