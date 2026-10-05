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
CODEX_SERENA_ADD="codex mcp add serena -- serena start-mcp-server --project-from-cwd --context=codex"
CLAUDE_SERENA_ADD="claude mcp add --scope user serena -- serena start-mcp-server --context claude-code --project-from-cwd"
SERENA_CLI_INSTALL="uv tool install -p 3.13 serena-agent"
# Commit id of the model repository that the installer downloads. Change it to change the model.
EMBEDDING_MODEL_REVISION="d8c86521100d3556476a063fc2342036d45c106f"

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
        warn "No releases found, using main branch" >&2
        echo "main"
        return
    }
    echo "$response" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
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

# Detect platform triple understood by the release archive naming convention
detect_platform() {
    local arch os
    arch=$(uname -m)
    os=$(uname -s)

    case "$arch" in
        x86_64)        arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *)             echo ""; return ;;
    esac

    case "$os" in
        Linux)  os="linux" ;;
        Darwin) os="mac" ;;
        *)      echo ""; return ;;
    esac

    echo "${arch}-${os}"
}

# Download a pre-built release archive and install it directly (no compilation).
# Returns 0 on success, 1 if no pre-built binary is available for this platform
# or if the download fails.
install_from_prebuilt() {
    local version="$1"

    # When a local source tarball is provided (Docker test mode), always compile
    [[ -n "${SPEQ_LOCAL_TARBALL:-}" ]] && return 1

    local platform
    platform=$(detect_platform)
    [[ -z "$platform" ]] && return 1

    # Strip leading 'v' for crate version, but the tag name is used in the asset URL
    local archive_name="speq-marketplace-${version}-${platform}.tar.gz"
    local download_url="https://github.com/$REPO/releases/download/${version}/${archive_name}"

    local tmp_dir
    tmp_dir=$(mktemp -d)
    trap "rm -rf $tmp_dir" RETURN

    info "Downloading pre-built binary for ${platform}..."
    if ! curl -fsSL "$download_url" -o "$tmp_dir/release.tar.gz" 2>/dev/null; then
        warn "No pre-built binary available for ${platform} (${version})."
        return 1
    fi

    step 1 3 "Extracting..."
    tar -xzf "$tmp_dir/release.tar.gz" -C "$tmp_dir"

    local extract_dir="$tmp_dir/speq-marketplace-${version}-${platform}"

    step 2 3 "Installing..."

    mkdir -p "$INSTALL_DIR"
    cp "$extract_dir/bin/speq" "$INSTALL_DIR/speq"
    chmod +x "$INSTALL_DIR/speq"
    info "Installed speq to $INSTALL_DIR/speq"

    rm -rf "$MARKETPLACE_DIR"
    mkdir -p "$MARKETPLACE_DIR"
    cp -r "$extract_dir/." "$MARKETPLACE_DIR/"
    info "Installed marketplace to $MARKETPLACE_DIR"

    step 3 3 "Registering plugins..."

    if command -v claude &> /dev/null; then
        info "Registering plugin with Claude CLI..."
        claude plugin uninstall speq-skill@speq-skill 2>/dev/null || true
        claude plugin marketplace remove speq-skill 2>/dev/null || true
        claude plugin marketplace add "$MARKETPLACE_DIR" 2>/dev/null || true
        claude plugin install speq-skill@speq-skill 2>/dev/null || true
    else
        warn "Claude CLI not found. Run these commands after installing Claude:"
        echo "  claude plugin marketplace add $MARKETPLACE_DIR"
        echo "  claude plugin install speq-skill@speq-skill"
    fi

    register_codex_plugin
    install_codex_skills

    return 0
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
    install_codex_skills
}

# Resolve the platform-native cache directory, mirroring the `dirs` crate's
# split used by the speq binary's own get_cache_path(): macOS uses
# ~/Library/Caches, everything else follows the XDG convention.
default_cache_dir() {
    local os
    os=$(uname -s)
    case "$os" in
        Darwin) echo "$HOME/Library/Caches" ;;
        *)      echo "${XDG_CACHE_HOME:-$HOME/.cache}" ;;
    esac
}

# Serena comes from the user's global install. The helpers below detect it and
# offer to install or enable it. They never install silently. Context7 is not
# managed here: the skills use it when it is there.
#
# Prompts read from /dev/tty, not stdin, so they also work under
# `curl ... | bash`. SPEQ_TTY overrides the device (used by the tests).
has_tty() { { : < "${SPEQ_TTY:-/dev/tty}"; } 2>/dev/null; }

ask_yes_no() {
    has_tty || return 1
    local reply
    read -p "$1 [y/N] " -n 1 -r reply < "${SPEQ_TTY:-/dev/tty}" || return 1
    echo ""
    [[ $reply =~ ^[Yy]$ ]]
}

# Make sure the Serena CLI exists for the MCP registration.
ensure_serena_cli() {
    command -v serena &> /dev/null && return 0

    if command -v uv &> /dev/null; then
        info "Installing Serena CLI via uv..."
        if $SERENA_CLI_INSTALL >/dev/null 2>&1; then
            return 0
        fi
        warn "Serena CLI installation failed."
        echo "  Run manually: $SERENA_CLI_INSTALL"
    else
        warn "uv not found. Install uv, then install the Serena CLI:"
        echo "  curl -LsSf https://astral.sh/uv/install.sh | sh"
        echo "  $SERENA_CLI_INSTALL"
    fi
    return 1
}

