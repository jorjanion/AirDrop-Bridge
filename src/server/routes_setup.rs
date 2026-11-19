use crate::server::AppState;
use axum::{
    Json,
    extract::State,
    response::{Html, IntoResponse},
};
use serde_json::{Value, json};

pub async fn setup_page_handler(State(_state): State<AppState>) -> impl IntoResponse {
    Html("<html><body>AirDrop Bridge Setup</body></html>")
}

pub async fn pair_url_handler(State(_state): State<AppState>) -> Json<Value> {
    Json(json!({ "pair_url": "" }))
}
