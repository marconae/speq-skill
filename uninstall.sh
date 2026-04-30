#!/usr/bin/env bash
# speq-skill uninstaller
# Usage: curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/uninstall.sh | bash

set -e

INSTALL_DIR="$HOME/.local/bin"
MARKETPLACE_DIR="$HOME/.speq-skill"
CODEX_MARKETPLACE_NAME="speq-skill-local"
CODEX_LEGACY_MARKETPLACE_FILE="$HOME/.agents/plugins/marketplace.json"
CODEX_SKILLS_DIR="${CODEX_HOME:-$HOME/.codex}/skills"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

info() { echo -e "${GREEN}==>${NC} $1"; }
warn() { echo -e "${YELLOW}Warning:${NC} $1"; }
error() { echo -e "${RED}Error:${NC} $1"; exit 1; }
step() { echo -e "${BLUE}[${1}/${2}]${NC} $3"; }

remove_codex_plugin_with_python() {
    python3 - "$CODEX_LEGACY_MARKETPLACE_FILE" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
if not path.exists() or not path.read_text().strip():
    raise SystemExit(0)

data = json.loads(path.read_text())
plugins = data.get("plugins")
if isinstance(plugins, list):
    data["plugins"] = [plugin for plugin in plugins if plugin.get("name") != "speq-skill"]
    path.write_text(json.dumps(data, indent=2) + "\n")
PY
}

remove_codex_plugin_with_node() {
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
}

remove_codex_plugin() {
    if command -v codex &> /dev/null; then
        if codex plugin marketplace remove "$CODEX_MARKETPLACE_NAME" >/dev/null 2>&1; then
            info "Removed Codex marketplace: $CODEX_MARKETPLACE_NAME"
        else
            warn "Codex marketplace not registered or already removed"
        fi
    else
        warn "Codex CLI not found, skipping Codex marketplace removal"
    fi

    if [[ ! -f "$CODEX_LEGACY_MARKETPLACE_FILE" ]]; then
        return
    elif command -v python3 &> /dev/null; then
        if remove_codex_plugin_with_python; then
            info "Removed legacy Codex marketplace entry from $CODEX_LEGACY_MARKETPLACE_FILE"
        else
            warn "Failed to update legacy Codex marketplace JSON with python3"
        fi
    elif command -v node &> /dev/null; then
        if remove_codex_plugin_with_node; then
            info "Removed legacy Codex marketplace entry from $CODEX_LEGACY_MARKETPLACE_FILE"
        else
            warn "Failed to update legacy Codex marketplace JSON with node"
        fi
    else
        warn "python3/node unavailable; remove speq-skill from $CODEX_LEGACY_MARKETPLACE_FILE manually"
    fi
}

remove_codex_skills() {
    if [[ ! -d "$CODEX_SKILLS_DIR" ]]; then
        warn "Codex skills directory not found, skipping Codex skill removal"
        return
    fi

    local removed=0
    for target in "$CODEX_SKILLS_DIR"/speq-*; do
        [[ -e "$target" || -L "$target" ]] || continue

        if [[ -L "$target" ]]; then
            local link_target
            link_target=$(readlink "$target")
            if [[ "$link_target" == "$MARKETPLACE_DIR/codex/plugins/speq-skill/skills/"* ]]; then
                rm -f "$target"
                removed=$((removed + 1))
            fi
        elif [[ -d "$target" && -f "$target/.speq-skill-managed" ]]; then
            rm -rf "$target"
            removed=$((removed + 1))
        fi
    done

    info "Removed $removed Codex skill installation(s) from $CODEX_SKILLS_DIR"
}

main() {
    echo ""
    echo "============================"
    echo "   speq-skill uninstaller"
    echo "============================"
    echo ""

    local total=6

    # Step 1: Uninstall Claude plugin
    step 1 $total "Removing Claude plugin..."
    if command -v claude &> /dev/null; then
        claude plugin uninstall speq-skill@speq-skill 2>/dev/null || warn "Plugin not installed or already removed"
    else
        warn "Claude CLI not found, skipping plugin removal"
    fi

    # Step 2: Remove marketplace registration
    step 2 $total "Removing marketplace registration..."
    if command -v claude &> /dev/null; then
        claude plugin marketplace remove speq-skill 2>/dev/null || warn "Marketplace not registered or already removed"
    else
        warn "Claude CLI not found, skipping marketplace removal"
    fi

    # Step 3: Remove Codex marketplace entry
    step 3 $total "Removing Codex marketplace entry..."
    remove_codex_plugin

    # Step 4: Remove Codex skills
    step 4 $total "Removing Codex skills..."
    remove_codex_skills

    # Step 5: Remove CLI binary
    step 5 $total "Removing CLI binary..."
    if [ -f "$INSTALL_DIR/speq" ]; then
        rm -f "$INSTALL_DIR/speq"
        info "Removed $INSTALL_DIR/speq"
    else
        warn "$INSTALL_DIR/speq not found (already removed?)"
    fi

    # Step 6: Remove marketplace directory
    step 6 $total "Removing marketplace directory..."
    if [ -d "$MARKETPLACE_DIR" ]; then
        rm -rf "$MARKETPLACE_DIR"
        info "Removed $MARKETPLACE_DIR"
    else
        warn "$MARKETPLACE_DIR not found (already removed?)"
    fi

    echo ""
    info "Uninstall complete!"
    echo ""
    echo "Note: Rust toolchain was not removed (you may need it for other projects)."
    echo "To remove Rust: rustup self uninstall"
    echo ""
}

main "$@"
