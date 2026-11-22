<p align="center">
  <img src="../../resources/icon.png" width="120" alt="AirDrop Bridge Logo" />
</p>

<h1 align="center">AirDrop Bridge</h1>

<p align="center">
  <strong>Transferencias nativas y ultrarrápidas desde iPhone y iPad hacia Windows y Linux.</strong><br>
  Sin aplicaciones de terceros en iOS, sin cuentas en la nube y sin suscripciones.
</p>

<p align="center">
  <a href="../../README.md">English</a> |
  <strong>Español</strong> |
  <a href="README.zh.md">简体中文</a> |
  <a href="README.pt.md">Português</a> |
  <a href="README.fr.md">Français</a> |
  <a href="README.de.md">Deutsch</a> |
  <a href="README.it.md">Italiano</a> |
  <a href="README.ja.md">日本語</a>
</p>

<p align="center">
  <a href="https://github.com/jorjanion/airdrop-bridge/releases"><img src="https://img.shields.io/github/v/release/jorjanion/airdrop-bridge?color=38bdf8&style=flat-square" alt="Release" /></a>
  <a href="https://github.com/jorjanion/airdrop-bridge/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue?style=flat-square" alt="Licencia" /></a>
  <img src="https://img.shields.io/badge/plataforma-Windows%20%7C%20Linux-emerald?style=flat-square" alt="Plataforma" />
  <img src="https://img.shields.io/badge/idiomas-8%20incluidos-purple?style=flat-square" alt="Idiomas" />
  <img src="https://img.shields.io/badge/rust-edición%202024-orange?style=flat-square" alt="Rust" />
</p>

> ⭐ **¿Te gusta AirDrop Bridge?** ¡Agradecemos mucho una estrella en GitHub! Ayuda a que más usuarios conozcan este proyecto gratuito de código abierto y nos motiva a seguir mejorándolo.

---

## Descripción general

**AirDrop Bridge** te permite enviar fotos, vídeos en 4K y documentos desde tu iPhone o iPad directamente a tu ordenador a través de tu red Wi-Fi local.

En lugar de requerir descargas pesadas desde la App Store o enviar tus archivos privados a través de servidores en la nube de terceros, AirDrop Bridge vincula tu PC directamente con un Atajo oficial de Apple. Se integra de forma totalmente nativa en el menú Compartir de iOS, en el Botón de Acción o mediante el toque trasero.

<p align="center">
  <img src="../../assets/screenshots/pairing-window.png" width="280" alt="Ventana de vinculación en PC" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="../../assets/screenshots/mobile-setup.png" width="240" alt="Portal web de configuración en Safari" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="../../assets/screenshots/ios-share-sheet.png" width="240" alt="Menú compartir de iOS" />
</p>

---

## Características principales

* **100% Local y Privado**: Transferencia directa punto a punto por Wi-Fi. Tus fotos y archivos nunca salen de tu red doméstica.
* **Sin apps de terceros en iOS**: Utiliza un Atajo oficial firmado por el sistema iOS.
* **Velocidad máxima**: Transmisión por bloques directamente al disco con uso de memoria constante (menos de 15 MB de RAM en tu PC). Transfiere vídeos gigantescos sin saturar la memoria.
* **Descubrimiento automático**: Detección inmediata en la red local mediante mDNS y alternativa por IP LAN.
* **Seguridad por defecto**: Autorización mediante token Bearer criptográfico que impide transferencias no deseadas en redes Wi-Fi compartidas.
* **Soporte global para 8 idiomas**: Interfaz traducida a English, Español, 简体中文, Português, Français, Deutsch, Italiano y 日本語.
* **Experiencia nativa en escritorio**: Ventana oscura de emparejamiento, icono en la bandeja del sistema, sonido de notificación personalizable y notificaciones toast interactivas en Windows.
* **Sincronización de portapapeles**: Envía texto copiado entre dispositivos con un solo toque.
* **Actualizaciones automáticas**: Un actualizador en segundo plano se encarga de mantenerte siempre en la versión más reciente.

<p align="center">
  <img src="../../assets/screenshots/tray-menus.png" width="620" alt="Menú y ajustes de la bandeja del sistema" />
</p>

---

## Guía de inicio rápido

Configurar AirDrop Bridge lleva menos de un minuto.

