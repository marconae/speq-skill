#!/usr/bin/env bash
set -euo pipefail

# Build speq plugin from .claude/ source
# Transforms skills, merges rules, and generates plugin.json

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
}

# Copy a skill with renaming
copy_skill() {
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
                sed -i '' 's|/speq-planner|/speq:planner|g' "$target_path/SKILL.md"
                sed -i '' 's|/speq-implementer|/speq:implementer|g' "$target_path/SKILL.md"
                sed -i '' 's|/speq-recorder|/speq:recorder|g' "$target_path/SKILL.md"
                sed -i '' 's|/speq-mission-creator|/speq:mission-creator|g' "$target_path/SKILL.md"
            else
                sed -i "s/^name: $source_name$/name: speq:$target_name/" "$target_path/SKILL.md"
                sed -i 's|/speq-planner|/speq:planner|g' "$target_path/SKILL.md"
                sed -i 's|/speq-implementer|/speq:implementer|g' "$target_path/SKILL.md"
                sed -i 's|/speq-recorder|/speq:recorder|g' "$target_path/SKILL.md"
                sed -i 's|/speq-mission-creator|/speq:mission-creator|g' "$target_path/SKILL.md"
            fi
        fi
    else
        log_warn "  Skill not found: $source_name"
    fi
}

# Copy and rename all skills
copy_skills() {
    log_info "Copying skills..."
    copy_skill "speq-planner" "planner"
    copy_skill "speq-implementer" "implementer"
    copy_skill "speq-recorder" "recorder"
    copy_skill "speq-mission-creator" "mission-creator"
}

# Merge rules into a skill's references directory
merge_rules_for_skill() {
    local skill_name="$1"
    local output_filename="$2"
    shift 2
    local rules=("$@")

    local skill_path="$PLUGIN_DIR/skills/$skill_name"
    local output_file="$skill_path/references/$output_filename"

    if [[ ! -d "$skill_path" ]]; then
        log_warn "  Skill directory not found: $skill_name"
        return
    fi

    mkdir -p "$skill_path/references"
    log_info "  Creating $output_filename for $skill_name"

    cat > "$output_file" << 'HEADER'
# Embedded Rules

These rules are embedded from the project's .claude/rules/ directory.
They apply when using this skill.

HEADER

    for rule in "${rules[@]}"; do
        local rule_path="$SOURCE_DIR/rules/$rule"
        if [[ -f "$rule_path" ]]; then
            echo "---" >> "$output_file"
            echo "" >> "$output_file"
            cat "$rule_path" >> "$output_file"
            echo "" >> "$output_file"
        else
            log_warn "    Rule file not found: $rule"
        fi
    done
}

# Merge rules into skills
merge_rules() {
    log_info "Merging rules into skills..."

    # planner: serena, context7, git
    merge_rules_for_skill "planner" "rules.md" \
        "mcp-serena.md" "mcp-context7.md" "default-git.md"

    # implementer: serena, context7, git, guardrails (core)
    merge_rules_for_skill "implementer" "rules-and-guardrails.md" \
        "mcp-serena.md" "mcp-context7.md" "default-git.md" "default-guardrails.md"

    # recorder: serena, context7, git
    merge_rules_for_skill "recorder" "rules.md" \
        "mcp-serena.md" "mcp-context7.md" "default-git.md"

    # mission-creator: serena, context7, git, guardrails (core)
    merge_rules_for_skill "mission-creator" "rules-and-guardrails.md" \
        "mcp-serena.md" "mcp-context7.md" "default-git.md" "default-guardrails.md"
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
    copy_skills
    merge_rules
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
    log_info "Skills available: /speq:planner, /speq:implementer, /speq:recorder, /speq:mission-creator"
}

main "$@"
