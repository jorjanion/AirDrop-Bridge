# AirDrop Bridge

AirDrop Bridge is an ultra-lightweight, native, open-source local transfer daemon connecting Apple devices (iPhone, iPad via iOS Shortcuts) with Windows and Linux computers.

## Highlights
- **Ultra-lightweight:** < 15 MB RAM at idle, 0.0% CPU usage.
- **Zero-config pairing:** Scan an on-screen QR code from your iOS camera.
- **mDNS / DNS-SD discovery:** Automatic network discovery over `_dropbridge._tcp.local` with fallback local IP support.
- **O(1) Memory Disk Streaming:** Stream multi-gigabyte photos and 4K videos directly to disk with constant memory footprint.
- **Bidirectional Clipboard:** Seamlessly sync text, links, and image bitmaps between iOS and Windows/Linux.
- **Native OS integration:** Windows 10/11 native Toast notifications with quick action buttons ("Open File", "Open Folder"), subtle native sound, and system tray menu.
- **Cryptographic Security:** High-entropy local Bearer tokens protect your computer on public Wi-Fi networks.
- **Auto-updater:** Integrated background checking with GitHub Releases and seamless binary hot-swapping.

## Architecture

```
[ iPhone / iPad (Apple Shortcut) ]
               |
        (mDNS / Local IP)
               v
[ AirDrop Bridge Daemon (Rust / Axum) ]
    ├── Bearer Token Authenticator
    ├── Multipart O(1) Chunk Streamer -> Downloads/AirDropBridge
    ├── Native Clipboard Sync (arboard)
    ├── System Tray & Toast Notifications
    └── Auto-updater (GitHub Releases API)
```

## License
Dual-licensed under MIT or Apache 2.0.