# offer_action <verb> <past-tense> <server> <host> <command> [prepare-function]
offer_action() {
    local verb="$1" past="$2" server="$3" host="$4" command="$5" prepare="${6:-}"

    if ! ask_yes_no "$verb $server for $host?"; then
        if ! has_tty; then
            warn "$server is not $(echo "$past" | tr 'A-Z' 'a-z') for $host. Run:"
            if [[ -n "$prepare" ]] && ! command -v serena &> /dev/null; then
                echo "  $SERENA_CLI_INSTALL"
            fi
            echo "  $command"
        fi
        return 0
    fi

    if [[ -n "$prepare" ]] && ! $prepare; then
        return 0
    fi

    if $command >/dev/null 2>&1; then
        info "$past $server for $host"
    else
        warn "$verb $server for $host failed."
        echo "  Run manually: $command"
    fi
    return 0
}

# Serena counts as installed for Claude Code when it is registered as an MCP
# server (any scope) or when a Serena plugin is enabled.
claude_has_serena() {
    claude mcp get serena >/dev/null 2>&1 && return 0
    claude plugin list 2>/dev/null | awk '
        /❯/ { cur = ($2 ~ /^serena@/) }
        cur && /Status:/ { found = ($0 !~ /✘|disabled/); exit }
        END { exit !found }'
}

offer_mcp_servers() {
    if command -v claude &> /dev/null && ! claude_has_serena; then
        offer_action Install Installed serena "Claude Code" "$CLAUDE_SERENA_ADD" ensure_serena_cli
    fi

    if command -v codex &> /dev/null && ! codex mcp get serena >/dev/null 2>&1; then
        offer_action Install Installed serena "Codex" "$CODEX_SERENA_ADD" ensure_serena_cli
    fi
    return 0
}

provision_embedding_model() {
    local HUGGINGFACE_BASE="https://huggingface.co/Snowflake/snowflake-arctic-embed-xs/resolve/$EMBEDDING_MODEL_REVISION"

    if [[ -n "${SPEQ_CACHE_DIR:-}" ]]; then
        local MODEL_DIR="${SPEQ_CACHE_DIR}/models"
    else
        local cache_dir
        cache_dir=$(default_cache_dir)
        local MODEL_DIR="${cache_dir}/speq/models"
    fi

    local files=("model.onnx" "tokenizer.json")
    local stamp="$MODEL_DIR/revision"

    local all_cached=true
    for filename in "${files[@]}"; do
        [[ -f "$MODEL_DIR/$filename" ]] || { all_cached=false; break; }
    done
    if [[ "$all_cached" == true && -f "$stamp" ]] \
        && [[ "$(cat "$stamp")" == "$EMBEDDING_MODEL_REVISION" ]]; then
        info "Embedding model already provisioned in $MODEL_DIR"
        return 0
    fi

    info "Provisioning embedding model into $MODEL_DIR..."
    mkdir -p "$MODEL_DIR"

    local urls=()
    for filename in "${files[@]}"; do
        case "$filename" in
            model.onnx) urls+=("$HUGGINGFACE_BASE/onnx/model.onnx") ;;
            *)          urls+=("$HUGGINGFACE_BASE/$filename") ;;
        esac
    done

    local i
    for i in "${!files[@]}"; do
        if ! curl -fsSL "${urls[$i]}" -o "$MODEL_DIR/${files[$i]}.$$.tmp"; then
            local failed="${files[$i]}"
            for filename in "${files[@]}"; do
                rm -f "$MODEL_DIR/$filename.$$.tmp"
            done
            echo ""
            echo -e "${RED}Error:${NC} Failed to download $failed from HuggingFace."
            echo "  To provision manually:"
            for i in "${!files[@]}"; do
                echo "    curl -fsSL \"${urls[$i]}\" -o \"$MODEL_DIR/${files[$i]}\""
            done
            echo "    printf '%s\\n' \"$EMBEDDING_MODEL_REVISION\" > \"$stamp\""
            echo ""
            exit 1
        fi
    done

    rm -f "$stamp"
    for filename in "${files[@]}"; do
        mv "$MODEL_DIR/$filename.$$.tmp" "$MODEL_DIR/$filename"
    done
    printf '%s\n' "$EMBEDDING_MODEL_REVISION" > "$stamp"

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

    # Get version
    info "Checking for latest release..."
    local version
    version=$(get_latest_version)
    info "Installing version: $version"
    echo ""

    # Pre-built binary only. The local tarball is the Docker test hook.
    if ! install_from_prebuilt "$version"; then
        if [[ -n "${SPEQ_LOCAL_TARBALL:-}" ]]; then
            build_from_source "$version"
        else
            error "No pre-built binary for this platform. Build speq-skill from source: https://github.com/$REPO/blob/main/docs/installation.md#install-from-source"
        fi
    fi

    # Serena comes from the global install
    offer_mcp_servers

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
