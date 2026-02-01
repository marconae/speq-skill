#!/bin/bash
# Install speq-skill CLI and Claude plugin from local build
# Usage: ./scripts/local-install.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BIN_DIR="${HOME}/.local/bin"
MARKETPLACE_DIR="${PROJECT_ROOT}/dist/marketplace"

cd "$PROJECT_ROOT"

echo "=== Installing speq-skill ==="

# 1. Build release binary if not present
if [ ! -f "target/release/speq" ]; then
    echo "Building release binary..."
    cargo build --release
fi

# 2. Build plugin/marketplace if not present
if [ ! -d "dist/marketplace" ]; then
    echo "Building plugin..."
    ./scripts/plugin/build.sh
fi

# 3. Install CLI binary
mkdir -p "$BIN_DIR"
echo "Installing CLI to ${BIN_DIR}/speq..."
cp "target/release/speq" "$BIN_DIR/speq"
chmod +x "$BIN_DIR/speq"

# 4. Install Claude plugin via marketplace
echo "Adding speq-skill marketplace..."
claude plugin marketplace add "$MARKETPLACE_DIR"

echo "Installing speq-skill plugin..."
claude plugin install speq-skill@speq-skill

# 5. Verify installation
echo ""
echo "=== Installation complete ==="

if command -v speq &> /dev/null; then
    echo "CLI: $(which speq)"
else
    echo "CLI: ${BIN_DIR}/speq"
    echo "  Add to PATH: export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

echo "Plugin: installed via 'claude plugin install speq-skill@speq-skill'"
echo ""
echo "To uninstall: ./scripts/uninstall.sh"
