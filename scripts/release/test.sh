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
CLI_ARCHIVE="dist/speq-${VERSION}-${SHORT_PLATFORM}.tar.gz"
PLUGIN_ARCHIVE="dist/speq-plugin-${VERSION}.tar.gz"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$PROJECT_ROOT"

echo "=== Testing release artifacts ==="

# 1. Build if not exists
if [ ! -f "$CLI_ARCHIVE" ]; then
    echo "Building release..."
    ./scripts/release/build.sh "$VERSION"
fi

# 2. Verify CLI archive exists
if [ ! -f "$CLI_ARCHIVE" ]; then
    echo "ERROR: CLI archive not found: $CLI_ARCHIVE"
    exit 1
fi

# 3. Extract to temp dir
TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

echo "Extracting CLI to $TMPDIR..."
tar -xzf "$CLI_ARCHIVE" -C "$TMPDIR"

# 4. Verify CLI contents
echo ""
echo "=== CLI archive contents ==="
tar -tzf "$CLI_ARCHIVE"

echo ""
echo "=== Checking CLI required files ==="
CLI_EXTRACT_DIR="$TMPDIR/speq-${VERSION}-${SHORT_PLATFORM}"

for file in speq LICENSE THIRD_PARTY_LICENSES; do
    if [ -f "$CLI_EXTRACT_DIR/$file" ]; then
        echo "✓ $file"
    else
        echo "✗ $file (MISSING)"
        exit 1
    fi
done

# 5. Test binary
echo ""
echo "=== Testing binary ==="
"$CLI_EXTRACT_DIR/speq" --help

# 6. Verify THIRD_PARTY_LICENSES content
echo ""
echo "=== THIRD_PARTY_LICENSES preview (first 30 lines) ==="
head -30 "$CLI_EXTRACT_DIR/THIRD_PARTY_LICENSES"

# 7. Check for required licenses (fastembed is Apache-2.0)
echo ""
echo "=== Checking for required licenses ==="
if grep -q "Apache" "$CLI_EXTRACT_DIR/THIRD_PARTY_LICENSES"; then
    echo "✓ Apache license found (required for fastembed)"
else
    echo "✗ Apache license NOT found"
    exit 1
fi

# 8. Test plugin artifact
echo ""
echo "=== Testing plugin artifact ==="

if [ -f "$PLUGIN_ARCHIVE" ]; then
    echo "Plugin archive found: $PLUGIN_ARCHIVE"
    tar -xzf "$PLUGIN_ARCHIVE" -C "$TMPDIR"
    PLUGIN_DIR="$TMPDIR/speq-plugin-${VERSION}"

    echo ""
    echo "=== Plugin archive contents ==="
    tar -tzf "$PLUGIN_ARCHIVE"

    echo ""
    echo "=== Checking plugin required files ==="
    for file in .claude-plugin/plugin.json skills/plan/SKILL.md skills/implement/SKILL.md; do
        if [ -f "$PLUGIN_DIR/$file" ]; then
            echo "✓ $file"
        else
            echo "✗ $file (MISSING)"
            exit 1
        fi
    done
else
    echo "⚠ Plugin archive not found: $PLUGIN_ARCHIVE"
    echo "  (Plugin is built alongside CLI, re-run build.sh if needed)"
fi

echo ""
echo "=== All tests passed ==="
