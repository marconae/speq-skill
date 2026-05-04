#!/bin/bash
# Uninstall speq-skill CLI and Claude/Codex plugin payloads
# Usage: ./scripts/uninstall.sh

set -euo pipefail

BIN_PATH="${HOME}/.local/bin/speq"
INSTALL_DIR="${HOME}/.speq-skill"
CODEX_MARKETPLACE_NAME="speq-skill-local"
CODEX_LEGACY_MARKETPLACE_FILE="${HOME}/.agents/plugins/marketplace.json"
CODEX_SKILLS_DIR="${CODEX_HOME:-$HOME/.codex}/skills"

echo "=== Uninstalling speq-skill ==="

remove_legacy_codex_marketplace_entry() {
    [ -f "$CODEX_LEGACY_MARKETPLACE_FILE" ] || return

    if command -v python3 &> /dev/null; then
        python3 - "$CODEX_LEGACY_MARKETPLACE_FILE" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
if not path.exists() or not path.read_text().strip():
    raise SystemExit(0)

data = json.loads(path.read_text())
if isinstance(data.get("plugins"), list):
    data["plugins"] = [
        plugin for plugin in data["plugins"] if plugin.get("name") != "speq-skill"
    ]
    path.write_text(json.dumps(data, indent=2) + "\n")
PY
        echo "Removed legacy Codex marketplace entry: ${CODEX_LEGACY_MARKETPLACE_FILE}"
    elif command -v node &> /dev/null; then
        node - "$CODEX_LEGACY_MARKETPLACE_FILE" <<'JS'
const fs = require("fs");
const file = process.argv[2];
if (!fs.existsSync(file) || !fs.readFileSync(file, "utf8").trim()) process.exit(0);
const data = JSON.parse(fs.readFileSync(file, "utf8"));
if (Array.isArray(data.plugins)) {
  data.plugins = data.plugins.filter((plugin) => plugin.name !== "speq-skill");
  fs.writeFileSync(file, JSON.stringify(data, null, 2) + "\n");
}
JS
        echo "Removed legacy Codex marketplace entry: ${CODEX_LEGACY_MARKETPLACE_FILE}"
    else
        echo "python3/node unavailable; remove speq-skill from ${CODEX_LEGACY_MARKETPLACE_FILE} manually"
    fi
}

remove_codex_plugin() {
    if command -v codex &> /dev/null; then
        codex plugin marketplace remove "$CODEX_MARKETPLACE_NAME" >/dev/null 2>&1 \
            && echo "Removed Codex marketplace: ${CODEX_MARKETPLACE_NAME}" \
            || echo "Codex marketplace not installed or already removed"
    else
        echo "Codex CLI not found, skipping Codex marketplace removal"
    fi

    remove_legacy_codex_marketplace_entry
}

remove_codex_skills() {
    if [ ! -d "$CODEX_SKILLS_DIR" ]; then
        echo "Codex skills directory not found, skipping Codex skill removal"
        return
    fi

    local removed=0
    for target in "$CODEX_SKILLS_DIR"/speq-*; do
        [ -e "$target" ] || [ -L "$target" ] || continue

        if [ -L "$target" ]; then
            local link_target
            link_target="$(readlink "$target")"
            case "$link_target" in
                "$INSTALL_DIR"/codex/plugins/speq-skill/skills/*)
                    rm -f "$target"
                    removed=$((removed + 1))
                    ;;
            esac
        elif [ -d "$target" ] && [ -f "$target/.speq-skill-managed" ]; then
            rm -rf "$target"
            removed=$((removed + 1))
        fi
    done

    echo "Removed ${removed} Codex skill installation(s) from ${CODEX_SKILLS_DIR}"
}

# 1. Uninstall Claude plugin
if command -v claude &> /dev/null; then
    echo "Uninstalling speq-skill plugin..."
    claude plugin uninstall speq-skill@speq-skill 2>/dev/null || echo "  Plugin not installed or already removed"

    echo "Removing speq-skill marketplace..."
    claude plugin marketplace remove speq-skill 2>/dev/null || echo "  Marketplace not installed or already removed"
else
    echo "Claude CLI not found, skipping plugin removal"
fi

# 2. Remove Codex marketplace entry
remove_codex_plugin

# 3. Remove Codex skills
remove_codex_skills

# 4. Remove CLI binary
if [ -f "$BIN_PATH" ]; then
    echo "Removing CLI: ${BIN_PATH}"
    rm -f "$BIN_PATH"
else
    echo "CLI not found: ${BIN_PATH}"
fi

# 5. Remove marketplace installation directory
if [ -d "$INSTALL_DIR" ]; then
    echo "Removing installation directory: ${INSTALL_DIR}"
    rm -rf "$INSTALL_DIR"
fi

echo ""
echo "=== Uninstall complete ==="
