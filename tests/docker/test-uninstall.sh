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

if [ -f ~/.codex/config.toml ] && grep -q '^\[marketplaces\.speq-skill-local\]' ~/.codex/config.toml; then
    echo "FAIL: Codex marketplace registration still exists"
    exit 1
fi
echo "PASS: Codex marketplace registration removed"

if [ -f ~/.agents/plugins/marketplace.json ] && grep -q '"name": "speq-skill"' ~/.agents/plugins/marketplace.json; then
    echo "FAIL: legacy Codex marketplace entry still exists"
    exit 1
fi
echo "PASS: legacy Codex marketplace entry removed"

if [ -e ~/.codex/skills/speq-plan ] || [ -L ~/.codex/skills/speq-plan ]; then
    echo "FAIL: Codex speq-plan skill still exists"
    exit 1
fi
echo "PASS: Codex skills removed"

echo ""
echo "=== All uninstallation tests passed! ==="
