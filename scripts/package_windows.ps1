# ZenLavaTerm Windows MSI packaging script.
# Builds native .msi installer using WiX Toolset and wix/main.wxs.

param (
    [string]$SourceBinaryDir = "target\x86_64-pc-windows-msvc\release",
    [string]$OutputDir = "dist"
)

$ErrorActionPreference = "Stop"

$VersionMatch = Select-String -Path "Cargo.toml" -Pattern '^version = "([^"]+)"'
if (-not $VersionMatch) {
    Write-Error "Could not determine package version from Cargo.toml"
    exit 1
}
$Version = $VersionMatch.Matches[0].Groups[1].Value
$Arch = "x86_64"
$MsiName = "ZenLavaTerm-v$Version-windows-$Arch.msi"

Write-Host "==> Packaging ZenLavaTerm v$Version (Windows $Arch MSI)..."
Write-Host "    Binary source directory: $SourceBinaryDir"
Write-Host "    Output MSI:              $OutputDir\$MsiName"

if (-not (Test-Path "$SourceBinaryDir\lavaterm.exe")) {
    Write-Error "Error: Binary not found at $SourceBinaryDir\lavaterm.exe"
    exit 1
}

New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null
New-Item -ItemType Directory -Force -Path "target\wix" | Out-Null

$ResolvedSourceDir = (Resolve-Path $SourceBinaryDir).Path

# Verify WiX Toolset
if (-not (Get-Command "candle.exe" -ErrorAction SilentlyContinue)) {
    if ($env:WIX -and (Test-Path "$env:WIX\bin\candle.exe")) {
        $env:PATH = "$env:WIX\bin;" + $env:PATH
    } elseif (Test-Path "C:\Program Files (x86)\WiX Toolset v3.11\bin\candle.exe") {
        $env:PATH = "C:\Program Files (x86)\WiX Toolset v3.11\bin;" + $env:PATH
    } else {
        Write-Error "Error: candle.exe (WiX Toolset) not found in PATH or standard installation directory."
        exit 1
    }
}

candle.exe -dVersion="$Version" -dSourceDir="$ResolvedSourceDir" -arch x64 wix\main.wxs -o target\wix\main.wixobj
light.exe -ext WixUIExtension -out "$OutputDir\$MsiName" target\wix\main.wixobj

if (Test-Path "$OutputDir\$MsiName") {
    Write-Host "✅ Created: $OutputDir\$MsiName"
} else {
    Write-Error "Error: WiX failed to produce $OutputDir\$MsiName"
    exit 1
}
