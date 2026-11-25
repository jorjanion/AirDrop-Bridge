use crate::server::AppState;
use axum::{Json, extract::State};
use serde_json::{Value, json};

pub async fn ping_handler(State(state): State<AppState>) -> Json<Value> {
    let device_name = {
        let conf = state.config.read().await;
        conf.device_name.clone()
    };

    Json(json!({
        "status": "ok",
        "app": "DropBridge",
        "version": env!("CARGO_PKG_VERSION"),
        "device_name": device_name
    }))
}
