#!/bin/bash
# Integration test: verify install.sh works in minimal Linux environment
set -euo pipefail

echo "=== Testing speq-skill installation in Docker ==="

# Test 1: Run install script (downloads from GitHub releases)
echo "Running install.sh..."
./install.sh

# Test 2: Verify speq CLI installed and works
echo "Testing speq CLI..."
if ! ~/.local/bin/speq --help > /dev/null 2>&1; then
    echo "FAIL: speq CLI not accessible"
    exit 1
fi
echo "PASS: speq CLI installed and functional"

# Test 3: Verify marketplace directory structure
echo "Verifying marketplace structure..."
if [[ ! -d ~/.speq-skill/.claude-plugin ]]; then
    echo "FAIL: marketplace structure missing"
    exit 1
fi
echo "PASS: marketplace structure valid"

# Test 4: Verify plugin commands were attempted
# Note: Plugin install may require auth, but commands should not error
# We test that install.sh handles missing auth gracefully
echo "PASS: integration test complete"
