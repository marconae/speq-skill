# Feature: CLI Search Index

The CLI SHALL build the semantic search index over a project's scenarios, store it in the cache directory, and reuse the stored embeddings of unchanged scenarios when it rebuilds the index.

## Background

* The cache directory is `$SPEQ_CACHE_DIR` when it is set. Otherwise it is the platform cache directory joined with `speq`: `${XDG_CACHE_HOME:-$HOME/.cache}/speq` on Linux
* The cache directory contains `indexes/`, which holds binary index files, one per project
* Index file named after project path slug (e.g., `-home-user-code-my-project.idx`)
* Slug format: absolute project path with `/` replaced by `-` (e.g., `/home/user/code/my-project` → `-home-user-code-my-project`)
* Searchable units: scenarios (domain/feature/scenario granularity)
* The index file stores its index format, the model revision, and for each scenario its path, its embedded text (the scenario name followed by its steps), and its stored embedding
* Index format: a value stored in the index that identifies how the index file arranges its data and how speq turns text into embeddings
* Model revision: the revision of the embedding model files, as recorded in the cache directory when the model files are provisioned
* Reusable index: an index file that the system can decode and whose index format and model revision equal the current ones
* Outdated index: an index file that exists but is not reusable

## Scenarios

### Scenario: Build search index

* *GIVEN* a specs directory with feature specifications and no search index for the project
* *WHEN* the user runs `speq search index`
* *THEN* the system SHALL parse all spec files
* *AND* the system SHALL generate an embedding for each scenario and store it in a binary index file named after the project slug
* *AND* the system SHALL display the number of scenarios indexed
* *AND* the system SHALL exit with code 0

### Scenario: Index inline code in scenario text

* *GIVEN* a scenario whose name and steps contain inline code spans
* *WHEN* the user runs `speq search index`
* *THEN* the indexed text of that scenario SHALL contain each inline code span wrapped in backticks

### Scenario: Rebuild reuses embeddings of unchanged scenarios

* *GIVEN* a reusable search index, built before one scenario's text changed, one scenario was added, and one scenario was removed
* *WHEN* the user runs `speq search index`
* *THEN* the system SHALL generate embeddings only for the changed scenario and the added scenario
* *AND* the system SHALL keep the stored embedding of every unchanged scenario
* *AND* the index MUST NOT contain the removed scenario
* *AND* the system SHALL display the total number of scenarios in the index

### Scenario: Rebuilt index stays searchable

* *GIVEN* a reusable search index, built before one scenario's text changed, one scenario was added, and one scenario was removed
* *WHEN* the user runs `speq search index`
* *THEN* the system SHALL write an index that it can decode, that holds each current scenario exactly once in the order of a full build, and that does not hold the removed scenario
* *AND* `speq search query` with the embedded text of the changed, the added, or an unchanged scenario SHALL list that scenario with the highest score of the results

### Scenario: Changed model revision or index format re-embeds every scenario

* *GIVEN* a search index whose stored model revision or index format differs from the current one
* *WHEN* the user runs `speq search index`
* *THEN* the system SHALL generate embeddings for every scenario
* *AND* the system MUST NOT reuse any stored embedding
* *AND* the system SHALL store the current model revision and index format in the index
* *AND* the system SHALL exit with code 0

### Scenario: Index stays reusable without a recorded model revision

* *GIVEN* no model revision is recorded in the cache directory, and a search index was built under that condition
* *WHEN* the user runs `speq search index` with no change to the specs
* *THEN* the system SHALL keep the stored embedding of every scenario
* *AND* the system MUST NOT generate any embedding

### Scenario: Outdated index is rebuilt in full

* *GIVEN* an index file for the project that the system cannot decode or that an older speq release wrote
* *WHEN* the user runs `speq search index`
* *THEN* the system SHALL generate embeddings for every scenario
* *AND* the system MUST NOT reuse any stored embedding
* *AND* the system SHALL store the current index format in the index
* *AND* the system SHALL exit with code 0

### Scenario: Full index build is deterministic

* *GIVEN* the same specs and the same embedding model
* *WHEN* the user runs `speq search index` twice, each time with no prior index for the project
* *THEN* the two runs SHALL write byte-identical index files
