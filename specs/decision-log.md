# Architecture Decision Records

## ADR-001: Adopt pure-Rust inference (Option B) to remove the ONNX Runtime dependency

**Date:** 2026-05-22
**Plan:** refactor-search-pure-rust-inference
**Status:** Accepted

### Context

`speq search` relied on `fastembed`/`ort`, which binds to the ONNX Runtime C++ library. On Intel macOS (`x86_64-apple-darwin`), no prebuilt `ort` binary exists, so the library is loaded via `dlopen` at runtime. When absent, `ort` panics inside `TextEmbedding::try_new` rather than returning an error, causing an unrecoverable process abort (exit 101). A narrow fix wrapping the call in `catch_unwind` (branch `fix-intel-mac-onnxruntime-panic`) makes the failure survivable but leaves Intel-Mac users unable to search without `brew install onnxruntime`, keeps the C++ dependency, and keeps the platform-split `Cargo.toml`. The user asked to remove `ort` entirely.

### Decision

Replace `fastembed`/`ort` with a pure-Rust inference path. Remove the ONNX Runtime C/C++ library from the dependency tree entirely. Deliver one uniform inference code path for all platforms with no `cfg`-gated dependency split.

### Options Considered

- **Option A (narrow panic fix):** Keep `fastembed`/`ort`; wrap `TextEmbedding::try_new` in `catch_unwind`. A safety net, not a cure — leaves Intel-Mac degraded and the C++ dependency intact.
- **Option B (pure-Rust inference) — chosen:** Replace the ONNX-based stack with a pure-Rust alternative. Eliminates the root cause, unifies platforms, shrinks binary.
- **Option C (lexical BM25/TF-IDF):** Drop the neural model entirely. Rejected — regresses search from semantic to lexical matching, changing the product.

### Consequences

No ONNX Runtime library is required at build time or runtime. `cargo install speq-skill` requires no native toolchain or system library. The Intel-Mac `dlopen` panic is eliminated structurally. The `Cargo.toml` platform-split `[target.'cfg(...)'.dependencies]` blocks are removed.

---

## ADR-002: Use candle native BERT (Option B2) as the pure-Rust inference backend

**Date:** 2026-05-22
**Plan:** refactor-search-pure-rust-inference
**Status:** Accepted

### Context

Having decided on a pure-Rust inference path (ADR-001), two sub-variants were evaluated: a pure-Rust ONNX runtime (`tract`, keeping the existing `.onnx` model file) and a native Rust BERT encoder (`candle`, storing the model as `safetensors`). The embedding model `Snowflake/snowflake-arctic-embed-xs` is architecturally `all-MiniLM-L6-v2`, a standard 6-layer BERT encoder — fully expressible natively in Rust.

### Decision

Implement inference with `candle-core` + `candle-nn` + `candle-transformers` (`models::bert::BertModel`) plus the `tokenizers` crate. Store the model as `model.safetensors` + `tokenizer.json` + `config.json`.

### Options Considered

- **Option B1 (`tract` pure-Rust ONNX):** Keeps the existing `.onnx` file but transformer-operator coverage in `tract` is uneven; ONNX graphs often need massaging; still requires a separate tokenizer crate. Higher integration risk with no advantage over B2.
- **Option B2 (`candle` native BERT) — chosen:** `candle-transformers` ships a maintained native `BertModel`; HuggingFace's own candle examples compute `all-MiniLM-L6-v2` sentence embeddings. `safetensors` is memory-mappable and has no code-execution surface. All new crates are Apache-2.0/MIT, compatible with this MIT project.

### Consequences

Inference is performed by `src/embedding.rs` (`Embedder` type) using `candle` on the CPU device. The model is loaded from `$SPEQ_CACHE/speq/models/` as three files. The `.onnx` model file is no longer used. Embedding dimensionality (384, L2-normalized) and the `.idx` index format are unchanged.

---

## ADR-003: Move model acquisition out of the binary into the installer

**Date:** 2026-05-22
**Plan:** refactor-search-pure-rust-inference
**Status:** Accepted

### Context

The previous `fastembed`/`ort` path downloaded the model on first run via `hf-hub`. With the switch to candle (ADR-002), two alternative model-delivery approaches were considered: keep an in-binary downloader or embed the weights with `include_bytes!`. Both have significant downsides for a CLI tool distributed via `cargo install` and a shell installer.

### Decision

The `speq` binary contains no model-download code. It only reads model files from `$SPEQ_CACHE/speq/models/`. `install.sh` (and the future Homebrew formula) provision the three model files at install time by downloading them from the GitHub release assets that match the installed version.

### Options Considered

- **In-binary downloader (`hf-hub`):** Re-introduces a network dependency and TLS stack into the binary; first-run search requires internet access.
- **`include_bytes!` embedded weights:** Bloats the binary by ~23 MB and ships the model on every `cargo install`, regardless of whether search is used.
- **Installer provisioning — chosen:** Keeps the binary small, makes first-run search offline once installed, and keeps `cargo install` free of a native build dependency or large download.

### Consequences

`speq search` exits with an actionable error (naming the cache directory and provisioning instruction) when model files are absent. The installer must be run (or the model provisioned manually) before search works. Publishing model files as GitHub release assets is required; task 5.3 (release asset publishing) is deferred to when binary distribution ships.

---

## ADR-004: Snowflake/snowflake-arctic-embed-xs model weights are Apache 2.0 — redistribute with attribution

**Date:** 2026-05-22
**Plan:** refactor-search-pure-rust-inference
**Status:** Accepted

### Context

Shipping `model.safetensors`, `tokenizer.json`, and `config.json` as release assets requires verifying the model's license is compatible with this MIT project and that redistribution obligations are met. `cargo deny check` audits Rust crate licenses only; model file compliance is a separate concern. Verification was performed on 2026-05-22.

### Decision

Ship the model weights as release assets. Include the model's Apache 2.0 `LICENSE` file (and `NOTICE` if present) alongside `THIRD_PARTY_LICENSES` in the marketplace archive and the Homebrew formula. `cargo deny` does not cover model files; model license compliance is verified as a separate manual/CI step in the release script (task 5.3).

### Options Considered

- **Do not redistribute model weights:** Require users to download from HuggingFace directly. Eliminates redistribution obligations but breaks the offline-install goal.
- **Redistribute without license file:** Non-compliant with Apache 2.0 attribution requirement.
- **Redistribute with attribution — chosen:** Apache 2.0 ↔ MIT is fully compatible (both permissive, no copyleft, no non-commercial clause). Attribution is satisfied by preserving the copyright notice and `NOTICE` file.

### Consequences

The release script (task 5.3) must bundle the model's Apache 2.0 `LICENSE` (and `NOTICE` if present) in the archive. `cargo deny check` passes because it sees only Rust crate licenses. A separate CI step or checklist item covers model file license compliance. `sentence-transformers/all-MiniLM-L6-v2` (base model, Apache 2.0) and `Snowflake/snowflake-arctic-embed-xs` (fine-tune, Apache 2.0) training data datasets are all permissively licensed.
