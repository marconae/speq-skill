#!/usr/bin/env bash
set -euo pipefail

# Build speq plugin from .claude/ source
# Transforms skills and generates plugin.json

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
SOURCE_DIR="$PROJECT_ROOT/.claude"
PLUGIN_DIR="$PROJECT_ROOT/plugin"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Clean and create plugin directory
setup_plugin_dir() {
    log_info "Setting up plugin directory..."
    rm -rf "$PLUGIN_DIR"
    mkdir -p "$PLUGIN_DIR/.claude-plugin"
    mkdir -p "$PLUGIN_DIR/skills"
    mkdir -p "$PLUGIN_DIR/agents"
}

# Copy a workflow skill with renaming
copy_workflow_skill() {
    local source_name="$1"
    local target_name="$2"
    local source_path="$SOURCE_DIR/skills/$source_name"
    local target_path="$PLUGIN_DIR/skills/$target_name"

    if [[ -d "$source_path" ]]; then
        log_info "  $source_name -> $target_name"
        cp -r "$source_path" "$target_path"

        # Update skill name in SKILL.md frontmatter and cross-references
        if [[ -f "$target_path/SKILL.md" ]]; then
            # macOS and Linux compatible sed
            if [[ "$OSTYPE" == "darwin"* ]]; then
                sed -i '' "s/^name: $source_name$/name: speq:$target_name/" "$target_path/SKILL.md"
                # Update workflow skill references
                sed -i '' 's|/speq-plan|/speq:plan|g' "$target_path/SKILL.md"
                sed -i '' 's|/speq-implement|/speq:implement|g' "$target_path/SKILL.md"
                sed -i '' 's|/speq-record|/speq:record|g' "$target_path/SKILL.md"
                sed -i '' 's|/speq-mission|/speq:mission|g' "$target_path/SKILL.md"
                # Update utility skill references
                sed -i '' 's|`/code-tools`|`/speq:code-tools`|g' "$target_path/SKILL.md"
                sed -i '' 's|`/ext-research`|`/speq:ext-research`|g' "$target_path/SKILL.md"
                sed -i '' 's|`/code-guardrails`|`/speq:code-guardrails`|g' "$target_path/SKILL.md"
                sed -i '' 's|`/git-discipline`|`/speq:git-discipline`|g' "$target_path/SKILL.md"
                sed -i '' 's|`/speq-cli`|`/speq:speq-cli`|g' "$target_path/SKILL.md"
            else
                sed -i "s/^name: $source_name$/name: speq:$target_name/" "$target_path/SKILL.md"
                # Update workflow skill references
                sed -i 's|/speq-plan|/speq:plan|g' "$target_path/SKILL.md"
                sed -i 's|/speq-implement|/speq:implement|g' "$target_path/SKILL.md"
                sed -i 's|/speq-record|/speq:record|g' "$target_path/SKILL.md"
                sed -i 's|/speq-mission|/speq:mission|g' "$target_path/SKILL.md"
                # Update utility skill references
                sed -i 's|`/code-tools`|`/speq:code-tools`|g' "$target_path/SKILL.md"
                sed -i 's|`/ext-research`|`/speq:ext-research`|g' "$target_path/SKILL.md"
                sed -i 's|`/code-guardrails`|`/speq:code-guardrails`|g' "$target_path/SKILL.md"
                sed -i 's|`/git-discipline`|`/speq:git-discipline`|g' "$target_path/SKILL.md"
                sed -i 's|`/speq-cli`|`/speq:speq-cli`|g' "$target_path/SKILL.md"
            fi
        fi

        # Update references files if they exist
        if [[ -d "$target_path/references" ]]; then
            for ref_file in "$target_path/references"/*.md; do
                if [[ -f "$ref_file" ]]; then
                    if [[ "$OSTYPE" == "darwin"* ]]; then
                        sed -i '' 's|`/code-tools`|`/speq:code-tools`|g' "$ref_file"
                        sed -i '' 's|`/ext-research`|`/speq:ext-research`|g' "$ref_file"
                        sed -i '' 's|`/code-guardrails`|`/speq:code-guardrails`|g' "$ref_file"
                        sed -i '' 's|`/git-discipline`|`/speq:git-discipline`|g' "$ref_file"
                        sed -i '' 's|`/speq-cli`|`/speq:speq-cli`|g' "$ref_file"
                    else
                        sed -i 's|`/code-tools`|`/speq:code-tools`|g' "$ref_file"
                        sed -i 's|`/ext-research`|`/speq:ext-research`|g' "$ref_file"
                        sed -i 's|`/code-guardrails`|`/speq:code-guardrails`|g' "$ref_file"
                        sed -i 's|`/git-discipline`|`/speq:git-discipline`|g' "$ref_file"
                        sed -i 's|`/speq-cli`|`/speq:speq-cli`|g' "$ref_file"
                    fi
                fi
            done
        fi
    else
        log_warn "  Skill not found: $source_name"
    fi
}

# Copy utility skills
copy_util_skills() {
    log_info "Copying utility skills..."
    mkdir -p "$PLUGIN_DIR/skills"

    for skill in code-tools ext-research code-guardrails git-discipline speq-cli; do
        local source_path="$SOURCE_DIR/skills/$skill"
        local target_path="$PLUGIN_DIR/skills/$skill"

        if [[ -d "$source_path" ]]; then
            log_info "  $skill"
            cp -r "$source_path" "$target_path"

            # Update skill name in frontmatter
            if [[ -f "$target_path/SKILL.md" ]]; then
                if [[ "$OSTYPE" == "darwin"* ]]; then
                    sed -i '' "s/^name: $skill$/name: speq:$skill/" "$target_path/SKILL.md"
                else
                    sed -i "s/^name: $skill$/name: speq:$skill/" "$target_path/SKILL.md"
                fi
            fi
        else
            log_warn "  Utility skill not found: $skill"
        fi
    done
}

# Copy and rename all workflow skills
copy_workflow_skills() {
    log_info "Copying workflow skills..."
    copy_workflow_skill "speq-plan" "plan"
    copy_workflow_skill "speq-implement" "implement"
    copy_workflow_skill "speq-record" "record"
    copy_workflow_skill "speq-mission" "mission"
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

                # Update skill references for plugin context
                if [[ "$OSTYPE" == "darwin"* ]]; then
                    sed -i '' 's|`/code-tools`|`/speq:code-tools`|g' "$PLUGIN_DIR/agents/$filename"
                    sed -i '' 's|`/ext-research`|`/speq:ext-research`|g' "$PLUGIN_DIR/agents/$filename"
                    sed -i '' 's|`/code-guardrails`|`/speq:code-guardrails`|g' "$PLUGIN_DIR/agents/$filename"
                    sed -i '' 's|`/git-discipline`|`/speq:git-discipline`|g' "$PLUGIN_DIR/agents/$filename"
                    sed -i '' 's|`/speq-cli`|`/speq:speq-cli`|g' "$PLUGIN_DIR/agents/$filename"
                else
                    sed -i 's|`/code-tools`|`/speq:code-tools`|g' "$PLUGIN_DIR/agents/$filename"
                    sed -i 's|`/ext-research`|`/speq:ext-research`|g' "$PLUGIN_DIR/agents/$filename"
                    sed -i 's|`/code-guardrails`|`/speq:code-guardrails`|g' "$PLUGIN_DIR/agents/$filename"
                    sed -i 's|`/git-discipline`|`/speq:git-discipline`|g' "$PLUGIN_DIR/agents/$filename"
                    sed -i 's|`/speq-cli`|`/speq:speq-cli`|g' "$PLUGIN_DIR/agents/$filename"
                fi
            fi
        done
    fi
}

# Copy plugin.json manifest
generate_manifest() {
    log_info "Copying plugin.json..."
    cp "$SCRIPT_DIR/plugin.json" "$PLUGIN_DIR/.claude-plugin/plugin.json"
}

# Copy MCP server configuration
generate_mcp_config() {
    log_info "Copying .mcp.json..."
    cp "$SCRIPT_DIR/mcp.json" "$PLUGIN_DIR/.mcp.json"
}

# Main build process
main() {
    log_info "Building speq plugin..."
    log_info "Source: $SOURCE_DIR"
    log_info "Target: $PLUGIN_DIR"
    echo ""

    setup_plugin_dir
    copy_util_skills
    copy_workflow_skills
    copy_agents
    generate_manifest
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
    log_info "Utility skills: /speq:code-tools, /speq:ext-research, /speq:code-guardrails, /speq:git-discipline, /speq:speq-cli"
    log_info "Agents: implementer-agent, code-reviewer"
}

main "$@"
