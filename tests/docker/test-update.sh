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
if [[ "$VERSION_OUTPUT" != "speq 0.4.1" ]]; then
    echo "FAIL: expected speq 0.4.1 after first install, got: $VERSION_OUTPUT"
    exit 1
fi
echo "PASS: first install version: $VERSION_OUTPUT"

if [[ ! -d ~/.speq-skill/.claude-plugin ]]; then
    echo "FAIL: marketplace missing after first install"
    exit 1
fi
echo "PASS: marketplace exists after first install"

if [[ ! -f ~/.codex/config.toml ]]; then
    echo "FAIL: Codex config missing after first install"
    exit 1
fi
if ! grep -q '^\[marketplaces\.speq-skill-local\]' ~/.codex/config.toml; then
    echo "FAIL: Codex marketplace missing after first install"
    cat ~/.codex/config.toml
    exit 1
fi
echo "PASS: Codex marketplace exists after first install"

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
if [[ "$VERSION_OUTPUT" != "speq 0.4.1" ]]; then
    echo "FAIL: expected speq 0.4.1 after update, got: $VERSION_OUTPUT"
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

if [[ ! -f ~/.speq-skill/codex/plugins/speq-skill/.codex-plugin/plugin.json ]]; then
    echo "FAIL: Codex plugin manifest missing after update"
    exit 1
fi
echo "PASS: Codex plugin manifest exists after update"

if [[ ! -f ~/.codex/skills/speq-plan/SKILL.md ]]; then
    echo "FAIL: Codex speq-plan skill missing after update"
    exit 1
fi
echo "PASS: Codex speq-plan skill exists after update"

CODEX_ENTRY_COUNT=$(grep -c '^\[marketplaces\.speq-skill-local\]' ~/.codex/config.toml || true)
if [[ "$CODEX_ENTRY_COUNT" -ne 1 ]]; then
    echo "FAIL: expected one Codex marketplace registration, found $CODEX_ENTRY_COUNT"
    cat ~/.codex/config.toml
    exit 1
fi
if ! grep -Fq "source = \"${HOME}/.speq-skill/codex\"" ~/.codex/config.toml; then
    echo "FAIL: Codex marketplace source missing after update"
    cat ~/.codex/config.toml
    exit 1
fi
echo "PASS: Codex marketplace registration is idempotent"

if ! ~/.local/bin/speq --help > /dev/null 2>&1; then
    echo "FAIL: speq --help failed after update"
    exit 1
fi
echo "PASS: speq --help works after update"

echo ""
echo "=== All update tests passed! ==="
