# AirDrop Bridge

AirDrop Bridge is a lightweight local network daemon that allows Apple devices (iPhone and iPad using iOS Shortcuts) to send and receive files, photos, and clipboard contents to and from Windows and Linux computers.

It requires no third-party mobile apps, no account creation, and no cloud relay. It runs as a background process with a system tray menu, using mDNS for discovery and streaming uploads directly to disk.

## Overview

AirDrop Bridge acts as a local HTTP receiver that integrates directly with iOS Shortcuts through the native iOS Share Sheet, Action Button, or Back Tap.

Key characteristics:
* Idle memory footprint under 15 MB RAM (typically around 8 to 14 MB).
* Negligible CPU usage when idle.
* O(1) memory disk streaming for uploads. Large video files and photo libraries are written directly to disk in chunks rather than buffered in RAM.
* Dual-address discovery: Advertises via mDNS (_dropbridge._tcp.local) while also providing local IP fallback for Wi-Fi networks with multicast isolation.
* Local Bearer token authorization to block unauthorized requests on public or shared Wi-Fi networks.
* Native system tray icon with dark and light theme support.
* Windows interactive toast notifications with options to open the received file or containing folder.
* Native audio chime on file arrival.
* Multi-language support with 8 built-in languages (English, Español, 简体中文, Português, Français, Deutsch, Italiano, 日本語).
* In-place background updater backed by GitHub Releases.

## Architecture

The mobile client is an Apple Shortcut that runs inside iOS. When invoked from the Share Sheet or Action Button, it determines whether the input is a file or clipboard data, resolves the computer address, and sends an authenticated HTTP request to the desktop daemon.

```
+---------------------------------+
| iPhone / iPad (Apple Shortcut)  |
+---------------------------------+
               |
               | HTTP Streaming + Bearer Token
               v
+---------------------------------+
| AirDrop Bridge Daemon (Rust)    |
|   - Axum HTTP Server            |
|   - mDNS Responder (DNS-SD)     |
|   - Chunk Streamer to Disk      |
|   - Native Clipboard (arboard)  |
|   - Windows Toast Notifications |
|   - System Tray Context Menu    |
|   - Self-Updater Engine         |
+---------------------------------+
```

## Quick Start

### Installation

Download the precompiled binary from GitHub Releases:
* Windows: Download airdrop-bridge-windows-x64.exe and run it directly.
* Linux: Download airdrop-bridge-linux-x64.tar.gz and run the binary.

To build from source:
```bash
git clone https://github.com/jorjanion/airdrop-bridge.git
cd airdrop-bridge
cargo build --release
```
The compiled binary will be located at target/release/airdrop-bridge.exe (or target/release/airdrop-bridge on Linux).

### Pairing with iOS

1. Run the AirDrop Bridge binary. It starts in your system tray.
2. Right-click the tray icon and select "Pair new device (QR)".
3. Open the Camera app on your iPhone or iPad and scan the QR code.
4. Safari opens a local setup page served by the daemon.
5. Tap "Install Shortcut" (or install directly via the [Official iCloud Shortcut](https://www.icloud.com/shortcuts/406580108b4e44a48674eb1087bcc9c6)) to add the shortcut to your device.
6. Tap "Configure" to save the local token and connection settings into the shortcut automatically.

Once configured, select AirDrop Bridge from the iOS Share Sheet whenever you share photos, videos, or documents.

## API Reference

All protected endpoints require the HTTP header:
`Authorization: Bearer <token>`

### GET /api/v1/ping
Connectivity and health check. Does not require authentication.
* Response: JSON with status, device name, and version.

### POST /api/v1/upload
Streams multipart file payloads directly to the downloads directory (Downloads/AirDropBridge).
* Payload: multipart/form-data.
* Behavior: Incoming data is written to a temporary .part file and renamed upon completion to avoid partial transfers.

### POST /api/v1/clipboard
Sets the host system clipboard.
* Content-Type: application/json
* Body format:
```json
{
  "type": "text",
  "content": "Copied string"
}
```
For images, type is set to "image" with base64-encoded image data.

### GET /api/v1/clipboard
Retrieves the current host clipboard content.
* Response format:
```json
{
  "status": "success",
  "type": "text",
  "content": "Clipboard text content"
}
```

### GET /setup
Local onboarding portal served directly by the daemon to pair new iOS devices via Safari.

## Configuration

Settings are saved in the user configuration directory:
* Windows: %APPDATA%\AirDropBridge\config.json
* Linux: ~/.config/AirDropBridge/config.json

Example configuration:
```json
{
  "device_name": "Desktop-PC",
  "port": 53317,
  "token": "d8f94a...",
  "download_dir": "C:\\Users\\user\\Downloads\\AirDropBridge",
  "auto_update": true,
  "autostart": false,
  "language": "en"
}
```

## License

Licensed under either of:
* Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (http://opensource.org/licenses/MIT)
at your option.
