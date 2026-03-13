#!/bin/bash
set -e

REPO="Raeproject99/CapsLock-as-a-modifier"
VERSION="0.1.0"
BINARY_URL="https://github.com/$REPO/releases/download/v$VERSION/capslock-untoggle-universal"
PLIST_URL="https://github.com/$REPO/releases/download/v$VERSION/com.local.capslock-untoggle.plist"
INSTALL_DIR="/usr/local/bin"
PLIST_DIR="$HOME/Library/LaunchAgents"
BINARY="$INSTALL_DIR/capslock-untoggle"
PLIST="$PLIST_DIR/com.local.capslock-untoggle.plist"

echo "==> Downloading capslock-untoggle..."
sudo curl -fsSL "$BINARY_URL" -o "$BINARY"
sudo chmod +x "$BINARY"
sudo xattr -cr "$BINARY"
codesign --force --sign - "$BINARY"

echo "==> Installing LaunchAgent..."
mkdir -p "$PLIST_DIR"
curl -fsSL "$PLIST_URL" -o "$PLIST"
sed -i '' "s|/usr/local/bin/capslock-untoggle|$BINARY|" "$PLIST"

echo ""
echo "==> Almost done! You need to grant Accessibility access."
echo "    Opening System Settings now..."
sleep 2
open "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility"

echo ""
echo "    Add '$BINARY' to the Accessibility list and enable it."
echo "    Press Enter when done..."
read -r

echo "==> Starting capslock-untoggle..."
launchctl bootstrap gui/$(id -u) "$PLIST"

echo ""
echo "✓ capslock-untoggle installed and running!"
echo "  It will start automatically at login."
