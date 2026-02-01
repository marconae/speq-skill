#!/bin/bash
# Install speq-skill marketplace and CLI
# Usage: curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/install.sh | bash

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

main() {
    echo "Installing speq-skill..."

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

    local tmpdir
    tmpdir=$(mktemp -d)
    trap "rm -rf $tmpdir" EXIT

    echo "Downloading ${archive}..."
    if ! curl -fsSL "$url" -o "$tmpdir/marketplace.tar.gz"; then
        echo "Error: Failed to download from ${url}"
        exit 1
    fi

    # Remove old installation
    if [ -d "$INSTALL_DIR" ]; then
        echo "Removing old installation..."
        rm -rf "$INSTALL_DIR"
    fi
    mkdir -p "$INSTALL_DIR"

    # Extract marketplace
    tar -xzf "$tmpdir/marketplace.tar.gz" -C "$tmpdir"
    cp -r "$tmpdir/speq-marketplace-${version}-${platform}/." "$INSTALL_DIR/"

    # Symlink CLI to bin
    mkdir -p "$BIN_DIR"
    ln -sf "$INSTALL_DIR/bin/speq" "$BIN_DIR/speq"
    chmod +x "$INSTALL_DIR/bin/speq"

    echo ""
    echo "Installed to: $INSTALL_DIR"
    echo "CLI symlinked: $BIN_DIR/speq -> $INSTALL_DIR/bin/speq"
    echo ""
    echo "Next steps:"
    echo "  1. Add marketplace:  claude plugin marketplace add $INSTALL_DIR"
    echo "  2. Install plugin:   claude plugin install speq@speq-skill"
    echo "  3. Start using:      claude -> /speq:mission"

    if ! echo "$PATH" | grep -q "$BIN_DIR"; then
        echo ""
        echo "Note: Add to PATH: export PATH=\"$BIN_DIR:\$PATH\""
    fi
}

main
