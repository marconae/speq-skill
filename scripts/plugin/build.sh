#!/usr/bin/env bash
set -euo pipefail

# Build speq plugin artifacts from the shared skill source.
# Outputs a Claude marketplace-compatible tree and a Codex plugin tree under
# dist/marketplace/.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
SOURCE_DIR="$PROJECT_ROOT/.claude"
DIST_DIR="$PROJECT_ROOT/dist"
MARKETPLACE_DIR="$DIST_DIR/marketplace"
CLAUDE_PLUGIN_DIR="$MARKETPLACE_DIR/plugins/speq-skill"
CODEX_PLUGIN_DIR="$MARKETPLACE_DIR/codex/plugins/speq-skill"
CODEX_MARKETPLACE_FILE="$MARKETPLACE_DIR/codex/.agents/plugins/marketplace.json"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

get_version() {
    grep '^version' "$PROJECT_ROOT/Cargo.toml" | sed 's/.*"\(.*\)".*/\1/'
}

get_author() {
    grep '^authors' "$PROJECT_ROOT/Cargo.toml" | sed 's/.*\["\([^"]*\)".*/\1/'
}

sed_in_place() {
    local expr="$1"
    local file="$2"

    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "$expr" "$file"
    else
        sed -i "$expr" "$file"
    fi
}

setup_dirs() {
    log_info "Setting up marketplace directory..."
    rm -rf "$MARKETPLACE_DIR"
    mkdir -p "$MARKETPLACE_DIR/.claude-plugin"
    mkdir -p "$MARKETPLACE_DIR/bin"
    mkdir -p "$CLAUDE_PLUGIN_DIR/.claude-plugin"
    mkdir -p "$CLAUDE_PLUGIN_DIR/skills"
    mkdir -p "$CLAUDE_PLUGIN_DIR/agents"
    mkdir -p "$CODEX_PLUGIN_DIR/.codex-plugin"
    mkdir -p "$CODEX_PLUGIN_DIR/skills"
    mkdir -p "$CODEX_PLUGIN_DIR/agents"
    mkdir -p "$(dirname "$CODEX_MARKETPLACE_FILE")"
}

transform_common_markdown() {
    local file="$1"
    local source_name="$2"
    local target_name="$3"

    sed_in_place "s/^name: $source_name$/name: speq:$target_name/" "$file"
    sed_in_place 's|/speq-\([a-zA-Z0-9_-]*\)|/speq:\1|g' "$file"
}

transform_codex_markdown() {
    local file="$1"

    sed_in_place 's/Claude Code/Codex/g' "$file"
    sed_in_place 's/AskUserQuestion/ask the user/g' "$file"
    sed_in_place 's/AskUserTool/ask the user/g' "$file"
    sed_in_place 's/ExitPlanMode/present the plan and ask the user to proceed/g' "$file"
    sed_in_place 's/TaskCreate/update_plan/g' "$file"
    sed_in_place 's/TaskUpdate/update_plan/g' "$file"
    sed_in_place 's/TaskList/review the current session plan/g' "$file"
    sed_in_place 's/Task(/spawn_agent(/g' "$file"
    sed_in_place 's/subagent_type=/agent_type=/g' "$file"
}

set_codex_skill_model() {
    local file="$1"
    local skill_name="$2"

    case "$skill_name" in
        speq-plan|speq-implement|speq-record)
            sed_in_place 's/^model: sonnet$/model: gpt-5.4/' "$file"
            if ! grep -q '^effort:' "$file"; then
                if [[ "$OSTYPE" == "darwin"* ]]; then
                    sed -i '' '/^model: gpt-5.4$/a\
effort: medium
' "$file"
                else
                    sed -i '/^model: gpt-5.4$/a effort: medium' "$file"
                fi
            fi
            ;;
        *)
            ;;
    esac
}

copy_skill_for_platform() {
    local platform="$1"
    local source_name="$2"
    local target_name="${source_name#speq-}"
    local source_path="$SOURCE_DIR/skills/$source_name"
    local plugin_dir

    case "$platform" in
        claude) plugin_dir="$CLAUDE_PLUGIN_DIR" ;;
        codex) plugin_dir="$CODEX_PLUGIN_DIR" ;;
        *) log_error "Unknown platform: $platform"; exit 1 ;;
    esac

    local target_path="$plugin_dir/skills/$target_name"

    if [[ -d "$source_path" ]]; then
        log_info "  [$platform] $source_name -> $target_name"
        cp -r "$source_path" "$target_path"

        find "$target_path" -name "*.md" -type f | while read -r file; do
            transform_common_markdown "$file" "$source_name" "$target_name"
            if [[ "$platform" == "codex" ]]; then
                transform_codex_markdown "$file"
                set_codex_skill_model "$file" "$source_name"
            fi
        done
    else
        log_warn "  Skill not found: $source_name"
    fi
}

copy_skills() {
    log_info "Copying skills..."

    for skill_dir in "$SOURCE_DIR/skills"/speq-*; do
        if [[ -d "$skill_dir" ]]; then
            local skill_name
            skill_name=$(basename "$skill_dir")
            copy_skill_for_platform claude "$skill_name"
            copy_skill_for_platform codex "$skill_name"
        fi
    done
}

