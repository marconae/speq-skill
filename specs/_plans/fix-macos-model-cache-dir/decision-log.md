# Decision Log: fix-macos-model-cache-dir

## Design Decisions

### [1] Match `dirs` crate's platform split with a bash `uname -s` branch

- **Decision:** Add a `default_cache_dir` helper to `install.sh` that branches on `uname -s` (`Darwin` → `$HOME/Library/Caches`, else → `${XDG_CACHE_HOME:-$HOME/.cache}`), mirroring how `src/search.rs::get_cache_path()` resolves via the `dirs` crate. `install.sh` already branches on `uname -s`/`Darwin` elsewhere (release asset naming), so this follows established local style.
- **Alternatives:** Have the Rust binary expose a `speq --print-cache-dir` the installer shells out to.
- **Rationale:** The installer must run before the binary is necessarily installed/extracted in some flows, so shelling out to it would add a bootstrapping dependency. Duplicating the `dirs` crate's two-line platform split in bash is simpler and self-contained.
- **Promotes to ADR:** no

### [2] Root cause and test gap

- **Decision:** `install.sh::provision_embedding_model` computed its default model directory as `${XDG_CACHE_HOME:-$HOME/.cache}/speq/models` unconditionally, diverging from `get_cache_path()`'s macOS behavior (`$HOME/Library/Caches`). CI's Docker integration tests run exclusively on Linux, where both sides happen to agree, so the mismatch shipped undetected — confirmed by reading both code paths and matching the divergent path against the user's reported error (`/Users/john.doe/Library/Caches/speq/models`). Every existing test in `tests/install_model_provisioning.rs` sets `$SPEQ_CACHE_DIR`, bypassing the default-resolution branch entirely, so none of them could have caught this.
- **Alternatives:** n/a — root-cause record, not a design choice.
- **Rationale:** New tests must leave `$SPEQ_CACHE_DIR` unset and mock `uname -s` to exercise both platform branches from a Linux CI runner.
- **Promotes to ADR:** no
