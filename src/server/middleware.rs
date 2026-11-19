use crate::server::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub async fn auth_middleware(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, Response> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(header_val) if header_val.starts_with("Bearer ") => {
            header_val.trim_start_matches("Bearer ").trim()
        }
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                axum::Json(json!({
                    "error": "Unauthorized",
                    "message": "Missing or malformed Authorization header. Expected: Bearer <token>"
                })),
            )
                .into_response());
        }
    };

    let expected_token = {
        let conf = state.config.read().await;
        conf.token.clone()
    };

    if token != expected_token {
        return Err((
            StatusCode::UNAUTHORIZED,
            axum::Json(json!({
                "error": "Unauthorized",
                "message": "Invalid token"
            })),
        )
            .into_response());
    }

    Ok(next.run(req).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use crate::server::create_router;
    use axum::body::to_bytes;
    use axum::http::Request;
    use std::path::PathBuf;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_auth_middleware_rejects_missing_token() {
        let config = AppConfig {
            device_name: "TestPC".to_string(),
            port: 53317,
            token: "secret123".to_string(),
            download_dir: PathBuf::from("./downloads"),
            auto_update: false,
            autostart: false,
        };
        let (state, _rx) = AppState::new(config);
        let router = create_router(state);

        let req = Request::builder()
            .method("GET")
            .uri("/api/v1/clipboard")
            .body(Body::empty())
            .unwrap();

        let resp = router.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_ping_endpoint_is_public() {
        let config = AppConfig {
            device_name: "TestPC".to_string(),
            port: 53317,
            token: "secret123".to_string(),
            download_dir: PathBuf::from("./downloads"),
            auto_update: false,
            autostart: false,
        };
        let (state, _rx) = AppState::new(config);
        let router = create_router(state);

        let req = Request::builder()
            .method("GET")
            .uri("/api/v1/ping")
            .body(Body::empty())
            .unwrap();

        let resp = router.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let body_bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let json_val: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(json_val["status"], "ok");
        assert_eq!(json_val["device_name"], "TestPC");
    }
}
