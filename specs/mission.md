# Mission: speq

> A CLI tool to get, list, search, and validate feature specifications for spec-driven development with AI coding agents.

## Problem Statement

AI coding agents work more effectively with structured, consistent specifications. Without tooling, feature specs:
- Lack structural consistency (missing sections, malformed scenarios)
- Cannot be verified programmatically
- Are difficult to discover and navigate across large codebases

speq provides structural verification and exploration tools so AI agents can reliably read and update specs that follow a predictable format.

## Target Users

| Persona | Goal | Key Workflow |
|---------|------|--------------|
| AI coding agent | Read/write consistent specs during development | Plan → Implement → Record: create delta specs, implement features, record to permanent specs |
| Developer | Investigate and maintain specs | List specs, validate structure, review changes |

## Core Capabilities

1. **Validate spec structure** — Check specs for required sections (Feature, Background, Scenarios), RFC2119 keyword usage, and scenario format
2. **List features** — Display all specs in a tree view organized by domain, optionally filtered
3. **Record plan deltas** — Merge approved delta specs from `_plans/` into permanent specs and archive the plan

## Out of Scope

- Execution engine — Does not run or execute specifications like a test runner
- Code generation — Does not generate implementation code from specs
- Version control — Does not manage spec history beyond archiving recorded plans

## Domain Glossary

| Term | Definition |
|------|------------|
| Spec | A feature specification file (`spec.md`) written in Gherkin-like Markdown |
| Domain | A grouping of related features (e.g., `cli/`, `validation/`) |
| Feature | A single capability defined with Background and Scenarios sections |
| Scenario | A specific behavior described with GIVEN/WHEN/THEN steps |
| Delta | A proposed change to a spec marked with `<!-- DELTA:NEW -->`, `<!-- DELTA:CHANGED -->`, or `<!-- DELTA:REMOVED -->` |
| Plan | A set of deltas in `_plans/<plan-name>/` awaiting approval |
| Record | The action of moving approved deltas from `_plans/` to permanent specs in `specs/` |

---

## Tech Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| Language | Rust (edition 2024) | Performance, reliability, single binary distribution |
| CLI | clap 4.5 | Command-line argument parsing with derive macros |
| Parsing | pulldown-cmark 0.13 | Markdown parsing for spec analysis |
| Errors | thiserror 2 | Ergonomic error type definitions |
| Testing | assert_cmd, predicates, tempfile | CLI integration testing |

## Commands

```bash
# Build
cargo build --release

# Test
cargo test

# Lint & Format
cargo fmt && cargo clippy

# Gather Code Coverage
cargo tarpaulin
```

## Project Structure

```
speq-skill/
├── src/                  # Application source
│   ├── main.rs           # Entry point and command handlers
│   ├── cli.rs            # CLI structure (clap derive)
│   ├── validate/         # Spec validation (parser, rules, report)
│   ├── feature.rs        # Feature discovery and listing
│   ├── tree.rs           # Tree view formatting
│   └── record.rs         # Delta recording logic
├── specs/                # Feature specifications
│   ├── <domain>/         # Domain grouping
│   │   └── <feature>/    # Feature directory
│   │       └── spec.md   # Specification file
│   ├── _plans/           # Pending plan deltas
│   └── _recorded/        # Archived recorded plans
└── tests/                # Integration tests
```

## Architecture

Simple modular CLI organized by feature. Each module handles a distinct capability:

- `cli` — Command definitions and argument parsing
- `validate` — Markdown parsing and structural validation rules
- `feature` — Spec discovery and listing
- `tree` — Tree view output formatting
- `record` — Delta merging and plan archiving

Data flows from CLI arguments → module handlers → formatted output.

## Constraints

- **Minimal dependencies**: Keep external crate count low for maintainability
- **No runtime dependencies**: Pure CLI tool, no services or databases required
- **Spec format stability**: Spec structure must remain backward compatible to avoid breaking existing specs

## External Dependencies

| Service | Purpose | Failure Impact |
|---------|---------|----------------|
| None | Pure CLI tool | No external dependencies |
