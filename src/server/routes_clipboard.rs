use crate::server::AppState;
use axum::{Json, extract::State};
use serde_json::{Value, json};

pub async fn set_clipboard_handler(State(_state): State<AppState>) -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

pub async fn get_clipboard_handler(State(_state): State<AppState>) -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
