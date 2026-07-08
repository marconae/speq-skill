[speq-skill](../README.md) / [Docs](./index.md) / CLI Reference

---

# CLI Reference

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

## Domain Commands

### `speq domain list`

List all domains in the spec library.

```bash
speq domain list
```

Output:
```
cli
validation
search
```

---

## Feature Commands

### `speq feature list`

Display all features in a tree view.

```bash
# All features
speq feature list

# Features in a specific domain
speq feature list <domain>
```

Output:
```
specs/
├── cli/
│   ├── validate/
│   └── record/
├── validation/
│   └── keyword-casing/
└── search/
    └── semantic/
```

### `speq feature get`

Get full feature spec or a single scenario.

```bash
# Full feature spec
speq feature get <domain>/<feature>

# Single scenario (quote if name has spaces)
speq feature get "<domain>/<feature>/<scenario-name>"
```

Examples:
```bash
speq feature get cli/validate
speq feature get "cli/validate/Validation fails on missing field"
```

### `speq feature validate`

Validate spec structure and syntax.

```bash
# Validate all specs
speq feature validate

# Validate a domain
speq feature validate <domain>

# Validate a single feature
speq feature validate <domain>/<feature>
```

Validation checks:
- Required sections (Feature, Background, Scenarios)
- RFC 2119 keyword usage
- Scenario step formatting
- DELTA marker syntax

---

## Plan Commands

### `speq plan list`

List all active plans in `specs/_plans/`.

```bash
speq plan list
```

Output:
```
add-auth
fix-validation
```

Plans are listed alphabetically, one per line. Prints "No active plans." if none exist.

### `speq plan validate`

Validate a plan directory structure and contents.

```bash
speq plan validate <plan-name>
```

Validates:
- Plan directory exists (`specs/_plans/<plan-name>/`)
- `plan.md` is present
- Delta markers properly formatted
- Spec syntax is valid
- `decision-log.md` structure, if the file is present (absence is not an error)

---

## Decision Log Commands

### `speq decision-log validate`

Validate every fragment under `specs/_decision/`.

```bash
speq decision-log validate
```

Validates:
- Each fragment's H1 is `# Decisions: <plan-name>`
- ADR headings follow `## ADR: <Title>`
- Each ADR contains all required fields: `**ID:**`, `**Plan:**`, `**Status:**`, `### Context`, `### Decision`
- `**ID:**` is a kebab-case slug, unique across every fragment
- `**Status:**` is one of: `Accepted`, `Deprecated`, `Superseded by <slug>`
- Every `**Supersedes:**` and `Superseded by <slug>` reference resolves to a slug defined somewhere in `specs/_decision/`

`### Options Considered` and `### Consequences` are optional and do not trigger errors when absent. An absent or empty `specs/_decision/` directory passes.

### `speq decision-log show`

Assemble every fragment under `specs/_decision/` into one `# Architecture Decision Records` view and print it to stdout.

```bash
speq decision-log show
```

Orders fragments by their numeric `NNN-` prefix, breaking ties by filename; ADRs within a fragment print in the order they were authored. Writes nothing to disk — there is no merged file to keep in sync.

See [Decision Log](./decision-log.md) for the full format reference.

---

## Record Command

### `speq record`

Merge approved plan deltas into permanent specs.

```bash
speq record <plan-name>
```

This command:
1. Reads delta specs from `specs/_plans/<plan-name>/`
2. Merges deltas into permanent specs in `specs/<domain>/<feature>/`
3. Strips DELTA markers
4. Archives plan to `specs/_recorded/NNN-<plan-name>/`, where `NNN` is a record-time sequence number

---

## Search Commands

### `speq search index`

Build or rebuild the semantic search index.

```bash
speq search index
```

The index is built automatically on first search. Use this command to manually rebuild after spec changes.

### `speq search query`

Semantic search across all scenarios.

```bash
speq search query "<query>"
speq search query "<query>" --limit <n>
```

Examples:
```bash
speq search query "error handling"
speq search query "validation" --limit 5
```

Output includes:
- Feature path
- Scenario name
- Relevance score
- Matching context

---

## Spec Format and Library Structure

See [Spec Library](./spec-library.md) for the full spec format reference, including BDD/Gherkin structure, RFC 2119 keywords, and step formatting rules.
