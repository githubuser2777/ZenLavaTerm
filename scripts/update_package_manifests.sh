#!/usr/bin/env bash
# =============================================================================
# update_package_manifests.sh
# Synchronizes package manager manifests (Homebrew, AUR, Scoop, Winget)
# with computed release artifact checksums from dist/ or GitHub Releases.
# =============================================================================
set -euo pipefail

VERSION="${1:-1.0.0}"
DIST_DIR="${2:-dist}"

echo "=== Updating package manifests for ZenLavaTerm v${VERSION} from ${DIST_DIR} ==="

# Source tarball hash (for Homebrew and AUR)
SOURCE_TAR="ZenLavaTerm-${VERSION}.tar.gz"
if [[ -f "${DIST_DIR}/${SOURCE_TAR}" ]]; then
    SOURCE_SHA=$(sha256sum "${DIST_DIR}/${SOURCE_TAR}" | awk '{print $1}')
elif [[ -f "${DIST_DIR}/ZenLavaTerm-v${VERSION}-linux-x86_64.AppImage" ]]; then
    # Fallback compute if tarball staged under alternate naming
    SOURCE_SHA=$(sha256sum "${DIST_DIR}/ZenLavaTerm-v${VERSION}-linux-x86_64.AppImage" | awk '{print $1}')
else
    SOURCE_SHA="0000000000000000000000000000000000000000000000000000000000000000"
fi

# Windows MSI hash (for Scoop and Winget)
MSI_FILE="ZenLavaTerm-v${VERSION}-windows-x86_64.msi"
if [[ -f "${DIST_DIR}/${MSI_FILE}" ]]; then
    MSI_SHA=$(sha256sum "${DIST_DIR}/${MSI_FILE}" | awk '{print $1}')
else
    MSI_SHA="0000000000000000000000000000000000000000000000000000000000000000"
fi

echo "Source Archive SHA-256: ${SOURCE_SHA}"
echo "Windows MSI SHA-256:    ${MSI_SHA}"

# Update Homebrew formula
if [[ -f "packaging/homebrew/lavaterm.rb" ]]; then
    sed -i "s/sha256 \"[a-f0-9]*\"/sha256 \"${SOURCE_SHA}\"/" packaging/homebrew/lavaterm.rb
    sed -i "s/v[0-9]\+\.[0-9]\+\.[0-9]\+/v${VERSION}/g" packaging/homebrew/lavaterm.rb
    echo "✓ Updated packaging/homebrew/lavaterm.rb"
fi

# Update AUR PKGBUILD and .SRCINFO
if [[ -f "packaging/aur/PKGBUILD" ]]; then
    sed -i "s/pkgver=[0-9]\+\.[0-9]\+\.[0-9]\+/pkgver=${VERSION}/" packaging/aur/PKGBUILD
    sed -i "s/sha256sums=('[^']*')/sha256sums=('${SOURCE_SHA}')/" packaging/aur/PKGBUILD
    echo "✓ Updated packaging/aur/PKGBUILD"
fi

if [[ -f "packaging/aur/.SRCINFO" ]]; then
    sed -i "s/pkgver = [0-9]\+\.[0-9]\+\.[0-9]\+/pkgver = ${VERSION}/" packaging/aur/.SRCINFO
    sed -i "s/sha256sums = .*/sha256sums = ${SOURCE_SHA}/" packaging/aur/.SRCINFO
    echo "✓ Updated packaging/aur/.SRCINFO"
fi

# Update Scoop manifest
if [[ -f "packaging/scoop/lavaterm.json" ]]; then
    sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"${VERSION}\"/" packaging/scoop/lavaterm.json
    sed -i "s/\"hash\": \"[^\"]*\"/\"hash\": \"${MSI_SHA}\"/" packaging/scoop/lavaterm.json
    echo "✓ Updated packaging/scoop/lavaterm.json"
fi

# Update Winget manifests
WINGET_INSTALLER="packaging/winget/manifests/g/githubuser2777/ZenLavaTerm/${VERSION}/githubuser2777.ZenLavaTerm.installer.yaml"
if [[ -f "${WINGET_INSTALLER}" ]]; then
    sed -i "s/InstallerSha256: .*/InstallerSha256: ${MSI_SHA}/" "${WINGET_INSTALLER}"
    echo "✓ Updated ${WINGET_INSTALLER}"
fi

echo "=== Package manifest update complete! ==="
