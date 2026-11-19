use arboard::{Clipboard, ImageData};
use image::{DynamicImage, GenericImageView, ImageFormat};
use std::borrow::Cow;
use std::io::Cursor;
use tracing::{error, info};

pub struct ClipboardManager;

#[derive(Debug, Clone)]
pub struct ClipboardImageResult {
    pub width: usize,
    pub height: usize,
    pub png_base64: String,
}

impl ClipboardManager {
    /// Copies plain text or URL to the system clipboard
    pub fn set_text(text: &str) -> Result<(), String> {
        let mut clipboard =
            Clipboard::new().map_err(|e| format!("Failed to access clipboard: {}", e))?;
        clipboard
            .set_text(text)
            .map_err(|e| format!("Failed to set clipboard text: {}", e))?;
        info!("Set text in OS clipboard ({} chars)", text.len());
        Ok(())
    }

    /// Reads plain text or URL from the system clipboard
    pub fn get_text() -> Result<Option<String>, String> {
        let mut clipboard =
            Clipboard::new().map_err(|e| format!("Failed to access clipboard: {}", e))?;
        match clipboard.get_text() {
            Ok(text) => Ok(Some(text)),
            Err(arboard::Error::ContentNotAvailable) => Ok(None),
            Err(e) => Err(format!("Failed to get clipboard text: {}", e)),
        }
    }

    /// Sets an image in the system clipboard from raw PNG/JPEG/WEBP bytes or decoded RGBA
    pub fn set_image_from_bytes(bytes: &[u8]) -> Result<(), String> {
        let img = image::load_from_memory(bytes)
            .map_err(|e| format!("Failed to decode image from bytes: {}", e))?;
        let rgba = img.to_rgba8();
        let (width, height) = img.dimensions();

        let image_data = ImageData {
            width: width as usize,
            height: height as usize,
            bytes: Cow::Owned(rgba.into_raw()),
        };

        let mut clipboard =
            Clipboard::new().map_err(|e| format!("Failed to access clipboard: {}", e))?;
        clipboard
            .set_image(image_data)
            .map_err(|e| format!("Failed to set clipboard image: {}", e))?;

        info!("Set image in OS clipboard ({}x{})", width, height);
        Ok(())
    }

    /// Sets an image in the system clipboard from a Base64-encoded string
    pub fn set_image_from_base64(b64_str: &str) -> Result<(), String> {
        let clean_b64 = if let Some(idx) = b64_str.find(";base64,") {
            &b64_str[idx + 8..]
        } else {
            b64_str
        };

        let decoded =
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, clean_b64.trim())
                .map_err(|e| format!("Failed to decode base64 image: {}", e))?;

        Self::set_image_from_bytes(&decoded)
    }

    /// Reads an image from the system clipboard and converts it to PNG Base64
    pub fn get_image() -> Result<Option<ClipboardImageResult>, String> {
        let mut clipboard =
            Clipboard::new().map_err(|e| format!("Failed to access clipboard: {}", e))?;
        match clipboard.get_image() {
            Ok(img_data) => {
                let rgba_image = image::RgbaImage::from_raw(
                    img_data.width as u32,
                    img_data.height as u32,
                    img_data.bytes.into_owned(),
                )
                .ok_or_else(|| "Failed to construct RGBA image from clipboard bytes".to_string())?;

                let dynamic_img = DynamicImage::ImageRgba8(rgba_image);
                let mut png_bytes = Vec::new();
                let mut cursor = Cursor::new(&mut png_bytes);

                dynamic_img
                    .write_to(&mut cursor, ImageFormat::Png)
                    .map_err(|e| format!("Failed to encode image to PNG: {}", e))?;

                let b64 =
                    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &png_bytes);

                Ok(Some(ClipboardImageResult {
                    width: img_data.width,
                    height: img_data.height,
                    png_base64: format!("data:image/png;base64,{}", b64),
                }))
            }
            Err(arboard::Error::ContentNotAvailable) => Ok(None),
            Err(e) => {
                error!("Error getting image from clipboard: {}", e);
                Ok(None)
            }
        }
    }
}
