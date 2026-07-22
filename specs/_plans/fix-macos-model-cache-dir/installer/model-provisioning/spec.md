# Feature: Embedding Model Provisioning

Ensures the semantic-search embedding model is placed into the speq model cache during installation, so the first `speq search` invocation works offline with no network access or download machinery in the binary.

## Background

* The embedding model identity is `Snowflake/snowflake-arctic-embed-xs`
* The model cache directory is `$XDG_CACHE_HOME/speq/models/` (falling back to the platform cache directory, or `.cache/speq/models/` when no writable system cache exists), matching the path the `speq` binary reads at search time

## Scenarios

<!-- DELTA:NEW -->
### Scenario: Model provisioning uses the macOS cache directory

* *GIVEN* `$SPEQ_CACHE_DIR` is unset
* *AND* the install script runs on macOS
* *WHEN* the install script provisions the embedding model
* *THEN* the script SHALL place the model files (`model.onnx` and `tokenizer.json`) under `$HOME/Library/Caches/speq/models/`
* *AND* a subsequent `speq search query` SHALL load the model from that same path
<!-- /DELTA:NEW -->

<!-- DELTA:NEW -->
### Scenario: Model provisioning uses the Linux XDG cache directory

* *GIVEN* `$SPEQ_CACHE_DIR` is unset
* *AND* the install script runs on Linux
* *WHEN* the install script provisions the embedding model
* *THEN* the script SHALL place the model files (`model.onnx` and `tokenizer.json`) under `${XDG_CACHE_HOME:-$HOME/.cache}/speq/models/`
* *AND* a subsequent `speq search query` SHALL load the model from that same path
<!-- /DELTA:NEW -->
