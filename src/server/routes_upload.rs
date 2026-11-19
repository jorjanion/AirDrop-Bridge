use crate::server::{AppState, ServerEvent};
use axum::{
    Json,
    extract::{Multipart, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

#[derive(Serialize, Deserialize)]
pub struct UploadedFileResult {
    pub filename: String,
    pub path: String,
    pub size_bytes: u64,
}

pub async fn upload_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let download_dir = {
        let conf = state.config.read().await;
        conf.download_dir.clone()
    };

    if let Err(e) = tokio::fs::create_dir_all(&download_dir).await {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "StorageError",
                "message": format!("Failed to create download directory: {}", e)
            })),
        ));
    }

    let mut results = Vec::new();

    while let Some(mut field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "MultipartError",
                "message": format!("Failed to read multipart stream: {}", e)
            })),
        )
    })? {
        let raw_filename = field
            .file_name()
            .unwrap_or("received_file")
            .to_string();

        let safe_name = sanitize_filename::sanitize(&raw_filename);
        let safe_name = if safe_name.trim().is_empty() {
            "file.bin".to_string()
        } else {
            safe_name
        };

        let target_path = resolve_unique_path(&download_dir, &safe_name);
        let temp_filename = format!("{}.part_{}", safe_name, rand::random::<u32>());
        let temp_path = download_dir.join(&temp_filename);

        let mut file = tokio::fs::File::create(&temp_path).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "DiskWriteError",
                    "message": format!("Failed to create destination file: {}", e)
                })),
            )
        })?;

        let mut total_bytes = 0u64;

        // Stream chunks directly from network to disk (O(1) memory usage)
        while let Some(chunk) = field.chunk().await.map_err(|e| {
            // Clean up temporary file on stream error
            let _ = std::fs::remove_file(&temp_path);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "StreamReadError",
                    "message": format!("Network error while reading stream: {}", e)
                })),
            )
        })? {
            if let Err(e) = file.write_all(&chunk).await {
                let _ = std::fs::remove_file(&temp_path);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": "DiskWriteError",
                        "message": format!("Error writing chunk to disk: {}", e)
                    })),
                ));
            }
            total_bytes += chunk.len() as u64;
        }

        if let Err(e) = file.flush().await {
            let _ = std::fs::remove_file(&temp_path);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "FlushError",
                    "message": format!("Failed to flush file to disk: {}", e)
                })),
            ));
        }

        drop(file);

        // Atomic rename from .part to final path
        if let Err(e) = tokio::fs::rename(&temp_path, &target_path).await {
            let _ = std::fs::remove_file(&temp_path);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "AtomicRenameError",
                    "message": format!("Failed to finalize file: {}", e)
                })),
            ));
        }

        let final_filename = target_path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or(&safe_name)
            .to_string();

        // Emit notification event
        let _ = state.event_tx.send(ServerEvent::FileReceived {
            filename: final_filename.clone(),
            path: target_path.clone(),
            size_bytes: total_bytes,
        });

        results.push(UploadedFileResult {
            filename: final_filename,
            path: target_path.to_string_lossy().to_string(),
            size_bytes: total_bytes,
        });
    }

    Ok(Json(json!({
        "status": "success",
        "received_count": results.len(),
        "files": results
    })))
}

/// Resolves a non-colliding destination path if a file already exists
pub fn resolve_unique_path(dir: &Path, filename: &str) -> PathBuf {
    let path = dir.join(filename);
    if !path.exists() {
        return path;
    }

    let stem = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file");
    let ext = Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| format!(".{}", e))
        .unwrap_or_default();

    let mut counter = 1;
    loop {
        let candidate = dir.join(format!("{} ({}){}", stem, counter, ext));
        if !candidate.exists() {
            return candidate;
        }
        counter += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_unique_path_non_existent() {
        let temp_dir = std::env::temp_dir();
        let unique_name = format!("test_file_{}.txt", rand::random::<u32>());
        let resolved = resolve_unique_path(&temp_dir, &unique_name);
        assert_eq!(resolved, temp_dir.join(&unique_name));
    }
}
