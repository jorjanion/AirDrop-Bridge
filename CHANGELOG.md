# Changelog

All notable changes to AirDrop Bridge will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Native dark mode GUI window (Win32/DWM) displaying centered QR pairing code and live connection status.
- Detached GUI process configuration for Windows (`windows_subsystem = "windows"`), preventing unwanted console windows on startup.
- Embedded application icons in PE binary header (`winres`) and system tray.
- Overhauled system tray integration using `tray-icon` and `muda` with responsive context menus and click handlers.

### Changed
- Refactored release pipeline to produce standalone `airdrop-bridge-windows-x64.exe` binaries directly without archive extraction.
- Removed loose checksum artifact generation from release assets.

## [0.1.0] - 2026-11-19

### Added
- Initial release of AirDrop Bridge daemon in Rust (Axum + Tokio).
- O(1) memory disk streaming for incoming file uploads, handling multi-gigabyte transfers with constant memory footprint.
- Bidirectional clipboard synchronization for plain text, URLs, and RGBA images using `arboard`.
- Zero-configuration local network discovery via mDNS / DNS-SD (`_dropbridge._tcp.local`) with fallback IP resolution.
- Cryptographic Bearer token generation and authorization for local network security.
- Onboarding pairing web portal served directly by the daemon for iOS Safari.
- Interactive native Windows toast notifications with quick actions ("Open File", "Open Folder") and system audio chime.
- In-daemon background auto-updater with hot-swap replacement against GitHub Releases.
- Multiplatform CI/CD release workflow in GitHub Actions for Windows and Linux.
