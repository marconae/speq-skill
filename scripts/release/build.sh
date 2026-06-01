#!/bin/bash
# Build release artifact for the current platform
# Usage: ./scripts/release/build.sh [version]
# Example: ./scripts/release/build.sh v0.2.0
#
# Environment variables:
#   TARGET   - Rust target triple (defaults to host). Set in CI for cross-compilation.

set -euo pipefail

VERSION="${1:-dev}"

# Map Rust target to short name
get_short_platform() {
    local target="${1:-$(rustc -vV | sed -n 's/host: //p')}"
    local arch os
    case "$target" in
        x86_64*)  arch="x86_64" ;;
        aarch64*) arch="aarch64" ;;
        *)        arch="unknown" ;;
    esac
    case "$target" in
        *linux*)   os="linux" ;;
        *darwin*)  os="mac" ;;
        *windows*) os="windows" ;;
        *)         os="unknown" ;;
    esac
    echo "${arch}-${os}"
}

HOST_TARGET=$(rustc -vV | sed -n 's/host: //p')
TARGET="${TARGET:-$HOST_TARGET}"
SHORT_PLATFORM=$(get_short_platform "$TARGET")
MARKETPLACE_ARCHIVE="speq-marketplace-${VERSION}-${SHORT_PLATFORM}"

BINARY_EXT=""
[[ "$TARGET" == *windows* ]] && BINARY_EXT=".exe"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$PROJECT_ROOT"

echo "Building speq CLI for ${TARGET}..."

# 1. Build release binary — use cross for Linux cross-compilation
if [[ "$TARGET" != "$HOST_TARGET" && "$TARGET" == *linux* ]] && command -v cross &> /dev/null; then
    cross build --release --target "${TARGET}"
else
    cargo build --release --target "${TARGET}"
fi

# 2. Build plugin and marketplace structure (outputs to dist/marketplace/)
echo "Building speq plugin and marketplace structure..."
./scripts/plugin/build.sh

# 3. Add CLI binary and licenses into dist/marketplace/bin/
if ! command -v cargo-about &> /dev/null; then
    echo "Installing cargo-about..."
    cargo install cargo-about --features cli
fi

cp "target/${TARGET}/release/speq${BINARY_EXT}" "dist/marketplace/bin/speq${BINARY_EXT}"
cp LICENSE dist/marketplace/bin/
cargo about generate about.hbs > dist/marketplace/bin/THIRD_PARTY_LICENSES

# 4. Package as versioned tarball — rename-copy approach works on both GNU tar (Linux) and BSD tar (macOS)
cp -r dist/marketplace "dist/${MARKETPLACE_ARCHIVE}"
tar -czvf "dist/${MARKETPLACE_ARCHIVE}.tar.gz" -C dist "${MARKETPLACE_ARCHIVE}/"
rm -rf "dist/${MARKETPLACE_ARCHIVE}"
echo "Created: dist/${MARKETPLACE_ARCHIVE}.tar.gz"
