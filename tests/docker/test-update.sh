#!/bin/bash
# Integration test: verify install.sh handles updates (run install twice)
set -euo pipefail

echo "=== Testing speq-skill update in Docker ==="

# Step 1: First install
echo ""
echo "--- Step 1: First install ---"
./install.sh

# Step 2: Verify first install
echo ""
echo "--- Step 2: Verify first install ---"
VERSION_OUTPUT=$(~/.local/bin/speq --version 2>&1)
if [[ -z "$VERSION_OUTPUT" ]]; then
    echo "FAIL: speq --version returned empty after first install"
    exit 1
fi
echo "PASS: first install version: $VERSION_OUTPUT"

if [[ ! -d ~/.speq-skill/.claude-plugin ]]; then
    echo "FAIL: marketplace missing after first install"
    exit 1
fi
echo "PASS: marketplace exists after first install"

# Step 3: Second install (update)
echo ""
echo "--- Step 3: Running install.sh again (update) ---"
./install.sh

# Step 4: Verify update
echo ""
echo "--- Step 4: Verify update ---"
VERSION_OUTPUT=$(~/.local/bin/speq --version 2>&1)
if [[ -z "$VERSION_OUTPUT" ]]; then
    echo "FAIL: speq --version returned empty after update"
    exit 1
fi
echo "PASS: update version: $VERSION_OUTPUT"

if [[ ! -d ~/.speq-skill/.claude-plugin ]]; then
    echo "FAIL: marketplace missing after update"
    exit 1
fi
echo "PASS: marketplace exists after update"

if [[ ! -f ~/.speq-skill/bin/speq ]]; then
    echo "FAIL: marketplace bin/speq missing after update"
    exit 1
fi
echo "PASS: marketplace bin/speq exists after update"

if ! ~/.local/bin/speq --help > /dev/null 2>&1; then
    echo "FAIL: speq --help failed after update"
    exit 1
fi
echo "PASS: speq --help works after update"

echo ""
echo "=== All update tests passed! ==="
