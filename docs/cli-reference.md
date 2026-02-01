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
| `plan` | Validate implementation plans |
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
4. Archives plan to `specs/_recorded/<plan-name>/`

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

## Spec Format

Specs use Gherkin-like Markdown with RFC 2119 keywords.

### Example Spec

```markdown
# Feature: User Login

The system SHALL provide a secure login mechanism for registered users.

## Background

* The system has a registered user with email "user@example.com"
* The user is not currently authenticated

## Scenarios

### Scenario: Successful login

* *GIVEN* valid credentials are provided
* *WHEN* the user submits the login form
* *THEN* the system SHALL authenticate the user
* *AND* the system SHALL redirect to the dashboard

### Scenario: Invalid password

* *GIVEN* an incorrect password is provided
* *WHEN* the user submits the login form
* *THEN* the system MUST reject the authentication
* *AND* the system MUST display an error message
```

### RFC 2119 Keywords

Use these keywords in `*THEN*` steps to specify requirements:

| Keyword | Meaning |
|---------|---------|
| `MUST` | Absolute requirement |
| `MUST NOT` | Absolute prohibition |
| `SHALL` | Same as MUST |
| `SHALL NOT` | Same as MUST NOT |
| `SHOULD` | Recommended |
| `SHOULD NOT` | Not recommended |
| `MAY` | Optional |

### Step Formatting

Steps use bold keywords with asterisks:

```markdown
* *GIVEN* <precondition>
* *WHEN* <action>
* *THEN* <expected result>
* *AND* <additional condition>
```

---

## Directory Structure

```
specs/
├── mission.md                    # Project purpose, tech stack, commands
├── <domain>/
│   └── <feature>/
│       └── spec.md               # Permanent specs
├── _plans/
│   └── <plan-name>/
│       ├── plan.md               # Implementation plan
│       ├── tasks.md              # Task tracking
│       ├── verification-report.md
│       └── <domain>/<feature>/spec.md  # Delta specs
└── _recorded/
    └── <plan-name>/              # Archived completed plans
```

### Directories

| Directory | Purpose |
|-----------|---------|
| `specs/<domain>/<feature>/` | Permanent feature specs |
| `specs/_plans/` | Active plans with delta specs |
| `specs/_recorded/` | Archived completed plans |

### Files

| File | Purpose |
|------|---------|
| `mission.md` | Project overview, tech stack, commands |
| `spec.md` | Feature specification |
| `plan.md` | Implementation plan with delta references |
| `tasks.md` | Task tracking during implementation |
| `verification-report.md` | Implementation verification evidence |
