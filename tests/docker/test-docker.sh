#!/bin/bash
# Run integration test in Docker
# Usage: ./tests/docker/test-docker.sh [version]
#
# This script tests the install.sh script by creating a mock marketplace archive.
# For full binary testing, use CI which has access to released binaries.
set -euo pipefail

VERSION="${1:-dev}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$PROJECT_ROOT"

# Build plugin structure (no binary needed for script testing)
echo "=== Building plugin structure ==="
"$PROJECT_ROOT/scripts/plugin/build.sh"

# Create a mock marketplace archive with a placeholder binary
echo "=== Creating mock marketplace archive ==="
ARCHIVE_DIR="$PROJECT_ROOT/dist/docker-build/speq-marketplace-${VERSION}-x86_64-linux"
rm -rf "$PROJECT_ROOT/dist/docker-build"
mkdir -p "$ARCHIVE_DIR/bin"
mkdir -p "$ARCHIVE_DIR/.claude-plugin"
mkdir -p "$ARCHIVE_DIR/plugins"

# Create a mock binary (just a shell script for testing)
cat > "$ARCHIVE_DIR/bin/speq" << 'EOF'
#!/bin/bash
echo "speq mock binary v${VERSION:-dev}"
echo "This is a test placeholder"
exit 0
EOF
chmod +x "$ARCHIVE_DIR/bin/speq"

cp -r "$PROJECT_ROOT/dist/marketplace/.claude-plugin/." "$ARCHIVE_DIR/.claude-plugin/"
cp -r "$PROJECT_ROOT/dist/marketplace/plugins/." "$ARCHIVE_DIR/plugins/"

# Create tarball
ARCHIVE="$PROJECT_ROOT/dist/docker-build/speq-marketplace-${VERSION}-x86_64-linux.tar.gz"
tar -czf "$ARCHIVE" -C "$PROJECT_ROOT/dist/docker-build" "speq-marketplace-${VERSION}-x86_64-linux"
echo "Created archive: $ARCHIVE"

echo ""
echo "=== Building Docker test image ==="
docker build -t speq-install-test -f tests/docker/Dockerfile .

echo ""
echo "=== Running integration test ==="
docker run --rm \
    -v "$PROJECT_ROOT/install.sh:/home/testuser/install.sh:ro" \
    -v "$PROJECT_ROOT/tests/docker/test-install.sh:/home/testuser/test-install.sh:ro" \
    -v "$ARCHIVE:/home/testuser/marketplace.tar.gz:ro" \
    -e "SPEQ_LOCAL_ARCHIVE=/home/testuser/marketplace.tar.gz" \
    speq-install-test -c "/home/testuser/test-install.sh"

echo ""
echo "=== Docker integration test passed! ==="
echo ""
echo "Note: This test uses a mock binary. For full binary testing,"
echo "run CI or use: SPEQ_LOCAL_ARCHIVE=<path> ./install.sh"
