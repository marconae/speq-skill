#!/usr/bin/env bash
# Release version — single source of truth is Cargo.toml.
# Source this file, then call get_version. The repo root is resolved from this
# file's own location, so callers need no PROJECT_ROOT.

get_version() {
    local root
    root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
    grep '^version' "$root/Cargo.toml" | sed 's/.*"\(.*\)".*/\1/'
}
