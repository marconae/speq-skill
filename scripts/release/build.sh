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
CLI_ARCHIVE="speq-${VERSION}-${SHORT_PLATFORM}"
PLUGIN_ARCHIVE="speq-plugin-${VERSION}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$PROJECT_ROOT"

echo "Building speq CLI for ${TARGET}..."

# 1. Build release binary
cargo build --release --target "${TARGET}"

# 2. Generate third-party licenses (requires cargo-about)
if ! command -v cargo-about &> /dev/null; then
    echo "Installing cargo-about..."
    cargo install cargo-about
fi
cargo about generate about.hbs > THIRD_PARTY_LICENSES

# 3. Package CLI
mkdir -p "dist/${CLI_ARCHIVE}"
cp "target/${TARGET}/release/speq" "dist/${CLI_ARCHIVE}/"
cp LICENSE "dist/${CLI_ARCHIVE}/"
cp THIRD_PARTY_LICENSES "dist/${CLI_ARCHIVE}/"
tar -czvf "dist/${CLI_ARCHIVE}.tar.gz" -C dist "${CLI_ARCHIVE}"
echo "Created: dist/${CLI_ARCHIVE}.tar.gz"

# 4. Build plugin (only once per release, not per platform)
if [ ! -f "dist/${PLUGIN_ARCHIVE}.tar.gz" ]; then
    echo "Building speq plugin..."
    ./scripts/plugin/build.sh

    mkdir -p "dist/${PLUGIN_ARCHIVE}"
    # Copy all files including hidden directories (.claude-plugin)
    cp -r plugin/. "dist/${PLUGIN_ARCHIVE}/"
    tar -czvf "dist/${PLUGIN_ARCHIVE}.tar.gz" -C dist "${PLUGIN_ARCHIVE}"
    echo "Created: dist/${PLUGIN_ARCHIVE}.tar.gz"
fi
