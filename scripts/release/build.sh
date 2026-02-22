#!/bin/bash
# Build release artifact for the current platform
# Usage: ./scripts/release/build.sh [version]
# Example: ./scripts/release/build.sh v0.2.0

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
        *linux*)  os="linux" ;;
        *darwin*) os="mac" ;;
        *)        os="unknown" ;;
    esac
    echo "${arch}-${os}"
}

TARGET="${TARGET:-$(rustc -vV | sed -n 's/host: //p')}"
SHORT_PLATFORM=$(get_short_platform "$TARGET")
MARKETPLACE_ARCHIVE="speq-marketplace-${VERSION}-${SHORT_PLATFORM}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$PROJECT_ROOT"

echo "Building speq CLI for ${TARGET}..."

# 1. Build release binary
cargo build --release --target "${TARGET}"

# 2. Build plugin and marketplace structure (outputs to dist/marketplace/)
echo "Building speq plugin and marketplace structure..."
./scripts/plugin/build.sh

# 3. Add CLI binary and licenses into dist/marketplace/bin/
if ! command -v cargo-about &> /dev/null; then
    echo "Installing cargo-about..."
    cargo install cargo-about
fi

cp "target/${TARGET}/release/speq" dist/marketplace/bin/
cp LICENSE dist/marketplace/bin/
cargo about generate about.hbs > dist/marketplace/bin/THIRD_PARTY_LICENSES

# 4. Tar directly from dist/marketplace/ with versioned prefix
mkdir -p dist/bin
tar -czvf "dist/${MARKETPLACE_ARCHIVE}.tar.gz" \
    -s ",^marketplace/,${MARKETPLACE_ARCHIVE}/," \
    -C dist marketplace/
echo "Created: dist/${MARKETPLACE_ARCHIVE}.tar.gz"
