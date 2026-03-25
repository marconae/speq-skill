#!/usr/bin/env bash
set -euo pipefail

# Install speq skill files into Codex's skills directory.
# Works from a local clone or can be piped from raw.githubusercontent.com.

REPO="marconae/speq-skill"
CODEX_HOME="${CODEX_HOME:-$HOME/.codex}"
DEST_DIR="$CODEX_HOME/skills"
TMP_DIR=""
SOURCE_DIR=""

cleanup() {
    if [[ -n "$TMP_DIR" && -d "$TMP_DIR" ]]; then
        rm -rf "$TMP_DIR"
    fi
}
trap cleanup EXIT

if [[ -d ".claude/skills" ]]; then
    SOURCE_DIR=".claude/skills"
else
    TMP_DIR="$(mktemp -d)"
    ARCHIVE_URL="https://github.com/$REPO/archive/refs/heads/main.tar.gz"
    curl -fsSL "$ARCHIVE_URL" -o "$TMP_DIR/source.tar.gz"
    tar -xzf "$TMP_DIR/source.tar.gz" -C "$TMP_DIR"
    SOURCE_DIR="$TMP_DIR/speq-skill-main/.claude/skills"
fi

mkdir -p "$DEST_DIR"

installed=0
for skill in "$SOURCE_DIR"/speq-*; do
    [[ -d "$skill" ]] || continue
    name="$(basename "$skill")"
    target="$DEST_DIR/$name"
    rm -rf "$target"
    cp -R "$skill" "$target"
    installed=$((installed + 1))
done

echo "Installed $installed speq skills into $DEST_DIR"
echo "Restart Codex to pick up new skills."
