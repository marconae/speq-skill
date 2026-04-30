#!/bin/bash
# Integration test: verify Codex can load installed speq skills
set -euo pipefail

echo "=== Testing Codex plugin loading in Docker ==="

echo ""
echo "--- Test 1: Verify Codex CLI is available ---"
codex --version

echo ""
echo "--- Test 2: Install speq-skill ---"
./install.sh

echo ""
echo "--- Test 3: Verify Codex plugin payload and MCP config ---"
CODEX_PLUGIN_DIR="${HOME}/.speq-skill/codex/plugins/speq-skill"

if [[ ! -f "${CODEX_PLUGIN_DIR}/.codex-plugin/plugin.json" ]]; then
    echo "FAIL: Codex plugin manifest missing"
    exit 1
fi
echo "PASS: Codex plugin manifest exists"

if [[ ! -f "${CODEX_PLUGIN_DIR}/.mcp.json" ]]; then
    echo "FAIL: Codex MCP config missing"
    exit 1
fi
echo "PASS: Codex MCP config exists"

if ! grep -q '"serena"' "${CODEX_PLUGIN_DIR}/.mcp.json"; then
    echo "FAIL: Serena MCP config missing"
    exit 1
fi
if ! grep -q '"context7"' "${CODEX_PLUGIN_DIR}/.mcp.json"; then
    echo "FAIL: Context7 MCP config missing"
    exit 1
fi
echo "PASS: Serena and Context7 MCP config present"

if grep -R -E "AskUserQuestion|AskUserTool|Task\\(|subagent_type=|ExitPlanMode|Claude Code|CodexSubagent|Codex user-input prompt|Codex task" "${CODEX_PLUGIN_DIR}" >/tmp/codex-plugin-claude-syntax.txt; then
    echo "FAIL: Codex plugin contains Claude-only workflow syntax"
    cat /tmp/codex-plugin-claude-syntax.txt
    exit 1
fi
echo "PASS: Codex plugin has no Claude-only workflow syntax"

echo ""
echo "--- Test 4: Verify Codex marketplace registration and skills ---"
if [[ ! -f "${HOME}/.speq-skill/codex/.agents/plugins/marketplace.json" ]]; then
    echo "FAIL: Codex marketplace manifest missing"
    exit 1
fi

if ! grep -q '"path": "./plugins/speq-skill"' "${HOME}/.speq-skill/codex/.agents/plugins/marketplace.json"; then
    echo "FAIL: Codex marketplace manifest path missing"
    exit 1
fi

if [[ ! -f "${HOME}/.codex/config.toml" ]]; then
    echo "FAIL: Codex config missing"
    exit 1
fi

if ! grep -q '^\[marketplaces\.speq-skill-local\]' "${HOME}/.codex/config.toml"; then
    echo "FAIL: Codex marketplace registration missing"
    cat "${HOME}/.codex/config.toml"
    exit 1
fi

if ! grep -Fq "source = \"${HOME}/.speq-skill/codex\"" "${HOME}/.codex/config.toml"; then
    echo "FAIL: Codex marketplace source missing"
    cat "${HOME}/.codex/config.toml"
    exit 1
fi
echo "PASS: Codex marketplace registered"

for skill in cli code-guardrails code-tools ext-research git-discipline implement mission plan record; do
    if [[ ! -f "${HOME}/.codex/skills/speq-${skill}/SKILL.md" ]]; then
        echo "FAIL: Codex skill missing: speq-${skill}"
        exit 1
    fi
done
echo "PASS: Codex skills exist"

echo ""
echo "--- Test 5: Verify Codex loads /speq:* skill metadata ---"
PROMPT_INPUT="$(codex debug prompt-input "/speq:mission" 2>/tmp/codex-debug-stderr.txt)"

if ! grep -q -- "- speq:mission: Create specs/mission.md" <<< "$PROMPT_INPUT"; then
    echo "FAIL: Codex prompt input did not include speq:mission skill metadata"
    echo "--- codex debug stderr ---"
    cat /tmp/codex-debug-stderr.txt
    echo "--- prompt input excerpt ---"
    grep -n "Available skills\\|speq:" <<< "$PROMPT_INPUT" || true
    exit 1
fi

if ! grep -q -- "- speq:plan: Plan and create spec deltas" <<< "$PROMPT_INPUT"; then
    echo "FAIL: Codex prompt input did not include speq:plan skill metadata"
    echo "--- prompt input excerpt ---"
    grep -n "Available skills\\|speq:" <<< "$PROMPT_INPUT" || true
    exit 1
fi

echo "PASS: Codex loaded /speq:* skill metadata"

echo ""
echo "=== Codex plugin loading test passed! ==="
