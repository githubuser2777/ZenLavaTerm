#!/usr/bin/env bash
# =============================================================================
# update_package_manifests.sh
# Synchronizes package manager manifests (Homebrew, AUR)
# with computed release artifact checksums from dist/ or staging.
#
# Usage:
#   ./scripts/update_package_manifests.sh [VERSION] [DIST_DIR] [--allow-placeholders]
#
# Defaults:
#   VERSION: 1.0.0
#   DIST_DIR: dist
#   Strict mode: Enabled (fails closed if release artifacts are missing)
# =============================================================================
set -euo pipefail

VERSION="${1:-1.0.0}"
DIST_DIR="${2:-dist}"
ALLOW_PLACEHOLDERS=0

for arg in "$@"; do
    if [[ "$arg" == "--allow-placeholders" ]]; then
        ALLOW_PLACEHOLDERS=1
    fi
done

echo "=== Updating package manifests for ZenLavaTerm v${VERSION} from ${DIST_DIR} ==="

SOURCE_TAR_CANDIDATES=(
    "${DIST_DIR}/ZenLavaTerm-${VERSION}.tar.gz"
    "${DIST_DIR}/lavaterm-${VERSION}.tar.gz"
    "${DIST_DIR}/v${VERSION}.tar.gz"
)

MSI_CANDIDATES=(
    "${DIST_DIR}/ZenLavaTerm-v${VERSION}-windows-x86_64.msi"
    "${DIST_DIR}/lavaterm-v${VERSION}-windows-x86_64.msi"
)

SOURCE_SHA=""
for f in "${SOURCE_TAR_CANDIDATES[@]}"; do
    if [[ -f "$f" ]]; then
        SOURCE_SHA=$(sha256sum "$f" | awk '{print $1}')
        echo "Found source archive: $f ($SOURCE_SHA)"
        break
    fi
done

MSI_SHA=""
for f in "${MSI_CANDIDATES[@]}"; do
    if [[ -f "$f" ]]; then
        MSI_SHA=$(sha256sum "$f" | awk '{print $1}')
        echo "Found Windows MSI: $f ($MSI_SHA)"
        break
    fi
done

# Fail closed check
if [[ -z "${SOURCE_SHA}" ]]; then
    if [[ "${ALLOW_PLACEHOLDERS}" -eq 1 ]]; then
        echo "::warning::Source tarball not found in ${DIST_DIR}. Using placeholder hash (--allow-placeholders active)."
        SOURCE_SHA="0000000000000000000000000000000000000000000000000000000000000000"
    else
        echo "::error::Source archive missing in ${DIST_DIR}. Manifest generation failed closed."
        echo "Required one of: ${SOURCE_TAR_CANDIDATES[*]}"
        exit 1
    fi
fi

if [[ -z "${MSI_SHA}" ]]; then
    if [[ "${ALLOW_PLACEHOLDERS}" -eq 1 ]]; then
        echo "::warning::Windows MSI not found in ${DIST_DIR}. Using placeholder hash (--allow-placeholders active)."
        MSI_SHA="0000000000000000000000000000000000000000000000000000000000000000"
    else
        echo "::error::Windows MSI missing in ${DIST_DIR}. Manifest generation failed closed."
        echo "Required one of: ${MSI_CANDIDATES[*]}"
        exit 1
    fi
fi

echo "Source Archive SHA-256: ${SOURCE_SHA}"
echo "Windows MSI SHA-256:    ${MSI_SHA}"

# 1. Update Homebrew formula
if [[ -f "packaging/homebrew/lavaterm.rb" ]]; then
    sed -i "s/sha256 \"[a-f0-9]*\"/sha256 \"${SOURCE_SHA}\"/" packaging/homebrew/lavaterm.rb
    sed -i "s/v[0-9]\+\.[0-9]\+\.[0-9]\+/v${VERSION}/g" packaging/homebrew/lavaterm.rb
    echo "✓ Updated packaging/homebrew/lavaterm.rb"
fi

# 2. Update AUR PKGBUILD and .SRCINFO
if [[ -f "packaging/aur/PKGBUILD" ]]; then
    sed -i "s/pkgver=[0-9]\+\.[0-9]\+\.[0-9]\+/pkgver=${VERSION}/" packaging/aur/PKGBUILD
    sed -i "s/sha256sums=('[^']*')/sha256sums=('__SOURCE_SHA__')/" packaging/aur/PKGBUILD
    echo "✓ Updated packaging/aur/PKGBUILD"
fi

# 2b. Update Arch source and bin PKGBUILDs (packaging/arch/)
if [[ -f "packaging/arch/PKGBUILD" ]]; then
    sed -i "s/pkgver=[0-9]\+\.[0-9]\+\.[0-9]\+/pkgver=${VERSION}/" packaging/arch/PKGBUILD
    sed -i "s/sha256sums=('[^']*')/sha256sums=('__SOURCE_SHA__')/" packaging/arch/PKGBUILD
    echo "✓ Updated packaging/arch/PKGBUILD"
fi

if [[ -f "packaging/arch/PKGBUILD.bin" ]]; then
    sed -i "s/pkgver=[0-9]\+\.[0-9]\+\.[0-9]\+/pkgver=${VERSION}/" packaging/arch/PKGBUILD.bin
    echo "✓ Updated packaging/arch/PKGBUILD.bin"
fi

if [[ -f "packaging/aur/.SRCINFO" ]]; then
    sed -i "s/pkgver = [0-9]\+\.[0-9]\+\.[0-9]\+/pkgver = ${VERSION}/" packaging/aur/.SRCINFO
    sed -i "s/__SOURCE_SHA__/${SOURCE_SHA}/g" packaging/aur/.SRCINFO
    echo "✓ Updated packaging/aur/.SRCINFO"
fi

echo "=== Package manifest synchronization successfully completed! ==="
