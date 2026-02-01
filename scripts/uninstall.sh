#!/bin/bash
# Uninstall speq-skill CLI and Claude plugin
# Usage: ./scripts/uninstall.sh

set -euo pipefail

BIN_PATH="${HOME}/.local/bin/speq"
PLUGIN_DIR="${HOME}/.claude/plugins/speq-skill"

echo "=== Uninstalling speq-skill ==="

# Remove CLI binary
if [ -f "$BIN_PATH" ]; then
    echo "Removing CLI: ${BIN_PATH}"
    rm -f "$BIN_PATH"
else
    echo "CLI not found: ${BIN_PATH}"
fi

# Remove Claude plugin
if [ -d "$PLUGIN_DIR" ]; then
    echo "Removing plugin: ${PLUGIN_DIR}"
    rm -rf "$PLUGIN_DIR"
else
    echo "Plugin not found: ${PLUGIN_DIR}"
fi

echo ""
echo "=== Uninstall complete ==="
