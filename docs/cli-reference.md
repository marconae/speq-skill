[speq-skill](../README.md) / [Docs](./index.md) / CLI Reference

---

# CLI reference

The `speq` CLI provides spec discovery, semantic search, and validation for spec-driven development.

---

## Overview

```bash
speq <command> [subcommand] [options]
```

| Command | Purpose |
|---------|---------|
| `domain` | List and explore spec domains |
| `feature` | List, get, and validate feature specs |
| `plan` | List and validate implementation plans |
| `decision-log` | Validate and show the permanent decision log |
| `record` | Merge plan deltas into permanent specs |
| `search` | Semantic search across scenarios |

---

## Domain commands

### `speq domain list`

This command lists all domains in the spec library.

```bash
speq domain list
```

**Example**
```bash
$ speq domain list
cli/
core/
...
```

---

## Feature commands

### `speq feature list`

This command displays all features in a tree view.

```bash
speq feature list [domain]
```

**Example**
```bash
$ speq feature list
├── cli/
│   ├── domain-list
│   ├── feature-get
│   ...  
├── core/
│   └── spec-hierarchy
├── decision/
│   ├── decision-log-show
│   ...  
├── plan/
│   ├── plan-list
│   ├── plan-validate
│   ...  
└── validation/
    ├── document-structure
    │   ...  
```

### `speq feature get`

This command prints a full feature spec: the description, the Background, and the scenarios. If you append a scenario name to the path, the command prints only that scenario. Single-quote a scenario path. In double quotes, the shell runs the text between backticks as a command.

```bash
speq feature get <domain>/<feature>
speq feature get '<domain>/<feature>/<scenario-name>'
```

**Example**
```bash
$ speq feature get cli/feature-validate
$ speq feature get 'cli/feature-validate/Validate single feature'
```

### `speq feature validate`

This command validates spec structure and syntax. If you omit the target, the command validates every spec. If you name a domain or a single feature, the command validates only that target.

```bash
speq feature validate [domain]
speq feature validate [domain/feature]
```

**Example**
```bash
$ speq feature validate validation
✓ validation/keyword-casing (0 errors, 0 warnings)
```

Checks:
- Required sections (Feature, Background, Scenarios)
- RFC 2119 keyword usage
- Scenario step formatting
- DELTA marker syntax

---

## Plan commands

### `speq plan list`

This command lists all active plans in `specs/_plans/`, in alphabetical order, one plan per line.

```bash
speq plan list
```

**Example**
```bash
$ speq plan list
add-auth
fix-validation
```

If no plans exist, the command prints `No active plans.`

### `speq plan validate`

This command validates the structure and contents of a plan directory.

```bash
speq plan validate <plan-name>
```

**Example**
```bash
$ speq plan validate add-auth
✓ add-auth (0 errors, 0 warnings)
```

Checks:
- Plan directory exists (`specs/_plans/<plan-name>/`)
- `plan.md` is present
- Delta markers are properly formatted
- Spec syntax is valid
- `decision-log.md` structure, if the file is present (absence is not an error)

---

## Decision log commands

### `speq decision-log validate`

This command validates every fragment under `specs/_decision/`.

```bash
speq decision-log validate
```

See [Decision Log](./decision-log.md) for the full format reference.

**Example**
```bash
$ speq decision-log validate
Permanent decision log validation passed.
```

### `speq decision-log show`

This command assembles every fragment under `specs/_decision/` into one `# Architecture Decision Records` view. It prints the view to stdout. It writes nothing to disk.

```bash
speq decision-log show
```

**Example**
```bash
$ speq decision-log show
# Architecture Decision Records

## ADR: Adopt pure-Rust inference to remove the ONNX Runtime dependency

**ID:** pure-rust-inference
**Plan:** refactor-search-pure-rust-inference
**Status:** Accepted
...
```

The command orders fragments by their numeric `NNN-` prefix and order within files.

See [Decision Log](./decision-log.md) for the full format reference.

---

## Record command

### `speq record`

This command merges approved plan deltas into permanent specs.

```bash
speq record <plan-name>
```

**Example**
```bash
$ speq record add-auth
Recorded plan 'add-auth' to specs/_recorded/
```

This command:
1. Reads delta specs from `specs/_plans/<plan-name>/`
2. Merges deltas into permanent specs in `specs/<domain>/<feature>/`
3. Strips DELTA markers
4. Archives the plan to `specs/_recorded/NNN-<plan-name>/`, where `NNN` is a record-time sequence number

---

## Search commands

### `speq search index`

This command builds or rebuilds the semantic search index. The index builds automatically on the first search. Run this command to rebuild the index manually after spec changes.

```bash
speq search index
```

**Example**
```bash
$ speq search index
Building search index...
Indexed 42 scenarios.
```

`Indexed N scenarios.` reports the total number of scenarios in the index. The command exits with code 1 when the `specs/` directory does not exist.

A rebuild embeds only new and changed scenarios. Each unchanged scenario keeps the vector that the index stores for it. speq rebuilds an outdated index in full.

### `speq search query`

This command runs a semantic search across all scenarios. It covers scenario names and steps. When no index exists, the command builds the index first and prints `Info: no search index found. Building it now, this may take a while.` on stderr. When the index is outdated, the command rebuilds it first and prints `Info: search index is outdated. Rebuilding it now, this may take a while.` on stderr. [`speq search index`](#speq-search-index) describes when an index is outdated.

```bash
speq search query "<query>" [--limit <n>]
```

**Example**
```bash
$ speq search query "validation" --limit 3
cli/feature-validate/Validate single feature (score: 0.856)
  Validate single feature

cli/feature-validate/Summary at end (score: 0.853)
  Summary at end
```

Each result includes the feature path, scenario name, relevance score, and matching context.

---

## Spec format and library structure

See [Spec Library](./spec-library.md) for the full spec format reference. It covers BDD/Gherkin structure, RFC 2119 keywords, and step formatting rules.
