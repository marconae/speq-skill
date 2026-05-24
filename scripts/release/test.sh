#!/bin/bash
# Test release artifact locally
# Usage: ./scripts/release/test.sh [version]
# Example: ./scripts/release/test.sh v0.1.0-test

set -euo pipefail

VERSION="${1:-dev}"

# Map Rust target to short name (same as build.sh)
get_short_platform() {
    local target="${1:-$(rustc -vV | sed -n 's/host: //p')}"
    local arch os
    case "$target" in
        x86_64*)  arch="x86_64" ;;
        aarch64*) arch="aarch64" ;;
        *)        arch="unknown" ;;
    esac
    case "$target" in
        *linux*)  os="linux" ;;
        *darwin*) os="mac" ;;
        *)        os="unknown" ;;
    esac
    echo "${arch}-${os}"
}

TARGET="${TARGET:-$(rustc -vV | sed -n 's/host: //p')}"
SHORT_PLATFORM=$(get_short_platform "$TARGET")
MARKETPLACE_ARCHIVE="dist/speq-marketplace-${VERSION}-${SHORT_PLATFORM}.tar.gz"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$PROJECT_ROOT"

echo "=== Testing release artifacts ==="

# 1. Build if not exists
if [ ! -f "$MARKETPLACE_ARCHIVE" ]; then
    echo "Building release..."
    ./scripts/release/build.sh "$VERSION"
fi

# 2. Verify marketplace archive exists
if [ ! -f "$MARKETPLACE_ARCHIVE" ]; then
    echo "ERROR: Marketplace archive not found: $MARKETPLACE_ARCHIVE"
    exit 1
fi

# 3. Extract to temp dir
TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

echo "Extracting marketplace to $TMPDIR..."
tar -xzf "$MARKETPLACE_ARCHIVE" -C "$TMPDIR"

# 4. Verify archive contents
echo ""
echo "=== Marketplace archive contents ==="
tar -tzf "$MARKETPLACE_ARCHIVE"

EXTRACT_DIR="$TMPDIR/speq-marketplace-${VERSION}-${SHORT_PLATFORM}"

# 5. Check marketplace structure
echo ""
echo "=== Checking marketplace structure ==="

for file in .claude-plugin/marketplace.json bin/speq bin/LICENSE bin/THIRD_PARTY_LICENSES; do
    if [ -f "$EXTRACT_DIR/$file" ]; then
        echo "OK $file"
    else
        echo "MISSING $file"
        exit 1
    fi
done

for dir in plugins/speq-skill plugins/speq-skill/.claude-plugin plugins/speq-skill/skills codex/.agents/plugins codex/plugins/speq-skill codex/plugins/speq-skill/.codex-plugin codex/plugins/speq-skill/skills; do
    if [ -d "$EXTRACT_DIR/$dir" ]; then
        echo "OK $dir/"
    else
        echo "MISSING $dir/"
        exit 1
    fi
done

# 6. Test binary
echo ""
echo "=== Testing binary ==="
"$EXTRACT_DIR/bin/speq" --help

# 7. Verify marketplace.json version is stamped
# Strip v prefix — build.sh stamps the bare semver from Cargo.toml (e.g. "0.2.5" not "v0.2.5")
BARE_VERSION="${VERSION#v}"
echo ""
echo "=== Checking marketplace.json ==="
if grep -q "\"version\": \"${BARE_VERSION}\"" "$EXTRACT_DIR/.claude-plugin/marketplace.json"; then
    echo "OK Version stamped correctly: ${BARE_VERSION}"
else
    echo "ERROR: Version not stamped in marketplace.json"
    cat "$EXTRACT_DIR/.claude-plugin/marketplace.json"
    exit 1
fi

# 8. Check plugin structure
echo ""
echo "=== Checking plugin structure ==="
PLUGIN_DIR="$EXTRACT_DIR/plugins/speq-skill"
CODEX_PLUGIN_DIR="$EXTRACT_DIR/codex/plugins/speq-skill"

for file in .claude-plugin/plugin.json skills/plan/SKILL.md skills/implement/SKILL.md skills/record/SKILL.md skills/mission/SKILL.md; do
    if [ -f "$PLUGIN_DIR/$file" ]; then
        echo "OK $file"
    else
        echo "MISSING $file"
        exit 1
    fi
done

for file in .codex-plugin/plugin.json .mcp.json skills/plan/SKILL.md skills/implement/SKILL.md skills/record/SKILL.md skills/mission/SKILL.md; do
    if [ -f "$CODEX_PLUGIN_DIR/$file" ]; then
        echo "OK codex/$file"
    else
        echo "MISSING codex/$file"
        exit 1
    fi
done

if grep -q '"path": "./plugins/speq-skill"' "$EXTRACT_DIR/codex/.agents/plugins/marketplace.json"; then
    echo "OK Codex marketplace manifest"
else
    echo "ERROR: Codex marketplace manifest missing plugin path"
    exit 1
fi

if grep -q '^name: speq:plan$' "$CODEX_PLUGIN_DIR/skills/plan/SKILL.md"; then
    echo "OK Codex /speq:plan skill name"
else
    echo "ERROR: Codex skill name is not speq:plan"
    exit 1
fi

if grep -R -E "AskUserQuestion|AskUserTool|Task\\(|subagent_type=|ExitPlanMode|Claude Code|CodexSubagent|Codex user-input prompt|Codex task" "$CODEX_PLUGIN_DIR" >/dev/null; then
    echo "ERROR: Codex plugin contains Claude-only workflow syntax"
    grep -R -E "AskUserQuestion|AskUserTool|Task\\(|subagent_type=|ExitPlanMode|Claude Code|CodexSubagent|Codex user-input prompt|Codex task" "$CODEX_PLUGIN_DIR"
    exit 1
fi

# 9. Verify THIRD_PARTY_LICENSES content
echo ""
echo "=== THIRD_PARTY_LICENSES preview (first 30 lines) ==="
head -30 "$EXTRACT_DIR/bin/THIRD_PARTY_LICENSES"

# 10. Check for required licenses (candle and tokenizers are Apache-2.0)
echo ""
echo "=== Checking for required licenses ==="
if grep -q "Apache" "$EXTRACT_DIR/bin/THIRD_PARTY_LICENSES"; then
    echo "OK Apache license found (required for candle and tokenizers)"
else
    echo "ERROR: Apache license NOT found"
    exit 1
fi

echo ""
echo "=== All tests passed ==="
echo ""
echo "To test installation locally:"
echo "  1. Extract: tar -xzf $MARKETPLACE_ARCHIVE -C /tmp"
echo "  2. Move: mv /tmp/speq-marketplace-${VERSION}-${SHORT_PLATFORM} ~/.speq-skill"
echo "  3. Symlink CLI: ln -sf ~/.speq-skill/bin/speq ~/.local/bin/speq"
echo "  4. Add marketplace: claude plugin marketplace add ~/.speq-skill"
echo "  5. Install plugin: claude plugin install speq@speq-skill"
echo "  6. Register Codex: codex plugin marketplace add ~/.speq-skill/codex"