### 1. Descarga y abre la app en tu PC
* Descarga `airdrop-bridge-windows-x64.exe` desde la sección de [Releases de GitHub](https://github.com/jorjanion/airdrop-bridge/releases/latest).
* Haz doble clic para iniciar. AirDrop Bridge se ejecutará silenciosamente en la bandeja del sistema y abrirá la ventana con el código QR.

### 2. Escanea con la Cámara de tu iPhone
* Abre la app Cámara de tu iPhone o iPad y enfoca el código QR en la pantalla de tu ordenador.
* Toca el enlace amarillo que aparece para abrir el portal de bienvenida en Safari.

### 3. Instala el Atajo oficial de Apple
* En la página web, pulsa en **"1. Descargar e Instalar Atajo AirDrop Bridge"**.
* Safari te preguntará si deseas abrirlo en Atajos. Pulsa **Abrir** y luego selecciona **Añadir atajo**.

### 4. Vincula tu PC
* Vuelve a Safari y pulsa en **"2. Vincular y Configurar este PC"**.
* El atajo importará automáticamente tu token de seguridad privado y la dirección IP de tu ordenador.

### 5. ¡Listo para compartir!
* Abre la app **Fotos** o **Archivos** en tu iPhone.
* Selecciona cualquier foto, vídeo o documento, pulsa el botón **Compartir** y elige **AirDrop Bridge**.
* ¡El archivo llegará al instante a tu carpeta de Descargas (`Descargas\AirDropBridge`) con un sonido de confirmación y una notificación en el escritorio!

---

## Aviso de Windows SmartScreen

Al abrir AirDrop Bridge por primera vez, es posible que Windows Defender SmartScreen muestre una ventana azul:

> *"Windows protegió su PC"*

### ¿Por qué aparece este aviso?
Los certificados comerciales de firma de código de Microsoft cuestan cientos de euros al año. Al ser AirDrop Bridge un proyecto independiente, gratuito y de código abierto, no adquirimos certificados empresariales de pago. Windows muestra temporalmente esta pantalla preventiva en ejecutables nuevos hasta que acumulan descargas suficientes.

### Cómo continuar de forma segura:
1. Haz clic en **"Más información"**.
2. Haz clic en el botón **"Ejecutar de todas formas"**.

El código fuente completo es 100% abierto, transparente y auditable en este mismo repositorio. Si lo prefieres, también puedes compilarlo tú mismo desde el código fuente con Cargo.

---

## Ajustes de la bandeja del sistema

Haz clic derecho en el icono de AirDrop Bridge en la barra de tareas de Windows para acceder a sus funciones:

* **Abrir AirDrop Bridge**: Vuelve a mostrar la ventana con el código QR.
* **Abrir descargas**: Acceso directo a la carpeta donde se guardan tus archivos.
* **Copiar token**: Copia tu token de seguridad al portapapeles.
* **Ajustes**:
  * **Iniciar con Windows**: Activa o desactiva el arranque automático al encender el PC.
  * **Notificaciones del sistema**: Muestra u oculta las notificaciones en el escritorio.
  * **Sonido al recibir**: Reproduce un sonido sutil cuando llega un archivo.
  * **Abrir carpeta al recibir**: Abre automáticamente la carpeta de destino al completarse la descarga.
  * **Sincronizar portapapeles**: Mantiene activa la sincronización de texto.
  * **Actualizaciones automáticas**: Comprueba e instala actualizaciones silenciosamente.
  * **Idioma**: Cambia al instante entre los 8 idiomas disponibles.

---

## Compilación desde el código fuente

Requisitos: [Rust](https://www.rust-lang.org/) (edición 2024 o superior).

```bash
git clone https://github.com/jorjanion/airdrop-bridge.git
cd airdrop-bridge
cargo build --release
```

El ejecutable optimizado se generará en la ruta `target/release/airdrop-bridge.exe`.

---

## Contribuciones, Issues y PRs

¡Toda contribución, reporte de fallos o sugerencia de la comunidad es bienvenida!

### Reportar errores o solicitar mejoras
Si encuentras algún problema o tienes una idea para mejorar AirDrop Bridge:
1. Entra en el [Rastreador de Issues](https://github.com/jorjanion/airdrop-bridge/issues).
2. Comprueba si alguien ya ha abierto un hilo con el mismo tema.
3. Haz clic en **New Issue** e incluye:
   * Qué ha ocurrido y qué esperabas que ocurriera.
   * Tu versión de sistema operativo (ejemplo: Windows 11 o distribución de Linux).
   * Tu versión de iOS/iPadOS (ejemplo: iOS 18.1).
   * Pasos claros para reproducir el fallo.

### Enviar un Pull Request (PR)
1. Haz un fork del repositorio y crea una rama con nombre descriptivo:
   ```bash
   git checkout -b feature/mi-mejora
   ```
2. Realiza tus cambios y asegúrate de que todos los tests pasen:
   ```bash
   cargo test
   ```
3. Crea tus commits siguiendo el estándar de commits convencionales (ejemplo: `feat: ...`, `fix: ...`).
4. Haz push a tu fork y abre un Pull Request hacia la rama `main`.
5. Explica brevemente los cambios propuestos. ¡Revisamos e integramos las aportaciones con entusiasmo!

---

## Licencia

AirDrop Bridge está disponible bajo cualquiera de las siguientes licencias, a tu elección:

* Licencia Apache, Versión 2.0 ([LICENSE-APACHE](../../LICENSE) o http://www.apache.org/licenses/LICENSE-2.0)
* Licencia MIT ([LICENSE-MIT](../../LICENSE) o http://opensource.org/licenses/MIT)
