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
if [[ "$VERSION_OUTPUT" != "speq 0.5.0" ]]; then
    echo "FAIL: expected speq 0.5.0, got: $VERSION_OUTPUT"
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

if [[ ! -f ~/.speq-skill/codex/plugins/speq-skill/.codex-plugin/plugin.json ]]; then
    echo "FAIL: Codex plugin manifest missing"
    exit 1
fi
echo "PASS: Codex plugin manifest exists"

if [[ ! -f ~/.speq-skill/codex/.agents/plugins/marketplace.json ]]; then
    echo "FAIL: Codex marketplace manifest missing"
    exit 1
fi
if ! grep -q '"path": "./plugins/speq-skill"' ~/.speq-skill/codex/.agents/plugins/marketplace.json; then
    echo "FAIL: Codex marketplace manifest path missing"
    exit 1
fi
echo "PASS: Codex marketplace manifest exists"

if [[ ! -f ~/.speq-skill/codex/plugins/speq-skill/.mcp.json ]]; then
    echo "FAIL: Codex MCP config missing"
    exit 1
fi
echo "PASS: Codex MCP config exists"

if ! grep -q '^name: speq:plan$' ~/.speq-skill/codex/plugins/speq-skill/skills/plan/SKILL.md; then
    echo "FAIL: Codex /speq:plan skill name missing"
    exit 1
fi
echo "PASS: Codex /speq:plan skill registered"

if [[ ! -f ~/.codex/skills/speq-plan/SKILL.md ]]; then
    echo "FAIL: Codex speq-plan skill missing"
    exit 1
fi
echo "PASS: Codex speq-plan skill exists"

if [[ ! -f ~/.codex/config.toml ]]; then
    echo "FAIL: Codex config missing"
    exit 1
fi
if ! grep -q '^\[marketplaces\.speq-skill-local\]' ~/.codex/config.toml; then
    echo "FAIL: Codex marketplace registration missing"
    cat ~/.codex/config.toml
    exit 1
fi
if ! grep -Fq "source = \"${HOME}/.speq-skill/codex\"" ~/.codex/config.toml; then
    echo "FAIL: Codex marketplace source missing"
    cat ~/.codex/config.toml
    exit 1
fi
echo "PASS: Codex marketplace registered"

if ! grep -q '^\[mcp_servers\.serena\]' ~/.codex/config.toml; then
    echo "FAIL: Codex Serena MCP registration missing"
    cat ~/.codex/config.toml
    exit 1
fi
if ! grep -q '^\[mcp_servers\.context7\]' ~/.codex/config.toml; then
    echo "FAIL: Codex Context7 MCP registration missing"
    cat ~/.codex/config.toml
    exit 1
fi
echo "PASS: Codex MCP servers registered"

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
