# Apple Shortcut: AirDrop Bridge

Este directorio contiene la especificación y guía de integración del **Atajo oficial de iOS / iPadOS** para conectar directamente con el daemon de AirDrop Bridge.

## 📱 Modos de Uso en iOS / iPadOS

1. **Menú Compartir (Share Sheet):**
   - Selecciona cualquier foto, vídeo 4K, archivo en la app Archivos, PDF o enlace en Safari.
   - Toca en el botón Compartir y selecciona **AirDrop Bridge**.
   - El archivo se envía instantáneamente a tu ordenador (`Descargas/AirDropBridge`).

2. **Action Button (iPhone 15 Pro / 16 Series):**
   - Asigna el atajo **AirDrop Bridge** al Botón de Acción.
   - Al pulsarlo, si no hay contenido compartido, muestra un menú rápido:
     - 📋 *Enviar portapapeles al PC*
     - 📥 *Traer portapapeles del PC al iPhone*
     - 📸 *Enviar última foto tomada*
     - ⚙️ *Re-emparejar / Probar conexión*

3. **Tocar Atrás (Back Tap):**
   - Accesibilidad > Tocar > Tocar atrás (Doble o triple toque).
   - Ideal para sincronizar el portapapeles o enviar la última captura de pantalla sin abrir menús.

---

## ⚙️ Estructura del Atajo (Apple Shortcuts Logic)

El atajo ejecuta el siguiente flujo lógico optimizado:

```
[ INICIO DEL ATAJO ]
        │
        ├─► ¿Recibió Entrada (Shortcut Input)?
        │     ├─► SÍ (Archivos / Fotos / Enlaces / Texto)
        │     │     │
        │     │     ├─► Si es Archivo o Foto:
        │     │     │     POST http://<host>/api/v1/upload
        │     │     │     Multipart Form-Data con Authorization: Bearer <token>
        │     │     │
        │     │     └─► Si es Texto o URL:
        │     │           POST http://<host>/api/v1/clipboard
        │     │           JSON {"type": "text", "content": "<texto>"}
        │     │
        │     └─► NO (Invocado desde Botón de Acción, Menú o Pantalla de Inicio)
        │           │
        │           └─► Desplegar Menú Rápido:
        │                 1. Enviar portapapeles actual
        │                 2. Obtener portapapeles del PC (GET /api/v1/clipboard)
        │                 3. Enviar última foto tomada
        │
        └─► Vibración háptica de éxito y notificación nativa
```

---

## 🔒 Formato del Archivo de Configuración Local

Al escanear el código QR en pantalla o abrir la URL de configuración, el Atajo guarda los parámetros de conexión en su almacenamiento persistente:

```json
{
  "device_name": "PC-Jorge",
  "host": "pc-jorge.local",
  "ip": "192.168.1.50",
  "port": 53317,
  "token": "d8f94a...64caracteres_hex"
}
```

### Tolerancia a Fallos (Estrategia Dual-Addressing):
1. El atajo consulta primero: `http://pc-jorge.local:53317/api/v1/ping`
2. Si la red Wi-Fi tiene aislamiento AP o mDNS bloqueado por el router, conmuta inmediatamente a: `http://192.168.1.50:53317/api/v1/ping`
