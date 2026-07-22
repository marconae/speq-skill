# Plan: fix-macos-model-cache-dir

## Summary

`install.sh` hardcodes the Linux/XDG cache-directory convention (`${XDG_CACHE_HOME:-$HOME/.cache}/speq/models`) for every platform, but the `speq` binary resolves its cache directory via the `dirs` crate, which on macOS returns `$HOME/Library/Caches/speq/models` instead. The installer provisions the model into the wrong directory on macOS, so `speq search query` reports the model missing even immediately after a fresh install — and re-running the installer just re-confirms the same wrong directory silently, since it never checks the path the binary actually reads.

## Design

### Context

- **Goals** — make `install.sh`'s default (no `$SPEQ_CACHE_DIR`) cache-directory resolution match the `speq` binary's platform-native resolution on every platform the binary supports.
- **Non-Goals** — no change to the `$SPEQ_CACHE_DIR` override path (already correct and already tested); no change to the Rust binary's `get_cache_path()` (already correct — it's the source of truth the installer must match).

### Decision

Branch `install.sh`'s default cache-directory computation on `uname -s`, mirroring the `dirs` crate's own platform split: `Darwin` → `$HOME/Library/Caches`, everything else → `${XDG_CACHE_HOME:-$HOME/.cache}` (the script's existing behavior, unchanged for Linux). The script already branches on `uname -s`/`Darwin` elsewhere (release asset naming), so this follows established local style rather than introducing a new pattern.

### Consequences

| Decision | Alternatives Considered | Rationale |
|----------|--------------------------|-----------|
| Branch on `uname -s` in `install.sh` | Have the Rust binary itself write a `speq --print-cache-dir` the installer shells out to | Rejected: the installer must run before the binary is even installed/extracted in some flows; duplicating the `dirs` crate's two-line platform split in bash is simpler and has no bootstrapping dependency |

## Features

| Feature | Status | Spec |
|---------|--------|------|
| installer/model-provisioning | CHANGED | `installer/model-provisioning/spec.md` |

## Implementation Tasks

1. Add a `default_cache_dir` helper to `install.sh` that branches on `uname -s` (`Darwin` → `$HOME/Library/Caches`, else → `${XDG_CACHE_HOME:-$HOME/.cache}`); use it in `provision_embedding_model` in place of the current hardcoded XDG-only computation, unless `$SPEQ_CACHE_DIR` is set (unchanged override path).
2. Add regression tests to `tests/install_model_provisioning.rs` covering both platform branches: fake `uname` on `PATH` returning `Darwin` asserts the model lands under `$HOME/Library/Caches/speq/models`; returning `Linux` (with `$XDG_CACHE_HOME` unset) asserts `$HOME/.cache/speq/models`. Neither test sets `$SPEQ_CACHE_DIR` — that's the exact gap that let this bug ship untested, since every existing test in this file sets it.

## Parallelization

None. Task 2's tests must fail against the pre-fix `install.sh` (TDD red) before task 1 makes them pass.

## Dead Code Removal

None. The change replaces one hardcoded path expression with a platform-branched helper; nothing becomes obsolete.

## Verification

### Scenario Coverage

| Scenario | Test Type | Test Location | Test Name |
|----------|-----------|---------------|-----------|
| Model provisioning uses the macOS cache directory | Integration | `tests/install_model_provisioning.rs` | `installer_provisions_model_into_macos_cache_dir` |
| Model provisioning uses the Linux XDG cache directory | Integration | `tests/install_model_provisioning.rs` | `installer_provisions_model_into_linux_cache_dir` |

### Manual Testing

| Feature | Command | Expected Output |
|---------|---------|------------------|
| installer/model-provisioning | `./tests/docker/test-docker.sh` (Linux container) | All four containers (install, codex-plugin, update, uninstall) pass; `install` container's model lands under `~/.cache/speq/models` inside the container as before |

### Checklist

| Step | Command | Expected |
|------|---------|----------|
| Build | `cargo build --release` | Exit 0 |
| Test | `cargo test` | 0 failures |
| Lint | `cargo fmt && cargo clippy` | No changes, 0 warnings |
| Docker integration | `./tests/docker/test-docker.sh` | All containers pass |
