# Feature: Embedding Model Provisioning

Ensures the semantic-search embedding model is placed into the speq model cache during installation, so the first `speq search` invocation works offline with no network access or download machinery in the binary.

## Background

* The embedding model identity is `Snowflake/snowflake-arctic-embed-xs`
* The model cache directory is `$XDG_CACHE_HOME/speq/models/` (falling back to the platform cache directory, or `.cache/speq/models/` when no writable system cache exists), matching the path the `speq` binary reads at search time
* The model is distributed as two files: model weights (`model.onnx`) and a tokenizer definition (`tokenizer.json`); the ONNX graph embeds the model configuration, so no separate config file is provisioned
* Model files are hosted as release assets on the speq-skill GitHub release that matches the installed version; `model.onnx` is located under the `onnx/` path segment on HuggingFace
* Provisioning is idempotent: if the model files already exist in the cache, the installer SHALL NOT re-download them
* The `speq` binary itself contains no model-download code; provisioning is performed exclusively by the installer

## Scenarios

### Scenario: Installer provisions model on a clean machine

* *GIVEN* the speq model cache directory contains no model files
* *AND* the install script is run for a published release version
* *WHEN* the install script reaches the model-provisioning step
* *THEN* the script SHALL download the model weights and tokenizer definition
* *AND* the script SHALL place both files (`model.onnx` and `tokenizer.json`) under the speq model cache directory
* *AND* a subsequent `speq search query` SHALL succeed without network access

### Scenario: Installer skips provisioning when the model is already cached

* *GIVEN* the speq model cache directory already contains all required model files (`model.onnx` and `tokenizer.json`)
* *WHEN* the install script reaches the model-provisioning step
* *THEN* the script SHALL detect the existing model files
* *AND* the script SHALL NOT re-download the model files
* *AND* the script SHALL report that the model is already provisioned

### Scenario: Model download fails during installation

* *GIVEN* the speq model cache directory contains no model files
* *AND* the model release assets cannot be downloaded
* *WHEN* the install script reaches the model-provisioning step
* *THEN* the script SHALL report a clear error identifying the failed download
* *AND* the script SHALL instruct the user how to provision the model manually
* *AND* the script MUST NOT leave a partially written model file in the cache directory

### Scenario: Model provisioning honors a custom cache directory

* *GIVEN* `$SPEQ_CACHE_DIR` is set to a custom path
* *WHEN* the install script provisions the embedding model
* *THEN* the script SHALL place the model files (`model.onnx` and `tokenizer.json`) under `$SPEQ_CACHE_DIR/models/`
* *AND* a subsequent `speq search query` run with the same `$SPEQ_CACHE_DIR` SHALL load the model from that path
