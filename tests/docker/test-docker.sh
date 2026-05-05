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
FORCE_MOCK_BINARY="${SPEQ_DOCKER_MOCK_BINARY:-0}"
if [[ "$HOST_OS" == "Darwin" ]] || [[ "$FORCE_MOCK_BINARY" == "1" ]]; then
    CROSS_PLATFORM=true
    echo "=== Mock binary mode enabled ==="
    echo "Note: Using mock binary for installer/plugin testing. Full binary testing requires Linux host with Cargo dependencies available."
    echo ""
fi

if [[ "$CROSS_PLATFORM" != "true" ]]; then
    echo "=== Building release binary ==="
    cargo build --release
fi

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
tar \
    --exclude='./.git' \
    --exclude='./target' \
    --exclude='./dist' \
    --exclude='./.DS_Store' \
    --exclude='./._*' \
    -cf - . | tar -xf - -C "$TARBALL_DIR/speq-skill-main/"

# Include binary - use mock for cross-platform, real binary for same-platform
mkdir -p "$TARBALL_DIR/speq-skill-main/target/release"
if [[ "$CROSS_PLATFORM" == "true" ]]; then
    # Create mock binary that simulates speq CLI
    cat > "$TARBALL_DIR/speq-skill-main/target/release/speq" << 'MOCK_EOF'
#!/bin/sh
# Mock speq binary for cross-platform Docker testing
VERSION="0.4.1"
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
if docker image inspect speq-install-test &>/dev/null; then
    echo "=== Docker test image already present, skipping build ==="
else
    echo "=== Building Docker test image ==="
    docker build -t speq-install-test -f tests/docker/Dockerfile .
fi

echo ""
echo "=== Running integration tests in parallel ==="

declare -a PIDS=()
declare -a NAMES=()

docker run --rm \
    -v "$PROJECT_ROOT/install.sh:/home/testuser/install.sh:ro" \
    -v "$PROJECT_ROOT/tests/docker/test-install.sh:/home/testuser/test-install.sh:ro" \
    -v "$TARBALL:/home/testuser/source.tar.gz:ro" \
    -e "SPEQ_LOCAL_TARBALL=/home/testuser/source.tar.gz" \
    -e "SPEQ_PREBUILT=1" \
    speq-install-test -c "/home/testuser/test-install.sh" &
PIDS+=($!); NAMES+=("install")

docker run --rm \
    -v "$PROJECT_ROOT/install.sh:/home/testuser/install.sh:ro" \
    -v "$PROJECT_ROOT/tests/docker/test-codex-plugin.sh:/home/testuser/test-codex-plugin.sh:ro" \
    -v "$TARBALL:/home/testuser/source.tar.gz:ro" \
    -e "SPEQ_LOCAL_TARBALL=/home/testuser/source.tar.gz" \
    -e "SPEQ_PREBUILT=1" \
    speq-install-test -c "/home/testuser/test-codex-plugin.sh" &
PIDS+=($!); NAMES+=("codex-plugin")

docker run --rm \
    -v "$PROJECT_ROOT/install.sh:/home/testuser/install.sh:ro" \
    -v "$PROJECT_ROOT/tests/docker/test-update.sh:/home/testuser/test-update.sh:ro" \
    -v "$TARBALL:/home/testuser/source.tar.gz:ro" \
    -e "SPEQ_LOCAL_TARBALL=/home/testuser/source.tar.gz" \
    -e "SPEQ_PREBUILT=1" \
    speq-install-test -c "/home/testuser/test-update.sh" &
PIDS+=($!); NAMES+=("update")

docker run --rm \
    -v "$PROJECT_ROOT/install.sh:/home/testuser/install.sh:ro" \
    -v "$PROJECT_ROOT/uninstall.sh:/home/testuser/uninstall.sh:ro" \
    -v "$PROJECT_ROOT/tests/docker/test-install.sh:/home/testuser/test-install.sh:ro" \
    -v "$PROJECT_ROOT/tests/docker/test-uninstall.sh:/home/testuser/test-uninstall.sh:ro" \
    -v "$TARBALL:/home/testuser/source.tar.gz:ro" \
    -e "SPEQ_LOCAL_TARBALL=/home/testuser/source.tar.gz" \
    -e "SPEQ_PREBUILT=1" \
    speq-install-test -c "/home/testuser/test-install.sh && /home/testuser/test-uninstall.sh" &
PIDS+=($!); NAMES+=("uninstall")

echo "Started ${#PIDS[@]} test containers in parallel..."

FAILED=0
for i in "${!PIDS[@]}"; do
    if wait "${PIDS[$i]}"; then
        echo "  ✓ ${NAMES[$i]}"
    else
        echo "  ✗ ${NAMES[$i]} FAILED"
        FAILED=1
    fi
done

[ "$FAILED" -eq 0 ] || exit 1

echo ""
echo "=== Docker integration tests passed! ==="
if [[ "$CROSS_PLATFORM" == "true" ]]; then
    echo ""
    echo "Note: Cross-platform test used mock binary."
    echo "For full binary testing, run on Linux host or in CI."
fi
