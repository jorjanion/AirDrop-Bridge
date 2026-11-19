use image::Luma;
use qrcode::QrCode;
use std::io::Cursor;

pub struct QrManager;

impl QrManager {
    /// Generates pairing setup URL
    pub fn pairing_url(ip: &str, port: u16, token: &str) -> String {
        format!("http://{}:{}/setup?token={}", ip, port, token)
    }

    /// Renders QR code as terminal ASCII string
    pub fn render_terminal(content: &str) -> Result<String, String> {
        let code = QrCode::new(content.as_bytes())
            .map_err(|e| format!("Failed to generate QR code: {}", e))?;
        Ok(code
            .render::<qrcode::render::unicode::Dense1x2>()
            .dark_color(qrcode::render::unicode::Dense1x2::Dark)
            .light_color(qrcode::render::unicode::Dense1x2::Light)
            .build())
    }

    /// Renders QR code to PNG byte buffer
    pub fn render_png(content: &str, width_px: u32) -> Result<Vec<u8>, String> {
        let code = QrCode::new(content.as_bytes())
            .map_err(|e| format!("Failed to generate QR code: {}", e))?;

        let image = code
            .render::<Luma<u8>>()
            .max_dimensions(width_px, width_px)
            .build();

        let mut png_bytes = Vec::new();
        let mut cursor = Cursor::new(&mut png_bytes);

        image
            .write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|e| format!("Failed to encode QR PNG: {}", e))?;

        Ok(png_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairing_url_format() {
        let url = QrManager::pairing_url("192.168.1.100", 53317, "abc123token");
        assert_eq!(url, "http://192.168.1.100:53317/setup?token=abc123token");
    }

    #[test]
    fn test_qr_png_generation() {
        let png = QrManager::render_png("http://localhost:53317", 256).unwrap();
        assert!(!png.is_empty());
        assert_eq!(&png[1..4], b"PNG");
    }
}
