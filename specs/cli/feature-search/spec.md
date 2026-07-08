# Feature: CLI Feature Search

The CLI SHALL provide semantic search for feature specifications using vector embeddings.

## Background

* Command syntax: `speq search query <query>` for searching, `speq search index` for rebuilding
* Search uses vector embeddings for semantic similarity
* Single app cache at `$XDG_CACHE_HOME/speq/` containing:
  - `models/` - the embedding model files (weights, tokenizer, config)
  - `indexes/` - binary index files, one per project
* Index file named after project path slug (e.g., `-home-user-code-my-project.idx`)
* Slug format: absolute project path with `/` replaced by `-` (e.g., `/home/user/code/my-project` → `-home-user-code-my-project`)
* Searchable units: scenarios (domain/feature/scenario granularity)
* Results ranked by cosine similarity
* If no index exists when searching, the system SHALL automatically build it
* Exit code 0 regardless of match count
* The embedding model runs entirely in pure Rust with no native ONNX Runtime library and no C/C++ runtime dependency, so behavior is identical on every supported platform including Intel macOS (`x86_64-apple-darwin`)

## Scenarios

### Scenario: Build search index

* *GIVEN* a specs directory with feature specifications
* *WHEN* the user runs `speq search index`
* *THEN* the system SHALL parse all spec files
* *AND* the system SHALL generate embeddings for each scenario
* *AND* the system SHALL store vectors in binary index file named after project slug
* *AND* the system SHALL display the number of scenarios indexed
* *AND* the system SHALL exit with code 0

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
* *THEN* the system SHALL automatically build the search index
* *AND* the system SHALL execute the search query
* *AND* the system SHALL display results if any match
* *AND* the system SHALL exit with code 0

### Scenario: No matches found

* *GIVEN* an index exists
* *AND* no scenarios are semantically similar to the query
* *WHEN* the user runs `speq search query "completely unrelated topic xyz"`
* *THEN* the system SHALL display "No matches found."
* *AND* the system SHALL exit with code 0

### Scenario: Cache storage location

* *GIVEN* `$XDG_CACHE_HOME` is set to `/home/user/.cache`
* *AND* the project path is `/home/user/code/my-project`
* *WHEN* the user runs `speq search index`
* *THEN* the index file SHALL be stored at `/home/user/.cache/speq/indexes/-home-user-code-my-project.idx`
* *AND* the embedding model files SHALL be cached under `/home/user/.cache/speq/models/`
* *AND* the model identity SHALL be `Snowflake/snowflake-arctic-embed-xs`
* *AND* the cached model files SHALL consist of model weights, a tokenizer definition, and a model configuration
* *AND* subsequent operations SHALL reuse the cached model files without re-downloading them

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
