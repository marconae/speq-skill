#!/usr/bin/env bash
# speq-skill installer
# Usage: curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/install.sh | bash

set -e

REPO="marconae/speq-skill"
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

# Get latest release tag from GitHub API
get_latest_version() {
    local response
    response=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" 2>/dev/null) || {
        warn "No releases found, using main branch"
        echo "main"
        return
    }
    echo "$response" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
}

# Check for Rust toolchain
check_rust() {
    if command -v cargo &> /dev/null; then
        info "Rust toolchain found: $(cargo --version)"
        return 0
    fi
    return 1
}

# Offer to install Rust
install_rust() {
    warn "Rust toolchain not found."
    echo ""
    echo "speq requires Rust to build from source."
    echo "Would you like to install Rust via rustup? (recommended)"
    echo ""
    read -p "Install Rust? [y/N] " -n 1 -r
    echo ""

    if [[ $REPLY =~ ^[Yy]$ ]]; then
        info "Installing Rust via rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        # Source cargo env for this session
        source "$HOME/.cargo/env"
        info "Rust installed successfully!"
    else
        error "Rust is required. Install manually: https://rustup.rs"
    fi
}

# Download and build from source
#
# Environment variables for testing:
#   SPEQ_LOCAL_TARBALL - Path to local source tarball (skips GitHub download)
#   SPEQ_PREBUILT      - If set and target/release/speq exists, skip cargo build
#
build_from_source() {
    local version="$1"
    local tmp_dir
    tmp_dir=$(mktemp -d)
    trap "rm -rf $tmp_dir" EXIT

    # Support local tarball for testing (skips GitHub download)
    if [[ -n "${SPEQ_LOCAL_TARBALL:-}" ]]; then
        step 1 5 "Using local tarball: $SPEQ_LOCAL_TARBALL"
        cp "$SPEQ_LOCAL_TARBALL" "$tmp_dir/source.tar.gz"
    else
        local archive_url
        if [[ "$version" == "main" ]]; then
            archive_url="https://github.com/$REPO/archive/refs/heads/main.tar.gz"
        else
            archive_url="https://github.com/$REPO/archive/refs/tags/${version}.tar.gz"
        fi
        step 1 5 "Downloading speq-skill ${version}..."
        curl -fsSL "$archive_url" -o "$tmp_dir/source.tar.gz"
    fi

    step 2 5 "Extracting source..."
    tar -xzf "$tmp_dir/source.tar.gz" -C "$tmp_dir"
    cd "$tmp_dir"/speq-skill-*

    # Support pre-built binary (skips cargo build)
    if [[ -n "${SPEQ_PREBUILT:-}" ]] && [[ -f "target/release/speq" ]]; then
        step 3 5 "Using pre-built binary..."
    else
        step 3 5 "Building from source (this may take a moment)..."
        cargo build --release
    fi

    step 4 5 "Building plugin..."
    ./scripts/plugin/build.sh

    step 5 5 "Installing..."

    # Install binary
    mkdir -p "$INSTALL_DIR"
    cp target/release/speq "$INSTALL_DIR/speq"
    chmod +x "$INSTALL_DIR/speq"
    info "Installed speq to $INSTALL_DIR/speq"

    # Install marketplace (use /. to include hidden files like .claude-plugin)
    rm -rf "$MARKETPLACE_DIR"
    mkdir -p "$MARKETPLACE_DIR"
    cp -r dist/marketplace/. "$MARKETPLACE_DIR/"
    mkdir -p "$MARKETPLACE_DIR/bin"
    cp target/release/speq "$MARKETPLACE_DIR/bin/speq"
    chmod +x "$MARKETPLACE_DIR/bin/speq"

    # Register with Claude CLI
    if command -v claude &> /dev/null; then
        info "Registering plugin with Claude CLI..."
        claude plugin marketplace add "$MARKETPLACE_DIR" 2>/dev/null || true
        claude plugin install speq-skill@speq-skill 2>/dev/null || true
    else
        warn "Claude CLI not found. Run these commands after installing Claude:"
        echo "  claude plugin marketplace add $MARKETPLACE_DIR"
        echo "  claude plugin install speq-skill@speq-skill"
    fi
}

# Check if ~/.local/bin is in PATH
check_path() {
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        warn "$INSTALL_DIR is not in your PATH"
        echo ""
        echo "Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
        echo ""
    fi
}

main() {
    echo ""
    echo "=========================="
    echo "   speq-skill installer"
    echo "=========================="
    echo ""

    # Check prerequisites (skip if using pre-built binary)
    if [[ -z "${SPEQ_PREBUILT:-}" ]]; then
        if ! check_rust; then
            install_rust
        fi
    fi

    # Get version
    info "Checking for latest release..."
    local version
    version=$(get_latest_version)
    info "Installing version: $version"
    echo ""

    # Build and install
    build_from_source "$version"

    # Post-install checks
    check_path

    echo ""
    info "Installation complete!"
    echo ""
    echo "Run 'speq --help' to get started."
    echo ""
}

main "$@"
