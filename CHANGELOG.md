# Changelog

All notable changes to DropBridge will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.1] - 2026-11-28

### Fixed
* Fixed immediate startup panic on Linux (including X11 / AppImage) when initializing the system tray menu (`GTK has not been initialized. Call gtk::init first.`) by calling `gtk::init()` on the tray thread and pumping the GTK event loop in `src/tray.rs` (resolves GitHub Issue #4).

## [1.2.0] - 2026-11-27

### Added
* **Native Android Quick Share (Nearby Share) support** (resolves GitHub Issue #1):
  * Integrated `rqs_lib` behind the `quickshare` Cargo feature (enabled by default; opt-out via `--no-default-features`).
  * Added `QuickShareManager` (`src/quickshare/mod.rs`) bridging incoming Quick Share file and text/URL transfers into DropBridge's `ServerEvent`, `NotificationManager`, and `ClipboardManager` pipelines.
  * Added native consent prompts with sender name & PIN verification (`MessageBoxW` on Windows, `zenity`/`kdialog` on Linux), plus optional `quickshare_auto_accept` in `AppConfig`.
  * Added `quickshare_enabled`, `quickshare_visibility`, and `quickshare_auto_accept` backwards-compatible settings to `AppConfig`.
  * Added **Quick Share (Android)** visibility toggle in the System Tray menu (`src/tray.rs`) and localized labels across all 8 languages (`src/i18n.rs`).
  * Added cross-platform `crates/sys_metrics` hostname shim so `rqs_lib` compiles natively on both Windows and Linux/macOS.
  * Updated `.github/workflows/release.yml` to install `protoc` on Windows and Linux release runners.

### Changed
* Project rebranded to **DropBridge** (`drop-bridge` / `Drop Bridge`):
  * Renamed repository and all project links to `DropBridge`.
  * Renamed binary executable to `drop-bridge.exe` (Windows) and `drop-bridge` (Linux).
  * Renamed Linux packaging artifacts to `drop-bridge.desktop`, `DropBridge-x86_64.AppImage`, and `drop-bridge-linux-x64.tar.gz`.
  * Updated official Apple Shortcut name to `Drop Bridge` with storage path `DropBridge.json`.
  * Updated autostart registry entries, window classes (`DropBridgeUIWindow`), and system notifications (`DropBridge`).

## [1.1.0] - 2026-11-23

### Added
* Support for Linux AppImage packaging (`AirDrop-Bridge-x86_64.AppImage`) (resolves GitHub Issue #3).
* Automated AppImage generation pipeline in GitHub Actions release workflow (`.github/workflows/release.yml`) and build script (`scripts/build-appimage.sh`).
* Desktop metadata specification (`resources/linux/airdrop-bridge.desktop`) and 512x512 app icon branding.
* Full AppImage support in Linux autostart manager (`src/autostart.rs`) and silent auto-updater (`src/updater.rs`), automatically preserving execute permissions across updates.
* Support for Tailscale, WireGuard, and custom VPN/FQDN connections (resolves GitHub Issue #2).
* Network interface autodetection: automatically identifies LAN IPv4, Tailscale CGNAT interfaces (`100.64.0.0/10`), and active VPN adapters.
* Dynamic endpoint selector on the web setup portal (`/setup`): lets users switch between LAN and Tailscale/VPN hosts with 1 click, generating matching QR codes and shortcut configuration payloads.
* Windows System Tray "Network Interface" menu: switch between Auto (LAN) and detected Tailscale/VPN IPs without restarting the app.
* Native Win32 UI display of active network type and dynamic QR code refresh when switching endpoints.
* Added `custom_host` configuration parameter to `AppConfig` for persistent custom IP or domain names.

## [1.0.0] - 2026-11-22

### Added
* Official 1.0.0 General Availability release for public distribution.
* Comprehensive multi-language documentation with native READMEs translated into 8 languages: English, Español, 简体中文, Português, Français, Deutsch, Italiano, and 日本語 (`docs/translations/`).
* High-resolution UI screenshots and visual walkthrough guide in `assets/screenshots/`.
* Quick-action language navigation bar at the top of every documentation page.

### Fixed
* Fixed millisecond console/CMD window flash on Windows when receiving files, syncing clipboard, or checking updates by enforcing `CREATE_NO_WINDOW (0x08000000)` process flags.

## [0.3.3] - 2026-11-22

### Added
* Support for 6 additional major languages:
  * Simplified Chinese (`zh` / 简体中文)
  * Portuguese (`pt` / Português)
  * French (`fr` / Français)
  * German (`de` / Deutsch)
  * Italian (`it` / Italiano)
  * Japanese (`ja` / 日本語)
* Dynamic, responsive language picker on the web onboarding portal (`/setup`) automatically rendering all supported languages.
* Full localization across the Win32 GUI window, System Tray context menu, and desktop toast notifications.

### Changed
* Official Apple Shortcut installation link updated to direct same-window Universal Link navigation, triggering iOS native *"Open this page in 'Shortcuts'?"* prompt immediately with zero download steps.

## [0.3.2] - 2026-11-22

### Added
* Visible running application version display across all interfaces:
  * System Tray menu header: `AirDrop Bridge vX.Y.Z (:PORT)`.
  * Native Win32 QR pairing window: Titlebar and emerald green version badge under QR code.
  * Web onboarding portal: Header status badge and information card.
* Integrated official iCloud Shortcut link signed by Apple (`https://www.icloud.com/shortcuts/406580108b4e44a48674eb1087bcc9c6`).

## [0.3.1] - 2026-11-22

### Added
* Official native Apple Shortcut (`AirDrop Bridge`) built from scratch with zero third-party dependencies, clean English strings, and official AirDrop glyph.
* Direct integration with `/api/v1/upload` (multipart streaming) and `/api/v1/clipboard` (text/image synchronization).
* Automated 1-tap configuration mechanism via `shortcuts://run-shortcut?name=AirDrop%20Bridge&input=text&text=...` saving settings directly to `AirDropBridge.json`.
* Direct shortcut installer button in Safari with `application/x-apple-shortcut` MIME type for instant iOS installation.
* Included Python generator script (`resources/shortcut/create_shortcut.py`) to reproducibly build the binary `.shortcut` plist.

### Fixed
* Removed all legacy third-party shortcut artifacts, Chinese texts, and mismatched endpoints (`/file/send`).
* Fixed shortcut name mismatch ("AirDrop Plus" vs "AirDrop Bridge") preventing iOS configuration linking.
* Fixed iOS 404/401 errors by aligning shortcut HTTP requests with standard Bearer token authorization and server endpoints.

## [0.3.0] - 2026-11-22

### Added
* Full internationalization (i18n) support with English (`en`) as the default system language and dynamic runtime switching to Spanish (`es`).
* Dynamic language selection in the System Tray menu (`Settings -> Language`) and on the web onboarding portal (`/setup?lang=en|es`).
* Extensible translation architecture with compile-time safety across all UI labels, tray menus, toast notifications, and updater messages.
* Interactive Settings submenu in the System Tray allowing real-time toggling of:
  * Start with Windows (Autostart)
  * System Notifications
  * Sound on Receive
  * Open Folder on Receive
  * Clipboard Synchronization
  * Automatic Updates
  * Language Selection
* Silent automated background updater checking GitHub Releases periodically and performing hot-swap binary replacement.
* Multi-resolution embedded application icon (16x16 to 256x256) with 32-bit alpha transparency for Windows Taskbar, Alt-Tab, and Titlebar.
* Direct `.shortcut` file download endpoint (`/shortcut/AirDrop_Bridge.shortcut`) to resolve iOS shortcut import issues.

### Changed
* Default language switched from Spanish to English across the entire application.
* Language preference is now automatically persisted in `config.json`.
* Improved tray icon rendering by cropping transparent margins and eliminating gray artifact borders.
* Optimized tray loop responsiveness using an integrated Win32 message pump.

### Fixed
* Fixed issue where closing the main window left the system tray icon unresponsive to clicks.
* Fixed Taskbar icon display issue when running in windowed mode.
* Fixed iOS shortcut import URL failure on modern iOS versions by providing official iCloud link and local `.shortcut` fallback.

## [0.2.0] - 2026-11-22

### Added
* Native dark mode GUI window (Win32/DWM) displaying centered QR pairing code, live connection status, and action buttons.
* Detached Windows GUI process configuration (`windows_subsystem = "windows"`), eliminating the command prompt console window on double click.
* Custom high-resolution glassmorphic logo and icons embedded directly into the PE executable header (`winres`) and loaded into the system tray.
* Complete overhaul of the System Tray integration using `tray-icon` (v0.26) and `muda` (v0.21) with left-click window restore and instant right-click context menu.

### Changed
* Release workflow now publishes standalone `airdrop-bridge-windows-x64.exe` binaries directly with zero archive extraction required.
* Removed loose checksum artifact files from release asset lists.
* Rewrote root documentation entirely in clear, human-made technical English with no emojis or em dashes.

### Fixed
* Fixed system tray right-click menu responsiveness on Windows 11.
* Fixed Linux cross-compilation imports and packaging paths.

## [0.1.0] - 2026-11-19

### Added
* Initial release of AirDrop Bridge daemon in Rust (Axum + Tokio).
* O(1) memory disk streaming for incoming file uploads, handling multi-gigabyte transfers with constant memory footprint.
* Bidirectional clipboard synchronization for plain text, URLs, and RGBA images using `arboard`.
* Zero-configuration local network discovery via mDNS / DNS-SD (`_dropbridge._tcp.local`) with fallback IP resolution.
* Cryptographic Bearer token generation and authorization for local network security.
* Onboarding pairing web portal served directly by the daemon for iOS Safari.
* Interactive native Windows toast notifications with quick actions ("Open File", "Open Folder") and system audio chime.
* In-daemon background auto-updater with hot-swap replacement against GitHub Releases.
* Multiplatform CI/CD release workflow in GitHub Actions for Windows and Linux.
