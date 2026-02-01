#!/usr/bin/env bash
set -euo pipefail

# Build speq plugin from .claude/ source
# Transforms skills and generates plugin.json
# Outputs to dist/plugin/ and dist/marketplace/

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
SOURCE_DIR="$PROJECT_ROOT/.claude"
DIST_DIR="$PROJECT_ROOT/dist"
PLUGIN_DIR="$DIST_DIR/plugin"
MARKETPLACE_DIR="$DIST_DIR/marketplace"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Extract version from Cargo.toml
get_version() {
    grep '^version' "$PROJECT_ROOT/Cargo.toml" | sed 's/.*"\(.*\)".*/\1/'
}

# Extract author from Cargo.toml
get_author() {
    grep '^authors' "$PROJECT_ROOT/Cargo.toml" | sed 's/.*\["\([^"]*\)".*/\1/'
}

# Clean and create plugin directory
setup_plugin_dir() {
    log_info "Setting up plugin directory..."
    rm -rf "$PLUGIN_DIR"
    mkdir -p "$PLUGIN_DIR/.claude-plugin"
    mkdir -p "$PLUGIN_DIR/skills"
    mkdir -p "$PLUGIN_DIR/agents"
}

# Copy and transform a skill
# All skills follow the same pattern:
#   - Source folder: speq-<name> (e.g., speq-plan, speq-code-tools)
#   - Target folder: <name> (e.g., plan, code-tools)
#   - Frontmatter name: speq-<name> → speq:<name>
#   - References: /speq-<name> → /speq:<name>
copy_skill() {
    local source_name="$1"
    local target_name="${source_name#speq-}"  # Remove speq- prefix for folder
    local source_path="$SOURCE_DIR/skills/$source_name"
    local target_path="$PLUGIN_DIR/skills/$target_name"

    if [[ -d "$source_path" ]]; then
        log_info "  $source_name -> $target_name"
        cp -r "$source_path" "$target_path"

        # Transform all files in the skill
        find "$target_path" -name "*.md" -type f | while read -r file; do
            if [[ "$OSTYPE" == "darwin"* ]]; then
                # Transform frontmatter name: speq-* → speq:*
                sed -i '' "s/^name: $source_name$/name: speq:$target_name/" "$file"
                # Transform all /speq-* references to /speq:*
                sed -i '' 's|/speq-\([a-zA-Z0-9_-]*\)|/speq:\1|g' "$file"
            else
                sed -i "s/^name: $source_name$/name: speq:$target_name/" "$file"
                sed -i 's|/speq-\([a-zA-Z0-9_-]*\)|/speq:\1|g' "$file"
            fi
        done
    else
        log_warn "  Skill not found: $source_name"
    fi
}

# Copy all skills (uniform pattern)
copy_skills() {
    log_info "Copying skills..."

    # All skills now have speq- prefix
    for skill_dir in "$SOURCE_DIR/skills"/speq-*; do
        if [[ -d "$skill_dir" ]]; then
            local skill_name=$(basename "$skill_dir")
            copy_skill "$skill_name"
        fi
    done
}

# Copy agent definitions
copy_agents() {
    log_info "Copying agents..."

    if [[ -d "$SOURCE_DIR/agents" ]]; then
        for agent_file in "$SOURCE_DIR/agents"/*.md; do
            if [[ -f "$agent_file" ]]; then
                local filename=$(basename "$agent_file")
                log_info "  $filename"
                cp "$agent_file" "$PLUGIN_DIR/agents/$filename"

                # Transform all /speq-* references to /speq:*
                if [[ "$OSTYPE" == "darwin"* ]]; then
                    sed -i '' 's|/speq-\([a-zA-Z0-9_-]*\)|/speq:\1|g' "$PLUGIN_DIR/agents/$filename"
                else
                    sed -i 's|/speq-\([a-zA-Z0-9_-]*\)|/speq:\1|g' "$PLUGIN_DIR/agents/$filename"
                fi
            fi
        done
    fi
}

# Copy and stamp plugin.json manifest
generate_manifest() {
    local version="$1"
    local author="$2"

    log_info "Copying plugin.json (version=$version, author=$author)..."
    cp "$SCRIPT_DIR/plugin.json" "$PLUGIN_DIR/.claude-plugin/plugin.json"

    # Replace placeholders
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s/VERSION_PLACEHOLDER/$version/g" "$PLUGIN_DIR/.claude-plugin/plugin.json"
        sed -i '' "s/AUTHOR_PLACEHOLDER/$author/g" "$PLUGIN_DIR/.claude-plugin/plugin.json"
    else
        sed -i "s/VERSION_PLACEHOLDER/$version/g" "$PLUGIN_DIR/.claude-plugin/plugin.json"
        sed -i "s/AUTHOR_PLACEHOLDER/$author/g" "$PLUGIN_DIR/.claude-plugin/plugin.json"
    fi
}

# Copy MCP server configuration
generate_mcp_config() {
    log_info "Copying .mcp.json..."
    cp "$SCRIPT_DIR/mcp.json" "$PLUGIN_DIR/.mcp.json"
}

# Build marketplace structure
build_marketplace() {
    local version="$1"
    local author="$2"

    log_info "Building marketplace structure..."

    rm -rf "$MARKETPLACE_DIR"
    mkdir -p "$MARKETPLACE_DIR/.claude-plugin"
    mkdir -p "$MARKETPLACE_DIR/plugins"
    mkdir -p "$MARKETPLACE_DIR/bin"

    # Copy and stamp marketplace manifest
    cp "$SCRIPT_DIR/marketplace.json" "$MARKETPLACE_DIR/.claude-plugin/marketplace.json"
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s/VERSION_PLACEHOLDER/$version/g" "$MARKETPLACE_DIR/.claude-plugin/marketplace.json"
        sed -i '' "s/AUTHOR_PLACEHOLDER/$author/g" "$MARKETPLACE_DIR/.claude-plugin/marketplace.json"
    else
        sed -i "s/VERSION_PLACEHOLDER/$version/g" "$MARKETPLACE_DIR/.claude-plugin/marketplace.json"
        sed -i "s/AUTHOR_PLACEHOLDER/$author/g" "$MARKETPLACE_DIR/.claude-plugin/marketplace.json"
    fi

    # Copy plugin as speq-skill
    cp -r "$PLUGIN_DIR" "$MARKETPLACE_DIR/plugins/speq-skill"

    log_info "Marketplace structure built at: $MARKETPLACE_DIR"
}

# Main build process
main() {
    local version=$(get_version)
    local author=$(get_author)

    log_info "Building speq plugin..."
    log_info "Source: $SOURCE_DIR"
    log_info "Target: $PLUGIN_DIR"
    log_info "Version: $version (from Cargo.toml)"
    log_info "Author: $author (from Cargo.toml)"
    echo ""

    setup_plugin_dir
    copy_skills
    copy_agents
    generate_manifest "$version" "$author"
    generate_mcp_config

    echo ""
    log_info "Plugin built successfully!"
    log_info "Structure:"
    find "$PLUGIN_DIR" -type f | sed "s|$PLUGIN_DIR/||" | sort | while read -r f; do
        echo "  $f"
    done

    echo ""
    log_info "To test: claude --plugin-dir $PLUGIN_DIR"
    log_info "Workflow skills: /speq:plan, /speq:implement, /speq:record, /speq:mission"
    log_info "Utility skills: /speq:code-tools, /speq:ext-research, /speq:code-guardrails, /speq:git-discipline, /speq:cli"
    log_info "Agents: implementer-agent, code-reviewer"

    # Build marketplace structure
    build_marketplace "$version" "$author"
}

main "$@"
