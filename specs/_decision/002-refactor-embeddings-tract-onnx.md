# Decisions: refactor-embeddings-tract-onnx

## ADR: Adopt tract-onnx and remove the vendored gemm-common patch

**ID:** tract-onnx-inference
**Plan:** refactor-embeddings-tract-onnx
**Status:** Accepted
**Supersedes:** candle-native-bert

> Supersedes the candle native BERT decision (`candle-native-bert`).

### Context

The `candle-native-bert` decision chose `candle` native BERT for inference because `candle-transformers` ships a ready-made `BertModel`. That choice transitively pulled in `gemm-common` 0.19.0, which panics on CPUs that expose an L4 cache: `all_info` is a 3-element array indexed by `all_info[level - 1]` with no bounds check. The only workaround was to vendor a patched copy of `gemm-common` under `vendor/gemm-common/` and override it via a `[patch.crates-io]` block in `Cargo.toml`. The user does not want third-party code carried in this repository.

### Decision

Replace `candle-core` + `candle-nn` + `candle-transformers` with `tract-onnx`, a pure-Rust CPU ONNX inference engine with no native dependencies. Load the upstream pre-built `onnx/model.onnx` graph for `Snowflake/snowflake-arctic-embed-xs`. The `tokenizers` crate stays. The model is provisioned as two files — `model.onnx` + `tokenizer.json`; `config.json` is no longer provisioned because the ONNX graph embeds the configuration. Delete `vendor/gemm-common/` and the `[patch.crates-io]` block. Embedding dimensionality (384, L2-normalized), CLS pooling, and the `.idx` index format are unchanged. Shipped in release 0.5.1.

### Options Considered

- **Keep candle + vendored gemm-common patch:** Rejected — leaves third-party code in the repo indefinitely and depends on an unfixed upstream bug.
- **Wait for an upstream gemm-common fix:** Rejected — unknown timeline; vendored code remains in the interim.
- **Adopt tract-onnx — chosen:** `tract-onnx` does not depend on `gemm-common`, so the vendored patch and `[patch.crates-io]` override disappear entirely. The `candle-native-bert` decision originally rejected tract for uneven transformer-operator coverage, but consuming the upstream pre-built ONNX graph (rather than converting our own) sidesteps that risk. `tract-onnx` is MIT OR Apache-2.0, compatible with this MIT project.

### Consequences

`vendor/gemm-common/` and the `Cargo.toml` `[patch.crates-io]` block are removed; `gemm-common` no longer appears in `Cargo.lock`. `src/embedding.rs` is rewritten on the tract runnable-model API; the `config.json` constant and read are removed. `src/search.rs` `get_model_file_paths` returns a 2-tuple. `install.sh` provisions `model.onnx` (from the `onnx/` path on HuggingFace) and `tokenizer.json`. The installer's idempotency, error-handling, and custom-cache behavior (`installer-model-provisioning`) and the no-in-binary-download architecture are unchanged. Existing 0.5.0 caches lack `model.onnx`, so a 0.5.1 install re-provisions it.
