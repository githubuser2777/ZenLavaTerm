#!/usr/bin/env bash
# LavaTerm Arch Linux local packaging script.
# Builds native .pkg.tar.zst package from local tree or PKGBUILD.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
BUILD_DIR="$ROOT_DIR/target/arch_pkg"

INSTALL=false
if [[ "${1:-}" == "--install" || "${1:-}" == "-i" ]]; then
    INSTALL=true
fi

echo "==> Preparing Arch Linux build directory at $BUILD_DIR..."
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

# Copy PKGBUILD and source files
cp "$ROOT_DIR/packaging/arch/PKGBUILD" "$BUILD_DIR/"

# Create source archive from current repository tree
VERSION=$(grep -m 1 '^version = ' "$ROOT_DIR/Cargo.toml" | awk -F '"' '{print $2}')
PKG_TAR="$BUILD_DIR/lavaterm-$VERSION.tar.gz"

echo "==> Creating source tarball for lavaterm v$VERSION..."
(cd "$ROOT_DIR" && git archive --format=tar.gz --prefix="lavaterm-$VERSION/" -o "$PKG_TAR" HEAD)

# Run makepkg inside BUILD_DIR
echo "==> Running makepkg..."
cd "$BUILD_DIR"
makepkg -f --nodeps --noconfirm

# Find built package
BUILT_PKG=$(ls lavaterm-*.pkg.tar.zst 2>/dev/null | head -n 1)

if [[ -n "$BUILT_PKG" ]]; then
    sha256sum "$BUILT_PKG" > "$BUILT_PKG.sha256"
    echo ""
    echo "========================================================"
    echo "✅ Arch Linux package built successfully:"
    echo "   File:   $BUILD_DIR/$BUILT_PKG"
    echo "   SHA256: $(cat "$BUILD_DIR/$BUILT_PKG.sha256")"
    echo "========================================================"
    echo ""
    echo "To install this package on your system, run:"
    echo "  sudo pacman -U $BUILD_DIR/$BUILT_PKG"
    echo ""

    if [[ "$INSTALL" == "true" ]]; then
        echo "==> Installing package with sudo pacman -U..."
        sudo pacman -U "$BUILT_PKG"
    fi
else
    echo "❌ Error: makepkg did not produce a .pkg.tar.zst package."
    exit 1
fi
