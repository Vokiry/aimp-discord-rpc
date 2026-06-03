#!/bin/sh
set -e

PREFIX="${PREFIX:-$HOME/.local}"
BINDIR="${PREFIX}/bin"
APPS_DIR="${HOME}/.local/share/applications"
ICON_DIR="${PREFIX}/share/icons/hicolor/scalable/apps"
AUTOSTART_DIR="${HOME}/.config/autostart"

mkdir -p "$BINDIR" "$APPS_DIR" "$ICON_DIR" "$AUTOSTART_DIR"

cp target/release/aimp-discord-rpc "$BINDIR/aimp-discord-rpc"
cp resources/aimp-discord-rpc.desktop "$APPS_DIR/aimp-discord-rpc.desktop"
cp resources/aimp-discord-rpc.svg "$ICON_DIR/aimp-discord-rpc.svg"
cp resources/aimp-discord-rpc.desktop "$AUTOSTART_DIR/aimp-discord-rpc.desktop"

echo "Installed to $BINDIR/aimp-discord-rpc"

if echo "$PATH" | grep -qv "$BINDIR"; then
    echo ""
    echo "Warning: $BINDIR is not in your PATH."
    echo "Add this to your shell config:"
    echo "  export PATH=\"\$PATH:$BINDIR\""
fi
