#!/usr/bin/env bash
# Vert CLI installer — macOS & Linux
# Usage: curl -fsSL https://raw.githubusercontent.com/jean3690/vert/master/install.sh | bash

set -euo pipefail

REPO="jean3690/vert"

# ── Colors ──
BOLD="\033[1m"
CYAN="\033[36m"
GREEN="\033[32m"
YELLOW="\033[33m"
GRAY="\033[90m"
RESET="\033[0m"

echo -e "${CYAN}${BOLD} Vert CLI installer${RESET}\n"

# ── Detect platform ──
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
    Linux)  PLATFORM="linux-x64" ;;
    Darwin) PLATFORM="macos-x64" ;;
    *)
        echo "Unsupported OS: $OS"
        echo "For Windows, use PowerShell: irm https://raw.githubusercontent.com/$REPO/master/install.ps1 | iex"
        exit 1
        ;;
esac

# ── Get latest release ──
echo -e "${GRAY}Fetching latest release...${RESET}"
API="https://api.github.com/repos/$REPO/releases/latest"
TAG=$(curl -fsSL "$API" | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"\(.*\)".*/\1/')

if [ -z "$TAG" ]; then
    echo "Failed to fetch latest release. Check your network or try again."
    exit 1
fi
echo -e "${GREEN} Latest: $TAG${RESET}"

# ── Download ──
BIN="vert-$PLATFORM"
URL="https://github.com/$REPO/releases/download/$TAG/$BIN"
TMP="/tmp/$BIN"

echo -e "${GRAY}Downloading $BIN...${RESET}"
curl -fsSL "$URL" -o "$TMP"
chmod +x "$TMP"

# ── Install ──
DIR="${XDG_BIN_HOME:-$HOME/.local/bin}"
mkdir -p "$DIR"
DEST="$DIR/vert"
mv -f "$TMP" "$DEST"

# ── Ensure in PATH ──
if [[ ":$PATH:" != *":$DIR:"* ]]; then
    echo -e "${YELLOW} Note: $DIR is not in PATH${RESET}"
    case "${SHELL:-}" in
        */zsh)  echo "  echo 'export PATH=\"\$PATH:$DIR\"' >> ~/.zshrc && source ~/.zshrc" ;;
        */bash) echo "  echo 'export PATH=\"\$PATH:$DIR\"' >> ~/.bashrc && source ~/.bashrc" ;;
        */fish) echo "  fish_add_path $DIR" ;;
        *)      echo "  Add $DIR to your PATH" ;;
    esac
fi

echo ""
echo -e "${GREEN} Done! vert installed to $DEST${RESET}"
echo -e "${GRAY} Try: vert --help${RESET}"
