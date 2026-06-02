#!/usr/bin/env bash
# speq-skill installer
# Usage: curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/install.sh | bash

set -e

REPO="marconae/speq-skill"
INSTALL_DIR="$HOME/.local/bin"
MARKETPLACE_DIR="$HOME/.speq-skill"
CODEX_MARKETPLACE_NAME="speq-skill-local"
CODEX_MARKETPLACE_ROOT="$MARKETPLACE_DIR/codex"
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
error_noexit() { echo -e "${RED}Error:${NC} $1"; }
step() { echo -e "${BLUE}[${1}/${2}]${NC} $3"; }

# Get latest release tag from GitHub API
get_latest_version() {
    local response
    response=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" 2>/dev/null) || {
        warn "No releases found, using main branch" >&2
        echo "main"
        return
    }
    echo "$response" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
}

# Check Linux build dependencies (OpenSSL dev headers, pkg-config)
check_linux_deps() {
    local os
    os=$(uname -s)
    [[ "$os" == "Linux" ]] || return 0

    local missing=()

    if ! command -v pkg-config &> /dev/null; then
        missing+=("pkg-config")
    fi

    if [[ ${#missing[@]} -eq 0 ]] && ! pkg-config --exists openssl 2>/dev/null; then
        missing+=("openssl-dev")
    fi

    if [[ ${#missing[@]} -eq 0 ]]; then
        local ssl_version
        ssl_version=$(pkg-config --modversion openssl 2>/dev/null || echo "0")
        local ssl_major="${ssl_version%%.*}"
        if [[ "$ssl_major" -lt 3 ]] 2>/dev/null; then
            echo ""
            error_noexit "OpenSSL ${ssl_version} found, but version 3.0+ is required (license compatibility)."
            echo ""
            echo "  Please upgrade your system's OpenSSL to 3.0 or later."
            echo "  On Ubuntu 22.04+, this is the default. On older systems, consider upgrading your distro."
            echo ""
            exit 1
        fi
        return 0
    fi

    echo ""
    error_noexit "Missing build dependencies: ${missing[*]}"
    echo ""
    echo "  speq requires OpenSSL development headers and pkg-config to build."
    echo ""
    echo "  Install them for your distro:"
    echo "    Debian/Ubuntu:  sudo apt-get install pkg-config libssl-dev"
    echo "    Fedora/RHEL:    sudo dnf install pkg-config openssl-devel"
    echo "    Arch:           sudo pacman -S pkg-config openssl"
    echo ""
    exit 1
}

# Check for Rust toolchain
check_rust() {
    if command -v cargo &> /dev/null; then
        info "Rust toolchain found: $(cargo --version)"
        return 0
    fi
    return 1
}

register_codex_plugin() {
    if [[ ! -f "$CODEX_MARKETPLACE_ROOT/.agents/plugins/marketplace.json" ]]; then
        warn "Codex plugin payload missing; skipping Codex registration"
        return
    fi

    if command -v codex &> /dev/null; then
        info "Registering Codex marketplace..."
        codex plugin marketplace remove "$CODEX_MARKETPLACE_NAME" >/dev/null 2>&1 || true
        if codex plugin marketplace add "$CODEX_MARKETPLACE_ROOT" >/dev/null 2>&1; then
            info "Registered Codex marketplace: $CODEX_MARKETPLACE_NAME"
        else
            warn "Codex marketplace registration failed."
            echo "  Run manually: codex plugin marketplace add $CODEX_MARKETPLACE_ROOT"
        fi
    else
        warn "Codex CLI not found. Register the marketplace after installing Codex:"
        echo "  codex plugin marketplace add $CODEX_MARKETPLACE_ROOT"
    fi
}

register_codex_mcp_servers() {
    if command -v codex &> /dev/null; then
        info "Registering Codex MCP servers..."

        if codex mcp get serena >/dev/null 2>&1; then
            info "Codex MCP server already registered: serena"
        elif codex mcp add serena -- uvx --from git+https://github.com/oraios/serena serena start-mcp-server --project-from-cwd --context=codex >/dev/null 2>&1; then
            info "Registered Codex MCP server: serena"
        else
            warn "Codex MCP server registration failed: serena"
            echo "  Run manually: codex mcp add serena -- uvx --from git+https://github.com/oraios/serena serena start-mcp-server --project-from-cwd --context=codex"
        fi

        if codex mcp get context7 >/dev/null 2>&1; then
            info "Codex MCP server already registered: context7"
        elif codex mcp add context7 -- npx -y @upstash/context7-mcp >/dev/null 2>&1; then
            info "Registered Codex MCP server: context7"
        else
            warn "Codex MCP server registration failed: context7"
            echo "  Run manually: codex mcp add context7 -- npx -y @upstash/context7-mcp"
        fi
    else
        warn "Codex CLI not found. Register MCP servers after installing Codex:"
        echo "  codex mcp add serena -- uvx --from git+https://github.com/oraios/serena serena start-mcp-server --project-from-cwd --context=codex"
        echo "  codex mcp add context7 -- npx -y @upstash/context7-mcp"
    fi
}

install_codex_skills() {
    local source_dir="$MARKETPLACE_DIR/codex/plugins/speq-skill/skills"

    if [[ ! -d "$source_dir" ]]; then
        warn "Codex skills payload missing; skipping Codex skill installation"
        return
    fi

    mkdir -p "$CODEX_SKILLS_DIR"

    for skill_dir in "$source_dir"/*; do
        [[ -d "$skill_dir" ]] || continue

        local skill_name
        local target
        skill_name=$(basename "$skill_dir")
        target="$CODEX_SKILLS_DIR/speq-$skill_name"

        if [[ -L "$target" ]]; then
            rm -f "$target"
        elif [[ -d "$target" && -f "$target/.speq-skill-managed" ]]; then
            rm -rf "$target"
        elif [[ -e "$target" ]]; then
            warn "Codex skill already exists and is not managed by speq-skill, skipping: $target"
            continue
        fi

        cp -R "$skill_dir" "$target"
        touch "$target/.speq-skill-managed"
    done

    info "Installed Codex /speq:* skills into $CODEX_SKILLS_DIR"
}

# Offer to install Rust
install_rust() {
    warn "Rust toolchain not found."
    echo ""
    echo "speq requires Rust to build from source."
    echo "Would you like to install Rust via rustup? (recommended)"
    echo ""

    if [[ -e /dev/tty ]]; then
        read -p "Install Rust? [y/N] " -n 1 -r < /dev/tty
        echo ""
    else
        # Non-interactive (CI/Docker) — auto-install
        REPLY="y"
    fi

    if [[ $REPLY =~ ^[Yy]$ ]]; then
        info "Installing Rust via rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        # Source cargo env for this session
        source "$HOME/.cargo/env" || true
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
        # Unregister old plugin/marketplace first (idempotent for updates)
        claude plugin uninstall speq-skill@speq-skill 2>/dev/null || true
        claude plugin marketplace remove speq-skill 2>/dev/null || true
        claude plugin marketplace add "$MARKETPLACE_DIR" 2>/dev/null || true
        claude plugin install speq-skill@speq-skill 2>/dev/null || true
    else
        warn "Claude CLI not found. Run these commands after installing Claude:"
        echo "  claude plugin marketplace add $MARKETPLACE_DIR"
        echo "  claude plugin install speq-skill@speq-skill"
    fi

    # Register with Codex marketplace
    register_codex_plugin
    register_codex_mcp_servers
    install_codex_skills
}

# Download the embedding model files from HuggingFace into the model cache directory
provision_embedding_model() {
    local HUGGINGFACE_BASE="https://huggingface.co/Snowflake/snowflake-arctic-embed-xs/resolve/main"

    if [[ -n "${SPEQ_CACHE_DIR:-}" ]]; then
        local MODEL_DIR="${SPEQ_CACHE_DIR}/models"
    else
        local xdg_cache="${XDG_CACHE_HOME:-$HOME/.cache}"
        local MODEL_DIR="${xdg_cache}/speq/models"
    fi

    local files=("model.onnx" "tokenizer.json")

    local all_cached=true
    for filename in "${files[@]}"; do
        [[ -f "$MODEL_DIR/$filename" ]] || { all_cached=false; break; }
    done
    if [[ "$all_cached" == true ]]; then
        info "Embedding model already provisioned in $MODEL_DIR"
        return 0
    fi

    # Drop any previously provisioned model files so upgrades always get a fresh copy
    if [[ -d "$MODEL_DIR" ]]; then
        for filename in "${files[@]}"; do
            rm -f "$MODEL_DIR/$filename"
        done
    fi

    info "Provisioning embedding model into $MODEL_DIR..."
    mkdir -p "$MODEL_DIR"

    for filename in "${files[@]}"; do
        local dest="$MODEL_DIR/$filename"
        local url
        case "$filename" in
            model.onnx) url="$HUGGINGFACE_BASE/onnx/model.onnx" ;;
            *)          url="$HUGGINGFACE_BASE/$filename" ;;
        esac
        if ! curl -fsSL "$url" -o "${dest}.tmp"; then
            rm -f "${dest}.tmp"
            echo ""
            echo -e "${RED}Error:${NC} Failed to download $filename from HuggingFace."
            echo "  To provision manually: curl -fsSL \"$url\" -o \"$MODEL_DIR/$filename\""
            echo ""
            exit 1
        fi
        mv "${dest}.tmp" "$dest"
    done

    info "Embedding model provisioned in $MODEL_DIR"
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
        check_linux_deps
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

    # Provision embedding model
    provision_embedding_model

    # Post-install checks
    check_path

    echo ""
    echo "========================================"
    info "Installation complete!"
    echo "========================================"
    echo ""
    echo "  Binary:      $INSTALL_DIR/speq"
    echo "  Plugin:      $MARKETPLACE_DIR/"
    echo "  Codex:       $CODEX_MARKETPLACE_ROOT"
    if command -v claude &> /dev/null; then
        echo "  Claude CLI:  plugin registered"
    else
        echo "  Claude CLI:  not found (register manually after installing)"
    fi
    if command -v codex &> /dev/null; then
        echo "  Codex CLI:   marketplace registered"
    else
        echo "  Codex CLI:   not found (marketplace entry created)"
    fi
    echo ""
    echo "Run 'speq --help' to get started."
    echo ""
}

if [[ -z "${BASH_SOURCE[0]:-}" ]] || [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
