# 🌉 AirDrop Bridge

[![CI](https://github.com/jorjanion/airdrop-bridge/actions/workflows/ci.yml/badge.svg)](https://github.com/jorjanion/airdrop-bridge/actions/workflows/ci.yml)
[![Release](https://github.com/jorjanion/airdrop-bridge/actions/workflows/release.yml/badge.svg)](https://github.com/jorjanion/airdrop-bridge/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-brightgreen.svg)]()
[![Rust](https://img.shields.io/badge/Rust-2024%20Edition-orange.svg)](https://www.rust-lang.org/)

**AirDrop Bridge** es una alternativa ultraligera, nativa, segura y de código abierto para transferir archivos, fotos, vídeos 4K y portapapeles bidireccional entre dispositivos Apple (**iPhone**, **iPad** vía **Apple Shortcuts**) y ordenadores con **Windows 10/11** y **Linux**.

Cero configuración manual de IPs, cero servicios en la nube, arranque instantáneo y una huella de memoria minúscula.

---

## ⚡ Características Principales

* 🪶 **Consumo Ultraligero:** `< 15 MB` de RAM en reposo (típico `6-9 MB`), `0.0%` de uso de CPU.
* 🚀 **Streaming Directo a Disco $O(1)$:** Transfiere archivos y vídeos 4K de cualquier tamaño (5 GB, 20 GB+) sin acumularlos en memoria RAM.
* 🔍 **Descubrimiento Zero-Config:** mDNS / DNS-SD (`_dropbridge._tcp.local`) con respaldo automático de IP local si el router bloquea multicast.
* 📲 **Emparejamiento por Código QR:** Escanea el QR generado en pantalla con la app Cámara de iOS para configurar el Atajo en 1 solo toque.
* 📋 **Portapapeles Bidireccional:** Inyecta y lee texto, enlaces e imágenes RGBA entre iOS y Windows/Linux al instante.
* 🔔 **Integración Nativa con el SO:** Notificaciones interactivas Toast de Windows con botones para abrir el archivo o la carpeta, acompañadas de sonido sutil del sistema.
* 🛡️ **Seguridad Criptográfica:** Token `Bearer` aleatorio de 256 bits para proteger tu PC en redes Wi-Fi compartidas (oficinas, cafeterías, universidades).
* 🔄 **Auto-Update Integrado:** Comprobación periódica contra GitHub Releases y reemplazo en caliente (*hot-swap*) del ejecutable sin interrumpir tu trabajo.

---

## 🏗️ Arquitectura del Sistema

```
┌─────────────────────────────────┐
│     iPhone / iPad (iOS 15+)     │
│  Apple Shortcut (Share Sheet /  │
│    Action Button / Back Tap)    │
└────────────────┬────────────────┘
                 │
                 │ HTTP Streaming + Bearer Token
                 │ (Resolución mDNS / IP LAN)
                 ▼
┌─────────────────────────────────────────────────────────────┐
│             AirDrop Bridge Daemon (Rust / Axum)             │
│                                                             │
│  ┌───────────────────────┐       ┌───────────────────────┐  │
│  │  mDNS Responder       │       │  Bearer Token Auth    │  │
│  │  _dropbridge._tcp     │       │  Middleware (256-bit) │  │
│  └───────────────────────┘       └───────────┬───────────┘  │
│                                              ▼              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  HTTP Endpoints:                                      │  │
│  │    GET  /api/v1/ping        (Latencia <10 ms)         │  │
│  │    POST /api/v1/upload      (Streaming O(1) a disco)  │  │
│  │    POST /api/v1/clipboard   (Inyección en SO)         │  │
│  │    GET  /api/v1/clipboard   (Lectura hacia iOS)       │  │
│  │    GET  /setup              (Portal de emparejamiento)│  │
│  └───────────────────────────┬───────────────────────────┘  │
│                              │                              │
│  ┌───────────────────────────┼───────────────────────────┐  │
│  │  Integración Nativa:      │                           │  │
│  │    • System Tray & Menú   ▼                           │  │
│  │    • Windows Toast Notifications & Sonido             │  │
│  │    • Auto-Update (GitHub Releases Hot-Swap)           │  │
│  │    • Inicio automático con Windows (Registry Run)     │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

---

## 🚀 Inicio Rápido

### 1. Descarga e Instalación (Windows / Linux)
Descarga el binario precompilado más reciente desde [GitHub Releases](https://github.com/jorjanion/airdrop-bridge/releases/latest) o compílalo desde el código fuente:

```powershell
git clone https://github.com/jorjanion/airdrop-bridge.git
cd airdrop-bridge
cargo build --release
```
El ejecutable compilado se encontrará en `target/release/airdrop-bridge.exe`.

### 2. Emparejar con tu iPhone / iPad
1. Ejecuta `airdrop-bridge.exe`. La aplicación se iniciará minimizada en la bandeja del sistema (System Tray junto al reloj).
2. Haz clic derecho en el icono de la bandeja y selecciona **📱 Emparejar dispositivo (QR)...**.
3. Abre la app **Cámara** en tu iPhone y escanea el código QR que aparece en pantalla.
4. En la página web de bienvenida:
   * Pulsa **1. Descargar Atajo de iOS**.
   * Pulsa **2. Conectar y Configurar este PC**.
5. ¡Listo! Ya puedes compartir fotos, vídeos, archivos o el portapapeles desde el menú Compartir de iOS seleccionando **AirDrop Bridge**.

---

## 📡 Referencia de la API Local

Todos los endpoints protegidos requieren la cabecera `Authorization: Bearer <token>`.

### 1. Verificación de Conexión (`GET /api/v1/ping`)
* **Público (no requiere token)**
```bash
curl http://localhost:53317/api/v1/ping
```
Respuesta:
```json
{
  "app": "AirDropBridge",
  "device_name": "PC-Jorge",
  "status": "ok",
  "version": "0.1.0"
}
```

### 2. Subida de Archivos en Streaming (`POST /api/v1/upload`)
* Escribe los fragmentos directamente a disco (`Descargas/AirDropBridge`) con consumo constante de RAM:
```bash
curl -X POST http://localhost:53317/api/v1/upload \
  -H "Authorization: Bearer <token>" \
  -F "file=@foto.jpg"
```

### 3. Inyectar Portapapeles (`POST /api/v1/clipboard`)
* Inyecta texto o imágenes en el portapapeles de Windows/Linux:
```bash
curl -X POST http://localhost:53317/api/v1/clipboard \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"type": "text", "content": "Texto copiado desde el iPhone"}'
```

### 4. Leer Portapapeles del PC (`GET /api/v1/clipboard`)
* Devuelve el contenido actual del portapapeles hacia iOS:
```bash
curl http://localhost:53317/api/v1/clipboard \
  -H "Authorization: Bearer <token>"
```

---

## 🛠️ Desarrollo y Pruebas

Para ejecutar el formateador, linteo y suite de pruebas unitarias e integración:

```powershell
# Comprobar formato estándar
cargo fmt --check

# Linteo estricto con Clippy (sin advertencias)
cargo clippy -- -D warnings

# Batería de pruebas automatizadas
cargo test
```

---

## 📦 Pipeline de Releases (GitHub Actions)

El proyecto cuenta con un workflow en `.github/workflows/release.yml` que se dispara automáticamente al empujar una etiqueta de versión Git:

```powershell
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

GitHub Actions compilará automáticamente los binarios optimizados con LTO (`.zip` para Windows, `.tar.gz` para Linux), generará sus sumas de verificación `SHA-256` y publicará la release oficial en GitHub.

---

## 📄 Licencia

Distribuido bajo la licencia **MIT** o **Apache-2.0** a tu elección.
