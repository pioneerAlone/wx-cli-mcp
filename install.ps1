<#
.SYNOPSIS
    Install wx-mcp on Windows.
.DESCRIPTION
    Downloads the latest wx-mcp release binary and adds it to PATH.
.EXAMPLE
    irm https://raw.githubusercontent.com/pioneerAlone/wx-cli-mcp/main/install.ps1 | iex
#>

$ErrorActionPreference = 'Stop'

$Repo = "pioneerAlone/wx-cli-mcp"
$Asset = "wx-mcp-windows-x64.exe"
$InstallDir = Join-Path $env:LOCALAPPDATA "Programs\wx-mcp"
$BinaryPath = Join-Path $InstallDir "wx-mcp.exe"

# Get latest release tag
Write-Host "Fetching latest release..."
$Release = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
$Version = $Release.tag_name

if (-not $Version) {
    Write-Error "Could not determine latest release"
    exit 1
}

Write-Host "Installing wx-mcp $Version..."

# Create install directory
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

# Download binary
$DownloadUrl = "https://github.com/$Repo/releases/download/$Version/$Asset"
Write-Host "Downloading from $DownloadUrl"
Invoke-WebRequest -Uri $DownloadUrl -OutFile $BinaryPath -UseBasicParsing

# Add to user PATH if not already present
$CurrentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($CurrentPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable(
        "PATH",
        "$InstallDir;$CurrentPath",
        "User"
    )
    Write-Host "Added $InstallDir to user PATH"
    Write-Host "NOTE: Restart your terminal for PATH changes to take effect"
}

Write-Host ""
Write-Host "Installed wx-mcp to $BinaryPath"
Write-Host ""
& $BinaryPath --version
Write-Host "Installation complete!"
