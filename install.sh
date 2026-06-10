#!/usr/bin/env bash
set -e

REPO="pioneerAlone/wx-cli-mcp"
BINARY="wx-mcp"

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin)
    case "$ARCH" in
      arm64)  ASSET="wx-mcp-macos-arm64" ;;
      x86_64) ASSET="wx-mcp-macos-x86_64" ;;
      *) echo "Unsupported macOS arch: $ARCH"; exit 1 ;;
    esac
    INSTALL_DIR="/usr/local/bin"
    ;;
  Linux)
    case "$ARCH" in
      x86_64)  ASSET="wx-mcp-linux-x86_64" ;;
      aarch64) ASSET="wx-mcp-linux-arm64" ;;
      *) echo "Unsupported Linux arch: $ARCH"; exit 1 ;;
    esac
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
    ;;
  *)
    echo "Unsupported OS: $OS"
    echo "For Windows, use:"
    echo "  irm https://raw.githubusercontent.com/${REPO}/main/install.ps1 | iex"
    exit 1
    ;;
esac

# Get latest release tag
echo "Fetching latest release..."
LATEST=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
  | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST" ]; then
  echo "Error: could not determine latest release"
  exit 1
fi

echo "Installing wx-mcp ${LATEST}..."

DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST}/${ASSET}"
DEST="${INSTALL_DIR}/${BINARY}"

curl -fsSL "$DOWNLOAD_URL" -o "$DEST"
chmod +x "$DEST"

echo ""
echo "Installed wx-mcp to ${DEST}"

# PATH hint for Linux ~/.local/bin
if [[ "$OS" == "Linux" && ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
  echo ""
  echo "NOTE: Add to your shell profile to put wx-mcp on PATH:"
  echo '  export PATH="$HOME/.local/bin:$PATH"'
fi

echo ""
"$DEST" --version
echo "Installation complete!"
