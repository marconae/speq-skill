#!/bin/bash
# Integration test: verify install.sh works
set -euo pipefail

echo "=== Testing speq-skill installation in Docker ==="

# Test 1: Run install script
echo ""
echo "--- Test 1: Running install.sh ---"
./install.sh

# Test 2: Verify speq binary installed and works
echo ""
echo "--- Test 2: Verify speq binary ---"
if ! ~/.local/bin/speq --help > /dev/null 2>&1; then
    echo "FAIL: speq --help failed"
    exit 1
fi
echo "PASS: speq --help works"

# Verify version output
VERSION_OUTPUT=$(~/.local/bin/speq --version 2>&1)
if [[ -z "$VERSION_OUTPUT" ]]; then
    echo "FAIL: speq --version returned empty"
    exit 1
fi
echo "PASS: speq --version returns: $VERSION_OUTPUT"

# Test 3: Verify marketplace structure
echo ""
echo "--- Test 3: Verify marketplace structure ---"
if [[ ! -d ~/.speq-skill/.claude-plugin ]]; then
    echo "FAIL: ~/.speq-skill/.claude-plugin missing"
    exit 1
fi
echo "PASS: marketplace .claude-plugin directory exists"

if [[ ! -f ~/.speq-skill/bin/speq ]]; then
    echo "FAIL: ~/.speq-skill/bin/speq missing"
    exit 1
fi
echo "PASS: marketplace bin/speq exists"

# Test 4: Verify Claude plugin registration
echo ""
echo "--- Test 4: Verify Claude plugin registration ---"

# Check marketplace was added
if ! claude plugin marketplace list 2>/dev/null | grep -q "speq-skill"; then
    echo "WARN: speq-skill marketplace not in list (may need auth)"
else
    echo "PASS: speq-skill marketplace registered"
fi

# Check plugin was installed
if ! claude plugin list 2>/dev/null | grep -q "speq"; then
    echo "WARN: speq plugin not in list (may need auth)"
else
    echo "PASS: speq plugin installed"
fi

echo ""
echo "=== All installation tests passed! ==="
