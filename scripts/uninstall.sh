#!/bin/bash
# Uninstall speq-skill CLI and Claude plugin
# Usage: ./scripts/uninstall.sh

set -euo pipefail

BIN_PATH="${HOME}/.local/bin/speq"
INSTALL_DIR="${HOME}/.speq-skill"

echo "=== Uninstalling speq-skill ==="

# 1. Uninstall Claude plugin
if command -v claude &> /dev/null; then
    echo "Uninstalling speq-skill plugin..."
    claude plugin uninstall speq-skill@speq-skill 2>/dev/null || echo "  Plugin not installed or already removed"

    echo "Removing speq-skill marketplace..."
    claude plugin marketplace remove speq-skill 2>/dev/null || echo "  Marketplace not installed or already removed"
else
    echo "Claude CLI not found, skipping plugin removal"
fi

# 2. Remove CLI binary
if [ -f "$BIN_PATH" ]; then
    echo "Removing CLI: ${BIN_PATH}"
    rm -f "$BIN_PATH"
else
    echo "CLI not found: ${BIN_PATH}"
fi

# 3. Remove marketplace installation directory (if exists from remote install)
if [ -d "$INSTALL_DIR" ]; then
    echo "Removing installation directory: ${INSTALL_DIR}"
    rm -rf "$INSTALL_DIR"
fi

echo ""
echo "=== Uninstall complete ==="
