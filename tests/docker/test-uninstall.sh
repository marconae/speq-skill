#!/bin/bash
# Integration test: verify uninstall.sh works (run after test-install.sh)
set -euo pipefail

echo "=== Testing speq-skill uninstallation in Docker ==="

# Run uninstall script
echo ""
echo "--- Running uninstall.sh ---"
./uninstall.sh

# Verify CLI binary removed
echo ""
echo "--- Verify CLI binary removed ---"
if [ -f ~/.local/bin/speq ]; then
    echo "FAIL: ~/.local/bin/speq still exists"
    exit 1
fi
echo "PASS: ~/.local/bin/speq removed"

# Verify marketplace directory removed
echo ""
echo "--- Verify marketplace directory removed ---"
if [ -d ~/.speq-skill ]; then
    echo "FAIL: ~/.speq-skill still exists"
    exit 1
fi
echo "PASS: ~/.speq-skill removed"

echo ""
echo "=== All uninstallation tests passed! ==="
