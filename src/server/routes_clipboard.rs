use crate::clipboard::ClipboardManager;
use crate::server::{AppState, ServerEvent};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Deserialize, Serialize)]
pub struct ClipboardPayload {
    #[serde(rename = "type")]
    pub payload_type: String, // "text" or "image"
    pub content: String,
}

pub async fn set_clipboard_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Result<Json<ClipboardPayload>, axum::extract::rejection::JsonRejection>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // If sent as JSON
    if let Ok(Json(payload)) = body {
        match payload.payload_type.as_str() {
            "text" => {
                ClipboardManager::set_text(&payload.content).map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ "error": "ClipboardError", "message": e })),
                    )
                })?;

                let summary = if payload.content.chars().count() > 50 {
                    format!(
                        "{}...",
                        payload.content.chars().take(47).collect::<String>()
                    )
                } else {
                    payload.content.clone()
                };

                let _ = state.event_tx.send(ServerEvent::ClipboardReceived {
                    content_type: "text".to_string(),
                    summary,
                });

                return Ok(Json(json!({
                    "status": "success",
                    "type": "text",
                    "length": payload.content.len()
                })));
            }
            "image" => {
                ClipboardManager::set_image_from_base64(&payload.content).map_err(|e| {
                    (
                        StatusCode::BAD_REQUEST,
                        Json(json!({ "error": "ImageDecodeError", "message": e })),
                    )
                })?;

                let _ = state.event_tx.send(ServerEvent::ClipboardReceived {
                    content_type: "image".to_string(),
                    summary: "Image received from iOS".to_string(),
                });

                return Ok(Json(json!({
                    "status": "success",
                    "type": "image"
                })));
            }
            other => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "InvalidType",
                        "message": format!("Unsupported clipboard type: {}. Use 'text' or 'image'", other)
                    })),
                ));
            }
        }
    }

    // Check if Content-Type was raw
    let content_type = headers
        .get("content-type")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    Err((
        StatusCode::BAD_REQUEST,
        Json(json!({
            "error": "BadRequest",
            "message": format!("Invalid JSON body or unsupported Content-Type: {}", content_type)
        })),
    ))
}

pub async fn get_clipboard_handler(
    State(_state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Check image first
    match ClipboardManager::get_image() {
        Ok(Some(img)) => {
            return Ok(Json(json!({
                "status": "success",
                "type": "image",
                "content": img.png_base64,
                "width": img.width,
                "height": img.height
            })));
        }
        Ok(None) => {}
        Err(e) => {
            tracing::warn!("Failed to query image clipboard: {}", e);
        }
    }

    // Check text
    match ClipboardManager::get_text() {
        Ok(Some(text)) => Ok(Json(json!({
            "status": "success",
            "type": "text",
            "content": text
        }))),
        Ok(None) => Ok(Json(json!({
            "status": "success",
            "type": "empty",
            "content": ""
        }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "ClipboardError",
                "message": e
            })),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use crate::server::create_router;
    use axum::body::{Body, to_bytes};
    use axum::http::Request;
    use std::path::PathBuf;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_clipboard_endpoints_with_valid_token() {
        let config = AppConfig {
            device_name: "TestPC".to_string(),
            port: 53317,
            token: "my-valid-token-123".to_string(),
            download_dir: PathBuf::from("./downloads"),
            auto_update: false,
            autostart: false,
            shortcut_url: None,
        };
        let (state, mut rx) = AppState::new(config);
        let router = create_router(state);

        // POST text to clipboard
        let payload = json!({
            "type": "text",
            "content": "Hello from iOS Shortcut!"
        });

        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/clipboard")
            .header("Authorization", "Bearer my-valid-token-123")
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_vec(&payload).unwrap()))
            .unwrap();

        let resp = router.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Verify ServerEvent was broadcast
        let event = rx.try_recv().unwrap();
        match event {
            ServerEvent::ClipboardReceived {
                content_type,
                summary,
            } => {
                assert_eq!(content_type, "text");
                assert!(summary.contains("Hello from iOS"));
            }
            _ => panic!("Expected ClipboardReceived event"),
        }

        // GET text from clipboard
        let req_get = Request::builder()
            .method("GET")
            .uri("/api/v1/clipboard")
            .header("Authorization", "Bearer my-valid-token-123")
            .body(Body::empty())
            .unwrap();

        let resp_get = router.oneshot(req_get).await.unwrap();
        assert_eq!(resp_get.status(), StatusCode::OK);

        let body_bytes = to_bytes(resp_get.into_body(), usize::MAX).await.unwrap();
        let json_val: Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(json_val["status"], "success");
    }
}
