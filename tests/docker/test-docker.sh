#!/bin/bash
# Run integration test in Docker
# Usage: ./tests/docker/test-docker.sh
#
# This script tests the install.sh script by:
# 1. Building the release binary locally (fast, native compilation)
# 2. Creating a source tarball with the pre-built binary
# 3. Running install.sh in Docker which extracts and installs
#
# Cross-platform note: When running on macOS but testing on Linux Docker,
# a mock binary is used since the native binary won't work. For full binary
# testing, run on a Linux host or in CI.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$PROJECT_ROOT"

# Detect if we're cross-platform (macOS host testing Linux Docker)
HOST_OS="$(uname -s)"
CROSS_PLATFORM=false
if [[ "$HOST_OS" == "Darwin" ]]; then
    CROSS_PLATFORM=true
    echo "=== Cross-platform detected: macOS host -> Linux Docker ==="
    echo "Note: Using mock binary for script testing. Full binary testing requires Linux host."
    echo ""
fi

echo "=== Building release binary ==="
cargo build --release

echo ""
echo "=== Building plugin ==="
./scripts/plugin/build.sh

echo ""
echo "=== Creating source tarball with pre-built binary ==="
# Create tarball that mimics GitHub's archive structure
TARBALL_DIR="$PROJECT_ROOT/dist/docker-build"
rm -rf "$TARBALL_DIR"
mkdir -p "$TARBALL_DIR/speq-skill-main"

# Copy source files (excluding .git, target except release binary, dist)
rsync -a \
    --exclude='.git' \
    --exclude='target' \
    --exclude='dist' \
    --exclude='.DS_Store' \
    --exclude='._*' \
    "$PROJECT_ROOT/" "$TARBALL_DIR/speq-skill-main/"

# Include binary - use mock for cross-platform, real binary for same-platform
mkdir -p "$TARBALL_DIR/speq-skill-main/target/release"
if [[ "$CROSS_PLATFORM" == "true" ]]; then
    # Create mock binary that simulates speq CLI
    cat > "$TARBALL_DIR/speq-skill-main/target/release/speq" << 'MOCK_EOF'
#!/bin/sh
# Mock speq binary for cross-platform Docker testing
VERSION="0.1.0-mock"
case "$1" in
    --version|-V)
        echo "speq $VERSION"
        ;;
    --help|-h|"")
        echo "speq $VERSION"
        echo "Spec-driven development CLI (mock binary for testing)"
        echo ""
        echo "Usage: speq <COMMAND>"
        echo ""
        echo "Commands:"
        echo "  tree      Show spec tree"
        echo "  search    Search specs"
        echo "  validate  Validate specs"
        echo "  help      Print this message"
        ;;
    *)
        echo "speq: mock command '$1' executed"
        ;;
esac
exit 0
MOCK_EOF
    chmod +x "$TARBALL_DIR/speq-skill-main/target/release/speq"
    echo "Created mock binary for cross-platform testing"
else
    cp "$PROJECT_ROOT/target/release/speq" "$TARBALL_DIR/speq-skill-main/target/release/"
    echo "Using native binary"
fi

# Create tarball
TARBALL="$TARBALL_DIR/source.tar.gz"
tar -czf "$TARBALL" -C "$TARBALL_DIR" "speq-skill-main"
echo "Created: $TARBALL"

echo ""
echo "=== Building Docker test image ==="
docker build -t speq-install-test -f tests/docker/Dockerfile .

echo ""
echo "=== Running install integration test ==="
docker run --rm \
    -v "$PROJECT_ROOT/install.sh:/home/testuser/install.sh:ro" \
    -v "$PROJECT_ROOT/tests/docker/test-install.sh:/home/testuser/test-install.sh:ro" \
    -v "$TARBALL:/home/testuser/source.tar.gz:ro" \
    -e "SPEQ_LOCAL_TARBALL=/home/testuser/source.tar.gz" \
    -e "SPEQ_PREBUILT=1" \
    speq-install-test -c "/home/testuser/test-install.sh"

echo ""
echo "=== Running update integration test ==="
docker run --rm \
    -v "$PROJECT_ROOT/install.sh:/home/testuser/install.sh:ro" \
    -v "$PROJECT_ROOT/tests/docker/test-update.sh:/home/testuser/test-update.sh:ro" \
    -v "$TARBALL:/home/testuser/source.tar.gz:ro" \
    -e "SPEQ_LOCAL_TARBALL=/home/testuser/source.tar.gz" \
    -e "SPEQ_PREBUILT=1" \
    speq-install-test -c "/home/testuser/test-update.sh"

echo ""
echo "=== Running uninstall integration test ==="
docker run --rm \
    -v "$PROJECT_ROOT/install.sh:/home/testuser/install.sh:ro" \
    -v "$PROJECT_ROOT/uninstall.sh:/home/testuser/uninstall.sh:ro" \
    -v "$PROJECT_ROOT/tests/docker/test-install.sh:/home/testuser/test-install.sh:ro" \
    -v "$PROJECT_ROOT/tests/docker/test-uninstall.sh:/home/testuser/test-uninstall.sh:ro" \
    -v "$TARBALL:/home/testuser/source.tar.gz:ro" \
    -e "SPEQ_LOCAL_TARBALL=/home/testuser/source.tar.gz" \
    -e "SPEQ_PREBUILT=1" \
    speq-install-test -c "/home/testuser/test-install.sh && /home/testuser/test-uninstall.sh"

echo ""
echo "=== Docker integration tests passed! ==="
if [[ "$CROSS_PLATFORM" == "true" ]]; then
    echo ""
    echo "Note: Cross-platform test used mock binary."
    echo "For full binary testing, run on Linux host or in CI."
fi