set_codex_agent_model() {
    local file="$1"
    local filename="$2"

    case "$filename" in
        planner-agent.md|implementer-expert-agent.md|code-reviewer.md)
            sed_in_place 's/^model: .*/model: gpt-5.5/' "$file"
            ;;
        implementer-agent.md|recorder-agent.md)
            sed_in_place 's/^model: .*/model: gpt-5.4/' "$file"
            ;;
        *)
            ;;
    esac
}

copy_agents_for_platform() {
    local platform="$1"
    local plugin_dir

    case "$platform" in
        claude) plugin_dir="$CLAUDE_PLUGIN_DIR" ;;
        codex) plugin_dir="$CODEX_PLUGIN_DIR" ;;
        *) log_error "Unknown platform: $platform"; exit 1 ;;
    esac

    if [[ ! -d "$SOURCE_DIR/agents" ]]; then
        return
    fi

    for agent_file in "$SOURCE_DIR/agents"/*.md; do
        if [[ -f "$agent_file" ]]; then
            local filename
            filename=$(basename "$agent_file")
            log_info "  [$platform] $filename"
            cp "$agent_file" "$plugin_dir/agents/$filename"
            sed_in_place 's|/speq-\([a-zA-Z0-9_-]*\)|/speq:\1|g' "$plugin_dir/agents/$filename"

            if [[ "$platform" == "codex" ]]; then
                transform_codex_markdown "$plugin_dir/agents/$filename"
                set_codex_agent_model "$plugin_dir/agents/$filename" "$filename"
            fi
        fi
    done
}

copy_agents() {
    log_info "Copying agents..."
    copy_agents_for_platform claude
    copy_agents_for_platform codex
}

stamp_file() {
    local file="$1"
    local version="$2"
    local author="$3"

    sed_in_place "s/VERSION_PLACEHOLDER/$version/g" "$file"
    sed_in_place "s/AUTHOR_PLACEHOLDER/$author/g" "$file"
}

generate_manifests() {
    local version="$1"
    local author="$2"

    log_info "Generating manifests (version=$version, author=$author)..."

    cp "$SCRIPT_DIR/plugin.json" "$CLAUDE_PLUGIN_DIR/.claude-plugin/plugin.json"
    stamp_file "$CLAUDE_PLUGIN_DIR/.claude-plugin/plugin.json" "$version" "$author"

    cp "$SCRIPT_DIR/marketplace.json" "$MARKETPLACE_DIR/.claude-plugin/marketplace.json"
    stamp_file "$MARKETPLACE_DIR/.claude-plugin/marketplace.json" "$version" "$author"

    cp "$SCRIPT_DIR/codex-plugin.json" "$CODEX_PLUGIN_DIR/.codex-plugin/plugin.json"
    stamp_file "$CODEX_PLUGIN_DIR/.codex-plugin/plugin.json" "$version" "$author"

    cp "$SCRIPT_DIR/codex-marketplace.json" "$CODEX_MARKETPLACE_FILE"
    stamp_file "$CODEX_MARKETPLACE_FILE" "$version" "$author"
}

generate_mcp_configs() {
    log_info "Generating MCP configs..."
    cp "$SCRIPT_DIR/mcp.json" "$CLAUDE_PLUGIN_DIR/.mcp.json"
    cp "$SCRIPT_DIR/mcp-codex.json" "$CODEX_PLUGIN_DIR/.mcp.json"
}

main() {
    local version
    local author
    version=$(get_version)
    author=$(get_author)

    log_info "Building speq plugin artifacts..."
    log_info "Source: $SOURCE_DIR"
    log_info "Claude target: $CLAUDE_PLUGIN_DIR"
    log_info "Codex target: $CODEX_PLUGIN_DIR"
    log_info "Version: $version (from Cargo.toml)"
    log_info "Author: $author (from Cargo.toml)"
    echo ""

    setup_dirs
    copy_skills
    copy_agents
    generate_manifests "$version" "$author"
    generate_mcp_configs

    echo ""
    log_info "Plugin artifacts built successfully!"
    log_info "Structure:"
    find "$MARKETPLACE_DIR" -type f | sed "s|$MARKETPLACE_DIR/||" | sort | while read -r f; do
        echo "  $f"
    done

    echo ""
    log_info "Claude test: claude --plugin-dir $CLAUDE_PLUGIN_DIR"
    log_info "Codex plugin: $CODEX_PLUGIN_DIR"
    log_info "Workflow skills: /speq:plan, /speq:implement, /speq:record, /speq:mission"
    log_info "Utility skills: /speq:code-tools, /speq:ext-research, /speq:code-guardrails, /speq:git-discipline, /speq:cli"
    log_info "Codex model routing: orchestration gpt-5.4/medium, heavy agents gpt-5.5/xhigh, standard implementation gpt-5.4/high, recording gpt-5.4/medium"
    log_info "Marketplace structure built at: $MARKETPLACE_DIR"
}

main "$@"
