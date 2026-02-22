#!/usr/bin/env bash
# speq-skill uninstaller
# Usage: curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/uninstall.sh | bash

set -e

INSTALL_DIR="$HOME/.local/bin"
MARKETPLACE_DIR="$HOME/.speq-skill"

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

main() {
    echo ""
    echo "============================"
    echo "   speq-skill uninstaller"
    echo "============================"
    echo ""

    local total=4

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

    # Step 3: Remove CLI binary
    step 3 $total "Removing CLI binary..."
    if [ -f "$INSTALL_DIR/speq" ]; then
        rm -f "$INSTALL_DIR/speq"
        info "Removed $INSTALL_DIR/speq"
    else
        warn "$INSTALL_DIR/speq not found (already removed?)"
    fi

    # Step 4: Remove marketplace directory
    step 4 $total "Removing marketplace directory..."
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
