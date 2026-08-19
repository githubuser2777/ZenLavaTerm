#!/usr/bin/env bash
# ZenLavaTerm Linux packaging script.
# Builds native .deb and .AppImage packages from target release binary.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

VERSION=$(grep -m 1 '^version = ' "$ROOT_DIR/Cargo.toml" | awk -F '"' '{print $2}')
TARGET_ARCH="x86_64"
BIN_SRC="${1:-$ROOT_DIR/target/release/lavaterm}"

if [[ ! -f "$BIN_SRC" ]]; then
    # Check target/x86_64-unknown-linux-gnu/release/lavaterm
    if [[ -f "$ROOT_DIR/target/x86_64-unknown-linux-gnu/release/lavaterm" ]]; then
        BIN_SRC="$ROOT_DIR/target/x86_64-unknown-linux-gnu/release/lavaterm"
    else
        echo "Error: Binary not found at $BIN_SRC" >&2
        exit 1
    fi
fi

OUTPUT_DIR="${2:-$ROOT_DIR/dist}"
mkdir -p "$OUTPUT_DIR"

echo "==> Packaging ZenLavaTerm v$VERSION (Linux $TARGET_ARCH)..."
echo "    Binary source: $BIN_SRC"
echo "    Output directory: $OUTPUT_DIR"

# 1. Build .deb package
DEB_NAME="ZenLavaTerm-v${VERSION}-linux-${TARGET_ARCH}.deb"
DEB_STAGING="$ROOT_DIR/target/deb_staging"
rm -rf "$DEB_STAGING"
mkdir -p "$DEB_STAGING/DEBIAN"
mkdir -p "$DEB_STAGING/usr/bin"
mkdir -p "$DEB_STAGING/usr/share/applications"
mkdir -p "$DEB_STAGING/usr/share/icons/hicolor/scalable/apps"
mkdir -p "$DEB_STAGING/usr/share/doc/zenlavaterm"

cat << CTRL > "$DEB_STAGING/DEBIAN/control"
Package: zenlavaterm
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: amd64
Maintainer: ZenLavaTerm Contributors <https://github.com/githubuser2777/ZenLavaTerm>
Description: Terminal-native ambient lava lamp and metaball visualizer
 A high-performance, terminal-native ambient lava lamp & metaball visualizer
 written in Rust. Features 2D scalar field isosurfaces, sub-pixel Unicode
 block and Braille character packing, and 24-bit True Color gradients.
CTRL

cp "$BIN_SRC" "$DEB_STAGING/usr/bin/lavaterm"
chmod 755 "$DEB_STAGING/usr/bin/lavaterm"
cp "$ROOT_DIR/packaging/debian/zenlavaterm.desktop" "$DEB_STAGING/usr/share/applications/"
cp "$ROOT_DIR/assets/lavaterm-banner.svg" "$DEB_STAGING/usr/share/icons/hicolor/scalable/apps/zenlavaterm.svg"
cp "$ROOT_DIR/README.md" "$ROOT_DIR/LICENSE" "$ROOT_DIR/CHANGELOG.md" "$DEB_STAGING/usr/share/doc/zenlavaterm/"

if command -v dpkg-deb >/dev/null 2>&1; then
    echo "==> Building .deb with dpkg-deb..."
    dpkg-deb --build --root-owner-group "$DEB_STAGING" "$OUTPUT_DIR/$DEB_NAME"
else
    echo "==> dpkg-deb not found, assembling .deb via ar/tar..."
    # Build control.tar.gz and data.tar.gz manually
    (cd "$DEB_STAGING/DEBIAN" && tar --owner=0 --group=0 -czf "$ROOT_DIR/target/control.tar.gz" ./*)
    (cd "$DEB_STAGING" && tar --owner=0 --group=0 --exclude="./DEBIAN" -czf "$ROOT_DIR/target/data.tar.gz" ./*)
    echo "2.0" > "$ROOT_DIR/target/debian-binary"
    ar -rc "$OUTPUT_DIR/$DEB_NAME" "$ROOT_DIR/target/debian-binary" "$ROOT_DIR/target/control.tar.gz" "$ROOT_DIR/target/data.tar.gz"
    rm -f "$ROOT_DIR/target/debian-binary" "$ROOT_DIR/target/control.tar.gz" "$ROOT_DIR/target/data.tar.gz"
fi

echo "✅ Created: $OUTPUT_DIR/$DEB_NAME"

# 2. Build .AppImage package
APPIMAGE_NAME="ZenLavaTerm-v${VERSION}-linux-${TARGET_ARCH}.AppImage"
APPDIR="$ROOT_DIR/target/AppDir"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin"
mkdir -p "$APPDIR/usr/share/icons/hicolor/scalable/apps"

cp "$BIN_SRC" "$APPDIR/usr/bin/lavaterm"
chmod 755 "$APPDIR/usr/bin/lavaterm"
cp "$ROOT_DIR/packaging/appimage/AppRun" "$APPDIR/AppRun"
chmod 755 "$APPDIR/AppRun"
cp "$ROOT_DIR/packaging/appimage/zenlavaterm.desktop" "$APPDIR/zenlavaterm.desktop"
cp "$ROOT_DIR/assets/lavaterm-banner.svg" "$APPDIR/zenlavaterm.svg"
cp "$ROOT_DIR/assets/lavaterm-banner.svg" "$APPDIR/usr/share/icons/hicolor/scalable/apps/zenlavaterm.svg"

# If appimagetool is available or downloadable
if command -v appimagetool >/dev/null 2>&1; then
    ARCH="$TARGET_ARCH" appimagetool "$APPDIR" "$OUTPUT_DIR/$APPIMAGE_NAME"
elif [[ -f "$ROOT_DIR/appimagetool" ]]; then
    ARCH="$TARGET_ARCH" "$ROOT_DIR/appimagetool" "$APPDIR" "$OUTPUT_DIR/$APPIMAGE_NAME"
else
    echo "==> Downloading appimagetool..."
    curl -fsSL "https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage" -o "$ROOT_DIR/target/appimagetool"
    chmod +x "$ROOT_DIR/target/appimagetool"
    (cd "$ROOT_DIR/target" && ./appimagetool --appimage-extract >/dev/null 2>&1 || true)
    if [[ -d "$ROOT_DIR/target/squashfs-root" ]]; then
        ARCH="$TARGET_ARCH" "$ROOT_DIR/target/squashfs-root/AppRun" "$APPDIR" "$OUTPUT_DIR/$APPIMAGE_NAME"
    else
        ARCH="$TARGET_ARCH" "$ROOT_DIR/target/appimagetool" "$APPDIR" "$OUTPUT_DIR/$APPIMAGE_NAME"
    fi
fi
chmod +x "$OUTPUT_DIR/$APPIMAGE_NAME"
echo "✅ Created: $OUTPUT_DIR/$APPIMAGE_NAME"
