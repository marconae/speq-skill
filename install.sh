#!/bin/bash
# Install speq CLI and Claude Code plugin
# Usage: curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/install.sh | bash
# Or: curl -fsSL ... | bash -s -- --to ~/bin --plugin-only

set -euo pipefail

REPO="marconae/speq-skill"
INSTALL_DIR="${HOME}/.local/bin"
PLUGIN_DIR="${HOME}/.claude/plugins/speq"
INSTALL_CLI=true
INSTALL_PLUGIN=true

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --to)
            INSTALL_DIR="$2"
            shift 2
            ;;
        --plugin-only)
            INSTALL_CLI=false
            shift
            ;;
        --cli-only)
            INSTALL_PLUGIN=false
            shift
            ;;
        --help|-h)
            echo "Install speq CLI and Claude Code plugin"
            echo ""
            echo "Usage:"
            echo "  curl -fsSL https://raw.githubusercontent.com/$REPO/main/install.sh | bash"
            echo "  curl -fsSL ... | bash -s -- [options]"
            echo ""
            echo "Options:"
            echo "  --to DIR       Install CLI to custom directory (default: ~/.local/bin)"
            echo "  --cli-only     Install only the CLI, skip plugin"
            echo "  --plugin-only  Install only the plugin, skip CLI"
            echo "  --help         Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

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

install_cli() {
    local version="$1"
    local platform="$2"
    local tmpdir="$3"

    local archive_name="speq-${version}-${platform}.tar.gz"
    local download_url="https://github.com/${REPO}/releases/download/${version}/${archive_name}"

    echo "Downloading CLI: ${archive_name}..."
    if ! curl -fsSL "$download_url" -o "$tmpdir/cli.tar.gz"; then
        echo "Error: Failed to download CLI from ${download_url}"
        exit 1
    fi

    tar -xzf "$tmpdir/cli.tar.gz" -C "$tmpdir"
    mkdir -p "$INSTALL_DIR"
    cp "$tmpdir/speq-${version}-${platform}/speq" "$INSTALL_DIR/speq"
    chmod +x "$INSTALL_DIR/speq"

    echo "Installed CLI to ${INSTALL_DIR}/speq"

    if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
        echo ""
        echo "Warning: ${INSTALL_DIR} is not in your PATH."
        echo "Add to your shell profile: export PATH=\"${INSTALL_DIR}:\$PATH\""
    fi
}

install_plugin() {
    local version="$1"
    local tmpdir="$2"

    local archive_name="speq-plugin-${version}.tar.gz"
    local download_url="https://github.com/${REPO}/releases/download/${version}/${archive_name}"

    echo "Downloading plugin: ${archive_name}..."
    if ! curl -fsSL "$download_url" -o "$tmpdir/plugin.tar.gz"; then
        echo "Error: Failed to download plugin from ${download_url}"
        exit 1
    fi

    tar -xzf "$tmpdir/plugin.tar.gz" -C "$tmpdir"

    # Remove existing plugin installation
    if [ -d "$PLUGIN_DIR" ]; then
        echo "Removing existing plugin installation..."
        rm -rf "$PLUGIN_DIR"
    fi

    # Install plugin
    mkdir -p "$(dirname "$PLUGIN_DIR")"
    cp -r "$tmpdir/speq-plugin-${version}" "$PLUGIN_DIR"

    echo "Installed plugin to ${PLUGIN_DIR}"
    echo ""
    echo "To activate the plugin, run:"
    echo "  claude plugin add $PLUGIN_DIR"
}

main() {
    echo "Installing speq..."

    local platform
    platform=$(detect_platform)
    echo "Detected platform: ${platform}"

    local version
    version=$(get_latest_version)
    if [[ -z "$version" ]]; then
        echo "Error: Could not determine latest version"
        exit 1
    fi
    echo "Latest version: ${version}"

    local tmpdir
    tmpdir=$(mktemp -d)
    trap "rm -rf $tmpdir" EXIT

    if [ "$INSTALL_CLI" = true ]; then
        install_cli "$version" "$platform" "$tmpdir"
    fi

    if [ "$INSTALL_PLUGIN" = true ]; then
        install_plugin "$version" "$tmpdir"
    fi

    echo ""
    echo "Installation complete!"

    if [ "$INSTALL_CLI" = true ]; then
        echo "  CLI: speq --help"
    fi
    if [ "$INSTALL_PLUGIN" = true ]; then
        echo "  Plugin: claude plugin add $PLUGIN_DIR"
    fi
}

main
