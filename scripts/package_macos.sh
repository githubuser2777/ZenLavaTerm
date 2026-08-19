#!/usr/bin/env bash
# ZenLavaTerm macOS packaging script.
# Assembles ZenLavaTerm.app bundle and generates standalone .dmg image.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

VERSION=$(grep -m 1 '^version = ' "$ROOT_DIR/Cargo.toml" | awk -F '"' '{print $2}')
ARCH_NAME="${1:-universal}"
BIN_SRC="${2:-$ROOT_DIR/target/release/lavaterm}"
OUTPUT_DIR="${3:-$ROOT_DIR/dist}"

DMG_NAME="ZenLavaTerm-v${VERSION}-macos-${ARCH_NAME}.dmg"
STAGING="$ROOT_DIR/target/macos_dmg_staging"

rm -rf "$STAGING"
mkdir -p "$STAGING/ZenLavaTerm.app/Contents/MacOS"
mkdir -p "$STAGING/ZenLavaTerm.app/Contents/Resources"
mkdir -p "$OUTPUT_DIR"

echo "==> Packaging ZenLavaTerm v$VERSION (macOS $ARCH_NAME)..."
echo "    Binary source: $BIN_SRC"
echo "    Output DMG:    $OUTPUT_DIR/$DMG_NAME"

cp "$BIN_SRC" "$STAGING/ZenLavaTerm.app/Contents/MacOS/lavaterm"
chmod 755 "$STAGING/ZenLavaTerm.app/Contents/MacOS/lavaterm"

# Generate Info.plist
sed "s/@VERSION@/${VERSION}/g" "$ROOT_DIR/packaging/macos/Info.plist" > "$STAGING/ZenLavaTerm.app/Contents/Info.plist"

# Copy documentation and assets
cp "$ROOT_DIR/README.md" "$ROOT_DIR/LICENSE" "$ROOT_DIR/CHANGELOG.md" "$STAGING/"
if [[ -f "$ROOT_DIR/assets/lavaterm-banner.svg" ]]; then
    cp "$ROOT_DIR/assets/lavaterm-banner.svg" "$STAGING/ZenLavaTerm.app/Contents/Resources/"
fi

# Applications symlink for drag-and-drop installation
ln -s /Applications "$STAGING/Applications"

if command -v hdiutil >/dev/null 2>&1; then
    echo "==> Creating DMG with hdiutil..."
    hdiutil create -volname "ZenLavaTerm" -srcfolder "$STAGING" -ov -format UDZO "$OUTPUT_DIR/$DMG_NAME"
    echo "✅ Created: $OUTPUT_DIR/$DMG_NAME"
else
    echo "⚠️  hdiutil is only available natively on macOS. Staging directory prepared at: $STAGING"
fi
