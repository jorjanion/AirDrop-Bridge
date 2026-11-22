# Changelog

All notable changes to AirDrop Bridge will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
