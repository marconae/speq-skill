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
- search (src/search.rs): builds and queries the semantic index over scenarios, resolves cache paths | owns: index file and cache path resolution | depends on: feature, validate, embedding
- embedding (src/embedding.rs): loads the ONNX model and tokenizer and produces 384-dimension L2-normalized vectors | owns: loaded model and tokenizer | depends on: none

## Data Flow

- arguments -> cli -> main -> module handler -> stdout: a command runs and prints its result, exit code 1 on failure
- specs/<domain>/<feature>/spec.md -> feature -> validate -> main: discovery feeds parsing and rule checks
- specs/_plans/<plan>/ -> plan -> validate: delta files are checked for marker balance, anchors, and structure
- specs/_plans/<plan>/ -> record -> specs/ and specs/_recorded: deltas are merged in memory, written, then the plan is archived
- specs/ scenarios -> search -> embedding -> index file: scenario text is embedded and stored with postcard
- query -> embedding -> search -> stdout: the query vector is ranked by cosine similarity against the index, the index is built first if missing
- record success -> search: main rebuilds the index after a recorded plan validates

## Interfaces

- CLI: `speq domain|feature|plan|record|search|decision-log`, output on stdout, errors exit non-zero
- Spec format: Gherkin-like Markdown in `specs/<domain>/<feature>/spec.md`
- Delta markers: `<!-- DELTA:NEW|CHANGED|REMOVED -->` blocks in plan spec files
- Cache directory: `SPEQ_CACHE_DIR`, else the platform cache directory under `speq`, else `.cache/speq` in the working directory
- Model files: `models/model.onnx` and `models/tokenizer.json` in the cache directory
- Index file: `indexes/<project slug>.idx` in the cache directory, postcard binary

## Constraints

- Minimal dependencies to keep maintenance low
- Single binary with no runtime services or databases
- Inference is pure Rust with tract-onnx, no native runtime
- Model inference runs fully offline once the model files are cached
- Search parsing runs in parallel with rayon, embedding runs as one batch

## External Dependencies

- HuggingFace (model download): the installer downloads the embedding model files once into the cache | failure impact: search is unavailable until the model is provisioned, and the binary reports an error naming the missing files and the cache directory
