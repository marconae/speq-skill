#!/bin/bash
# Install speq-skill marketplace and CLI
# Usage: curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/install.sh | bash
#
# For local testing (skips download):
#   SPEQ_LOCAL_ARCHIVE=/path/to/marketplace.tar.gz ./install.sh

set -euo pipefail

REPO="marconae/speq-skill"
INSTALL_DIR="${HOME}/.speq-skill"
BIN_DIR="${HOME}/.local/bin"

# Detect OS and architecture (short names)
detect_platform() {
    local os arch
    case "$(uname -s)" in
        Linux)  os="linux" ;;
        Darwin) os="mac" ;;
        *)
            echo "Error: Unsupported operating system: $(uname -s)"
            exit 1
            ;;
    esac
    case "$(uname -m)" in
        x86_64|amd64)   arch="x86_64" ;;
        arm64|aarch64)  arch="aarch64" ;;
        *)
            echo "Error: Unsupported architecture: $(uname -m)"
            exit 1
            ;;
    esac
    echo "${arch}-${os}"
}

# Get latest release version
get_latest_version() {
    curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name":' \
        | sed -E 's/.*"([^"]+)".*/\1/'
}

# Check if claude CLI is available
check_claude_cli() {
    if ! command -v claude &> /dev/null; then
        echo ""
        echo "Warning: Claude CLI not found in PATH"
        echo "Install from: https://docs.anthropic.com/en/docs/claude-code"
        echo ""
        echo "After installing Claude CLI, run:"
        echo "  claude plugin marketplace add $INSTALL_DIR"
        echo "  claude plugin install speq-skill@speq-skill"
        return 1
    fi
    return 0
}

# Install plugins via Claude CLI
install_plugins() {
    echo ""
    echo "Configuring Claude plugins..."

    # Add speq-skill marketplace
    echo "Adding speq-skill marketplace..."
    claude plugin marketplace add "$INSTALL_DIR"

    # Install speq-skill plugin (includes Serena and Context7 MCP servers)
    echo "Installing speq-skill plugin..."
    claude plugin install speq-skill@speq-skill

    echo ""
    echo "Plugin installed successfully!"
}

main() {
    echo "Installing speq-skill..."

    local tmpdir
    tmpdir=$(mktemp -d)
    trap "rm -rf $tmpdir" EXIT

    # Support local archive for testing (skips download)
    if [[ -n "${SPEQ_LOCAL_ARCHIVE:-}" ]]; then
        echo "Using local archive: $SPEQ_LOCAL_ARCHIVE"
        cp "$SPEQ_LOCAL_ARCHIVE" "$tmpdir/marketplace.tar.gz"
    else
        local platform
        platform=$(detect_platform)
        echo "Platform: ${platform}"

        local version
        version=$(get_latest_version)
        if [[ -z "$version" ]]; then
            echo "Error: Could not determine latest version"
            exit 1
        fi
        echo "Version: ${version}"

        # Download only the archive for this platform
        local archive="speq-marketplace-${version}-${platform}.tar.gz"
        local url="https://github.com/${REPO}/releases/download/${version}/${archive}"

        echo "Downloading ${archive}..."
        if ! curl -fsSL "$url" -o "$tmpdir/marketplace.tar.gz"; then
            echo "Error: Failed to download from ${url}"
            exit 1
        fi
    fi

    # Remove old installation
    if [ -d "$INSTALL_DIR" ]; then
        echo "Removing old installation..."
        rm -rf "$INSTALL_DIR"
    fi
    mkdir -p "$INSTALL_DIR"

    # Extract marketplace (handle both versioned and local directory names)
    tar -xzf "$tmpdir/marketplace.tar.gz" -C "$tmpdir"
    local extracted_dir
    extracted_dir=$(find "$tmpdir" -maxdepth 1 -type d -name "speq-marketplace-*" | head -1)
    if [[ -z "$extracted_dir" ]]; then
        echo "Error: Could not find extracted marketplace directory"
        exit 1
    fi
    cp -r "$extracted_dir/." "$INSTALL_DIR/"

    # Symlink CLI to bin
    mkdir -p "$BIN_DIR"
    ln -sf "$INSTALL_DIR/bin/speq" "$BIN_DIR/speq"
    chmod +x "$INSTALL_DIR/bin/speq"

    echo ""
    echo "Installed to: $INSTALL_DIR"
    echo "CLI symlinked: $BIN_DIR/speq -> $INSTALL_DIR/bin/speq"

    # Attempt to install plugins if claude CLI is available
    if check_claude_cli; then
        install_plugins
        echo ""
        echo "Ready to use! Start with: claude -> /speq:mission"
    fi

    if ! echo "$PATH" | grep -q "$BIN_DIR"; then
        echo ""
        echo "Note: Add to PATH: export PATH=\"$BIN_DIR:\$PATH\""
    fi
}

main
