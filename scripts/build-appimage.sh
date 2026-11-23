#!/usr/bin/env bash
set -euo pipefail

# Directory locations
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

TARGET="${1:-x86_64-unknown-linux-gnu}"
BINARY_PATH="$ROOT_DIR/target/$TARGET/release/airdrop-bridge"

if [ ! -f "$BINARY_PATH" ]; then
    BINARY_PATH="$ROOT_DIR/target/release/airdrop-bridge"
fi

if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Binary not found at $BINARY_PATH"
    echo "Please build the release binary first: cargo build --release --target $TARGET"
    exit 1
fi

APP_DIR="$ROOT_DIR/build/AppDir"
DIST_DIR="$ROOT_DIR/dist"

echo "==> Preparing AppDir at $APP_DIR..."
rm -rf "$APP_DIR"
mkdir -p "$APP_DIR/usr/bin"
mkdir -p "$APP_DIR/usr/lib"
mkdir -p "$APP_DIR/usr/share/applications"
mkdir -p "$APP_DIR/usr/share/icons/hicolor/256x256/apps"
mkdir -p "$APP_DIR/usr/share/icons/hicolor/512x512/apps"
mkdir -p "$DIST_DIR"

# 1. Copy Binary
cp "$BINARY_PATH" "$APP_DIR/usr/bin/airdrop-bridge"
chmod +x "$APP_DIR/usr/bin/airdrop-bridge"

# 2. Copy Desktop Entry
DESKTOP_SRC="$ROOT_DIR/resources/linux/airdrop-bridge.desktop"
cp "$DESKTOP_SRC" "$APP_DIR/airdrop-bridge.desktop"
cp "$DESKTOP_SRC" "$APP_DIR/usr/share/applications/airdrop-bridge.desktop"

# 3. Copy Icon
ICON_SRC="$ROOT_DIR/resources/icon.png"
cp "$ICON_SRC" "$APP_DIR/airdrop-bridge.png"
cp "$ICON_SRC" "$APP_DIR/.DirIcon"
cp "$ICON_SRC" "$APP_DIR/usr/share/icons/hicolor/512x512/apps/airdrop-bridge.png"
cp "$ICON_SRC" "$APP_DIR/usr/share/icons/hicolor/256x256/apps/airdrop-bridge.png"

# 4. Create AppRun Entrypoint
cat << 'EOF' > "$APP_DIR/AppRun"
#!/bin/sh
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH}"
export XDG_DATA_DIRS="${HERE}/usr/share:${XDG_DATA_DIRS:-/usr/local/share:/usr/share}"
exec "${HERE}/usr/bin/airdrop-bridge" "$@"
EOF
chmod +x "$APP_DIR/AppRun"

echo "==> AppDir successfully populated."

# 5. Locate or Download appimagetool
APPIMAGETOOL="${APPIMAGETOOL:-}"
if [ -z "$APPIMAGETOOL" ]; then
    if command -v appimagetool &> /dev/null; then
        APPIMAGETOOL="$(command -v appimagetool)"
    else
        echo "==> Downloading appimagetool..."
        TOOLS_DIR="$ROOT_DIR/build/tools"
        mkdir -p "$TOOLS_DIR"
        TOOL_APPIMAGE="$TOOLS_DIR/appimagetool-x86_64.AppImage"
        if [ ! -f "$TOOL_APPIMAGE" ]; then
            curl -sSL -o "$TOOL_APPIMAGE" "https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage"
            chmod +x "$TOOL_APPIMAGE"
            # Extract to bypass FUSE requirements in CI / Docker
            (cd "$TOOLS_DIR" && "$TOOL_APPIMAGE" --appimage-extract)
        fi
        APPIMAGETOOL="$TOOLS_DIR/squashfs-root/AppRun"
    fi
fi

# 6. Generate the AppImage
OUTPUT_NAME="AirDrop-Bridge-x86_64.AppImage"
OUTPUT_PATH="$DIST_DIR/$OUTPUT_NAME"

echo "==> Generating $OUTPUT_PATH using $APPIMAGETOOL..."
export ARCH=x86_64
"$APPIMAGETOOL" "$APP_DIR" "$OUTPUT_PATH"

echo "==> Successfully created $OUTPUT_PATH"
