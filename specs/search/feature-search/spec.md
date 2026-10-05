# Feature: CLI Feature Search

The CLI SHALL provide semantic search for feature specifications using vector embeddings.

## Background

* Command syntax: `speq search query <query>` for searching, `speq search index` for rebuilding
* Search uses vector embeddings for semantic similarity
* The `search/search-index` feature defines the cache directory, the reusable index, and the outdated index
* The cache directory contains `models/`, which holds the embedding model files `model.onnx` and `tokenizer.json`
* Results ranked by cosine similarity
* If no index exists when searching, the system SHALL automatically build it
* Exit code 0 regardless of match count
* The embedding model runs entirely in pure Rust with no runtime dependency, so behavior is identical on every supported platform

## Scenarios

### Scenario: Search semantically similar scenarios

* *GIVEN* an index exists with scenarios about "validating documents"
* *WHEN* the user runs `speq search query "check file format"`
* *THEN* the system SHALL find semantically similar scenarios
* *AND* the system SHALL display results ranked by similarity score
* *AND* the system SHALL show the scenario path (domain/feature/scenario)
* *AND* the system SHALL show a snippet of the scenario content
* *AND* the system SHALL exit with code 0

### Scenario: Search with limit

* *GIVEN* an index exists with many scenarios
* *WHEN* the user runs `speq search query "validation" --limit 5`
* *THEN* the system SHALL return at most 5 results
* *AND* the system SHALL exit with code 0

### Scenario: No index exists

* *GIVEN* no search index exists for the project
* *WHEN* the user runs `speq search query "validation"`
* *THEN* the system SHALL print the notice `Info: no search index found. Building it now, this may take a while.` to stderr
* *AND* the system SHALL automatically build the search index and execute the search query
* *AND* the system SHALL print the results to stdout without the notice
* *AND* the system SHALL exit with code 0

### Scenario: No matches found

* *GIVEN* an index exists
* *AND* no scenarios are semantically similar to the query
* *WHEN* the user runs `speq search query "completely unrelated topic xyz"`
* *THEN* the system SHALL display "No matches found."
* *AND* the system SHALL exit with code 0

### Scenario: Cache storage location

* *GIVEN* the system runs on Linux with `$XDG_CACHE_HOME` set to `/home/user/.cache` and `$SPEQ_CACHE_DIR` unset
* *AND* the project path is `/home/user/code/my-project`
* *WHEN* the user runs `speq search index`
* *THEN* the system SHALL store the index file at `/home/user/.cache/speq/indexes/-home-user-code-my-project.idx`
* *AND* the system SHALL load the model files `model.onnx` and `tokenizer.json` from `/home/user/.cache/speq/models/`

### Scenario: Model files present in cache

* *GIVEN* the embedding model files have been provisioned into the speq model cache directory
* *WHEN* the user runs `speq search query "validation"`
* *THEN* the system SHALL load the embedding model from the cache directory
* *AND* the system MUST NOT attempt any network access
* *AND* the system SHALL execute the search query
* *AND* the system SHALL exit with code 0

### Scenario: Model files missing from cache

* *GIVEN* the embedding model files are absent from the speq model cache directory
* *WHEN* the user runs `speq search query "validation"`
* *THEN* the system MUST report an actionable error naming the expected model cache directory
* *AND* the error message SHALL instruct the user how to provision the model
* *AND* the system MUST NOT panic with a runtime crash
* *AND* the system MUST exit with a non-zero status code

### Scenario: Search runs without a native ONNX Runtime library

* *GIVEN* the speq binary is built for any supported platform
* *AND* no ONNX Runtime shared library is installed on the system
* *WHEN* the user runs `speq search query "validation"`
* *THEN* the system SHALL execute the search query using the pure-Rust inference path
* *AND* the system MUST NOT require, load, or dynamically link an ONNX Runtime library
* *AND* the system SHALL exit with code 0

### Scenario: Existing index prints no build notice

* *GIVEN* a reusable search index exists for the project
* *WHEN* the user runs `speq search query "validation"`
* *THEN* the system MUST NOT print the `Info: no search index found.` notice
* *AND* the system MUST NOT print the `Info: search index is outdated.` notice
* *AND* the system SHALL execute the search query against the existing index
* *AND* the system SHALL exit with code 0

### Scenario: Outdated index is rebuilt before searching

* *GIVEN* an outdated search index exists for the project
* *WHEN* the user runs `speq search query "validation"`
* *THEN* the system SHALL print the notice `Info: search index is outdated. Rebuilding it now, this may take a while.` to stderr instead of the `Info: no search index found.` notice
* *AND* the system SHALL rebuild the search index and execute the search query
* *AND* the system SHALL print the results to stdout without the notice
* *AND* the system SHALL exit with code 0
