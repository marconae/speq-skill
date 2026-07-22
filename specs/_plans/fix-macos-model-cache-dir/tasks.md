# Tasks: fix-macos-model-cache-dir

## Phase 2: Implementation

- [x] 2.1 Add failing tests to `tests/install_model_provisioning.rs`: `installer_provisions_model_into_macos_cache_dir` (fake `uname` returning `Darwin`, `$SPEQ_CACHE_DIR` unset, assert model lands under `$HOME/Library/Caches/speq/models`) and `installer_provisions_model_into_linux_cache_dir` (fake `uname` returning `Linux`, `$XDG_CACHE_HOME` unset, `$SPEQ_CACHE_DIR` unset, assert model lands under `$HOME/.cache/speq/models`). Confirm both fail against the current `install.sh` (RED).
- [x] 2.2 Add a `default_cache_dir` helper to `install.sh` branching on `uname -s` (`Darwin` → `$HOME/Library/Caches`, else → `${XDG_CACHE_HOME:-$HOME/.cache}`); use it in `provision_embedding_model`'s default (non-`$SPEQ_CACHE_DIR`) branch. Confirm both new tests pass (GREEN), and the four existing tests in the same file still pass (they all set `$SPEQ_CACHE_DIR`, so the override branch — unchanged — must still work).

## Phase 4: Verification

- [x] 4.1 Run full test suite (`cargo test`) — 0 failures
- [x] 4.2 Run `cargo fmt && cargo clippy` — no changes, 0 warnings
- [x] 4.3 Run `speq plan validate fix-macos-model-cache-dir` — pass
