# Architecture

## Overview

```
                    ┌──────────────┐
 argv ─────────────▶│ cli (clap)   │
                    └──────┬───────┘
                           ▼
                    ┌──────────────┐
                    │ main.rs      │ dispatches, prints output
                    └──┬──┬──┬──┬──┘
       ┌───────────────┘  │  │  └────────────────┐
       ▼                  ▼  ▼                   ▼
 ┌──────────┐   ┌────────┐ ┌────────┐     ┌──────────┐
 │ feature  │   │ plan   │ │ record │     │ search   │
 │ tree     │   └───┬────┘ └───┬────┘     └────┬─────┘
 └────┬─────┘       │          │               │
      │             ▼          ▼               ▼
      │        ┌─────────────────────┐   ┌───────────┐
      └───────▶│ validate            │   │ embedding │
               │ parser rules        │   │ (tract)   │
               │ report decision_log │   └─────┬─────┘
               └─────────────────────┘         ▼
                        │              model files + index
                        ▼              in the cache directory
                  specs/ on disk
```

- Modular single-binary CLI. One module per capability. The library crate exposes the modules and `main.rs` is a thin dispatcher.

## Components

- cli (src/cli.rs): defines commands and arguments with clap derive | owns: argument definitions | depends on: none
- main (src/main.rs): parses arguments, calls module handlers, prints results, sets the exit code | owns: output formatting of command results | depends on: cli, feature, plan, record, search, tree, validate
- validate (src/validate/): parses spec Markdown and applies structural rules and RFC2119 checks | owns: parse results and validation reports | depends on: feature
- validate::decision_log (src/validate/decision_log.rs): validates decision fragments in specs/_decision and plan decision logs, and assembles the permanent log | owns: none | depends on: none
- feature (src/feature.rs): discovers domains and features by walking directories under specs | owns: feature path model | depends on: none
- tree (src/tree.rs): renders the domain and feature tree view | owns: none | depends on: feature
- plan (src/plan.rs): lists active plans and validates a plan's delta specs and decision log | owns: plan validation results | depends on: validate, record
- record (src/record.rs): parses delta markers, merges deltas into permanent specs in memory, writes them, archives the plan to specs/_recorded | owns: recorded plan archive | depends on: none
- search (src/search.rs): builds and queries the semantic index over scenarios, reuses the stored vectors of unchanged scenarios from a reusable index, decides when an index is outdated, resolves cache paths | owns: index file, index reuse rule, and cache path resolution | depends on: feature, validate, embedding
- embedding (src/embedding.rs): reads the model revision recorded next to the model files, loads the ONNX model and the tokenizer, produces 384-dimension L2-normalized vectors by running the model on each text alone with at most 8 model calls in parallel, and defines the index format | owns: loaded model and tokenizer, model revision text, index format constant, parallel inference | depends on: none

## Data Flow

- arguments -> cli -> main -> module handler -> stdout: a command runs and prints its result, exit code 1 on failure
- specs/<domain>/<feature>/spec.md -> feature -> validate -> main: discovery feeds parsing and rule checks
- specs/_plans/<plan>/ -> plan -> validate: delta files are checked for marker balance, anchors, and structure
- specs/_plans/<plan>/ -> record -> specs/ and specs/_recorded: deltas are merged in memory, written, then the plan is archived
- models/revision -> embedding -> search: the recorded model revision, or `unknown` when none is recorded, is compared with the stored one and written into the index
- specs/ scenarios + existing index file -> search -> embedding -> index file: a scenario whose text has a stored vector in a reusable index keeps that vector, the other scenario texts are embedded, and the index is written with postcard to a temporary file that replaces the index file by rename
- query -> embedding -> search -> stdout: the query vector is ranked by cosine similarity against the index, the index is built first if it is missing or outdated
- record success -> search: main rebuilds the index after a recorded plan validates, embedding only new or changed scenario texts

## Interfaces

- CLI: `speq domain|feature|plan|record|search|decision-log`, output on stdout, errors exit non-zero
- Spec format: Gherkin-like Markdown in `specs/<domain>/<feature>/spec.md`
- Delta markers: `<!-- DELTA:NEW|CHANGED|REMOVED -->` blocks in plan spec files
- Cache directory: `SPEQ_CACHE_DIR`, else the platform cache directory under `speq`, else `.cache/speq` in the working directory
- Model files: `models/model.onnx` and `models/tokenizer.json` in the cache directory, which the installer downloads from a pinned model revision, and `models/revision`, a text file in which the installer records that revision
- Index file: `indexes/<project slug>.idx` in the cache directory, postcard binary holding the index format, the model revision, and each scenario's path, embedded text, and vector

## Constraints

- Minimal dependencies to keep maintenance low
- Single binary with no runtime services or databases
- Inference is pure Rust with tract-onnx, no native runtime
- Model inference runs fully offline once the model files are cached
- Search parsing runs in parallel with rayon. Embedding runs one text per model call, at most 8 model calls in parallel with rayon, and returns the vectors in input order
- An index build assumes at least 1 GB of memory. The memory of model inference does not grow with the number of scenarios
- An index build reuses stored vectors only from an index that decodes and whose index format and model revision equal the current ones. Any other index is rebuilt in full
- A build replaces the index file by rename, so a reader sees either the complete old file or the complete new one

## External Dependencies

- HuggingFace (model download): the installer downloads the embedding model files from a pinned model revision into the cache, and downloads them again when the revision recorded in the cache differs from the pin | failure impact: on a cache without model files, search is unavailable until the model is provisioned, and the binary reports an error naming the missing files and the cache directory. A failed download on a cache that holds model files leaves those files in place
