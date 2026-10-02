# Decisions: refactor-search-pure-rust-inference

## ADR: Run search embeddings in pure Rust

**ID:** pure-rust-inference
**Plan:** refactor-search-pure-rust-inference
**Status:** Accepted

### Context

Semantic search needs a neural embedding model. ONNX Runtime is a native C++ library with no prebuilt binary for Intel macOS, and a missing library aborts the process instead of returning an error.

### Decision

Run embedding inference in pure Rust, with no native library and one code path on all platforms.

### Options Considered

- ONNX Runtime with the panic caught: rejected, it leaves Intel macOS users without search and keeps the native dependency.
- Lexical BM25 or TF-IDF search: rejected, it changes search from semantic to lexical.

## ADR: Use tract-onnx for embedding inference

**ID:** tract-onnx-inference
**Plan:** refactor-search-pure-rust-inference
**Status:** Accepted

### Context

Pure-Rust inference needs an engine for the embedding model. The repository must not carry vendored third-party code.

### Decision

Run the upstream pre-built ONNX model on the tract-onnx engine. It is pure Rust and needs no vendored code.

### Options Considered

- The candle BERT encoder: rejected, a transitive dependency crashes on some CPUs and needs a vendored patch.

## ADR: Provision the embedding model in the installer

**ID:** installer-model-provisioning
**Plan:** refactor-search-pure-rust-inference
**Status:** Accepted

### Context

The binary needs an embedding model at runtime. It is distributed through `cargo install` and a shell installer.

### Decision

The binary contains no download code. The installer downloads the model once into the cache directory, and search then runs offline.

### Options Considered

- Download in the binary on first run: rejected, it adds a network and TLS dependency and needs internet on first use.
- Embed the model in the binary: rejected, it adds about 23 MB to every install.
